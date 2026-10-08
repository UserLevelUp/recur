//! Pure inspection of companion-owned dispatch records. Never starts processes.
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn key(id: &str) -> String {
    crate::warp_evidence::fingerprint(id.as_bytes()).replace(':', "-")
}
pub fn directory(root: &Path, warp: &str) -> PathBuf {
    root.join(".recur/dispatch").join(key(warp))
}
pub fn inspect(root: &Path, warp: &str) -> Result<Value> {
    let root = root.canonicalize()?;
    let dir = directory(&root, warp);
    let mut attempts = vec![];
    if dir.exists() {
        ensure!(
            dir.canonicalize()?.starts_with(&root),
            "dispatch records escape root"
        );
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            ensure!(attempts.len() < 4096, "too many dispatch records");
            ensure!(
                !fs::symlink_metadata(&path)?.file_type().is_symlink(),
                "symlink dispatch record"
            );
            ensure!(
                fs::metadata(&path)?.len() <= 1_048_576,
                "dispatch record exceeds 1 MiB"
            );
            let value: Value =
                serde_json::from_slice(&fs::read(&path)?).context("invalid dispatch record")?;
            ensure!(
                value["schema"] == "warp-dispatch-attempt-v1" && value["warp_id"] == warp,
                "dispatch record identity mismatch"
            );
            attempts.push(value);
        }
    }
    attempts.sort_by_key(|v| {
        (
            v["slice_id"].as_str().unwrap_or("").to_owned(),
            v["attempt"].as_u64().unwrap_or(0),
        )
    });
    Ok(json!({"schema":"warp-dispatch-view-v1", "warp_id":warp, "attempts":attempts}))
}
