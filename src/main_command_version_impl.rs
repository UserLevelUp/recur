//! Pure version query surface and `recur-version` companion implementation.
//!
//! This module maps to hierarchical name: main.command.version.impl

#![allow(dead_code)]

use anyhow::{bail, Context};
use clap::Subcommand;
use csv::StringRecord;
use recur::project_config;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::{DirEntry, WalkDir};

const VERSION_STATE_DIR: &str = ".recur/version";
const STATUS_PREFIX: &str = "recur-version.";
const STATUS_SUFFIX: &str = ".status.current.md";

#[derive(Subcommand)]
pub enum VersionQuerySubcommand {
    /// Show current artifact, manifest, latest version, and next version
    Status {
        /// Subject such as care.subject.routine, or a current artifact path
        subject: String,
    },

    /// Show the version manifest for a subject
    Manifest {
        /// Subject such as care.subject.routine, or a current artifact path
        subject: String,
    },

    /// Show configured versioning policy for a subject
    Policy {
        /// Subject such as care.subject.routine
        subject: String,
    },

    /// Show configured artifact/query schema for a subject
    Schema {
        /// Subject such as care.subject.routine
        subject: String,
    },

    /// Query preserved version history with evidence
    Query {
        /// Subject such as care.subject.routine
        subject: String,

        /// Natural-language question over configured version history
        #[arg(long)]
        question: String,
    },

    /// Explain the pure query / companion writer split
    Explain,
}

#[derive(Subcommand)]
pub enum VersionWriteSubcommand {
    /// Print the next version token for a current artifact
    Next {
        /// Current artifact path, usually <subject>.<lifecycle>.current.<ext>
        artifact: String,
    },

    /// Save a current artifact snapshot and update its manifest
    Save {
        /// Current artifact path, usually <subject>.<lifecycle>.current.<ext>
        artifact: String,

        /// Slug to include in the preserved version filename
        #[arg(long)]
        slug: String,

        /// Reason to write into the manifest entry
        #[arg(long)]
        reason: Option<String>,

        /// Operator or actor responsible for the save
        #[arg(long)]
        operator: Option<String>,

        /// Runtime id used for .recur/version status records
        #[arg(long)]
        id: Option<String>,
    },
}

#[derive(Debug, Clone)]
struct CurrentArtifact {
    path: PathBuf,
    subject: String,
    lifecycle: String,
    format: String,
}

#[derive(Debug, Clone)]
struct VersionFile {
    path: PathBuf,
    version: String,
    slug: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct ManifestInfo {
    path: PathBuf,
    latest_version: Option<String>,
    entries: Vec<ManifestEntry>,
}

#[derive(Debug, Clone)]
struct ManifestEntry {
    version: String,
    summary: String,
}

#[derive(Debug, Clone)]
struct ArtifactPolicy {
    configured: bool,
    subject: String,
    kind: Option<String>,
    format: Option<String>,
    risk_class: Option<String>,
    privacy_root: Option<String>,
    persona: Option<String>,
    strategy: String,
    manifest_required: Option<bool>,
    queryable: Option<bool>,
    operator_required_for: Vec<String>,
    identity_fields: Vec<String>,
    tracked_fields: Vec<String>,
    state_field: Option<String>,
    note_fields: Vec<String>,
    states: BTreeMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct VersionStatusOutput {
    subject: String,
    lifecycle: String,
    current_artifact: String,
    format: String,
    manifest: String,
    latest_version: Option<String>,
    next_version: String,
    policy_configured: bool,
    risk_class: Option<String>,
    privacy_root: Option<String>,
}

#[derive(Serialize)]
struct PolicyOutput {
    subject: String,
    configured: bool,
    kind: Option<String>,
    format: Option<String>,
    risk_class: Option<String>,
    privacy_root: Option<String>,
    persona: Option<String>,
    strategy: String,
    manifest_required: Option<bool>,
    queryable: Option<bool>,
    operator_required_for: Vec<String>,
}

#[derive(Serialize)]
struct SchemaOutput {
    subject: String,
    configured: bool,
    identity_fields: Vec<String>,
    tracked_fields: Vec<String>,
    state_field: Option<String>,
    note_fields: Vec<String>,
    states: BTreeMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct QueryOutput {
    subject: String,
    question: String,
    answer: String,
    evidence: Option<QueryEvidence>,
}

#[derive(Serialize)]
struct QueryEvidence {
    artifact: String,
    version: String,
    version_file: String,
    manifest_entry: Option<String>,
    changed_field: String,
    observed_state: String,
    lifecycle: String,
    identity: String,
}

#[derive(Serialize)]
struct VersionWriteOutput {
    artifact: String,
    subject: String,
    lifecycle: String,
    version: String,
    snapshot: Option<String>,
    manifest: Option<String>,
}

struct VersionStateWriter {
    id: String,
    path: PathBuf,
}

struct VersionStateRequest<'a> {
    command: &'a str,
    artifact: &'a str,
    subject: Option<&'a str>,
    lifecycle: Option<&'a str>,
    version: Option<&'a str>,
    snapshot: Option<&'a Path>,
    manifest: Option<&'a Path>,
    slug: Option<&'a str>,
    reason: Option<&'a str>,
    operator: Option<&'a str>,
}

pub fn execute(command: VersionQuerySubcommand, dir: PathBuf, json: bool) -> anyhow::Result<()> {
    let root = resolve_root(&dir)?;

    match command {
        VersionQuerySubcommand::Status { subject } => {
            let statuses = collect_statuses(&root, &subject)?;
            emit_statuses(&statuses, json)
        }
        VersionQuerySubcommand::Manifest { subject } => {
            let artifact = resolve_current_artifact(&root, &subject)?;
            let manifest = read_manifest_for(&root, &artifact)?;
            emit_manifest(&root, &manifest, json)
        }
        VersionQuerySubcommand::Policy { subject } => {
            let policy = load_artifact_policy(&root, &subject)?;
            emit_policy(&policy, json)
        }
        VersionQuerySubcommand::Schema { subject } => {
            let policy = load_artifact_policy(&root, &subject)?;
            emit_schema(&policy, json)
        }
        VersionQuerySubcommand::Query { subject, question } => {
            let output = query_history(&root, &subject, &question)?;
            emit_query(&output, json)
        }
        VersionQuerySubcommand::Explain => emit_explain(json),
    }
}

pub fn execute_write(
    command: VersionWriteSubcommand,
    dir: PathBuf,
    json: bool,
) -> anyhow::Result<()> {
    match command {
        VersionWriteSubcommand::Next { artifact } => {
            let root = resolve_root(&dir)?;
            let current = resolve_current_artifact(&root, &artifact)?;
            let manifest = read_manifest_for(&root, &current)?;
            let latest = latest_version(&root, &current, &manifest)?;
            let next = next_version(latest.as_deref());
            let output = VersionWriteOutput {
                artifact: relative_display_path(&root, &current.path),
                subject: current.subject,
                lifecycle: current.lifecycle,
                version: next,
                snapshot: None,
                manifest: Some(relative_display_path(&root, &manifest.path)),
            };
            emit_write_output(&output, json)
        }
        VersionWriteSubcommand::Save {
            artifact,
            slug,
            reason,
            operator,
            id,
        } => {
            let root = resolve_root(&dir)?;
            let status_id = id.unwrap_or_else(|| default_status_id(&artifact));
            let writer = VersionStateWriter::new(status_id, &root);
            let result = save_version(
                &root,
                &artifact,
                &slug,
                reason.as_deref(),
                operator.as_deref(),
            );

            match result {
                Ok(output) => {
                    let request = VersionStateRequest {
                        command: "save",
                        artifact: &artifact,
                        subject: Some(&output.subject),
                        lifecycle: Some(&output.lifecycle),
                        version: Some(&output.version),
                        snapshot: output.snapshot.as_deref().map(Path::new),
                        manifest: output.manifest.as_deref().map(Path::new),
                        slug: Some(&slug),
                        reason: reason.as_deref(),
                        operator: operator.as_deref(),
                    };
                    writer.write_accepted(&request)?;
                    emit_write_output(&output, json)
                }
                Err(error) => {
                    let request = VersionStateRequest {
                        command: "save",
                        artifact: &artifact,
                        subject: None,
                        lifecycle: None,
                        version: None,
                        snapshot: None,
                        manifest: None,
                        slug: Some(&slug),
                        reason: reason.as_deref(),
                        operator: operator.as_deref(),
                    };
                    let _ = writer.write_rejected(&request, &error.to_string());
                    Err(error)
                }
            }
        }
    }
}

fn collect_statuses(root: &Path, subject: &str) -> anyhow::Result<Vec<VersionStatusOutput>> {
    let artifacts = collect_current_artifacts(root, subject)?;
    if artifacts.is_empty() {
        bail!("no current artifact found for '{}'", subject);
    }

    let mut statuses = Vec::new();
    for artifact in artifacts {
        let manifest = read_manifest_for(root, &artifact)?;
        let latest = latest_version(root, &artifact, &manifest)?;
        let policy = load_artifact_policy(root, &artifact.subject)?;
        statuses.push(VersionStatusOutput {
            subject: artifact.subject.clone(),
            lifecycle: artifact.lifecycle.clone(),
            current_artifact: relative_display_path(root, &artifact.path),
            format: artifact.format.clone(),
            manifest: relative_display_path(root, &manifest.path),
            latest_version: latest.clone(),
            next_version: next_version(latest.as_deref()),
            policy_configured: policy.configured,
            risk_class: policy.risk_class,
            privacy_root: policy.privacy_root,
        });
    }

    statuses.sort_by(|left, right| {
        left.subject
            .cmp(&right.subject)
            .then_with(|| left.lifecycle.cmp(&right.lifecycle))
    });
    Ok(statuses)
}

fn save_version(
    root: &Path,
    artifact: &str,
    slug: &str,
    reason: Option<&str>,
    operator: Option<&str>,
) -> anyhow::Result<VersionWriteOutput> {
    let current = resolve_current_artifact(root, artifact)?;
    let manifest = read_manifest_for(root, &current)?;
    let latest = latest_version(root, &current, &manifest)?;
    let next = next_version(latest.as_deref());
    let slug = sanitize_slug(slug);
    if slug.is_empty() {
        bail!("--slug must contain at least one alphanumeric character");
    }

    let snapshot_name = format!(
        "{}.{}.version.{}.{}.{}",
        current.subject, current.lifecycle, next, slug, current.format
    );
    let snapshot_path = current.path.parent().unwrap_or(root).join(snapshot_name);
    if snapshot_path.exists() {
        bail!("snapshot already exists: {}", snapshot_path.display());
    }

    fs::copy(&current.path, &snapshot_path).with_context(|| {
        format!(
            "failed to copy '{}' to '{}'",
            current.path.display(),
            snapshot_path.display()
        )
    })?;

    update_manifest(
        &manifest.path,
        &current,
        &next,
        &slug,
        reason.unwrap_or("unspecified"),
        operator.unwrap_or("unspecified"),
    )?;

    Ok(VersionWriteOutput {
        artifact: relative_display_path(root, &current.path),
        subject: current.subject,
        lifecycle: current.lifecycle,
        version: next,
        snapshot: Some(relative_display_path(root, &snapshot_path)),
        manifest: Some(relative_display_path(root, &manifest.path)),
    })
}

fn query_history(root: &Path, subject: &str, question: &str) -> anyhow::Result<QueryOutput> {
    let artifact = resolve_current_artifact(root, subject)?;
    let policy = load_artifact_policy(root, &artifact.subject)?;
    let state_field = policy
        .state_field
        .clone()
        .unwrap_or_else(|| "Status".to_string());
    let state_group = infer_state_group(question, &policy)?;
    let state_words = policy.states.get(&state_group).cloned().unwrap_or_default();
    let versions = collect_version_files(root, &artifact)?;
    if versions.is_empty() {
        bail!("no version files found for '{}'", artifact.subject);
    }

    let item = infer_question_item(root, &artifact, &versions, &policy, question)?;
    let manifest = read_manifest_for(root, &artifact)?;

    for version in versions {
        if let Some(evidence) = scan_csv_version(
            root,
            &artifact,
            &version,
            &policy,
            &item,
            &state_field,
            &state_group,
            &state_words,
            &manifest,
        )? {
            let answer = format!(
                "{} first appears with {} state in version {}.",
                item, state_group, evidence.version
            );
            return Ok(QueryOutput {
                subject: artifact.subject,
                question: question.to_string(),
                answer,
                evidence: Some(evidence),
            });
        }
    }

    Ok(QueryOutput {
        subject: artifact.subject,
        question: question.to_string(),
        answer: format!(
            "No preserved version shows {} entering {} state.",
            item, state_group
        ),
        evidence: None,
    })
}

fn scan_csv_version(
    root: &Path,
    artifact: &CurrentArtifact,
    version: &VersionFile,
    policy: &ArtifactPolicy,
    item: &str,
    state_field: &str,
    state_group: &str,
    state_words: &[String],
    manifest: &ManifestInfo,
) -> anyhow::Result<Option<QueryEvidence>> {
    if artifact.format.to_ascii_lowercase() != "csv" {
        bail!(
            "query currently supports CSV artifacts; found '{}'",
            artifact.format
        );
    }

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(&version.path)
        .with_context(|| format!("failed to read CSV '{}'", version.path.display()))?;
    let headers = reader.headers()?.clone();
    let state_index = header_index(&headers, state_field).with_context(|| {
        format!(
            "state field '{}' not found in {}",
            state_field,
            version.path.display()
        )
    })?;
    let identity_indices = identity_indices(&headers, &policy.identity_fields);
    if identity_indices.is_empty() {
        bail!(
            "none of the configured identity fields were found in {}",
            version.path.display()
        );
    }

    for record in reader.records() {
        let record = record?;
        let identity = identity_text(&record, &identity_indices);
        if !contains_case_insensitive(&identity, item) {
            continue;
        }

        let observed_state = record.get(state_index).unwrap_or("").trim().to_string();
        if state_matches(&observed_state, state_group, state_words) {
            return Ok(Some(QueryEvidence {
                artifact: relative_display_path(root, &artifact.path),
                version: version.version.clone(),
                version_file: relative_display_path(root, &version.path),
                manifest_entry: manifest_summary_for(manifest, &version.version),
                changed_field: state_field.to_string(),
                observed_state,
                lifecycle: artifact.lifecycle.clone(),
                identity,
            }));
        }
    }

    Ok(None)
}

fn infer_question_item(
    root: &Path,
    artifact: &CurrentArtifact,
    versions: &[VersionFile],
    policy: &ArtifactPolicy,
    question: &str,
) -> anyhow::Result<String> {
    let question_lower = question.to_ascii_lowercase();

    for version in versions {
        if artifact.format.to_ascii_lowercase() != "csv" {
            continue;
        }
        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
            .from_path(&version.path)
            .with_context(|| format!("failed to read CSV '{}'", version.path.display()))?;
        let headers = reader.headers()?.clone();
        let indices = identity_indices(&headers, &policy.identity_fields);
        for record in reader.records() {
            let record = record?;
            for index in &indices {
                let value = record.get(*index).unwrap_or("").trim();
                if !value.is_empty() && question_lower.contains(&value.to_ascii_lowercase()) {
                    return Ok(value.to_string());
                }
            }
        }
    }

    for token in question
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'))
        .filter(|token| !token.is_empty())
    {
        if token.contains('-') || token.contains('_') {
            return Ok(token.to_string());
        }
    }

    bail!(
        "could not infer item from question for '{}'",
        relative_display_path(root, &artifact.path)
    )
}

fn infer_state_group(question: &str, policy: &ArtifactPolicy) -> anyhow::Result<String> {
    let question_lower = question.to_ascii_lowercase();
    for (state, words) in &policy.states {
        if question_lower.contains(&state.to_ascii_lowercase()) {
            return Ok(state.clone());
        }
        for word in words {
            if question_lower.contains(&word.to_ascii_lowercase()) {
                return Ok(state.clone());
            }
        }
    }

    if question_lower.contains("discontinu") {
        return Ok("discontinued".to_string());
    }

    bail!("could not infer target state from question");
}

fn state_matches(observed: &str, state_group: &str, state_words: &[String]) -> bool {
    let observed_lower = observed.to_ascii_lowercase();
    if observed_lower.contains(&state_group.to_ascii_lowercase()) {
        return true;
    }

    state_words
        .iter()
        .any(|word| observed_lower.contains(&word.to_ascii_lowercase()))
}

fn header_index(headers: &StringRecord, field: &str) -> Option<usize> {
    headers
        .iter()
        .position(|header| header.eq_ignore_ascii_case(field))
}

fn identity_indices(headers: &StringRecord, identity_fields: &[String]) -> Vec<usize> {
    identity_fields
        .iter()
        .filter_map(|field| header_index(headers, field))
        .collect()
}

fn identity_text(record: &StringRecord, indices: &[usize]) -> String {
    indices
        .iter()
        .filter_map(|index| record.get(*index))
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().to_string())
        .collect::<Vec<_>>()
        .join(" | ")
}

fn collect_current_artifacts(
    root: &Path,
    subject_or_path: &str,
) -> anyhow::Result<Vec<CurrentArtifact>> {
    if let Ok(artifact) = resolve_current_artifact(root, subject_or_path) {
        return Ok(vec![artifact]);
    }

    let mut artifacts = Vec::new();
    for entry in walk_files(root) {
        let entry = entry?;
        if let Some(artifact) = parse_current_artifact_path(entry.path()) {
            if artifact.subject == subject_or_path {
                artifacts.push(artifact);
            }
        }
    }

    Ok(artifacts)
}

fn resolve_current_artifact(root: &Path, subject_or_path: &str) -> anyhow::Result<CurrentArtifact> {
    let input = Path::new(subject_or_path);
    let candidate = if input.is_absolute() {
        input.to_path_buf()
    } else {
        root.join(input)
    };

    if candidate.exists() {
        return parse_current_artifact_path(&candidate).with_context(|| {
            format!(
                "'{}' is not a current artifact named <subject>.<lifecycle>.current.<ext>",
                candidate.display()
            )
        });
    }

    for entry in walk_files(root) {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().and_then(|value| value.to_str()) == Some(subject_or_path) {
            return parse_current_artifact_path(path).with_context(|| {
                format!(
                    "'{}' is not a current artifact named <subject>.<lifecycle>.current.<ext>",
                    path.display()
                )
            });
        }
        if let Some(artifact) = parse_current_artifact_path(path) {
            if artifact.subject == subject_or_path {
                return Ok(artifact);
            }
        }
    }

    bail!("current artifact not found for '{}'", subject_or_path)
}

fn parse_current_artifact_path(path: &Path) -> Option<CurrentArtifact> {
    let format = path.extension()?.to_str()?.to_string();
    let stem = path.file_stem()?.to_str()?;
    let parts: Vec<&str> = stem.split('.').collect();
    if parts.len() < 3 || parts.last().copied() != Some("current") {
        return None;
    }

    let lifecycle = parts.get(parts.len() - 2)?.to_string();
    let subject = parts[..parts.len() - 2].join(".");
    if subject.is_empty() || lifecycle.is_empty() {
        return None;
    }

    Some(CurrentArtifact {
        path: path.to_path_buf(),
        subject,
        lifecycle,
        format,
    })
}

fn read_manifest_for(root: &Path, artifact: &CurrentArtifact) -> anyhow::Result<ManifestInfo> {
    let path = manifest_path_for(artifact);
    if !path.exists() {
        return Ok(ManifestInfo {
            path,
            ..Default::default()
        });
    }

    let text = fs::read_to_string(&path)
        .with_context(|| format!("failed to read manifest '{}'", path.display()))?;
    let mut info = ManifestInfo {
        path,
        ..Default::default()
    };

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("Latest version:") {
            let value = value.trim();
            if !value.is_empty() && value != "none" {
                info.latest_version = Some(value.to_string());
            }
            continue;
        }

        if let Some(entry) = parse_manifest_entry(trimmed) {
            info.entries.push(entry);
        }
    }

    if info.latest_version.is_none() {
        info.latest_version = info.entries.last().map(|entry| entry.version.clone());
    }

    let _ = root;
    Ok(info)
}

fn manifest_path_for(artifact: &CurrentArtifact) -> PathBuf {
    artifact
        .path
        .parent()
        .unwrap_or(Path::new("."))
        .join(format!(
            "{}.{}.version.manifest.current.md",
            artifact.subject, artifact.lifecycle
        ))
}

fn parse_manifest_entry(line: &str) -> Option<ManifestEntry> {
    let body = line.strip_prefix("- ")?;
    let (version, summary) = match body.split_once(" - ") {
        Some((version, summary)) => (version.trim(), summary.trim()),
        None => {
            let mut parts = body.splitn(2, char::is_whitespace);
            let version = parts.next()?.trim();
            let summary = parts.next().unwrap_or("").trim();
            (version, summary)
        }
    };
    if version.is_empty() {
        return None;
    }
    Some(ManifestEntry {
        version: version.to_string(),
        summary: summary.to_string(),
    })
}

fn collect_version_files(
    root: &Path,
    artifact: &CurrentArtifact,
) -> anyhow::Result<Vec<VersionFile>> {
    let mut files = Vec::new();
    for entry in walk_files(root) {
        let entry = entry?;
        if let Some(file) = parse_version_file(entry.path(), artifact) {
            files.push(file);
        }
    }

    files.sort_by(|left, right| compare_versions(&left.version, &right.version));
    Ok(files)
}

fn parse_version_file(path: &Path, artifact: &CurrentArtifact) -> Option<VersionFile> {
    let format = path.extension()?.to_str()?;
    if !format.eq_ignore_ascii_case(&artifact.format) {
        return None;
    }

    let stem = path.file_stem()?.to_str()?;
    let parts: Vec<&str> = stem.split('.').collect();
    let version_index = parts.iter().position(|part| *part == "version")?;
    if version_index < 2 || version_index + 1 >= parts.len() {
        return None;
    }
    if parts.get(version_index + 1).copied() == Some("manifest") {
        return None;
    }

    let lifecycle = parts.get(version_index - 1)?;
    let subject = parts[..version_index - 1].join(".");
    if subject != artifact.subject || *lifecycle != artifact.lifecycle {
        return None;
    }

    Some(VersionFile {
        path: path.to_path_buf(),
        version: parts[version_index + 1].to_string(),
        slug: if version_index + 2 < parts.len() {
            Some(parts[version_index + 2..].join("."))
        } else {
            None
        },
    })
}

fn latest_version(
    root: &Path,
    artifact: &CurrentArtifact,
    manifest: &ManifestInfo,
) -> anyhow::Result<Option<String>> {
    if manifest.latest_version.is_some() {
        return Ok(manifest.latest_version.clone());
    }

    Ok(collect_version_files(root, artifact)?
        .last()
        .map(|file| file.version.clone()))
}

fn next_version(latest: Option<&str>) -> String {
    let Some(latest) = latest else {
        return "a1".to_string();
    };

    let mut letters = String::new();
    let mut digits = String::new();
    for ch in latest.chars() {
        if ch.is_ascii_alphabetic() && digits.is_empty() {
            letters.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_digit() {
            digits.push(ch);
        }
    }

    if letters.is_empty() || digits.is_empty() {
        return "a1".to_string();
    }

    let number = digits.parse::<u32>().unwrap_or(0);
    if number == 0 {
        return format!("{letters}1");
    }
    if number < 9 {
        return format!("{letters}{}", number + 1);
    }

    format!("{}1", increment_letters(&letters))
}

fn increment_letters(letters: &str) -> String {
    let mut bytes = letters.as_bytes().to_vec();
    for index in (0..bytes.len()).rev() {
        if bytes[index] < b'z' {
            bytes[index] += 1;
            for item in bytes.iter_mut().skip(index + 1) {
                *item = b'a';
            }
            return String::from_utf8(bytes).unwrap_or_else(|_| "a".to_string());
        }
    }

    format!("{}a", letters)
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    version_key(left).cmp(&version_key(right))
}

fn version_key(version: &str) -> (String, u32, String) {
    let mut letters = String::new();
    let mut digits = String::new();
    for ch in version.chars() {
        if ch.is_ascii_alphabetic() && digits.is_empty() {
            letters.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_digit() {
            digits.push(ch);
        }
    }
    (
        letters,
        digits.parse::<u32>().unwrap_or(0),
        version.to_string(),
    )
}

fn update_manifest(
    path: &Path,
    artifact: &CurrentArtifact,
    version: &str,
    slug: &str,
    reason: &str,
    operator: &str,
) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    let mut entries = if path.exists() {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read '{}'", path.display()))?;
        text.lines()
            .filter_map(parse_manifest_entry)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    entries.retain(|entry| entry.version != version);
    entries.push(ManifestEntry {
        version: version.to_string(),
        summary: format!("{slug}; reason={reason}; operator={operator}"),
    });

    let mut body = String::new();
    body.push_str(&format!("# Version Manifest: {}\n\n", artifact.subject));
    body.push_str(&format!(
        "Artifact: {}\n",
        artifact
            .path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("")
    ));
    body.push_str(&format!("Format: {}\n", artifact.format));
    body.push_str(&format!("Lifecycle: {}\n", artifact.lifecycle));
    body.push_str(&format!("Latest version: {}\n\n", version));
    body.push_str("Versions:\n");
    for entry in entries {
        body.push_str(&format!("- {} - {}\n", entry.version, entry.summary));
    }

    fs::write(path, body).with_context(|| format!("failed to write '{}'", path.display()))
}

fn load_artifact_policy(root: &Path, subject: &str) -> anyhow::Result<ArtifactPolicy> {
    let value = read_config_value(root)?;
    let artifact_table = value
        .as_ref()
        .and_then(|value| value.get("artifact"))
        .and_then(|value| value.as_table())
        .and_then(|table| table.get(subject))
        .and_then(|value| value.as_table());

    let mut policy = ArtifactPolicy {
        configured: artifact_table.is_some(),
        subject: subject.to_string(),
        kind: None,
        format: None,
        risk_class: None,
        privacy_root: None,
        persona: None,
        strategy: "letter-number".to_string(),
        manifest_required: None,
        queryable: None,
        operator_required_for: Vec::new(),
        identity_fields: vec![
            "TaskOrItem".to_string(),
            "Item".to_string(),
            "Name".to_string(),
            "Id".to_string(),
        ],
        tracked_fields: Vec::new(),
        state_field: Some("Status".to_string()),
        note_fields: Vec::new(),
        states: BTreeMap::from([(
            "discontinued".to_string(),
            vec!["DISCONTINUED".to_string(), "OUT CURRENTLY".to_string()],
        )]),
    };

    let Some(table) = artifact_table else {
        return Ok(policy);
    };

    policy.kind = get_string(table, "kind");
    policy.format = get_string(table, "format");
    policy.risk_class = get_string(table, "risk_class");
    policy.privacy_root = get_string(table, "privacy_root");
    policy.persona = get_string(table, "persona");

    if let Some(fields) = table.get("fields").and_then(|value| value.as_table()) {
        if let Some(values) = get_string_array(fields, "identity") {
            policy.identity_fields = values;
        }
        if let Some(values) = get_string_array(fields, "tracked") {
            policy.tracked_fields = values;
        }
        policy.state_field = get_string(fields, "state").or(policy.state_field);
        if let Some(values) = get_string_array(fields, "notes") {
            policy.note_fields = values;
        }
    }

    if let Some(states) = table.get("states").and_then(|value| value.as_table()) {
        policy.states.clear();
        for (key, value) in states {
            if let Some(items) = value.as_array() {
                let values = items
                    .iter()
                    .filter_map(|item| item.as_str().map(|value| value.to_string()))
                    .collect::<Vec<_>>();
                policy.states.insert(key.clone(), values);
            }
        }
    }

    if let Some(versioning) = table.get("versioning").and_then(|value| value.as_table()) {
        if let Some(strategy) = get_string(versioning, "strategy") {
            policy.strategy = strategy;
        }
        policy.manifest_required = get_bool(versioning, "manifest_required");
        policy.queryable = get_bool(versioning, "queryable");
        if let Some(values) = get_string_array(versioning, "operator_required_for") {
            policy.operator_required_for = values;
        }
    }

    Ok(policy)
}

fn read_config_value(root: &Path) -> anyhow::Result<Option<toml::Value>> {
    let path = root.join(".recur").join("config.toml");
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path)
        .with_context(|| format!("failed to read '{}'", path.display()))?;
    let value =
        toml::from_str(&text).with_context(|| format!("failed to parse '{}'", path.display()))?;
    Ok(Some(value))
}

fn get_string(table: &toml::value::Table, key: &str) -> Option<String> {
    table
        .get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn get_bool(table: &toml::value::Table, key: &str) -> Option<bool> {
    table.get(key).and_then(|value| value.as_bool())
}

fn get_string_array(table: &toml::value::Table, key: &str) -> Option<Vec<String>> {
    table.get(key).and_then(|value| {
        value.as_array().map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(|value| value.to_string()))
                .collect()
        })
    })
}

fn manifest_summary_for(manifest: &ManifestInfo, version: &str) -> Option<String> {
    manifest
        .entries
        .iter()
        .find(|entry| entry.version == version)
        .map(|entry| entry.summary.clone())
}

fn resolve_root(dir: &Path) -> anyhow::Result<PathBuf> {
    if dir.join(".recur").exists() {
        return Ok(dir.to_path_buf());
    }

    Ok(project_config::load_nearest(dir)?
        .map(|config| config.project_root)
        .unwrap_or_else(|| dir.to_path_buf()))
}

fn walk_files(root: &Path) -> impl Iterator<Item = walkdir::Result<DirEntry>> + '_ {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| should_keep_walk_entry(entry.path()))
        .filter(|entry| match entry {
            Ok(entry) => entry.file_type().is_file(),
            Err(_) => true,
        })
}

fn should_keep_walk_entry(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return true;
    };
    !matches!(name, ".git" | "target" | "node_modules")
}

fn emit_statuses(statuses: &[VersionStatusOutput], json: bool) -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if json {
        serde_json::to_writer(&mut handle, statuses)?;
        writeln!(handle)?;
        return Ok(());
    }

    for status in statuses {
        writeln!(handle, "Subject: {}", status.subject)?;
        writeln!(handle, "Current artifact: {}", status.current_artifact)?;
        writeln!(handle, "Format: {}", status.format)?;
        writeln!(handle, "Lifecycle: {}", status.lifecycle)?;
        writeln!(
            handle,
            "Latest version: {}",
            status.latest_version.as_deref().unwrap_or("none")
        )?;
        writeln!(handle, "Next version: {}", status.next_version)?;
        writeln!(handle, "Manifest: {}", status.manifest)?;
        writeln!(
            handle,
            "Policy configured: {}",
            if status.policy_configured {
                "yes"
            } else {
                "no"
            }
        )?;
        if let Some(value) = &status.risk_class {
            writeln!(handle, "Risk class: {}", value)?;
        }
        if let Some(value) = &status.privacy_root {
            writeln!(handle, "Privacy root: {}", value)?;
        }
    }
    Ok(())
}

fn emit_manifest(root: &Path, manifest: &ManifestInfo, json: bool) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct ManifestOutput {
        path: String,
        latest_version: Option<String>,
        entries: Vec<ManifestEntryOutput>,
    }

    #[derive(Serialize)]
    struct ManifestEntryOutput {
        version: String,
        summary: String,
    }

    let output = ManifestOutput {
        path: relative_display_path(root, &manifest.path),
        latest_version: manifest.latest_version.clone(),
        entries: manifest
            .entries
            .iter()
            .map(|entry| ManifestEntryOutput {
                version: entry.version.clone(),
                summary: entry.summary.clone(),
            })
            .collect(),
    };

    if json {
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }

    if manifest.path.exists() {
        print!("{}", fs::read_to_string(&manifest.path)?);
    } else {
        println!("Manifest: {}", output.path);
        println!("Latest version: none");
        println!("Versions: none");
    }
    Ok(())
}

fn emit_policy(policy: &ArtifactPolicy, json: bool) -> anyhow::Result<()> {
    let output = PolicyOutput {
        subject: policy.subject.clone(),
        configured: policy.configured,
        kind: policy.kind.clone(),
        format: policy.format.clone(),
        risk_class: policy.risk_class.clone(),
        privacy_root: policy.privacy_root.clone(),
        persona: policy.persona.clone(),
        strategy: policy.strategy.clone(),
        manifest_required: policy.manifest_required,
        queryable: policy.queryable,
        operator_required_for: policy.operator_required_for.clone(),
    };

    if json {
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }

    println!("Subject: {}", output.subject);
    println!("Policy configured: {}", yes_no(output.configured));
    println!("Artifact kind: {}", option_text(output.kind.as_deref()));
    println!("Format: {}", option_text(output.format.as_deref()));
    println!("Risk class: {}", option_text(output.risk_class.as_deref()));
    println!(
        "Privacy root: {}",
        option_text(output.privacy_root.as_deref())
    );
    println!("Persona: {}", option_text(output.persona.as_deref()));
    println!("Strategy: {}", output.strategy);
    println!(
        "Manifest required: {}",
        option_bool_text(output.manifest_required)
    );
    println!("Queryable history: {}", option_bool_text(output.queryable));
    println!(
        "Operator required for: {}",
        list_text(&output.operator_required_for)
    );
    Ok(())
}

fn emit_schema(policy: &ArtifactPolicy, json: bool) -> anyhow::Result<()> {
    let output = SchemaOutput {
        subject: policy.subject.clone(),
        configured: policy.configured,
        identity_fields: policy.identity_fields.clone(),
        tracked_fields: policy.tracked_fields.clone(),
        state_field: policy.state_field.clone(),
        note_fields: policy.note_fields.clone(),
        states: policy.states.clone(),
    };

    if json {
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }

    println!("Subject: {}", output.subject);
    println!("Schema configured: {}", yes_no(output.configured));
    println!("Identity fields: {}", list_text(&output.identity_fields));
    println!("Tracked fields: {}", list_text(&output.tracked_fields));
    println!(
        "State field: {}",
        option_text(output.state_field.as_deref())
    );
    println!("Note fields: {}", list_text(&output.note_fields));
    println!("States:");
    for (state, words) in output.states {
        println!("- {}: {}", state, list_text(&words));
    }
    Ok(())
}

fn emit_query(output: &QueryOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string(output)?);
        return Ok(());
    }

    println!("Question: {}", output.question);
    println!();
    println!("Answer:");
    println!("{}", output.answer);
    if let Some(evidence) = &output.evidence {
        println!();
        println!("Evidence:");
        println!("- artifact: {}", evidence.artifact);
        println!("- version: {}", evidence.version);
        println!("- version file: {}", evidence.version_file);
        if let Some(entry) = &evidence.manifest_entry {
            println!("- manifest entry: {}", entry);
        }
        println!("- changed field: {}", evidence.changed_field);
        println!("- observed state: {}", evidence.observed_state);
        println!("- lifecycle branch: {}", evidence.lifecycle);
        println!("- identity: {}", evidence.identity);
    }
    Ok(())
}

fn emit_write_output(output: &VersionWriteOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string(output)?);
        return Ok(());
    }

    println!("Artifact: {}", output.artifact);
    println!("Subject: {}", output.subject);
    println!("Lifecycle: {}", output.lifecycle);
    println!("Version: {}", output.version);
    if let Some(snapshot) = &output.snapshot {
        println!("Snapshot: {}", snapshot);
    }
    if let Some(manifest) = &output.manifest {
        println!("Manifest: {}", manifest);
    }
    Ok(())
}

fn emit_explain(json: bool) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct Explain<'a> {
        query_surface: &'a str,
        writer: &'a str,
        state_dir: &'a str,
    }

    let output = Explain {
        query_surface: "recur version reads artifact policy, manifests, and history",
        writer: "recur-version saves snapshots and updates manifests",
        state_dir: VERSION_STATE_DIR,
    };

    if json {
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }

    println!("recur version: pure artifact-version query");
    println!("recur-version: snapshot/manifest writer");
    println!("state: {}", VERSION_STATE_DIR);
    Ok(())
}

impl VersionStateWriter {
    fn new(id: String, root: &Path) -> Self {
        let safe_id = sanitize_id(&id);
        Self {
            path: root
                .join(VERSION_STATE_DIR)
                .join(format!("{STATUS_PREFIX}{safe_id}{STATUS_SUFFIX}")),
            id: safe_id,
        }
    }

    fn write_accepted(&self, request: &VersionStateRequest<'_>) -> anyhow::Result<()> {
        self.write_state("complete", "accepted", "", request)
    }

    fn write_rejected(
        &self,
        request: &VersionStateRequest<'_>,
        reason: &str,
    ) -> anyhow::Result<()> {
        self.write_state("stopped", "rejected", reason, request)
    }

    fn write_state(
        &self,
        state: &str,
        ack: &str,
        nak_reason: &str,
        request: &VersionStateRequest<'_>,
    ) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create '{}'", parent.display()))?;
        }

        let body = format!(
            "id = \"{}\"\nstate = \"{}\"\nack = \"{}\"\nnak_reason = \"{}\"\ncommand = \"{}\"\nartifact = \"{}\"\nsubject = \"{}\"\nlifecycle = \"{}\"\nversion = \"{}\"\nsnapshot = \"{}\"\nmanifest = \"{}\"\nslug = \"{}\"\nreason = \"{}\"\noperator = \"{}\"\npid = \"{}\"\nfinished_at = \"{}\"\n",
            escape_value(&self.id),
            escape_value(state),
            escape_value(ack),
            escape_value(nak_reason),
            escape_value(request.command),
            escape_value(request.artifact),
            escape_value(request.subject.unwrap_or("")),
            escape_value(request.lifecycle.unwrap_or("")),
            escape_value(request.version.unwrap_or("")),
            escape_value(&request.snapshot.map(path_text).unwrap_or_default()),
            escape_value(&request.manifest.map(path_text).unwrap_or_default()),
            escape_value(request.slug.unwrap_or("")),
            escape_value(request.reason.unwrap_or("")),
            escape_value(request.operator.unwrap_or("")),
            std::process::id(),
            escape_value(&now_stamp()),
        );

        fs::write(&self.path, body)
            .with_context(|| format!("failed to write '{}'", self.path.display()))
    }
}

fn path_text(path: &Path) -> String {
    path.display().to_string()
}

fn default_status_id(artifact: &str) -> String {
    Path::new(artifact)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(artifact)
        .to_string()
}

fn sanitize_id(id: &str) -> String {
    id.chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            _ => ch,
        })
        .collect()
}

fn sanitize_slug(slug: &str) -> String {
    let mut out = String::new();
    for ch in slug.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if ch == '-' || ch == '_' {
            out.push(ch);
        } else if ch.is_whitespace() || ch == '.' {
            out.push('-');
        }
    }
    while out.contains("--") {
        out = out.replace("--", "-");
    }
    out.trim_matches('-').to_string()
}

fn escape_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn now_stamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{seconds}")
}

fn relative_display_path(base_dir: &Path, path: &Path) -> String {
    path.strip_prefix(base_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn option_text(value: Option<&str>) -> &str {
    value.unwrap_or("not configured")
}

fn option_bool_text(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "not configured",
    }
}

fn list_text(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn next_version_advances_letter_number_tokens() {
        assert_eq!(next_version(None), "a1");
        assert_eq!(next_version(Some("a1")), "a2");
        assert_eq!(next_version(Some("a9")), "b1");
        assert_eq!(next_version(Some("z9")), "za1");
    }

    #[test]
    fn parses_current_artifact_name() {
        let artifact =
            parse_current_artifact_path(Path::new("care.subject.routine.proposed.current.csv"))
                .unwrap();

        assert_eq!(artifact.subject, "care.subject.routine");
        assert_eq!(artifact.lifecycle, "proposed");
        assert_eq!(artifact.format, "csv");
    }

    #[test]
    fn save_version_copies_snapshot_and_updates_manifest() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join(".recur")).unwrap();
        let artifact = root.join("care.subject.routine.proposed.current.csv");
        fs::write(&artifact, "TaskOrItem,Status\nitem-a,DRAFT\n").unwrap();

        let output = save_version(
            root,
            "care.subject.routine.proposed.current.csv",
            "initial save",
            Some("test reason"),
            Some("operator-a"),
        )
        .unwrap();

        assert_eq!(output.version, "a1");
        assert!(root
            .join("care.subject.routine.proposed.version.a1.initial-save.csv")
            .exists());
        let manifest = fs::read_to_string(
            root.join("care.subject.routine.proposed.version.manifest.current.md"),
        )
        .unwrap();
        assert!(manifest.contains("Latest version: a1"));
        assert!(manifest.contains("operator-a"));
    }
}
