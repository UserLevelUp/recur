"""Supplemental discovery probes; no changes to tested application inputs."""
from pathlib import Path
import importlib.util
import json
import shutil

ROOT = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('lab_tests', ROOT / 'main.test.py')
lab = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lab)
recur = lab.REPO / 'target/release-safe/recur.exe'
results = json.loads((ROOT / 'main.results.json').read_text(encoding='utf-8'))
out = ROOT / 'evidence' / results['run_id']
fixture = ROOT / 'scratch' / results['run_id'] / 'configured-trace'
shutil.copytree(ROOT / 'apps/10', fixture, dirs_exist_ok=True)
(fixture / '.recur/config.toml').write_text('''[app]
dir = "."
sep = "."
[traits.trace_id]
enabled = true
producer_keywords = "produces:,dispatchevent"
consumer_keywords = "consumes:,addeventlistener"
trigger_keywords = "triggers:"
''', encoding='utf-8')
probe = {}
probe['configured_trace'] = lab.command([recur, 'trace-id', 'main.greeting.sent', '--scope', 'main.**', '-d', fixture, '--json'])
probe['final_files'] = lab.command([recur, 'files', 'main.**', '-d', ROOT / 'apps/10', '--json'])
for depth in [1, 2, 5]:
    probe[f'tree_depth_{depth}'] = lab.command([recur, 'tree', 'main', '--depth', depth, '-d', ROOT / 'apps/10', '--json'])
probe['warp_inventory'] = lab.command([recur, 'warp', 'list', '--all', '-d', ROOT, '--json'])
event_file = fixture / 'main.history.integration.current.md'
event_file.write_text('Intent: check the parent greeting event reaches history.\nConstraint: child acceptance cannot substitute for parent wiring.\nNext: inspect the recorded child_vs_parent experiment.\n', encoding='utf-8')
probe['eventness_current'] = lab.command([recur, 'files', 'main.history.**.current', '-d', fixture, '--json'])
failure_file = fixture / 'main.history.integration.strange.md'
event_file.replace(failure_file)
failure_file.write_text('Observed in the saved browser experiment: child passed; parent expected one entry and got zero.\nCause: main.greeting.lost was emitted instead of main.greeting.sent.\n', encoding='utf-8')
probe['eventness_strange'] = lab.command([recur, 'files', 'main.history.**.strange', '-d', fixture, '--json'])
complete_file = fixture / 'main.history.integration.complete.md'
failure_file.replace(complete_file)
complete_file.write_text(f'Repair observed in evidence/{results["run_id"]}/run.json, experiments.child_vs_parent.repair_passed.\nResidue: verify the parent event boundary after changes to either feature.\n', encoding='utf-8')
probe['eventness_complete'] = lab.command([recur, 'files', 'main.history.**.complete', '-d', fixture, '--json'])
trace = probe['configured_trace']['json']
assert trace['produce'] and trace['consume'], trace
lab.dump(out / 'discovery-probes.json', probe)
print(json.dumps({'roles': {k:len(trace[k]) for k in ['define','produce','consume','trigger']},
                  'final_files':len(probe['final_files']['json']),
                  'depth_1_equals_5':probe['tree_depth_1']['json'] == probe['tree_depth_5']['json']}, indent=2))
