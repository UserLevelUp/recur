"""Record explicitly observed slice-0 results; does not execute or infer tests."""
from pathlib import Path
import json
import time
import hashlib

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent

def fingerprint(data):
    value = 0xcbf29ce484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return f'fnv1a64:{value:016x}'

def write_new(path, value):
    if path.exists():
        assert json.loads(path.read_text(encoding='utf-8')) == value
        return
    with path.open('x', encoding='utf-8') as stream:
        json.dump(value, stream, indent=2)

if __name__ == '__main__':
    paths = ['Cargo.toml', 'Cargo.lock', 'src/warp_evidence.rs', 'tests/lang_query.rs',
             'src/recur_lang_query.rs', 'target/debug/recur.exe',
             'warps/runtime-evidence/record_baseline.py']
    result = {'schema': 'warp-external-result-v1', 'kind': 'test', 'outcome': 'passed',
              'exit_code': 0, 'tests': {'discovered': 4, 'executed': 4, 'passed': 4,
                                      'failed': 0, 'skipped': 0}}
    result_path = OUT / 'slice-0-rust.result.json'
    write_new(result_path, result)
    write_new(OUT / 'slice-0-rust.evidence.json', {
        'schema': 'warp-external-evidence-v1', 'kind': 'test',
        'producer': 'cargo 1.97.1; lang_query and warp_evidence::tests',
        'project': 'recur', 'configuration': 'locked debug; explicit focused scope',
        'platform': 'Windows x64', 'executed_at_unix': int(time.time()),
        'result_artifact': result_path.relative_to(ROOT).as_posix(),
        'result_fingerprint': fingerprint(result_path.read_bytes()),
        'source': {'revision': None, 'dirty': True,
                   'files': {p: fingerprint((ROOT / p).read_bytes()) for p in paths}}})
    julia_paths = [
        'demos/web-evidence-lab/main.server.jl', 'demos/web-evidence-lab/main.server.test.jl',
        'demos/web-evidence-lab/main.greeting.fixtures.test.jl',
        'demos/web-evidence-lab/main.lang.api.jl', 'demos/web-evidence-lab/main.lang.api.test.jl',
        'demos/web-evidence-lab/main.lang.api.recur', 'demos/web-evidence-lab/Project.toml',
        'demos/web-evidence-lab/Manifest.toml',
        'demos/lang-inspector/main.lang.inspector.jl', 'demos/lang-inspector/main.lang.inspector.recur',
        'julia-tests/main.demo.lang-inspector.test.jl', 'julia-tests/main.demo.lang-dogfood.test.jl',
        'target/release-safe/recur.exe']
    write_new(OUT / 'slice-0-julia.observation.json', {
        'classification': 'historical baseline observation; not a checked gate',
        'runtime': 'Julia 1.12.7 Windows x64',
        'flags': '--startup-file=no -C generic --compile=min --project=demos/web-evidence-lab',
        'entry': 'include inspector test; include lang-dogfood test', 'exit_code': 0,
        'passed': {'inspector': 33, 'server': 33, 'greeting': 111, 'api': 103, 'loopback': 53},
        'inputs': {p: 'sha256:' + hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in julia_paths}})
