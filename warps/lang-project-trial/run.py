"""Read-only Recur project trial; mutations are confined to temporary fixtures.

Run from the repository root with an existing local debug executable.
Records observations, including unmet expectations, without changing product code
or converting discovery/static findings into acceptance or runtime evidence.
"""
import hashlib
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import tempfile
import time
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[2]
EXE = ROOT / "target/debug/recur.exe"
OUT = Path(__file__).with_name("observations.json")
if OUT.exists():
    raise SystemExit("Refusing to overwrite historical observations")

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

observations = []
checks = []

def run(name, args, root=ROOT, structured=True, save=True):
    cmd = [str(EXE), "lang", *args, "-d", str(root)]
    if structured:
        cmd.append("--json")
    start = time.perf_counter()
    p = subprocess.run(cmd, cwd=ROOT, capture_output=True, timeout=30)
    elapsed = (time.perf_counter() - start) * 1000
    stdout = p.stdout.decode("utf-8", errors="replace")
    value = json.loads(stdout) if structured and stdout else None
    item = dict(name=name, command=cmd, exit=p.returncode, elapsed_ms=elapsed,
                stdout_bytes=len(p.stdout), stdout_lines=len(stdout.splitlines()),
                stderr=p.stderr.decode("utf-8", errors="replace"),
                result=value if structured else stdout)
    if save:
        observations.append(item)
    return item

def expect(name, condition, explanation):
    checks.append(dict(name=name, met=bool(condition), expectation=explanation))

def codes(item):
    value = item["result"] or {}
    return [x["code"] for x in value.get("diagnostics", []) +
            value.get("footer", {}).get("findings", [])]

listing = run("repository-list", ["list"])
sources = listing["result"]["sources"]
tracked_inputs = [ROOT / s["source"] for s in sources]
tracked_inputs += [ROOT / ".recur/config.toml", ROOT / "src/recur_lang_query.rs",
                   ROOT / "src/recur_lang_ir.rs", ROOT / "src/recur_lang_graph.rs",
                   ROOT / "docs/main.command.lang.query.readme.md", Path(__file__), EXE]
before = {str(p.relative_to(ROOT)): sha(p) for p in tracked_inputs}
run("repository-overview", [], structured=False)
for source in sources:
    path = source["source"]
    result = run("check:" + path, ["check", path])
    expect("check agrees with inventory:" + path,
           result["exit"] == (2 if source["status"] == "input-error" else 0),
           "Direct check agrees with discovered supported/unsupported status")
    for symbol in source.get("symbols", []):
        scoped = run("scope:" + path + ":" + symbol,
                     ["show", path, "--scope", symbol])
        identities = [h["identity"] for h in scoped["result"].get("header", [])]
        expect("exact scope:" + path + ":" + symbol,
               scoped["exit"] == 0 and identities == [symbol],
               "An advertised function can be selected exactly")

algorithm = "demos/main.lang/main.lang.algorithm-lab.recur"
coordination = "demos/main.lang/main.lang.skippy-watch-coordination.recur"
compact = run("alias-compact", ["show", algorithm, "--scope", "merge.f"])
expanded = run("alias-expanded", ["show", algorithm, "--scope", "merge.f", "--expand"])
run("alias-readable", ["show", algorithm, "--scope", "merge.f"], structured=False)
expect("alias retained", compact["result"]["header"][0]["input"]["canonical_identity"] == "bubble.o(b)",
       "The merge input remains an alias of bubble.o(b)")
expect("expansion preserves edges", compact["result"]["body"] == expanded["result"]["body"],
       "Expanded fields must not change dependencies or flow")
expect("boundary retained", bool(compact["result"]["body"].get("boundary_edges")),
       "Scoped merge output must retain its external input dependency")

benchmark = {}
for name, args, root in [
    ("list-repo", ["list"], ROOT),
    ("show-repo", ["show", algorithm, "--scope", "gcd.f"], ROOT),
    ("show-narrow", ["show", "main.lang.algorithm-lab.recur", "--scope", "gcd.f"], ROOT / "demos/main.lang"),
]:
    run(name + "-warmup", args, root, save=False)
    samples = [run(name, args, root, save=False) for _ in range(7)]
    times = [s["elapsed_ms"] for s in samples]
    benchmark[name] = dict(samples_ms=times, median_ms=statistics.median(times),
                           min_ms=min(times), max_ms=max(times), exits=[s["exit"] for s in samples])

algorithm_text = (ROOT / algorithm).read_text(encoding="utf-8")
coordination_text = (ROOT / coordination).read_text(encoding="utf-8")
with tempfile.TemporaryDirectory(prefix="recur-lang-trial-") as temp:
    base = Path(temp)
    fixture = base / "project"
    fixture.mkdir()
    (fixture / "algorithm.recur").write_text(algorithm_text, encoding="utf-8")
    (base / "outside.recur").write_text(algorithm_text, encoding="utf-8")
    missing_join = coordination_text.replace(
        "await [csharp_monkey.o(b), web_monkey.o(b), test_bird.o(b)]",
        "await [csharp_monkey.o(b), web_monkey.o(b)]")
    assert missing_join != coordination_text
    (fixture / "missing-join.recur").write_text(missing_join, encoding="utf-8")
    cycle = coordination_text.replace(
        'i(a) := project skippy.plan.o(b).orders["csharp_monkey"]',
        'i(a) := join(project skippy.plan.o(b).orders["csharp_monkey"], git_monkey.o(b))', 1)
    assert cycle != coordination_text
    (fixture / "cycle.recur").write_text(cycle, encoding="utf-8")
    for name, args, code, diagnostic in [
        ("missing-join", ["check", "missing-join.recur", "--scope", "git_monkey"], 1, "SGR004"),
        ("dependency-cycle", ["check", "cycle.recur", "--scope", "test_bird"], 1, "SGR001"),
        ("ambiguous-letter", ["show", "algorithm.recur", "--scope", "f"], 2, "LANG004"),
        ("unknown-scope", ["show", "algorithm.recur", "--scope", "absent.f"], 2, "LANG003"),
        ("outside-root", ["check", "../outside.recur"], 2, "LANG002"),
    ]:
        inventory = {str(p.relative_to(base)): sha(p) for p in base.rglob("*") if p.is_file()}
        result = run(name, args, fixture)
        expect(name, result["exit"] == code and diagnostic in codes(result),
               f"exit {code}, diagnostic {diagnostic}, including findings outside selected scope")
        expect(name + " query purity", inventory == {str(p.relative_to(base)): sha(p) for p in base.rglob("*") if p.is_file()},
               "Query leaves every isolated fixture byte and file unchanged")

    (fixture / "demo.algorithm.gcd.complete.md").write_text("recorded only", encoding="utf-8")
    control = run("eventness-default-control", ["report", "algorithm.recur", "--eventness", "complete"], fixture)
    expect("default recorded state", len(control["result"].get("header", [])) == 1,
           "Default suffix selects the recorded gcd completion")
    (fixture / ".recur").mkdir()
    (fixture / ".recur/config.toml").write_bytes((ROOT / ".recur/config.toml").read_bytes())
    configured = run("eventness-repo-config", ["report", "algorithm.recur", "--eventness", "complete.md"], fixture)
    expect("repository suffix convention", len(configured["result"].get("header", [])) == 1,
           "The repo's complete_suffix='.complete.md' should select the same recorded completion")
    run("eventness-repo-config-unfiltered", ["show", "algorithm.recur", "--scope", "gcd.f"], fixture)
    damaged_worker = algorithm_text.replace("emit b(value: abs(a.left))", "emit b(value: 999999)")
    assert damaged_worker != algorithm_text
    (fixture / "damaged-worker.recur").write_text(damaged_worker, encoding="utf-8")
    ignored = run("excluded-worker-semantics", ["check", "damaged-worker.recur", "--scope", "gcd.f"], fixture)
    expect("worker exclusion explicit", ignored["exit"] == 0 and ignored["result"]["coverage"]["whole_source_validated"] is False,
           "Wrong algorithm output remains outside static coverage; report must disclose partial validation")

expect("repository selected inputs unchanged", before == {str(p.relative_to(ROOT)): sha(p) for p in tracked_inputs},
       "All 10 original programs, root configuration, selected implementation/docs and binary unchanged")
cir_sources = [s for s in sources if s.get("ir_schema") == "recur-lang-concurrent-ir-v1"]
expect("CIR1 list documentation", all(s["recorded_eventness"] == [] for s in cir_sources),
       "Current query README says CIR1 recorded_eventness is an empty array")
result = dict(observed_at=datetime.now(timezone.utc).isoformat(), root=str(ROOT),
              platform=platform.platform(), python=platform.python_version(),
              executable=str(EXE), executable_sha256=sha(EXE),
              build="existing debug build; wall clock includes process startup, filesystem inventory and JSON serialization",
              inputs_sha256=before, benchmark=benchmark, expectations=checks, observations=observations)
OUT.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(json.dumps(dict(sources=len(sources), supported=sum(s["status"] != "input-error" for s in sources),
                     symbols=sum(len(s.get("symbols", [])) for s in sources),
                     observations=len(observations), expectations=len(checks),
                     unmet=[c for c in checks if not c["met"]], benchmark=benchmark), indent=2))
