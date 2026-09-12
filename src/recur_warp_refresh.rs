//! Confirmed companion writer for immutable evidence refresh records.
use std::path::{Path, PathBuf};
#[derive(clap::Args)]
pub struct Args {
    pub map: PathBuf,
    pub layer: PathBuf,
    #[arg(long)]
    pub gate: String,
    #[arg(long)]
    pub from: String,
    #[arg(long)]
    pub to: String,
    #[arg(long)]
    pub refresh_id: String,
    #[arg(long)]
    pub reason: String,
}
pub fn run(root: &Path, args: &Args, confirm: bool) -> anyhow::Result<serde_json::Value> {
    use recur::warp_refresh::{inventory, plan, relative, safe};
    let root = root.canonicalize()?;
    let make = || {
        plan(
            &root,
            relative(&args.map)?,
            relative(&args.layer)?,
            &args.gate,
            &args.from,
            &args.to,
            &args.refresh_id,
            &args.reason,
        )
    };
    let initial = make()?;
    if confirm {
        let fresh = make()?;
        anyhow::ensure!(
            initial.record == fresh.record
                && initial.reads.hashes == fresh.reads.hashes
                && initial.inventory == fresh.inventory,
            "refresh changed before publication"
        );
        fresh.reads.verify(&root)?;
        let target = safe(&root, &fresh.target)?;
        let parent = target
            .parent()
            .ok_or_else(|| anyhow::anyhow!("missing receipt parent"))?;
        let dir = parent
            .strip_prefix(&root)?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("non UTF8 path"))?
            .replace('\\', "/");
        anyhow::ensure!(
            inventory(&root, &dir)? == fresh.inventory,
            "refresh inventory changed before publication"
        );
        std::fs::create_dir_all(parent)?;
        publish(
            &root,
            &fresh.target,
            &serde_json::to_vec_pretty(&fresh.record)?,
            |_| Ok(()),
        )?;
    }
    Ok(
        serde_json::json!({"schema":"recur-warp-refresh-v1","state":if !confirm{"planned"}else if initial.state=="idempotent"{"idempotent"}else{"written"},"path":initial.target,"record":initial.record,"execution":"not-run"}),
    )
}
fn publish(
    root: &Path,
    name: &str,
    bytes: &[u8],
    mut hook: impl FnMut(&str) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    use recur::warp_refresh::{safe, Reads};
    use std::{
        fs::{self, OpenOptions},
        io::Write,
    };
    let target = safe(root, name)?;
    let tmp_name = format!("{name}.tmp");
    let tmp = safe(root, &tmp_name)?;
    if target.try_exists()? {
        anyhow::ensure!(
            Reads::default().read(root, name, true)? == bytes,
            "conflicting existing receipt: {name}"
        );
        if tmp.try_exists()? {
            anyhow::ensure!(
                Reads::default().read(root, &tmp_name, true)? == bytes,
                "conflicting staging: {tmp_name}"
            );
            fs::remove_file(tmp)?;
        }
        return Ok(());
    }
    if tmp.try_exists()? {
        anyhow::ensure!(
            Reads::default().read(root, &tmp_name, true)? == bytes,
            "torn/conflicting staging: {tmp_name}"
        );
        OpenOptions::new().write(true).open(&tmp)?.sync_all()?;
    } else {
        let mut file = OpenOptions::new().write(true).create_new(true).open(&tmp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }
    hook("staged")?;
    fs::hard_link(&tmp, &target)?;
    hook("published")?;
    fs::remove_file(tmp)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interruption_and_matching_retry() {
        for stage in ["staged", "published"] {
            let d = tempfile::tempdir().unwrap();
            let r = d.path();
            assert!(publish(r, "r.json", b"receipt", |s| {
                if s == stage {
                    anyhow::bail!("injected")
                }
                Ok(())
            })
            .is_err());
            assert_eq!(r.join("r.json").exists(), stage == "published");
            publish(r, "r.json", b"receipt", |_| Ok(())).unwrap();
            assert_eq!(std::fs::read(r.join("r.json")).unwrap(), b"receipt");
            assert!(!r.join("r.json.tmp").exists());
            assert!(publish(r, "r.json", b"conflict", |_| Ok(())).is_err());
        }
    }
}
