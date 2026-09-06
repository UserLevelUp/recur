"""Real Chromium interaction tests. Install playwright in a disposable venv.
Uses installed Edge (--channel chrome also supported), an ephemeral HTTP port,
and an isolated browser profile. Never calls the user's running server.
"""
import argparse
import functools
import http.server
import json
import threading
import copy
from pathlib import Path
from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parents[1]


def solve(board, limit=2):
    """Independent fixture check; does not use JS or Julia's stored answers."""
    results = []
    def visit():
        if len(results) >= limit:
            return
        best = None
        for r in range(9):
            for c in range(9):
                if board[r][c]:
                    continue
                used = set(board[r]) | {board[i][c] for i in range(9)} | {
                    board[i][j] for i in range(r//3*3, r//3*3+3) for j in range(c//3*3, c//3*3+3)}
                values = set(range(1,10)) - used
                if not values:
                    return
                if best is None or len(values) < len(best[2]):
                    best = (r,c,values)
        if best is None:
            results.append(copy.deepcopy(board))
            return
        r,c,values = best
        for value in sorted(values):
            board[r][c] = value
            visit()
        board[r][c] = 0
    visit()
    return results


def fixture(board):
    solutions = solve(copy.deepcopy(board))
    assert len(solutions) == 1, 'screenshot fixture must be unique'
    solution = solutions[0]
    entries = [{'cell':f'sudoku.r{r+1}.c{c+1}', 'value':solution[r][c],
                'cascade':{'define':[], 'produce':[], 'consume':[], 'trigger':[]}}
               for r in range(9) for c in range(9)]
    preset = {'givens':board, 'gaps':sum(v == 0 for row in board for v in row),
              'grade':{'rubric':'naked-singles-v1', 'label':'ungraded'}}
    return {'schema':'sudoku-playable-v1', 'puzzle_id':'synthetic-screenshot-v1',
            'solution_text':'\n'.join(f"{e['cell']} = {e['value']}" for e in entries),
            'cascades':entries, 'presets':{key:copy.deepcopy(preset) for key in ('easy','medium','hard')}}, solution


def interactions(page):
    board = page.evaluate("async () => (await import('./tests/teaching.test.js')).screenshotBoard")
    package, solution = fixture(board)
    contract_count = page.evaluate("""async pkg => {
      const {parsePlayable} = await import('./js/puzzle.js');
      let count=0;
      for(const mutate of [p=>p.schema='future',p=>p.solution_text='',p=>p.presets.medium.givens[0][0]=8,
          p=>p.presets.medium.gaps=99,p=>p.presets.medium.grade.label='hard',p=>p.cascades.pop(),
          p=>p.cascades[0].value=0,p=>p.presets.medium.givens=[[1]]]) {
        const bad=structuredClone(pkg); mutate(bad);
        let rejected=false; try {parsePlayable(bad,'medium');} catch {rejected=true;}
        if(!rejected) throw new Error('Invalid package accepted'); count++;
      }
      return count;
    }""",package)
    assert contract_count==8
    page.route('**/sudoku.playable.json', lambda route: route.fulfill(json=package))
    page.reload()
    page.wait_for_selector('.sudoku-grid td.empty')
    cell = lambda r,c: page.locator(f'td[data-cell="sudoku.r{r}.c{c}"]')
    content = page.locator('#teaching-content')
    cell(8,9).click()
    assert 'Candidates:' not in content.inner_text() and 'must be' not in content.inner_text()
    page.locator('#help-cell').click()
    assert 'No naked-single proof' in content.inner_text()
    page.locator('#easier-move').click()
    assert cell(8,9).evaluate("e => e.classList.contains('selected')")
    assert cell(2,9).evaluate("e => e.classList.contains('strategy-highlight')")
    assert 'Which digits are missing?' in content.inner_text()
    assert page.locator('#stat-remaining').inner_text() == '19'
    page.locator('#jump-hint').click()
    assert cell(2,9).evaluate('e => e === document.activeElement')
    page.keyboard.press('h')
    assert 'missing 4, 5' in content.inner_text()
    page.locator('#next-hint').focus()
    page.keyboard.press('Enter')
    assert 'column 9 contains 5' in content.inner_text()
    assert page.locator('#next-hint').evaluate('e => e === document.activeElement')
    page.keyboard.press('Enter')
    assert 'must be 4' in content.inner_text()
    assert page.locator('#next-hint').evaluate('e => e === document.activeElement')
    page.locator('#pencil-autofill-btn').click()
    assert page.locator('.pencil-count').inner_text() == '19 cells annotated'
    page.keyboard.press('4')
    assert page.locator('#stat-remaining').inner_text() == '18'
    assert page.locator('.pencil-count').inner_text() == '18 cells annotated'
    assert page.locator('.pencil-entry[data-cell="sudoku.r2.c9"]').count() == 0
    assert page.locator('.strategy-highlight').count() == 0
    # Manual notes survive other correct moves; solved notes disappear.
    cell(8,9).click()
    page.keyboard.press('p'); page.keyboard.press('7'); page.keyboard.press('p')
    assert '(manual)' in page.locator('.pencil-entry[data-cell="sudoku.r8.c9"]').inner_text()
    cell(2,7).click(); page.keyboard.press('5')
    assert '(manual)' in page.locator('.pencil-entry[data-cell="sudoku.r8.c9"]').inner_text()
    # Tentative attempt, blocked query, correction, fresh hint.
    cell(8,9).click(); page.keyboard.press('7')
    assert 'No deeper contradiction has been proved' in content.inner_text()
    page.keyboard.press('h')
    assert 'Clear the tentative attempt' in content.inner_text()
    assert cell(8,9).evaluate("e => e.classList.contains('wrong')")
    page.keyboard.press('Backspace')
    assert not cell(8,9).evaluate("e => e.classList.contains('wrong')")
    page.locator('#show-solution').click()
    assert 'Answer disclosure, not a logical proof' in content.inner_text()
    page.locator('#show-candidates').click()
    assert 'Candidates for' in content.inner_text()
    # Clicks/keys all the way through zero remaining.
    for r in range(1,10):
        for c in range(1,10):
            target = cell(r,c)
            if target.evaluate("e => e.classList.contains('empty')"):
                if not target.evaluate("e => e.classList.contains('selected')"):
                    target.click()
                page.keyboard.press(str(solution[r-1][c-1]))
    assert page.locator('#stat-remaining').inner_text() == '0'
    assert page.locator('.pencil-count').inner_text() == '0 cells annotated'
    assert page.locator('#win-banner').is_visible()
    page.locator('#easier-move').click()
    assert 'No supported naked single' in content.inner_text()
    with page.expect_navigation():
        page.locator('[data-difficulty="hard"]').click()
    page.wait_for_selector('.sudoku-grid td.empty')
    assert page.locator('#stat-remaining').inner_text() == '19'
    assert page.locator('#next-hint').is_hidden()
    # Failed generation keeps this board; successful generation reloads a new package.
    page.route('**/api/generate', lambda route: route.fulfill(status=500, json={'status':'error','message':'controlled failure'}))
    page.locator('#new-puzzle-btn').click()
    page.wait_for_function("document.querySelector('#generate-status').textContent.includes('current puzzle retained')")
    assert page.locator('#stat-remaining').inner_text() == '19'
    page.unroute('**/api/generate')
    package['puzzle_id'] = 'reset-fixture-v2'
    page.route('**/api/generate', lambda route: route.fulfill(json={'status':'ok'}))
    with page.expect_navigation():
        page.locator('#new-puzzle-btn').click()
    page.wait_for_selector('.sudoku-grid td.empty')
    assert page.locator('#next-hint').is_hidden()
    return 'unique fixture, progressive keys/clicks, selection/focus, notes, tentative recovery, finish, difficulty, reset/failure'


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--channel', default='msedge')
    parser.add_argument('--baseline', action='store_true')
    parser.add_argument('--screenshot')
    parser.add_argument('--package', help='Also validate a real Julia-generated package')
    args = parser.parse_args()
    class QuietHandler(http.server.SimpleHTTPRequestHandler):
        def log_message(self, *args):
            pass
    server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), functools.partial(QuietHandler, directory=str(ROOT)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    try:
        with sync_playwright() as p:
            browser = p.chromium.launch(channel=args.channel, headless=True)
            page = browser.new_page(viewport={'width':1100, 'height':900})
            errors = []
            console_errors = []
            page.on('pageerror', lambda error: errors.append(str(error)))
            page.on('console', lambda msg: console_errors.append(msg.text) if msg.type == 'error' else None)
            page.goto(f'http://127.0.0.1:{server.server_port}/')
            page.wait_for_selector('.sudoku-grid td.empty')
            count = page.evaluate("async () => (await import('./tests/teaching.test.js')).runTests()")
            page.locator('.sudoku-grid td.empty').first.click()
            if args.baseline:
                assert 'Candidates:' in page.locator('#cascade-container').inner_text()
                print('Baseline reproduced: selection immediately reveals candidates')
            else:
                assert 'Legacy puzzle:' in page.locator('#puzzle-grade').inner_text()
                print(interactions(page))
                if args.package:
                    pkg = json.loads(Path(args.package).read_text(encoding='utf-8'))
                    for difficulty in ('easy','medium','hard'):
                        given = pkg['presets'][difficulty]['givens']
                        assert len(solve(copy.deepcopy(given))) == 1
                        page.evaluate("async ({pkg,difficulty}) => (await import('./js/puzzle.js')).parsePlayable(pkg,difficulty)", {'pkg':pkg,'difficulty':difficulty})
                    page.unroute('**/sudoku.playable.json')
                    page.route('**/sudoku.playable.json', lambda route: route.fulfill(json=pkg))
                    page.reload(); page.wait_for_selector('.sudoku-grid td.empty')
                    assert 'Julia-validated unique' in page.locator('#puzzle-grade').inner_text()
            if args.screenshot:
                page.screenshot(path=args.screenshot, full_page=True)
            assert not errors, errors
            unexpected = [e for e in console_errors if 'status of 404 (' not in e and '500 (Internal Server Error)' not in e]
            assert not unexpected, unexpected
            print(json.dumps({'browser':browser.version, 'deduction_assertions':count, 'page_errors':errors,
                             'unexpected_console_errors':unexpected, 'expected_legacy_404_or_injected_500':len(console_errors)}))
            browser.close()
    finally:
        server.shutdown()


if __name__ == '__main__':
    main()
