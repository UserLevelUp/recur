//! Opinionated polling coordinator; each pass reuses companion claims/readiness.
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path, process::Command, thread, time::Duration};

pub fn coordinate(root: &Path, warp: &str, confirm: bool, cycles: u64) -> Result<()> {
    let root = root.canonicalize()?;
    let config = root
        .ancestors()
        .map(|p| p.join(".recur/config.toml"))
        .find(|p| p.is_file())
        .context("coordinator requires project configuration")?;
    let settings: toml::Value = toml::from_str(&fs::read_to_string(config)?)?;
    let poll = match settings
        .get("watch")
        .and_then(|w| w.get("dispatch"))
        .and_then(|d| d.get("poll_seconds"))
    {
        Some(value) => value
            .as_integer()
            .context("watch.dispatch.poll_seconds must be an integer")?,
        None => 2,
    };
    ensure!(
        (1..=3600).contains(&poll),
        "coordinator poll_seconds must be 1 through 3600"
    );
    let actor = std::env::current_exe()?.with_file_name(if cfg!(windows) {
        "recur-warp.exe"
    } else {
        "recur-warp"
    });
    ensure!(
        actor.is_file(),
        "coordinator requires sibling recur-warp binary"
    );
    let status_dir = root.join(".recur/watch");
    let status = status_dir.join(format!(
        "recur-watch.dispatch-{}.status.current.md",
        recur::warp_dispatch::key(warp)
    ));
    if confirm {
        fs::create_dir_all(&status_dir)?;
        ensure!(
            status_dir.canonicalize()?.starts_with(&root),
            "watch status directory outside root"
        );
    }
    let mut pass = 0;
    loop {
        pass += 1;
        let mut cmd = Command::new(&actor);
        cmd.args([
            "dispatch",
            warp,
            "-d",
            root.to_str().context("non UTF8 root")?,
            "--json",
        ]);
        if confirm {
            cmd.arg("--confirm");
        }
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000);
        }
        let output = cmd.output()?;
        ensure!(
            output.status.success(),
            "scheduler failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let dispatch: Value = serde_json::from_slice(&output.stdout)?;
        let recorded = recur::warp_dispatch::inspect(&root, warp)?;
        let active = recorded["attempts"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| matches!(a["state"].as_str(), Some("claimed" | "running")))
            .count();
        let retry_limit = settings
            .get("warp")
            .and_then(|w| w.get("dispatch"))
            .and_then(|d| d.get("max_attempts"))
            .and_then(|n| n.as_integer())
            .unwrap_or(0) as u64;
        // consumer: main.command.warp.dispatch.retry.transient keep polling delayed retries
        let waiting = dispatch["skipped"].as_array().unwrap().iter().any(|s| {
            s["retry_after_seconds"]
                .as_u64()
                .is_some_and(|delay| delay > 0)
                // A worker may finish after dispatch skipped it as active.
                // Recheck that transition once instead of falsely going idle.
                || (s["reason"] == "active, awaiting acceptance, or attempts exhausted"
                    && recorded["attempts"].as_array().unwrap().iter()
                        .filter(|a| a["slice_id"] == s["slice_id"])
                        .max_by_key(|a| a["attempt"].as_u64().unwrap_or(0))
                        .is_some_and(|a| matches!(a["state"].as_str(), Some("host_failed" | "test_failed" | "verification_error" | "verification_stale"))
                            && a["failure"]["retryable"] != false
                            && a["attempt"].as_u64().unwrap_or(u64::MAX) < retry_limit))
        });
        let done = !confirm
            || (active == 0 && !waiting && dispatch["launched"].as_array().unwrap().is_empty())
            || cycles > 0 && pass >= cycles;
        if confirm {
            let body = format!("watch.id = dispatch-{}\nstate = {}\nack = accepted\nmode = dispatch\nfilter = worker.**\nwarp.id = {warp}\npasses = {pass}\nactive = {active}\n",recur::warp_dispatch::key(warp),if done {"stopped"} else {"active"});
            let mut file = tempfile::NamedTempFile::new_in(&status_dir)?;
            file.write_all(body.as_bytes())?;
            file.persist(&status)
                .map_err(|e| anyhow::anyhow!("watch status publication: {e}"))?;
        }
        println!(
            "{}",
            serde_json::to_string(
                &json!({"schema":"warp-coordinator-pass-v1","warp_id":warp,"pass":pass,"active":active,"state":if done {"quiescent"} else {"coordinating"},"dispatch":dispatch})
            )?
        );
        if done {
            break;
        }
        thread::sleep(Duration::from_secs(poll as u64));
    }
    Ok(())
}
