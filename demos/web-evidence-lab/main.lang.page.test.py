"""Browser expectations from main.lang.api.contract.md, authored before the page.

Run against a running Julia server: python main.lang.page.test.py BASE_URL.
Uses the lab's existing Playwright environment; no evidence/receipt writes.
"""
import json
import os
import sys
from playwright.sync_api import sync_playwright, expect

base = sys.argv[1].rstrip('/')
with sync_playwright() as p:
    browser = p.chromium.launch(executable_path=os.environ.get('CHROME_BIN', 'C:/Program Files/Google/Chrome/Application/chrome.exe'))
    page = browser.new_page()
    errors = []
    page.on('pageerror', lambda error: errors.append(str(error)))
    page.goto(base + '/lang.html')
    expect(page.locator('#status')).to_have_text('Choose a capability and scope.')
    page.get_by_label('Capability', exact=True).select_option('inspector')
    page.get_by_label('Scope', exact=True).select_option('model.m')
    page.get_by_role('button', name='Inspect', exact=True).click()
    expect(page.locator('#status')).to_have_text('Report loaded.')
    expect(page.locator('#header')).to_contain_text('model.i(b) = query.o(b)')
    expect(page.locator('#body')).to_contain_text('m(b)')
    expect(page.locator('#footer')).to_contain_text('not-run')
    expect(page.locator('#footer')).to_contain_text('consume demo.lang.inspector.packet')
    expect(page.locator('#footer')).not_to_contain_text('start_byte')
    expect(page.locator('#observed')).to_have_text('No observed test evidence supplied by this API.')
    page.get_by_text('Complete query packet', exact=True).click()
    expect(page.locator('#packet')).to_contain_text('source_hash')
    expect(page.locator('#recorded')).to_contain_text('Recorded state (not acceptance)')

    original = page.request.get(base + '/api/lang?id=greeting&scope=greeting.g').json()
    original['cards'][0]['meaning'] = '<img src=x onerror=alert(1)>'
    original['packet']['header'][0]['meaning'] = '<img src=x onerror=alert(1)>'
    page.route('**/api/lang?*', lambda route: route.fulfill(json=original))
    page.get_by_role('button', name='Inspect', exact=True).click()
    expect(page.locator('#header')).to_contain_text('<img src=x onerror=alert(1)>')
    expect(page.locator('#header img')).to_have_count(0)

    original['cards'] = []
    original['packet']['header'] = []
    page.get_by_role('button', name='Inspect', exact=True).click()
    expect(page.locator('#status')).to_have_text('No functions selected.')
    expect(page.locator('#header')).to_contain_text('No functions selected')

    page.unroute('**/api/lang?*')
    page.route('**/api/lang?*', lambda route: route.fulfill(status=502, json={'error': {'code': 'query-failed', 'message': 'LANG003 fixture diagnostic'}}))
    page.get_by_role('button', name='Inspect', exact=True).click()
    expect(page.locator('#status')).to_contain_text('LANG003 fixture diagnostic')
    expect(page.locator('#report')).to_be_hidden()

    page.unroute('**/api/lang?*')
    pending = []
    page.route('**/api/lang?*', lambda route: pending.append(route))
    page.get_by_role('button', name='Inspect', exact=True).click()
    expect(page.locator('#status')).to_have_text('Loading report…')
    expect(page.get_by_role('button', name='Inspect', exact=True)).to_be_disabled()
    expect(page.get_by_label('Capability', exact=True)).to_be_disabled()
    page.wait_for_timeout(100)
    for route in pending:
        route.fulfill(json=original)
    expect(page.locator('#status')).to_have_text('No functions selected.')
    assert not errors, errors
    browser.close()
print('PASS: real report, aliases/composition, separate evidence, markup as text, empty/error/loading states')
