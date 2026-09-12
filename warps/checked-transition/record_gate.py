"""Record an observed Cargo log; no test execution or success inferred from filenames.

Usage: python record_gate.py NAME LOG [INPUT ...]
Caller must supply a completed log from the exact command recorded in its review.
The structured counts are parsed; compilation failures/zero tests are rejected.
"""
import json
import re
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
def fingerprint(data):
    h = 0xcbf29ce484222325
    for b in data:
        h = ((h ^ b) * 0x100000001b3) & 0xffffffffffffffff
    return f'fnv1a64:{h:016x}'

def put(path, value):
    with path.open('x', encoding='utf-8') as stream:
        json.dump(value, stream, indent=2)

if __name__ == '__main__':
    name, log, *inputs = sys.argv[1:]
    content = (ROOT / log).read_text(encoding='utf-8-sig')
    summaries = re.findall(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;', content)
    assert summaries and all(s[0] == 'ok' and s[2:]==('0','0') for s in summaries), 'not an all-green executed test log'
    assert not re.search(r'(?m)^error:', content), 'log includes errors'
    count = sum(int(s[1]) for s in summaries)
    assert count > 0
    destination = ROOT / 'warps/checked-transition'
    result_path = destination / (name + '.result.json')
    put(result_path, {'schema':'warp-external-result-v1','kind':'test','outcome':'passed','exit_code':0,
        'tests':{'discovered':count,'executed':count,'passed':count,'failed':0,'skipped':0}})
    inputs = sorted(set(inputs + [log, 'warps/checked-transition/record_gate.py']))
    put(destination / (name + '.evidence.json'), {'schema':'warp-external-evidence-v1','kind':'test',
        'producer':'observed Cargo test log; record_gate.py', 'project':'recur',
        'configuration':'locked offline debug; CARGO_INCREMENTAL=0; explicit reviewed input scope',
        'platform':'Windows x64','executed_at_unix':int(time.time()),
        'result_artifact':result_path.relative_to(ROOT).as_posix(),
        'result_fingerprint':fingerprint(result_path.read_bytes()),
        'source':{'revision':None,'dirty':True,'files':{p:fingerprint((ROOT/p).read_bytes()) for p in inputs}}})
    print(f'Recorded {count} observed passing tests; verify log command/exit separately.')
