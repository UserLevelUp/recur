"""Real Chromium acceptance plus Recur discovery and checked-evidence experiments."""
from pathlib import Path
from datetime import datetime, timezone
import argparse
import hashlib
import importlib.util
import importlib.metadata
import json
import platform
import shutil
import subprocess
import sys
import time
from playwright.sync_api import sync_playwright, expect

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parents[1]


def module(name):
    spec = importlib.util.spec_from_file_location(name, ROOT / f'{name}.py')
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


def dump(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')


def fingerprint(path):
    value = 0xcbf29ce484222325
    for byte in path.read_bytes():
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return f'fnv1a64:{value:016x}'


def command(args, cwd=ROOT):
    started = time.monotonic()
    result = subprocess.run([str(arg) for arg in args], cwd=cwd, capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=60)
    record = {'argv': [str(arg) for arg in args], 'cwd': str(cwd), 'exit_code': result.returncode,
              'seconds': round(time.monotonic() - started, 3), 'stdout': result.stdout, 'stderr': result.stderr}
    try:
        record['json'] = json.loads(result.stdout)
    except ValueError:
        pass
    return record


def checks(level):
    yield 'initial greeting', lambda p: expect(p.locator('#greeting')).to_have_text('Hello, World!')
    if level >= 2:
        yield 'stylesheet applied', lambda p: expect(p.locator('body')).to_have_css('background-color', 'rgb(16, 25, 35)')
    if level >= 3:
        def view(p):
            p.evaluate("async () => (await import('./main.greeting.view.js')).render('Hello, module!')")
            expect(p.locator('#greeting')).to_have_text('Hello, module!')
        yield 'extracted view contract', view
    if level >= 4:
        yield 'trimmed name', lambda p: greet(p, '  Joe  ', 'Hello, Joe!')
        def invalid(p):
            greet(p, '   ', error='Please enter a name.')
            greet(p, 'x' * 41, error='Use 40 characters or fewer.')
        yield 'empty and overlong validation', invalid
        def safe(p):
            greet(p, '<img src=x onerror=alert(1)>', 'Hello, <img src=x onerror=alert(1)>!')
            expect(p.locator('#greeting img')).to_have_count(0)
        yield 'untrusted input remains text', safe
    if level >= 5:
        def spanish(p):
            p.locator('#locale').select_option('es')
            greet(p, 'Joe', 'Hola, Joe!')
        yield 'Spanish greeting', spanish
    if level >= 6:
        def persistence(p):
            p.locator('#locale').select_option('es')
            greet(p, 'Joe', 'Hola, Joe!')
            p.reload()
            expect(p.locator('#greeting')).to_have_text('Hola, Joe!')
            expect(p.locator('#name')).to_have_value('Joe')
        yield 'preferences survive reload', persistence
        def corrupt(p):
            p.evaluate("localStorage.setItem('main.preferences.v1', '{broken')")
            p.reload()
            expect(p.locator('#greeting')).to_have_text('Hello, World!')
        yield 'malformed stored JSON recovers', corrupt
    if level >= 7:
        def routes(p):
            p.get_by_role('link', name='About', exact=True).click()
            expect(p.locator('#about-panel')).to_be_visible()
            expect(p.locator('#greeting-panel')).to_be_hidden()
            p.reload()
            expect(p.locator('#about-panel')).to_be_visible()
            p.get_by_role('link', name='Greeting', exact=True).click()
            expect(p.locator('#greeting-panel')).to_be_visible()
        yield 'route and reload', routes
    if level >= 8:
        def api(p):
            greet(p, 'API Joe', 'Hello, API Joe!')
            expect(p.locator('#transport')).to_have_text('network')
            assert p.request.get('/api/greeting?name=').status == 400
        yield 'real HTTP success and validation', api
        def failure(p):
            p.route('**/api/greeting?*', lambda route: route.fulfill(status=503, body='unavailable'))
            greet(p, 'Uncached', error='Greeting service unavailable (503).')
            expect(p.locator('#greet-form button')).to_be_enabled()
        yield 'HTTP failure is visible and form recovers', failure
    if level >= 9:
        def retry(p):
            calls = []
            def intercept(route):
                calls.append(route.request.url)
                if len(calls) == 1:
                    route.fulfill(status=503, body='transient')
                else:
                    route.continue_()
            p.route('**/api/greeting?*', intercept)
            greet(p, 'Retry Joe', 'Hello, Retry Joe!')
            assert len(calls) == 2
        yield 'one retry recovers from transient failure', retry
        def cache(p):
            greet(p, 'Cached Joe', 'Hello, Cached Joe!')
            p.route('**/api/greeting?*', lambda route: route.abort())
            greet(p, 'Cached Joe', 'Hello, Cached Joe!')
            expect(p.locator('#transport')).to_have_text('cache')
            p.locator('#locale').select_option('es')
            greet(p, 'Cached Joe', error='Failed to fetch')
        yield 'offline cache and locale isolation', cache
    if level >= 10:
        def model(p):
            value = p.evaluate("async () => (await import('./main.history.model.js')).append(['a','b','c','d','e'], 'f')")
            assert value == ['b', 'c', 'd', 'e', 'f']
        yield 'history child contract', model
        def integrated(p):
            for index in range(6):
                greet(p, f'Joe {index}', f'Hello, Joe {index}!')
            expect(p.locator('#history li')).to_have_count(5)
            expect(p.locator('#history li').first).to_have_text('Hello, Joe 1!')
            p.locator('#clear-history').click()
            expect(p.locator('#history li')).to_have_count(0)
        yield 'parent event wiring, history bound, and clear', integrated


def greet(page, name, message=None, error=None):
    page.locator('#name').fill(name)
    page.locator('#greet-form button').click()
    expect(page.locator('#greet-form button')).to_be_enabled()
    if message is not None:
        expect(page.locator('#greeting')).to_have_text(message)
    if error is not None:
        expect(page.locator('#error')).to_have_text(error)


def run():
    parser = argparse.ArgumentParser()
    parser.add_argument('--browser', type=Path, default=Path('C:/Program Files/Google/Chrome/Application/chrome.exe'))
    parser.add_argument('--recur', type=Path, default=REPO / 'target/release-safe/recur.exe')
    parser.add_argument('--recur-warp', type=Path, default=REPO / 'target/release-safe/recur-warp.exe')
    args = parser.parse_args()
    module('main.build').build()
    run_id = datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    out = ROOT / 'evidence' / run_id
    out.mkdir(parents=True)
    # Preserve old acceptance declarations without mixing runs in the live projection.
    for layer in ROOT.glob('*.warp-layer.json'):
        archive = out / 'previous-layers' / (layer.name + '.archived')
        archive.parent.mkdir(exist_ok=True)
        layer.rename(archive)
    head = command(['git', 'rev-parse', 'HEAD'], REPO)['stdout'].strip()
    summary = {'run_id': run_id, 'repository_base_revision': head, 'tested_source_is_uncommitted': True,
               'python': sys.version, 'python_executable': sys.executable, 'platform': platform.platform(),
               'playwright': importlib.metadata.version('playwright'), 'browser_executable': str(args.browser),
               'recur_sha256': hashlib.sha256(args.recur.read_bytes()).hexdigest(),
               'recur_warp_sha256': hashlib.sha256(args.recur_warp.read_bytes()).hexdigest(),
               'invocation': [sys.executable, *sys.argv], 'stages': [], 'experiments': {}}
    server = module('main.server.process').make_server()
    summary['server'] = {'runtime': server.version, 'executable': server.executable, 'argv': server.argv}
    summary['server_contract'] = command([server.executable, '--startup-file=no', f'--project={ROOT}', ROOT / 'main.server.test.jl'])
    if summary['server_contract']['exit_code'] != 0:
        server.shutdown()
        raise RuntimeError(summary['server_contract'])
    base = f'http://127.0.0.1:{server.server_port}'
    required = [{'slice_id': f'level-{i:02}', 'contract_hash': fingerprint(ROOT / 'apps' / f'{i:02}' / 'main.contract.md'),
                 'depends_on': [], 'evidence_mode': 'checked', 'evidence_gates': ['browser'],
                 'gate_rules': {'browser': {'kind': 'test', 'allow_skipped': False}}} for i in range(1, 11)]
    warp_map = {'schema': 'warp-bubble-map-v1', 'warp_id': 'main.web-lab', 'required_slices': required}
    dump(ROOT / 'main.web-lab.warp-map.json', warp_map)
    # Companion writes live layers beside the root map; earlier layers are archived.
    try:
        with sync_playwright() as playwright:
            browser = playwright.chromium.launch(executable_path=str(args.browser), headless=True)
            summary['browser_version'] = browser.version
            for level in range(1, 11):
                app = ROOT / 'apps' / f'{level:02}'
                note = app / 'main.acceptance.current.md'
                note.write_text('Intent: verify visible behavior and feature boundaries.\nNext: run main.test.py and inspect failures.\n', encoding='utf-8')
                cases = []
                for name, check in checks(level):
                    context = browser.new_context(base_url=base, viewport={'width': 1100, 'height': 850})
                    page = context.new_page()
                    errors = []
                    page.on('pageerror', lambda error: errors.append(str(error)))
                    started = time.monotonic()
                    result = {'name': name}
                    try:
                        page.goto(f'/apps/{level:02}/main.html')
                        check(page)
                        assert not errors, errors
                        result['outcome'] = 'passed'
                    except Exception as error:
                        result.update(outcome='failed', error=str(error))
                        page.screenshot(path=str(out / f'{level:02}-failure-{len(cases)}.png'))
                    result['seconds'] = round(time.monotonic() - started, 3)
                    result['page_errors'] = errors
                    cases.append(result)
                    context.close()
                context = browser.new_context(base_url=base, viewport={'width': 1100, 'height': 850})
                page = context.new_page()
                page.goto(f'/apps/{level:02}/main.html')
                expect(page.locator('#greeting')).to_have_text('Hello, World!')
                if level >= 4:
                    greet(page, 'Joe', 'Hello, Joe!')
                page.screenshot(path=str(out / f'{level:02}.png'), full_page=True)
                context.close()
                failed = sum(case['outcome'] == 'failed' for case in cases)
                result_path = out / f'{level:02}.result.json'
                dump(result_path, {'schema': 'warp-external-result-v1', 'kind': 'test', 'outcome': 'failed' if failed else 'passed',
                                  'exit_code': int(bool(failed)), 'tests': {'discovered': len(cases), 'executed': len(cases),
                                  'passed': len(cases)-failed, 'failed': failed, 'skipped': 0}})
                inputs = sorted(p for p in app.rglob('*') if p.is_file() and p.suffix in {'.html', '.css', '.js', '.toml'} or p.is_file() and p.name.endswith('.contract.md'))
                inputs += [ROOT / name for name in ['main.build.py', 'main.server.jl', 'main.server.test.jl', 'main.server.process.py', 'main.test.py', 'requirements.txt', 'Project.toml', 'Manifest.toml']]
                evidence_path = out / f'{level:02}.evidence.json'
                dump(evidence_path, {'schema': 'warp-external-evidence-v1', 'kind': 'test', 'producer': f'Playwright Python {summary["playwright"]}; Chromium {browser.version}',
                    'project': 'main', 'configuration': f'level-{level:02}; headless; 1100x850; loopback API', 'platform': platform.platform(),
                    'executed_at_unix': int(time.time()), 'result_artifact': result_path.relative_to(ROOT).as_posix(), 'result_fingerprint': fingerprint(result_path),
                    'source': {'revision': head, 'dirty': True, 'files': {p.relative_to(ROOT).as_posix(): fingerprint(p) for p in inputs}}})
                queries = {}
                for label, query in [('tree', ['tree', 'main']), ('all', ['files', 'main.**']),
                                     ('greeting', ['files', 'main.greeting.**']), ('current', ['files', '**.current']),
                                     ('trace', ['trace-id', 'main.greeting.sent', '--scope', 'main.**', '--format', 'full'])]:
                    queries[label] = command([args.recur, *query, '-d', app, '--sep', '.', '--json'])
                assessment = command([args.recur, 'warp', 'evidence', evidence_path.relative_to(ROOT), '-d', ROOT, '--json'])
                assert assessment.get('json', {}).get('status') == ('failed' if failed else 'checked'), assessment
                completion = command([args.recur_warp, 'complete', 'main.web-lab', f'level-{level:02}', '--attempt-id', run_id,
                    '--result-hash', fingerprint(result_path), '--evidence', f'browser=evidence:{evidence_path.relative_to(ROOT).as_posix()}', '-d', ROOT, '--json', '--confirm']) if not failed else None
                assert completion is None or completion['exit_code'] == 0, completion
                if not failed:
                    note.rename(app / 'main.acceptance.complete.md') if not (app / 'main.acceptance.complete.md').exists() else note.unlink()
                    (app / 'main.acceptance.complete.md').write_text(f'Observed acceptance in run {run_id}.\nEvidence: ../../{evidence_path.relative_to(ROOT).as_posix()}\nRecorded state only: reassess source fingerprints and parent integration.\n', encoding='utf-8')
                queries['complete'] = command([args.recur, 'files', '**.complete', '-d', app, '--json', '--sep', '.'])
                stage = {'level': level, 'cases': cases, 'queries': queries, 'assessment': assessment, 'completion': completion,
                         'source_file_count': len([p for p in app.iterdir() if p.suffix in {'.html', '.css', '.js'}])}
                summary['stages'].append(stage)
                dump(out / 'run.json', summary)
                print(f'Level {level:02}: {len(cases)-failed}/{len(cases)} passed; evidence {assessment.get("json", {})}', flush=True)
            summary['experiments'] = experiments(args, browser, base, out)
            browser.close()
    finally:
        server.shutdown()
        server.server_close()
    summary['projection'] = command([args.recur, 'warp', 'show', 'main.web-lab', '-d', ROOT, '--json'])
    summary['git_receipt_help'] = command([args.recur.with_name('recur-git.exe'), 'test-receipt', '--help'])
    dump(out / 'run.json', summary)
    dump(ROOT / 'main.results.json', summary)
    print(f'Results: {out / "run.json"}', flush=True)
    assert summary['projection']['exit_code'] == 0, summary['projection']
    return int(any(case['outcome'] != 'passed' for stage in summary['stages'] for case in stage['cases']))


def experiments(args, browser, base, out):
    results = {}
    scratch = ROOT / 'scratch' / out.name
    fixture = scratch / 'parent'
    shutil.copytree(ROOT / 'apps/10', fixture)
    source = fixture / 'main.js'
    original = source.read_text(encoding='utf-8')
    changed = original.replace("new CustomEvent('main.greeting.sent'", "new CustomEvent('main.greeting.lost'")
    assert original != changed
    source.write_text(changed, encoding='utf-8')
    context = browser.new_context(base_url=base)
    page = context.new_page()
    page.goto('/' + fixture.relative_to(ROOT).as_posix() + '/main.html')
    child = page.evaluate("async () => (await import('./main.history.model.js')).append([], 'Hello, Joe!')")
    greet(page, 'Joe', 'Hello, Joe!')
    entries = page.locator('#history li').count()
    page.screenshot(path=str(out / 'parent-wiring-failure.png'))
    results['child_vs_parent'] = {'mutation': "parent emits main.greeting.lost instead of main.greeting.sent", 'child_test_passed': child == ['Hello, Joe!'],
                                'greeting_visible': page.locator('#greeting').inner_text(), 'parent_expected_history_count': 1,
                                'parent_actual_history_count': entries, 'parent_test_passed': entries == 1,
                                'complete_marker_still_exists': (fixture / 'main.acceptance.complete.md').exists()}
    assert child == ['Hello, Joe!'] and entries == 0
    source.write_text(original, encoding='utf-8')
    page.reload()
    greet(page, 'Joe', 'Hello, Joe!')
    expect(page.locator('#history li')).to_have_count(1)
    results['child_vs_parent']['repair_passed'] = True
    context.close()
    # Independent evidence fixture uses an actual passing browser outcome as its observation.
    evidence = json.loads((out / '10.evidence.json').read_text(encoding='utf-8'))
    evidence['source']['files'] = {source.relative_to(ROOT).as_posix(): fingerprint(source)}
    evidence['configuration'] = 'deliberately narrow parent-source-only scope; scope-gap experiment'
    fixture_evidence = scratch / 'narrow.evidence.json'
    dump(fixture_evidence, evidence)
    def assess():
        return command([args.recur, 'warp', 'evidence', fixture_evidence.relative_to(ROOT), '-d', ROOT, '--json'])
    results['fresh'] = assess()
    source.write_text(changed, encoding='utf-8')
    results['source_changed'] = assess()
    source.write_text(original, encoding='utf-8')
    unlisted = fixture / 'main.history.model.js'
    unlisted.write_text('export function append() { return []; }\n', encoding='utf-8')
    results['unlisted_dependency_changed'] = assess()
    context = browser.new_context(base_url=base)
    page = context.new_page()
    page.goto('/' + fixture.relative_to(ROOT).as_posix() + '/main.html')
    greet(page, 'Dependency Joe', 'Hello, Dependency Joe!')
    results['unlisted_dependency_runtime'] = {'expected_history_count': 1, 'actual_history_count': page.locator('#history li').count()}
    assert results['unlisted_dependency_runtime']['actual_history_count'] == 0
    context.close()
    assert results['fresh']['json']['status'] == 'checked'
    assert results['source_changed']['json']['status'] == 'stale'
    assert results['unlisted_dependency_changed']['json']['status'] == 'checked'
    # Contract change must invalidate completion even if the browser result is intact.
    map_path = ROOT / 'main.web-lab.warp-map.json'
    saved_map = map_path.read_bytes()
    mapping = json.loads(saved_map)
    mapping['required_slices'][9]['contract_hash'] = 'contract:main:v-next'
    dump(map_path, mapping)
    results['contract_changed'] = command([args.recur, 'warp', 'show', 'main.web-lab', '-d', ROOT, '--json'])
    map_path.write_bytes(saved_map)
    # A flat alias loses the semantic feature scope even when its content is unchanged.
    flat = scratch / 'flat'
    flat.mkdir()
    aliases = {}
    for index, path in enumerate(sorted((ROOT / 'apps/10').glob('*.js')), 1):
        name = f'file{index:02}.js'
        shutil.copyfile(path, flat / name)
        aliases[name] = path.name
    results['flat_aliases'] = aliases
    results['flat_greeting_query'] = command([args.recur, 'files', 'main.greeting.**', '-d', flat, '--json', '--sep', '.'])
    results['semantic_greeting_query'] = command([args.recur, 'files', 'main.greeting.**', '-d', ROOT / 'apps/10', '--json', '--sep', '.'])
    results['flat_note'] = 'Discovery-only same-content aliases; imports were not rewritten and this is not an executable alternative app.'
    # Result tampering and failed runs must not satisfy a checked gate.
    copied_result = scratch / 'result.json'
    copied_result.write_bytes((out / '10.result.json').read_bytes())
    evidence['result_artifact'] = copied_result.relative_to(ROOT).as_posix()
    dump(fixture_evidence, evidence)
    copied_result.write_text(copied_result.read_text(encoding='utf-8') + ' ', encoding='utf-8')
    results['result_changed'] = assess()
    assert results['result_changed']['json']['status'] == 'stale'
    failed = json.loads(copied_result.read_text(encoding='utf-8'))
    failed.update(outcome='failed', exit_code=1)
    failed['tests'].update(passed=15, failed=1)
    dump(copied_result, failed)
    evidence['result_fingerprint'] = fingerprint(copied_result)
    dump(fixture_evidence, evidence)
    results['failed_result'] = assess()
    assert results['failed_result']['json']['status'] == 'failed'
    return results


if __name__ == '__main__':
    raise SystemExit(run())
