//! Opinionated deterministic advice; the pure projector remains in core.
use recur::{recur_lang_query, warp_evidence::fingerprint};
use serde_json::{json, Value};
use std::{fs, path::Path};

fn prepare(root: &Path, source: &Path, scope: Option<&str>) -> Result<Value, Value> {
    let packet = recur_lang_query::report_packet(source, scope, root)?;
    let preview = crate::recur_lang_init::init(root, true).map_err(|e|
        json!({"schema":"recur-lang-plan-error-v1","diagnostics":[{"code":e.code,"message":e.message}]}))?;
    let config_path = preview["config_path"].as_str().unwrap();
    let config_hash = match fs::read(config_path) {
        Ok(bytes) => Some(fingerprint(&bytes)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(
                json!({"schema":"recur-lang-plan-error-v1","diagnostics":[{"code":"LINIT002","message":e.to_string()}]}),
            )
        }
    };
    let config: toml::Value = toml::from_str(preview["preview"].as_str().unwrap()).unwrap();
    // Only known preferences leave the companion. Unrelated config and unknown
    // fields are never copied into the advice packet or interpreted as commands.
    let lang = &config["recur-lang"];
    let target = lang["target"].as_str().unwrap();
    let planning = &lang["planning"];
    let specification_first = planning["specification_first"].as_bool().unwrap();
    let prioritize = planning["prioritize_graph_findings"].as_bool().unwrap();
    let include_tests = planning["include_test_plan"].as_bool().unwrap();
    let settings = json!({"schema_version":1,"target":target,"planning":{
        "specification_first":specification_first,"prioritize_graph_findings":prioritize,"include_test_plan":include_tests}});
    let findings = packet["footer"]["findings"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut work = Vec::new();
    let mut tests = Vec::new();
    for header in packet["header"].as_array().unwrap() {
        work.push(
            json!({"kind":"implement-symbol","symbol":header["identity"],"declaration":header}),
        );
        if include_tests {
            tests.push(json!({"kind":"behavior","symbol":header["identity"],
                "requirement":header["meaning"],"suggestion":"Write positive, boundary and rejection cases for this declared behavior; derive expected results independently."}));
        }
    }
    if include_tests {
        tests.push(json!({"kind":"dependency-conformance","suggestion":"Use the Lang reference to review every implementation call, including private helpers, closures and default callback targets. Map missing edges, check cycles, distinguish bounded state feedback, and record source hashes and unresolved targets in Eventness. This requires reviewer judgment, not automatic Julia inspection."}));
        tests.push(json!({"kind":"contracts-and-aliases","suggestion":"Check required/optional fields, canonical aliases, output shape and preservation of caller-owned data."}));
        if !packet["footer"]["graph"].is_null() {
            tests.push(json!({"kind":"joins-and-producers","suggestion":"Fail or omit a producer, swap identities and verify the join refuses incomplete or incorrect messages."}));
            tests.push(json!({"kind":"cycle-faults","suggestion":"Inject dependency and wait cycles and a missing await; require exact graph findings even under scope filtering."}));
        }
    }
    if !findings.is_empty() {
        let repair = json!({"kind":"repair-static-findings","findings":findings});
        if prioritize {
            work.insert(0, repair);
        } else {
            work.push(repair);
        }
    }
    let mut unresolved = vec![
        "Excluded grammar and implementation-only dependencies require separate review.",
        "Define semantic invariants and obtain runtime test evidence; this plan is not acceptance.",
    ];
    if target == "unspecified" {
        unresolved.push("Choose a concrete target in recur-lang project preferences.");
    }
    if !include_tests {
        unresolved.push(
            "Suggested test plan is disabled by project preference; validation is still required.",
        );
    }
    let state = if !findings.is_empty() {
        "blocked"
    } else if target == "unspecified" {
        "needs-target"
    } else {
        "planned"
    };
    Ok(
        json!({"schema":"recur-lang-implementation-plan-v1","state":state,"execution":"not-run",
        "source":packet["source"],"source_hash":packet["source_hash"],
        "policy":{"settings":settings,"config_path":config_path,"config_hash":config_hash,
            "effective_hash":fingerprint(serde_json::to_string(&settings).unwrap().as_bytes())},
        "phases":if specification_first { vec!["specification","tests","implementation"] } else { vec!["implementation","tests"] },
        "work_items":work,"findings":findings,"test_plan":tests,"unresolved_decisions":unresolved,"packet":packet}),
    )
}
pub fn plan(root: &Path, source: &Path, scope: Option<&str>) -> (Value, i32) {
    match prepare(root, source, scope) {
        Ok(value) => {
            let code = i32::from(value["state"] == "blocked");
            (value, code)
        }
        Err(value) => (value, 2),
    }
}
pub fn text(value: &Value) -> String {
    if value.get("diagnostics").is_some() {
        return serde_json::to_string_pretty(value).unwrap();
    }
    let mut out = format!(
        "Lang implementation plan: {}\nTarget: {}\nSource: {} ({})\n",
        value["state"].as_str().unwrap(),
        value["policy"]["settings"]["target"].as_str().unwrap(),
        value["source"].as_str().unwrap(),
        value["source_hash"].as_str().unwrap()
    );
    out.push_str(&format!(
        "Phases: {}\n",
        value["phases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|phase| phase.as_str().unwrap())
            .collect::<Vec<_>>()
            .join(" -> ")
    ));
    for item in value["work_items"].as_array().unwrap() {
        out.push_str(&format!(
            "- {}: {}\n",
            item["kind"].as_str().unwrap(),
            item.get("symbol").unwrap_or(&item["findings"])
        ));
        if let Some(meaning) = item["declaration"]["meaning"].as_str() {
            out.push_str(&format!("  Requirement: {meaning}\n"));
        }
    }
    for test in value["test_plan"].as_array().unwrap() {
        out.push_str(&format!(
            "Test [{}]: {}\n",
            test["kind"].as_str().unwrap(),
            test["suggestion"].as_str().unwrap()
        ));
    }
    for decision in value["unresolved_decisions"].as_array().unwrap() {
        out.push_str(&format!("Review: {}\n", decision.as_str().unwrap()));
    }
    out.push_str("Execution: not-run\n");
    out
}
