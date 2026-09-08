"""Verify the local evidence board selects and loads all ten demos."""
from pathlib import Path
import importlib.util
import json
from playwright.sync_api import sync_playwright, expect

ROOT = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('server', ROOT / 'main.server.process.py')
server_module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(server_module)
server = server_module.make_server()
results = json.loads((ROOT / 'main.results.json').read_text(encoding='utf-8'))
out = ROOT / 'evidence' / results['run_id']
try:
    with sync_playwright() as p:
        browser = p.chromium.launch(executable_path=results['browser_executable'], headless=True)
        page = browser.new_page(base_url=f'http://127.0.0.1:{server.server_port}', viewport={'width': 1440, 'height': 1100})
        errors = []
        page.on('pageerror', lambda error: errors.append(str(error)))
        page.goto(f'http://127.0.0.1:{server.server_port}')
        runtime = page.request.get('/api/server').json()
        assert runtime['runtime'] == 'Julia', runtime
        expect(page.locator('#summary')).to_contain_text('80/80 browser checks passed')
        for index in range(1, 11):
            page.get_by_role('button', name=f'Level {index}', exact=True).click()
            expect(page.locator('#title')).to_contain_text(f'{index:02}')
            expect(page.locator('#app')).to_have_attribute('src', f'apps/{index:02}/main.html')
            expect(page.frame_locator('#app').locator('#greeting')).to_have_text('Hello, World!')
            for selector in ['#open-app', '#screenshot', '#receipt']:
                response = page.request.get(page.locator(selector).get_attribute('href'))
                assert response.ok, (selector, response.status)
        assert page.request.get('main.report.md').ok
        assert not errors, errors
        page.screenshot(path=str(out / 'board.png'), full_page=True)
        (out / 'board-check.json').write_text(json.dumps({'outcome':'passed', 'demo_selections_checked':10,
            'links_checked':31, 'page_errors':errors, 'browser_version':browser.version, 'server':runtime}, indent=2)+'\n', encoding='utf-8')
        browser.close()
        print('Board: ten demos and 31 artifact links passed; no page errors.')
finally:
    server.shutdown()
    server.server_close()
