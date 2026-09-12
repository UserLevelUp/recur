"""Normalize an actually completed Cargo test log and fingerprint explicit inputs.

Caller separately verifies exit status. --retain preserves predecessor input
paths without reusing predecessor hashes or claiming its old run happened now.
"""
import argparse
import json
import re
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def fingerprint(data):
    value = 0xcbf29ce484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return f"fnv1a64:{value:016x}"

def write(path, value):
    with path.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(value, stream, indent=2)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("name")
    parser.add_argument("log")
    parser.add_argument("inputs", nargs="*")
    parser.add_argument("--retain", action="append", default=[])
    args = parser.parse_args()
    content = (ROOT / args.log).read_text(encoding="utf-8-sig")
    summaries = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;", content)
    assert summaries and all(row[0] == "ok" and row[2:] == ("0", "0") for row in summaries)
    assert not re.search(r"(?m)^error:", content)
    count = sum(int(row[1]) for row in summaries)
    assert count > 0
    inputs = set(args.inputs + [args.log, "warps/evidence-refresh/record_observation.py"])
    for name in args.retain:
        inputs.update(json.loads((ROOT / name).read_text(encoding="utf-8-sig"))["source"]["files"])
    folder = ROOT / "warps/evidence-refresh"
    result = folder / (args.name + ".result.json")
    write(result, {"schema": "warp-external-result-v1", "kind": "test", "outcome": "passed", "exit_code": 0,
        "tests": {"discovered": count, "executed": count, "passed": count, "failed": 0, "skipped": 0}})
    write(folder / (args.name + ".evidence.json"), {
        "schema": "warp-external-evidence-v1", "kind": "test", "producer": "observed Cargo log; record_observation.py",
        "project": "recur", "configuration": "locked offline; CARGO_INCREMENTAL=0; explicit reviewed inputs",
        "platform": "Windows x64", "executed_at_unix": int(time.time()),
        "result_artifact": result.relative_to(ROOT).as_posix(), "result_fingerprint": fingerprint(result.read_bytes()),
        "source": {"revision": None, "dirty": True, "files": {p: fingerprint((ROOT / p).read_bytes()) for p in sorted(inputs)}}})
    print(f"Recorded {count} observed passes against {len(inputs)} actual inputs.")
