//! Bounded path discovery and explicit trace declarations, separate from heuristic roles.
//! defines: main.improvement.34.discovery
//! defines: main.improvement.34.declarations
//! defines: main.improvement.34.revision_dependencies

use crate::parser::{HierarchicalName, HierarchyPattern};
use crate::search::{has_binary_extension, is_binary_sample, SearchOptions};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

#[path = "trace_inspection.rs"]
mod inspection;
pub use inspection::{inspect, InspectionOptions};

pub const DEFAULT_ENTRIES: usize = 50_000;
pub const DEFAULT_BYTES: usize = 32 * 1024 * 1024;

#[derive(Default, Serialize)]
struct Coverage {
    complete: bool,
    entries: usize,
    files_read: usize,
    bytes_read: usize,
    max_entries: usize,
    max_bytes: usize,
    omissions: BTreeMap<&'static str, usize>,
}

impl Coverage {
    fn omit(&mut self, reason: &'static str) {
        *self.omissions.entry(reason).or_default() += 1;
        self.complete = false;
    }
}

/// All data reads (including binary samples) spend the same remaining allowance.
/// No look-ahead byte is read beyond it. Callers compare observed length and
/// file metadata to detect a growing/truncated input without escaping the cap.
fn read_budgeted(reader: &mut impl Read, remaining: &mut usize, maximum: usize) -> (Vec<u8>, bool) {
    let limit = (*remaining).min(maximum);
    let mut bytes = Vec::new();
    let ok = if limit == 0 {
        true
    } else {
        reader.take(limit as u64).read_to_end(&mut bytes).is_ok()
    };
    *remaining -= bytes.len();
    (bytes, ok)
}

fn visible(entry: &walkdir::DirEntry) -> bool {
    if entry.depth() == 0 {
        return true;
    }
    let name = entry.file_name().to_string_lossy();
    !name.starts_with('.')
        && !(entry.file_type().is_dir()
            && ["target", "target2", "node_modules", "build", "dist"]
                .iter()
                .any(|excluded| name.eq_ignore_ascii_case(excluded)))
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn scan(
    options: &SearchOptions,
    scope: Option<&HierarchyPattern>,
    markdown_only: bool,
    max_entries: usize,
    max_bytes: usize,
    mut visit: impl FnMut(String, &str),
) -> Result<Coverage> {
    ensure!(
        (1..=250_000).contains(&max_entries),
        "--max-entries must be in 1..=250000"
    );
    ensure!(
        (1..=128 * 1024 * 1024).contains(&max_bytes),
        "--max-bytes must be in 1..=134217728"
    );
    let root = options.root.canonicalize()?;
    ensure!(root.is_dir(), "-d must be a directory");
    let mut coverage = Coverage {
        complete: true,
        max_entries,
        max_bytes,
        ..Default::default()
    };
    let mut walker = WalkDir::new(&root)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter();
    walker.next(); // The canonical root itself is not a candidate.
    while let Some(entry) = walker.next() {
        if coverage.entries == max_entries {
            coverage.omit("entry_budget");
            break;
        }
        coverage.entries += 1;
        let Ok(entry) = entry else {
            coverage.omit("unreadable_entry");
            continue;
        };
        if !visible(&entry) {
            if entry.file_type().is_dir() {
                walker.skip_current_dir();
            }
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let Some(name) = entry.file_name().to_str() else {
            coverage.omit("invalid_path_encoding");
            continue;
        };
        if !options.extensions.is_empty()
            && !options.extensions.iter().any(|ext| name.ends_with(ext))
        {
            continue;
        }
        if markdown_only && !name.to_ascii_lowercase().ends_with(".md") {
            continue;
        }
        if let Some(scope) = scope {
            let stem = name.rsplit_once('.').map_or(name, |(stem, _)| stem);
            if !scope.matches(&HierarchicalName::with_separator(stem, scope.separator)) {
                continue;
            }
        }
        if options.skip_binary_files && has_binary_extension(path) {
            continue;
        }
        let mut remaining = max_bytes - coverage.bytes_read;
        let limit = options
            .max_file_bytes
            .unwrap_or(DEFAULT_BYTES)
            .min(remaining);
        let file = match path.canonicalize() {
            Ok(resolved) if resolved.starts_with(&root) => File::open(resolved),
            _ => {
                coverage.omit("unavailable_or_escaping_file");
                continue;
            }
        };
        let Ok(mut file) = file else {
            coverage.omit("unreadable_file");
            continue;
        };
        let Ok(metadata) = file.metadata() else {
            coverage.omit("unreadable_file");
            continue;
        };
        if metadata.len() > limit as u64 {
            coverage.omit(if remaining == limit {
                "byte_budget"
            } else {
                "file_byte_limit"
            });
            continue;
        }
        let (mut bytes, sample_ok) = read_budgeted(&mut file, &mut remaining, limit.min(1024));
        coverage.bytes_read = max_bytes - remaining;
        if !sample_ok {
            coverage.omit("unreadable_file");
            continue;
        }
        if options.skip_binary_files && is_binary_sample(&bytes) {
            continue;
        }
        let (tail, read_ok) = read_budgeted(&mut file, &mut remaining, limit - bytes.len());
        bytes.extend(tail);
        coverage.bytes_read = max_bytes - remaining;
        let stable = file.metadata().is_ok_and(|after| {
            after.len() == metadata.len() && after.modified().ok() == metadata.modified().ok()
        });
        if !read_ok || bytes.len() as u64 != metadata.len() || !stable {
            coverage.omit("unreadable_or_growing_file");
            continue;
        }
        let Ok(text) = std::str::from_utf8(&bytes) else {
            coverage.omit("invalid_utf8");
            continue;
        };
        coverage.files_read += 1;
        visit(relative(&root, path), text);
    }
    Ok(coverage)
}

fn matches(pattern: &HierarchyPattern, id: &str) -> bool {
    pattern.matches(&HierarchicalName::with_separator(id, pattern.separator))
}

fn token_ids(text: &str, separator: char) -> impl Iterator<Item = &str> {
    text.split(move |c: char| {
        !(c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == separator)
    })
    // Sentence-ending periods are punctuation for every hierarchy separator.
    // Do not trim '_' or '-' suffixes into a shorter valid identifier.
    .map(|token| token.trim_end_matches('.'))
    .filter(move |id| id.contains(separator))
}

pub fn hunt(
    options: &SearchOptions,
    pattern: &HierarchyPattern,
    entries: usize,
    bytes: usize,
) -> Result<Value> {
    let mut files = BTreeSet::new();
    let mut folders = BTreeSet::new();
    let coverage = scan(options, None, false, entries, bytes, |path, text| {
        let found = token_ids(text, pattern.separator).any(|id| matches(pattern, id));
        if found {
            folders.insert(
                path.rsplit_once('/')
                    .map_or(".", |(folder, _)| folder)
                    .to_owned(),
            );
            files.insert(path);
        }
    })?;
    Ok(
        json!({"schema":"recur-trace-id-paths-v1","folders":folders,"files":files,"coverage":coverage}),
    )
}

#[derive(Clone, Debug, Serialize)]
struct Span {
    path: String,
    line: usize,
    start_column: usize,
    end_column: usize,
}

#[derive(Clone, Serialize)]
struct Declaration {
    role: String,
    id: String,
    span: Span,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Artifact {
    id: String,
    context: String,
    revision: String,
    path: String,
    sha256: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Pin {
    id: String,
    context: String,
    expected: String,
    actual: String,
}

#[derive(Default)]
struct Records {
    roles: Vec<Declaration>,
    artifacts: Vec<(Artifact, Span)>,
    pins: Vec<(Pin, Span)>,
    diagnostics: Vec<Value>,
}

fn atom(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn valid_id(id: &str) -> bool {
    id.len() <= 256 && id.contains('.') && id.split('.').all(atom)
}

fn valid_digest(digest: &str) -> bool {
    digest.len() == 64
        && digest
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

impl Records {
    fn parse(&mut self, path: &str, text: &str) {
        let mut lines = text.trim_start_matches('\u{feff}').lines().enumerate();
        if lines.next().map(|(_, line)| line) != Some("<!-- recur-trace-v1") {
            return;
        }
        let mut body = Vec::new();
        for (index, line) in lines {
            if line == "-->" {
                for (index, line) in body {
                    self.parse_line(path, index, line);
                }
                return;
            }
            body.push((index, line));
        }
        self.diagnostics.push(json!({"code":"unterminated_metadata","span":{"path":path,"line":1,"start_column":1,"end_column":19}}));
    }

    fn parse_line(&mut self, path: &str, index: usize, line: &str) {
        let span = Span {
            path: path.into(),
            line: index + 1,
            start_column: 1,
            end_column: line.len() + 1,
        };
        if line.trim().is_empty() {
            return;
        }
        let (role, value) = line
            .split_once(':')
            .map_or(("", line), |(role, value)| (role, value.trim()));
        let code = match role {
            "defines" | "publish" | "consumer" | "trigger" => {
                if valid_id(value) {
                    self.roles.push(Declaration {
                        role: role.into(),
                        id: value.into(),
                        span,
                    });
                    return;
                }
                "invalid_identifier"
            }
            "artifact" => {
                if let Ok(record) = serde_json::from_str::<Artifact>(value) {
                    if valid_id(&record.id)
                        && atom(&record.revision)
                        && atom(&record.context)
                        && valid_digest(&record.sha256)
                    {
                        self.artifacts.push((record, span));
                        return;
                    }
                }
                "invalid_artifact"
            }
            "requires" => {
                if let Ok(record) = serde_json::from_str::<Pin>(value) {
                    if valid_id(&record.id)
                        && atom(&record.expected)
                        && atom(&record.actual)
                        && atom(&record.context)
                    {
                        self.pins.push((record, span));
                        return;
                    }
                }
                "invalid_pin"
            }
            _ => "unknown_record",
        };
        self.diagnostics
            .push(json!({"code":code,"token":value,"span":span}));
    }
}

fn digest(root: &Path, path: &str, remaining: &mut usize) -> Option<String> {
    let mut file = File::open(crate::warp_evidence::contained_file(root, path).ok()?).ok()?;
    let limit = (*remaining).min(4 * 1024 * 1024);
    let before = file.metadata().ok()?;
    if before.len() > limit as u64 {
        return None;
    }
    let (bytes, ok) = read_budgeted(&mut file, remaining, limit);
    let after = file.metadata().ok()?;
    if !ok
        || bytes.len() as u64 != before.len()
        || after.len() != before.len()
        || after.modified().ok() != before.modified().ok()
    {
        return None;
    }
    Some(format!("{:x}", Sha256::digest(bytes)))
}

pub fn declared(
    options: &SearchOptions,
    pattern: &HierarchyPattern,
    scope: &HierarchyPattern,
    entries: usize,
    bytes: usize,
    unique_defines: bool,
) -> Result<Value> {
    let mut records = Records::default();
    let mut coverage = scan(options, Some(scope), true, entries, bytes, |path, text| {
        records.parse(&path, text)
    })?;
    let scope_complete = coverage.complete;
    let roles = records
        .roles
        .into_iter()
        .filter(|r| matches(pattern, &r.id))
        .collect::<Vec<_>>();
    if unique_defines {
        let mut by_id = BTreeMap::<&str, Vec<&Span>>::new();
        for role in roles.iter().filter(|r| r.role == "defines") {
            by_id.entry(&role.id).or_default().push(&role.span);
        }
        for (id, sites) in by_id.into_iter().filter(|(_, sites)| sites.len() > 1) {
            records
                .diagnostics
                .push(json!({"code":"duplicate_authority","id":id,"sites":sites}));
        }
    }
    let mut by_revision = BTreeMap::<(&str, &str, &str), Vec<(&Artifact, &Span, &str)>>::new();
    let mut remaining = bytes - coverage.bytes_read;
    let mut observed = BTreeMap::<&str, Option<String>>::new();
    let mut artifacts = Vec::new();
    for (artifact, source) in records
        .artifacts
        .iter()
        .filter(|(a, _)| matches(pattern, &a.id))
    {
        let actual = observed
            .entry(&artifact.path)
            .or_insert_with(|| digest(&options.root, &artifact.path, &mut remaining));
        let integrity = match actual.as_deref() {
            Some(value) if value == artifact.sha256 => "matched",
            Some(_) => "mismatch",
            None => "unknown",
        };
        if integrity == "unknown" {
            coverage.omit("unavailable_artifact_digest");
        }
        by_revision
            .entry((&artifact.id, &artifact.context, &artifact.revision))
            .or_default()
            .push((artifact, source, integrity));
        artifacts.push(json!({"artifact":artifact,"source":source,"integrity":integrity,"observed_sha256":actual}));
    }
    for ((id, context, revision), definitions) in &by_revision {
        let variants = definitions
            .iter()
            .map(|(a, _, _)| a.sha256.as_str())
            .collect::<BTreeSet<_>>();
        if variants.len() > 1 {
            let sites = definitions.iter().map(|(_, s, _)| s).collect::<Vec<_>>();
            records.diagnostics.push(json!({"code":"conflicting_revision","id":id,"context":context,"revision":revision,"sites":sites}));
        }
    }
    let pins = records.pins.iter().filter(|(p, _)| matches(pattern,&p.id)).map(|(pin,source)| {
        let resolved = by_revision.get(&(pin.id.as_str(),pin.context.as_str(),pin.actual.as_str()));
        let (revision, integrity, reason) = match resolved.map(Vec::as_slice) {
            Some([(_,_,integrity)]) if !scope_complete => ("unknown",*integrity,"incomplete_scope"),
            Some([(_,_,"unknown")]) => ("unknown","unknown","unavailable_artifact"),
            Some([(_,_,integrity)]) if pin.actual == pin.expected => ("matched",*integrity,"explicit_pin"),
            Some([(_,_,integrity)]) => ("mismatch",*integrity,"wrong_revision"),
            Some(_) => ("unknown","unknown","ambiguous_artifact"),
            None => ("unknown","unknown","missing_in_scope"),
        };
        json!({"pin":pin,"source":source,"revision":revision,"integrity":integrity,"reason":reason})
    }).collect::<Vec<_>>();
    coverage.bytes_read = bytes - remaining;
    Ok(
        json!({"schema":"recur-trace-id-declarations-v1","declarations":roles,"artifacts":artifacts,
        "pins":pins,"diagnostics":records.diagnostics,"coverage":coverage}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    struct CountingReader {
        data: Cursor<Vec<u8>>,
        calls: usize,
        bytes: usize,
    }
    impl Read for CountingReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.calls += 1;
            let n = self.data.read(buf)?;
            self.bytes += n;
            Ok(n)
        }
    }

    #[test]
    fn samples_and_tail_share_one_physical_read_budget() {
        let mut reader = CountingReader {
            data: Cursor::new(vec![b'x'; 4096]),
            calls: 0,
            bytes: 0,
        };
        let mut remaining = 1100;
        let (sample, ok) = read_budgeted(&mut reader, &mut remaining, 1024);
        assert!(ok);
        let (tail, ok) = read_budgeted(&mut reader, &mut remaining, 4096);
        assert!(ok);
        assert_eq!(
            (sample.len(), tail.len(), reader.bytes, remaining),
            (1024, 76, 1100, 0)
        );
        let calls = reader.calls;
        assert!(read_budgeted(&mut reader, &mut remaining, 1).0.is_empty());
        assert_eq!(
            reader.calls, calls,
            "exhausted budget must not touch the reader"
        );
    }

    #[test]
    fn growing_input_cannot_force_a_lookahead_byte() {
        let mut reader = CountingReader {
            data: Cursor::new(vec![b'x'; 4096]),
            calls: 0,
            bytes: 0,
        };
        let mut remaining = 1;
        assert_eq!(read_budgeted(&mut reader, &mut remaining, 4096).0.len(), 1);
        assert_eq!(reader.bytes, 1);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn partial_errors_still_spend_the_bytes_read() {
        struct Broken(bool);
        impl Read for Broken {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                if self.0 {
                    return Err(io::Error::other("fixture interruption"));
                }
                self.0 = true;
                buf[0] = b'x';
                Ok(1)
            }
        }
        let mut remaining = 4;
        let (bytes, ok) = read_budgeted(&mut Broken(false), &mut remaining, 4);
        assert!(!ok);
        assert_eq!(bytes, b"x");
        assert_eq!(remaining, 3);
    }

    #[test]
    fn binary_samples_are_counted_and_can_exhaust_the_scan() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.dat"), [0; 1024]).unwrap();
        std::fs::write(dir.path().join("b.md"), "capability.baseline").unwrap();
        let options = SearchOptions {
            root: dir.path().into(),
            ..Default::default()
        };
        let report = hunt(
            &options,
            &HierarchyPattern::parse("capability.baseline").unwrap(),
            10,
            1024,
        )
        .unwrap();
        assert_eq!(report["coverage"]["bytes_read"], 1024);
        assert_eq!(report["coverage"]["complete"], false);
        assert_eq!(report["coverage"]["omissions"]["byte_budget"], 1);
        assert_eq!(report["files"], json!([]));
    }

    #[test]
    fn excluded_binary_extension_does_not_spend_data_budget() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.exe"), [0; 1024]).unwrap();
        let options = SearchOptions {
            root: dir.path().into(),
            ..Default::default()
        };
        let report = hunt(
            &options,
            &HierarchyPattern::parse("capability.baseline").unwrap(),
            10,
            1,
        )
        .unwrap();
        assert_eq!(report["coverage"]["bytes_read"], 0);
        assert_eq!(report["coverage"]["complete"], true);
    }

    #[test]
    fn punctuation_never_trims_identifier_suffixes() {
        for separator in ['.', '_', '-'] {
            let id = format!("capability{separator}baseline");
            let text = format!("{id}. {id}_extra {id}-extra {id}{separator}child");
            let tokens = token_ids(&text, separator).collect::<Vec<_>>();
            assert_eq!(tokens.len(), 4);
            assert_eq!(tokens[0], id);
            let pattern = HierarchyPattern::parse_with_separator(&id, separator).unwrap();
            assert_eq!(
                tokens
                    .iter()
                    .filter(|token| matches(&pattern, token))
                    .count(),
                1
            );
        }
    }

    #[test]
    fn digest_obeys_remaining_allowance_and_root() {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join("root");
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join("artifact.txt"), b"data").unwrap();
        std::fs::write(parent.path().join("outside.txt"), b"data").unwrap();
        let mut remaining = 3;
        assert_eq!(digest(&root, "artifact.txt", &mut remaining), None);
        assert_eq!(remaining, 3);
        remaining = 4;
        assert_eq!(
            digest(&root, "artifact.txt", &mut remaining),
            Some(format!("{:x}", Sha256::digest(b"data")))
        );
        assert_eq!(remaining, 0);
        assert_eq!(digest(&root, "../outside.txt", &mut remaining), None);
        assert_eq!(remaining, 0);
    }
}
