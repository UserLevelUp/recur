//! Durable Eventness topic state owned by the `recur-watch` companion.
//! Core queries call only `inspect`; registration and reconciliation never execute artifacts.
//! defines: main.command.watch.eventness.native durable signatures over persisted artifacts
use crate::{
    parser::{HierarchicalName, HierarchyPattern},
    r#trait::resolve_trace_id_policy,
};
use anyhow::{ensure, Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Component, Path, PathBuf},
};

const STATE: &str = ".recur/watch/topics";
const MAX_ENTRIES: usize = 10_000;
const MAX_FILE: u64 = 16 * 1024 * 1024;
const MAX_SCAN: u64 = 64 * 1024 * 1024;
const MAX_JOURNAL: u64 = 64 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Binding {
    schema: String,
    topic: String,
    root: PathBuf,
    eventness_dir: PathBuf,
    filter: String,
    producer_keywords: Vec<String>,
    warp: Option<String>,
    warp_uuid: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Notification {
    trace_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Transaction {
    schema: String,
    binding: String,
    subscriber: String,
    sequence: u64,
    observations: Vec<String>,
    notifications: Vec<Notification>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    checksum: String,
    transaction: Transaction,
}

struct Journal {
    transactions: Vec<Transaction>,
    seen: BTreeSet<String>,
    valid_bytes: u64,
    torn_tail: bool,
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn valid_id(id: &str) -> bool {
    id.len() <= 512
        && id.contains('.')
        && id
            .split('.')
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
}
fn identity(id: &str) -> Result<()> {
    ensure!(
        !id.is_empty() && id.len() <= 512 && !id.chars().any(char::is_control),
        "identity must be nonempty, at most 512 bytes, without control characters"
    );
    Ok(())
}
fn relative(path: &Path) -> Result<()> {
    ensure!(
        !path.as_os_str().is_empty()
            && path
                .components()
                .all(|p| matches!(p, Component::Normal(_) | Component::CurDir)),
        "path must be relative without traversal"
    );
    Ok(())
}
fn private(root: &Path, path: &Path) -> bool {
    within(path, &root.join(".recur/watch"))
}
fn within(path: &Path, base: &Path) -> bool {
    #[cfg(windows)]
    {
        let mut path = path.components();
        base.components().all(|component| {
            path.next()
                .map(|next| {
                    next.as_os_str()
                        .to_string_lossy()
                        .eq_ignore_ascii_case(&component.as_os_str().to_string_lossy())
                })
                .unwrap_or(false)
        })
    }
    #[cfg(not(windows))]
    {
        path.starts_with(base)
    }
}
fn contained(root: &Path, path: &Path) -> Result<PathBuf> {
    relative(path)?;
    let resolved = root
        .join(path)
        .canonicalize()
        .with_context(|| format!("missing artifact '{}'", path.display()))?;
    ensure!(
        within(&resolved, root) && !private(root, &resolved),
        "artifact escapes root or enters Watch private state"
    );
    Ok(resolved)
}

// Refuse private-state symlinks even when they point inside the project. Check
// every existing parent before creating anything, including on read-only paths.
fn state_directory(root: &Path, topic: &str, create: bool) -> Result<PathBuf> {
    identity(topic)?;
    let dir = root.join(STATE).join(hash(topic.as_bytes()));
    let mut current = root.to_path_buf();
    for component in dir.strip_prefix(root)?.components() {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(meta) => {
                ensure!(
                    !meta.file_type().is_symlink() && current.canonicalize()?.starts_with(root),
                    "Watch private state must not contain escaping links"
                );
                ensure!(
                    meta.is_dir(),
                    "Watch private state parent is not a directory"
                );
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if create {
                    fs::create_dir(&current)?;
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(dir)
}
fn state_file(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(meta) => ensure!(
            meta.is_file() && !meta.file_type().is_symlink(),
            "Watch state file must be a regular file"
        ),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    Ok(())
}
fn read_bounded(path: &Path, budget: &mut u64) -> Result<Vec<u8>> {
    let file = File::open(path)?;
    ensure!(
        file.metadata()?.is_file() && file.metadata()?.len() <= MAX_FILE,
        "artifact is not a bounded regular file: {}",
        path.display()
    );
    let mut bytes = Vec::new();
    file.take(MAX_FILE + 1).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 <= MAX_FILE,
        "artifact exceeded byte limit"
    );
    *budget += bytes.len() as u64;
    ensure!(*budget <= MAX_SCAN, "Eventness scan exceeded byte budget");
    Ok(bytes)
}
fn publish_new(path: &Path, bytes: &[u8]) -> Result<()> {
    state_file(path)?;
    let mut file = tempfile::NamedTempFile::new_in(path.parent().context("missing state parent")?)?;
    file.write_all(bytes)?;
    file.as_file().sync_all()?;
    match file.persist_noclobber(path) {
        Ok(_) => Ok(()),
        Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(error.error.into()),
    }
}
fn binding_hash(binding: &Binding) -> Result<String> {
    Ok(hash(&serde_json::to_vec(binding)?))
}
fn load(root: &Path, topic: &str) -> Result<(PathBuf, Binding)> {
    let root = root.canonicalize()?;
    let dir = state_directory(&root, topic, false)?;
    let path = dir.join("topic.json");
    state_file(&path)?;
    let binding: Binding = serde_json::from_slice(&read_bounded(&path, &mut 0)?)?;
    ensure!(
        binding.schema == "watch-eventness-topic-v1"
            && binding.topic == topic
            && binding.root == root,
        "topic binding does not match root/identity/schema"
    );
    validate_scope(&root, &binding.eventness_dir)?;
    Ok((dir, binding))
}
fn validate_scope(root: &Path, relative_dir: &Path) -> Result<PathBuf> {
    let dir = contained(root, relative_dir)?;
    ensure!(
        dir.is_dir() && !within(&root.join(".recur/watch"), &dir),
        "Eventness scope must be a directory disjoint from Watch private state"
    );
    Ok(dir)
}

pub fn create(
    root: &Path,
    topic: &str,
    warp: Option<&str>,
    filter: &str,
    eventness_dir: &Path,
    confirm: bool,
) -> Result<Value> {
    identity(topic)?;
    ensure!(
        !filter.is_empty() && filter.len() <= 512 && !filter.chars().any(char::is_control),
        "filter must be a bounded hierarchy pattern"
    );
    HierarchyPattern::parse(filter)?;
    let root = root.canonicalize()?;
    let scope = validate_scope(&root, eventness_dir)?;
    let warp_uuid = if let Some(warp) = warp {
        let progress = crate::warp_query::bubble_progress(&root, warp)?;
        Some(
            progress["bubble_uuid"]
                .as_str()
                .context("bound Warp requires a stable bubble UUID")?
                .to_string(),
        )
    } else {
        None
    };
    let binding = Binding {
        schema: "watch-eventness-topic-v1".into(),
        topic: topic.into(),
        root: root.clone(),
        eventness_dir: scope.strip_prefix(&root)?.into(),
        filter: filter.into(),
        producer_keywords: resolve_trace_id_policy(&root)?.producer_keywords,
        warp: warp.map(str::to_owned),
        warp_uuid,
    };
    let dir = state_directory(&root, topic, false)?;
    let path = dir.join("topic.json");
    state_file(&path)?;
    if path.exists() {
        ensure!(
            load(&root, topic)?.1 == binding,
            "existing topic has conflicting bindings"
        );
    }
    if confirm {
        let dir = state_directory(&root, topic, true)?;
        publish_new(
            &dir.join("topic.json"),
            &serde_json::to_vec_pretty(&binding)?,
        )?;
        ensure!(
            load(&root, topic)?.1 == binding,
            "concurrent topic has conflicting bindings"
        );
    }
    Ok(json!({"operation":"create","confirmed":confirm,"binding":binding}))
}
fn journal_path(dir: &Path, subscriber: &str) -> Result<PathBuf> {
    identity(subscriber)?;
    let path = dir.join(format!(
        "subscription-{}.jsonl",
        hash(subscriber.as_bytes())
    ));
    state_file(&path)?;
    Ok(path)
}
fn transaction_line(transaction: Transaction) -> Result<Vec<u8>> {
    let checksum = hash(&serde_json::to_vec(&transaction)?);
    let mut bytes = serde_json::to_vec(&Envelope {
        checksum,
        transaction,
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn read_journal(path: &Path, binding: &Binding, subscriber: &str) -> Result<Journal> {
    state_file(path)?;
    let file = File::open(path).context("subscriber not registered; run topic subscribe first")?;
    ensure!(
        file.metadata()?.len() <= MAX_JOURNAL,
        "subscription journal exceeded byte budget; use a new subscriber identity"
    );
    let mut bytes = Vec::new();
    file.take(MAX_JOURNAL + 1).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 <= MAX_JOURNAL,
        "subscription journal exceeded byte budget"
    );
    let expected_binding = binding_hash(binding)?;
    let mut journal = Journal {
        transactions: Vec::new(),
        seen: BTreeSet::new(),
        valid_bytes: 0,
        torn_tail: false,
    };
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        if line.last() != Some(&b'\n') {
            journal.torn_tail = true;
            break;
        }
        let envelope: Envelope =
            serde_json::from_slice(line).context("corrupt committed cursor journal")?;
        let tx = envelope.transaction;
        ensure!(
            envelope.checksum == hash(&serde_json::to_vec(&tx)?),
            "cursor journal checksum mismatch"
        );
        ensure!(
            tx.schema == "watch-eventness-cursor-v1"
                && tx.binding == expected_binding
                && tx.subscriber == subscriber
                && tx.sequence == journal.transactions.len() as u64,
            "cursor journal binding or sequence mismatch"
        );
        ensure!(
            tx.observations.len() == tx.notifications.len(),
            "cursor observations/notifications mismatch"
        );
        ensure!(
            (tx.sequence == 0 && tx.observations.is_empty())
                || (tx.sequence > 0 && !tx.observations.is_empty()),
            "invalid cursor transaction"
        );
        for notification in &tx.notifications {
            ensure!(
                !notification.trace_ids.is_empty()
                    && notification.trace_ids.iter().all(|id| valid_id(id))
                    && notification.trace_ids.iter().collect::<BTreeSet<_>>().len()
                        == notification.trace_ids.len(),
                "invalid cursor notification"
            );
        }
        for observation in &tx.observations {
            ensure!(
                journal.seen.insert(observation.clone()),
                "duplicate cursor observation"
            );
        }
        journal.valid_bytes += line.len() as u64;
        journal.transactions.push(tx);
    }
    ensure!(
        !journal.transactions.is_empty(),
        "missing committed subscriber header"
    );
    Ok(journal)
}
fn lock(dir: &Path, subscriber: &str) -> Result<File> {
    let path = dir.join(format!("subscription-{}.lock", hash(subscriber.as_bytes())));
    state_file(&path)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    file.try_lock_exclusive()
        .context("subscriber is busy; retry after the active drain finishes")?;
    Ok(file)
}
fn read_lock(dir: &Path, subscriber: &str) -> Result<File> {
    let path = dir.join(format!("subscription-{}.lock", hash(subscriber.as_bytes())));
    state_file(&path)?;
    let file = File::open(path).context("subscriber lock not found")?;
    file.try_lock_shared()
        .context("subscriber is committing a batch; retry when it finishes")?;
    Ok(file)
}
pub fn subscribe(root: &Path, topic: &str, subscriber: &str, confirm: bool) -> Result<Value> {
    let (dir, binding) = load(root, topic)?;
    let path = journal_path(&dir, subscriber)?;
    let _lock = if confirm {
        Some(lock(&dir, subscriber)?)
    } else if path.exists() {
        Some(read_lock(&dir, subscriber)?)
    } else {
        None
    };
    if confirm {
        if !path.exists() {
            publish_new(
                &path,
                &transaction_line(Transaction {
                    schema: "watch-eventness-cursor-v1".into(),
                    binding: binding_hash(&binding)?,
                    subscriber: subscriber.into(),
                    sequence: 0,
                    observations: vec![],
                    notifications: vec![],
                })?,
            )?;
        }
    }
    let committed_batches = if path.exists() {
        read_journal(&path, &binding, subscriber)?
            .transactions
            .len()
            - 1
    } else {
        0
    };
    Ok(
        json!({"operation":"subscribe","confirmed":confirm,"topic":topic,"subscriber":subscriber,"committed_batches":committed_batches}),
    )
}

fn field<'a>(text: &'a str, name: &str) -> Result<Option<&'a str>> {
    let mut found = None;
    for line in text.lines() {
        if let Some((key, value)) = line.trim().split_once('=') {
            if key.trim() == name {
                ensure!(found.is_none(), "duplicate Eventness field '{name}'");
                found = Some(value.trim());
            }
        }
    }
    Ok(found)
}
fn unquote(value: &str) -> &str {
    value.trim_matches(['\"', '\''])
}
fn declarations(text: &str, binding: &Binding) -> Result<Vec<String>> {
    let pattern = HierarchyPattern::parse(&binding.filter)?;
    let mut ids = BTreeSet::new();
    for line in text.lines() {
        let line = line
            .trim()
            .trim_start_matches("//")
            .trim()
            .trim_start_matches('#')
            .trim();
        let Some((role, value)) = line.split_once(':') else {
            continue;
        };
        if !binding
            .producer_keywords
            .iter()
            .any(|word| word.eq_ignore_ascii_case(role.trim()))
        {
            continue;
        }
        let token = value
            .split_whitespace()
            .next()
            .context("producer declaration requires a trace ID")?;
        for id in token.split(',') {
            ensure!(valid_id(id), "invalid producer trace ID '{id}'");
            if pattern.matches(&HierarchicalName::new(id)) {
                ids.insert(id.into());
            }
        }
    }
    Ok(ids.into_iter().collect())
}
fn scan(root: &Path, binding: &Binding) -> Result<Vec<(String, Notification)>> {
    let scope = validate_scope(root, &binding.eventness_dir)?;
    let mut files = Vec::new();
    for (n, entry) in walkdir::WalkDir::new(&scope).into_iter().enumerate() {
        ensure!(n < MAX_ENTRIES, "Eventness scan exceeded entry budget");
        let entry = entry?;
        if entry.file_type().is_file() || entry.file_type().is_symlink() {
            let resolved = entry.path().canonicalize()?;
            ensure!(
                within(&resolved, root) && !private(root, &resolved),
                "Eventness artifact link escapes scope"
            );
            ensure!(
                !resolved.is_dir(),
                "linked Eventness directories are unsupported; bind that directory explicitly"
            );
            files.push(entry.into_path());
        }
    }
    files.sort();
    let mut budget = 0;
    let mut publications = Vec::new();
    for path in files {
        let bytes = read_bounded(&path, &mut budget)?;
        let Ok(text) = std::str::from_utf8(&bytes) else {
            continue;
        };
        let ids = declarations(text, binding)?;
        if ids.is_empty() {
            continue;
        }
        let warp = field(text, "warp.id")?.map(unquote);
        let uuid = field(text, "warp.uuid")?.map(unquote);
        if binding.warp.is_some() {
            ensure!(
                !(warp == binding.warp.as_deref()
                    && uuid.is_some()
                    && uuid != binding.warp_uuid.as_deref())
                    && !(uuid == binding.warp_uuid.as_deref()
                        && warp.is_some()
                        && warp != binding.warp.as_deref()),
                "conflicting publication Warp identity/UUID"
            );
            if (warp.is_some() && warp != binding.warp.as_deref())
                || (uuid.is_some() && uuid != binding.warp_uuid.as_deref())
            {
                continue;
            }
        }
        let refs: Vec<String> = match field(text, "artifact.refs")? {
            Some(value) => serde_json::from_str(value)
                .context("artifact.refs must be an array of root-relative paths")?,
            None => vec![],
        };
        ensure!(refs.len() <= 64, "publication reference limit exceeded");
        let mut fingerprint = Sha256::new();
        // Length-prefixed path/bytes prevent ambiguous multi-file fingerprints.
        fn add(hash: &mut Sha256, bytes: &[u8]) {
            hash.update((bytes.len() as u64).to_le_bytes());
            hash.update(bytes);
        }
        add(
            &mut fingerprint,
            path.strip_prefix(root)?
                .to_str()
                .context("producer path must be valid UTF-8 to preserve publication identity")?
                .as_bytes(),
        );
        add(&mut fingerprint, &bytes);
        let mut read_refs = Vec::new();
        for reference in refs {
            let resolved = contained(root, Path::new(&reference))?;
            let artifact = read_bounded(&resolved, &mut budget)?;
            add(&mut fingerprint, reference.as_bytes());
            add(&mut fingerprint, &artifact);
            read_refs.push((reference, artifact));
        }
        // Refuse a publication that changes while the snapshot is assembled.
        ensure!(
            read_bounded(&path, &mut budget)? == bytes,
            "producer changed during reconciliation; retry after publication completes"
        );
        for (reference, artifact) in read_refs {
            ensure!(
                read_bounded(&contained(root, Path::new(&reference))?, &mut budget)? == artifact,
                "referenced artifact changed during reconciliation; retry"
            );
        }
        publications.push((
            format!("{:x}", fingerprint.finalize()),
            Notification { trace_ids: ids },
        ));
    }
    Ok(publications)
}

#[cfg(feature = "watch-test-hooks")]
fn fault(point: &str) -> Result<()> {
    if std::env::var("RECUR_WATCH_TEST_FAULT").ok().as_deref() == Some(point) {
        std::process::exit(86);
    }
    Ok(())
}
#[cfg(not(feature = "watch-test-hooks"))]
fn fault(_: &str) -> Result<()> {
    Ok(())
}

trait CursorStore: Write + Seek {
    fn sync_cursor(&self) -> std::io::Result<()>;
    fn truncate_cursor(&self, len: u64) -> std::io::Result<()>;
}
impl CursorStore for File {
    fn sync_cursor(&self) -> std::io::Result<()> {
        self.sync_all()
    }
    fn truncate_cursor(&self, len: u64) -> std::io::Result<()> {
        self.set_len(len)
    }
}
fn append_commit(
    file: &mut impl CursorStore,
    valid_bytes: u64,
    bytes: &[u8],
    inject_write_failure: bool,
) -> Result<()> {
    file.seek(SeekFrom::End(0))?;
    let append = if inject_write_failure {
        // Isolated test build: exercise the real partial-write rollback path.
        file.write_all(&bytes[..bytes.len() / 2]).and_then(|_| {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "injected partial cursor write failure",
            ))
        })
    } else {
        file.write_all(bytes).and_then(|_| file.sync_cursor())
    };
    if let Err(error) = append {
        file.truncate_cursor(valid_bytes).and_then(|_| file.sync_cursor()).context("cursor persistence failed and rollback could not be synced; inspect journal before retrying")?;
        return Err(error).context("cursor persistence failed; no batch committed");
    }
    Ok(())
}

pub fn drain(
    root: &Path,
    topic: &str,
    subscriber: &str,
    max_events: usize,
    confirm: bool,
) -> Result<Value> {
    ensure!(
        (1..=1000).contains(&max_events),
        "max-events must be between 1 and 1000"
    );
    let (dir, binding) = load(root, topic)?;
    let path = journal_path(&dir, subscriber)?;
    let _lock = if confirm {
        Some(lock(&dir, subscriber)?)
    } else {
        Some(read_lock(&dir, subscriber)?)
    };
    let journal = read_journal(&path, &binding, subscriber)?;
    // Validate the entire bounded scope before advancing any observations.
    let pending: Vec<_> = scan(&binding.root, &binding)?
        .into_iter()
        .filter(|(key, _)| !journal.seen.contains(key))
        .take(max_events)
        .collect();
    if !confirm {
        return Ok(
            json!({"operation":"drain","confirmed":false,"topic":topic,"subscriber":subscriber,"pending_notifications":pending.len(),"next_sequence":journal.transactions.len()}),
        );
    }
    let mut file = OpenOptions::new().read(true).write(true).open(&path)?;
    if journal.torn_tail {
        file.set_len(journal.valid_bytes)?;
        file.sync_all()?;
    }
    if pending.is_empty() {
        return Ok(json!([]));
    }
    let transaction = Transaction {
        schema: "watch-eventness-cursor-v1".into(),
        binding: binding_hash(&binding)?,
        subscriber: subscriber.into(),
        sequence: journal.transactions.len() as u64,
        observations: pending.iter().map(|(key, _)| key.clone()).collect(),
        notifications: pending.into_iter().map(|(_, n)| n).collect(),
    };
    let notifications = serde_json::to_value(&transaction.notifications)?;
    let bytes = transaction_line(transaction)?;
    ensure!(
        journal.valid_bytes + bytes.len() as u64 <= MAX_JOURNAL,
        "cursor journal byte budget exceeded; use a new subscriber identity"
    );
    fault("before-commit")?;
    file.seek(SeekFrom::End(0))?;
    #[cfg(feature = "watch-test-hooks")]
    if std::env::var("RECUR_WATCH_TEST_FAULT").ok().as_deref() == Some("partial-commit") {
        file.write_all(&bytes[..bytes.len() / 2])?;
        file.sync_all()?;
        std::process::exit(86);
    }
    let inject_write_failure = cfg!(feature = "watch-test-hooks")
        && std::env::var("RECUR_WATCH_TEST_FAULT").ok().as_deref() == Some("write-failure");
    append_commit(&mut file, journal.valid_bytes, &bytes, inject_write_failure)?;
    fault("after-commit")?;
    Ok(notifications)
}

pub fn replay(root: &Path, topic: &str, subscriber: &str, sequence: u64) -> Result<Value> {
    ensure!(sequence > 0, "replay sequence must be positive");
    let (dir, binding) = load(root, topic)?;
    let _lock = read_lock(&dir, subscriber)?;
    let journal = read_journal(&journal_path(&dir, subscriber)?, &binding, subscriber)?;
    let batch = journal
        .transactions
        .iter()
        .find(|tx| tx.sequence == sequence)
        .context("committed batch sequence not found")?;
    Ok(serde_json::to_value(&batch.notifications)?)
}

/// Pure metadata query. Does not acquire locks, register interest or repair tails.
pub fn inspect(root: &Path) -> Result<Value> {
    let root = root.canonicalize()?;
    let base = root.join(STATE);
    // Validate the parent chain even when no topics have been created.
    state_directory(&root, "inspection", false)?;
    if !base.exists() {
        return Ok(json!([]));
    }
    let mut topics = Vec::new();
    for (n, entry) in fs::read_dir(&base)?.enumerate() {
        ensure!(n < MAX_ENTRIES, "topic inventory budget exceeded");
        let entry = entry?;
        ensure!(
            entry.file_type()?.is_dir(),
            "unexpected file/link in topic inventory"
        );
        let path = entry.path().join("topic.json");
        state_file(&path)?;
        if !path.exists() {
            continue;
        }
        let binding: Binding = serde_json::from_slice(&read_bounded(&path, &mut 0)?)?;
        ensure!(
            entry.file_name().to_string_lossy() == hash(binding.topic.as_bytes()),
            "topic storage identity mismatch"
        );
        load(&root, &binding.topic)?;
        topics.push(binding);
    }
    topics.sort_by(|a, b| a.topic.cmp(&b.topic));
    Ok(serde_json::to_value(topics)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Result<(tempfile::TempDir, PathBuf)> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().canonicalize()?;
        fs::create_dir(root.join("eventness"))?;
        create(
            &root,
            "results",
            None,
            "task.**",
            Path::new("eventness"),
            true,
        )?;
        subscribe(&root, "results", "coordinator", true)?;
        fs::write(
            root.join("eventness/result.md"),
            "publish: task.one.ready\nintelligence\n",
        )?;
        Ok((dir, root))
    }
    #[test]
    fn torn_append_is_recovered_without_consuming_publication() -> Result<()> {
        let (_dir, root) = fixture()?;
        let (dir, _) = load(&root, "results")?;
        let path = journal_path(&dir, "coordinator")?;
        OpenOptions::new()
            .append(true)
            .open(&path)?
            .write_all(b"{\"checksum\":\"torn")?;
        assert_eq!(
            drain(&root, "results", "coordinator", 10, true)?,
            json!([{"trace_ids":["task.one.ready"]}])
        );
        assert_eq!(drain(&root, "results", "coordinator", 10, true)?, json!([]));
        Ok(())
    }
    #[test]
    fn complete_corrupt_cursor_fails_closed() -> Result<()> {
        let (_dir, root) = fixture()?;
        let (dir, _) = load(&root, "results")?;
        let path = journal_path(&dir, "coordinator")?;
        OpenOptions::new()
            .append(true)
            .open(&path)?
            .write_all(b"{}\n")?;
        let before = fs::read(&path)?;
        assert!(drain(&root, "results", "coordinator", 10, true).is_err());
        assert_eq!(fs::read(path)?, before);
        Ok(())
    }
    #[test]
    fn locks_serialize_subscribers_and_release_on_drop() -> Result<()> {
        let (_dir, root) = fixture()?;
        drain(&root, "results", "coordinator", 10, true)?;
        fs::write(
            root.join("eventness/second.md"),
            "publish: task.two.ready\n",
        )?;
        let (dir, _) = load(&root, "results")?;
        let held = lock(&dir, "coordinator")?;
        assert!(drain(&root, "results", "coordinator", 10, true).is_err());
        assert!(replay(&root, "results", "coordinator", 1).is_err());
        assert!(subscribe(&root, "results", "coordinator", false).is_err());
        drop(held);
        assert_eq!(
            drain(&root, "results", "coordinator", 10, true)?
                .as_array()
                .unwrap()
                .len(),
            1
        );
        Ok(())
    }
    #[test]
    fn partial_write_and_sync_errors_rollback_real_file_bytes() -> Result<()> {
        use std::{cell::Cell, io};
        struct FailingStore {
            file: File,
            fail_sync: Cell<bool>,
        }
        impl Write for FailingStore {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                self.file.write(bytes)
            }
            fn flush(&mut self) -> io::Result<()> {
                self.file.flush()
            }
        }
        impl Seek for FailingStore {
            fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
                self.file.seek(position)
            }
        }
        impl CursorStore for FailingStore {
            fn sync_cursor(&self) -> io::Result<()> {
                if self.fail_sync.replace(false) {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        "injected sync failure",
                    ))
                } else {
                    self.file.sync_all()
                }
            }
            fn truncate_cursor(&self, len: u64) -> io::Result<()> {
                self.file.set_len(len)
            }
        }
        for fail_sync in [false, true] {
            let temp = tempfile::NamedTempFile::new()?;
            fs::write(temp.path(), b"committed\n")?;
            let mut store = FailingStore {
                file: OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(temp.path())?,
                fail_sync: Cell::new(fail_sync),
            };
            assert!(append_commit(&mut store, 10, b"new transaction\n", !fail_sync).is_err());
            assert_eq!(fs::read(temp.path())?, b"committed\n");
            append_commit(&mut store, 10, b"later transaction\n", false)?;
            assert_eq!(fs::read(temp.path())?, b"committed\nlater transaction\n");
        }
        Ok(())
    }
    #[test]
    fn invalid_publication_never_partially_advances() -> Result<()> {
        let (_dir, root) = fixture()?;
        fs::write(root.join("eventness/z.md"), "publish: task..bad\n")?;
        assert!(drain(&root, "results", "coordinator", 1, true).is_err());
        fs::remove_file(root.join("eventness/z.md"))?;
        assert_eq!(
            drain(&root, "results", "coordinator", 10, true)?
                .as_array()
                .unwrap()
                .len(),
            1
        );
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn non_utf8_producer_paths_fail_before_commit() -> Result<()> {
        use std::os::unix::ffi::OsStringExt;
        let (_dir, root) = fixture()?;
        for byte in [0xff, 0xfe] {
            let name = std::ffi::OsString::from_vec(vec![b'a', byte]);
            fs::write(
                root.join("eventness").join(name),
                "publish: task.two.ready\n",
            )?;
        }
        let (dir, _) = load(&root, "results")?;
        let path = journal_path(&dir, "coordinator")?;
        let before = fs::read(&path)?;
        assert!(drain(&root, "results", "coordinator", 10, true).is_err());
        assert_eq!(fs::read(path)?, before);
        Ok(())
    }
    #[cfg(windows)]
    #[test]
    fn windows_private_state_casing_cannot_bypass_scope_or_references() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().canonicalize()?;
        fs::create_dir_all(root.join(".Recur/Watch"))?;
        assert!(create(
            &root,
            "private",
            None,
            "task.**",
            Path::new(".Recur/Watch"),
            true
        )
        .is_err());
        assert!(create(&root, "parent", None, "task.**", Path::new(".Recur"), true).is_err());
        fs::create_dir(root.join("eventness"))?;
        fs::write(root.join(".Recur/Watch/private.txt"), "private")?;
        create(
            &root,
            "results",
            None,
            "task.**",
            Path::new("eventness"),
            true,
        )?;
        subscribe(&root, "results", "coordinator", true)?;
        fs::write(
            root.join("eventness/result.md"),
            "publish: task.one.ready\nartifact.refs = [\".Recur/Watch/private.txt\"]\n",
        )?;
        assert!(drain(&root, "results", "coordinator", 10, true).is_err());
        fs::write(
            root.join("eventness/result.md"),
            "publish: task.one.ready\n",
        )?;
        assert_eq!(
            drain(&root, "results", "coordinator", 10, true)?
                .as_array()
                .unwrap()
                .len(),
            1
        );
        Ok(())
    }
}
