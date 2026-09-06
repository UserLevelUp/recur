"""Real Julia HTTP + browser generation test; all generated data is temporary."""
import copy
import json
import os
import socket
import subprocess
import tempfile
import time
import urllib.request
from pathlib import Path
from playwright.sync_api import sync_playwright
from browser_test import ROOT, solve


def main():
    with socket.socket() as sock:
        sock.bind(('127.0.0.1',0))
        port = sock.getsockname()[1]
    with tempfile.TemporaryDirectory(prefix='recur-sudoku-api-') as directory:
        env = {**os.environ,'SUDOKU_PORT':str(port),'SUDOKU_DATA_DIR':directory}
        with tempfile.TemporaryFile(mode='w+b') as log:
            process = subprocess.Popen(['julia',str(ROOT/'serve.jl')],env=env,stdout=log,stderr=log,
                                       creationflags=getattr(subprocess,'CREATE_NO_WINDOW',0))
            try:
                url=f'http://127.0.0.1:{port}'
                deadline=time.monotonic()+60
                while True:
                    try:
                        urllib.request.urlopen(url,timeout=1).close()
                        break
                    except OSError:
                        if process.poll() is not None or time.monotonic()>deadline:
                            log.seek(0); raise RuntimeError(log.read().decode(errors='replace'))
                        time.sleep(.2)
                with sync_playwright() as p:
                    browser=p.chromium.launch(channel='msedge',headless=True)
                    page=browser.new_page()
                    errors=[]
                    page.on('pageerror',lambda e:errors.append(str(e)))
                    page.goto(url)
                    page.wait_for_selector('.sudoku-grid td.empty')
                    assert 'Legacy puzzle:' in page.locator('#puzzle-grade').inner_text()
                    page.locator('#new-puzzle-btn').click()
                    page.wait_for_function("document.querySelector('#puzzle-grade').textContent.includes('Julia-validated unique')",timeout=60000)
                    package=json.loads((Path(directory)/'sudoku.playable.json').read_text())
                    for preset,gaps in [('easy',25),('medium',35),('hard',45)]:
                        page.locator(f'[data-difficulty="{preset}"]').click()
                        page.wait_for_function(f"document.querySelector('#stat-remaining').textContent === '{gaps}'")
                        assert len(solve(copy.deepcopy(package['presets'][preset]['givens'])))==1
                    assert len(package['cascades'])==81
                    assert all(e['cascade']['produce'] and e['cascade']['consume'] for e in package['cascades'])
                    assert not errors,errors
                    print(json.dumps({'real_api':'passed','presets':[25,35,45],
                                      'independent_uniqueness':'passed','recur_cascades':81,'page_errors':errors}))
                    browser.close()
            finally:
                process.terminate()
                process.wait(timeout=10)


if __name__=='__main__':
    main()
