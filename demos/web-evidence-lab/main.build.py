"""Generate ten independent main-rooted web projects. Python standard library only."""
from pathlib import Path
import json

ROOT = Path(__file__).resolve().parent
LEVELS = [
    'Static document', 'Separate presentation', 'Greeting module', 'Input and validation',
    'Localized greeting', 'Persistent preferences', 'Client routing', 'HTTP greeting API',
    'Retry and offline cache', 'Composed history feature',
]


def write(path, content):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('\n'.join(line.rstrip() for line in content.strip().splitlines()) + '\n', encoding='utf-8')


def build():
    write(ROOT / '.recur/config.toml', '[app]\ndir = "."\nsep = "."\n[traversal]\nmax_depth = 12\ndepth_guard = "hard-fail"')
    for level, title in enumerate(LEVELS, 1):
        root = ROOT / 'apps' / f'{level:02}'
        def put(name, content):
            write(root / name, content)
        # This exact config is reproducible; nested .recur directories are private.
        put('.recur/config.toml', '[app]\ndir = "."\nsep = "."\n[traversal]\nmax_depth = 12\ndepth_guard = "hard-fail"')
        form = '''<form id="greet-form"><label>Name <input id="name" maxlength="80" value="World"></label>
          <label>Language <select id="locale"><option value="en">English</option><option value="es">Español</option></select></label>
          <button>Say hello</button></form>''' if level >= 4 else ''
        nav = '<nav><a href="#greeting">Greeting</a> <a href="#about">About</a></nav>' if level >= 7 else ''
        history = '<section><h2>Greeting history</h2><ol id="history"></ol><button id="clear-history">Clear history</button></section>' if level >= 10 else ''
        put('main.html', f'''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
          <title>Main · {level:02} · {title}</title>
          {'<link rel="stylesheet" href="main.style.css">' if level >= 2 else ''}
          <main><p class="eyebrow">MAIN / LEVEL {level:02}</p><h1>{title}</h1>{nav}
          <section id="greeting-panel">{form}<p id="greeting" role="status">{'Hello, World!' if level < 3 else ''}</p>
          <p id="error" role="alert"></p><small id="transport"></small>{history}</section>
          <section id="about-panel" hidden><h2>About main</h2><p>A small project with explicit feature boundaries.</p></section>
          <footer><a href="../../main.report.html">Back to the evidence board</a></footer></main>
          {'<script type="module" src="main.js"></script>' if level >= 3 else ''}</html>''')
        if level >= 2:
            put('main.style.css', '''body{font:18px/1.6 system-ui,sans-serif;background:#101923;color:#e9eef4;margin:0;padding:7vh 6vw}
            main{max-width:760px;margin:auto}h1{font-size:2.6rem;line-height:1.1}a{color:#7ed8ce}nav{display:flex;gap:24px}
            .eyebrow{color:#b5a3ee;letter-spacing:.2em}form{display:flex;gap:16px;flex-wrap:wrap;align-items:end}
            label{display:grid;gap:6px}input,select,button{font:inherit;padding:9px;border-radius:8px;border:1px solid #50647a}
            button{background:#b5a3ee;color:#101923;cursor:pointer}#greeting{font-size:2rem;color:#7ed8ce}#error{color:#ffb9a5}
            footer{margin-top:60px}small{color:#b7c4d0}[hidden]{display:none}''')
        if level >= 3:
            put('main.greeting.view.js', '''// consumes: main.greeting.message formatted greeting
            export function render(message) { document.querySelector('#greeting').textContent = message; }''')
            imports = ["import {render} from './main.greeting.view.js';"]
            setup = []
            if level >= 4:
                put('main.greeting.model.js', '''// defines: main.greeting.name normalized input contract
                export function normalize(value) {
                  const name = value.trim();
                  if (!name) throw new Error('Please enter a name.');
                  if (name.length > 40) throw new Error('Use 40 characters or fewer.');
                  return name;
                }''')
                imports.append("import {normalize} from './main.greeting.model.js';")
            if level >= 5:
                put('main.greeting.locale.js', '''// defines: main.greeting.message locale formatting contract
                const words = {en: 'Hello', es: 'Hola'};
                export function format(name, locale = 'en') { return `${words[locale] || words.en}, ${name}!`; }''')
                imports.append("import {format} from './main.greeting.locale.js';")
            else:
                setup.append("const format = name => `Hello, ${name}!`;")
            if level >= 6:
                put('main.preferences.store.js', '''// defines: main.preferences.selection persistent preference contract
                const key = 'main.preferences.v1';
                export function load() { try { return JSON.parse(localStorage.getItem(key)) || {}; } catch { return {}; } }
                export function save(value) { try { localStorage.setItem(key, JSON.stringify(value)); } catch {} }''')
                imports.append("import {load, save} from './main.preferences.store.js';")
                setup.append("const saved = load(); document.querySelector('#name').value = saved.name || 'World'; document.querySelector('#locale').value = saved.locale || 'en';")
            if level >= 7:
                put('main.navigation.router.js', '''// consumes: main.navigation.route URL fragment
                export function mountRouter() {
                  const route = () => { const about = location.hash === '#about';
                    document.querySelector('#greeting-panel').hidden = about;
                    document.querySelector('#about-panel').hidden = !about; };
                  window.addEventListener('hashchange', route); route();
                }''')
                imports.append("import {mountRouter} from './main.navigation.router.js';")
                setup.append('mountRouter();')
            if level >= 8:
                put('main.greeting.api.client.js', '''// consumes: main.greeting.api.response server greeting contract
                export async function request(name, locale) {
                  const response = await fetch(`/api/greeting?${new URLSearchParams({name, locale})}`);
                  if (!response.ok) throw new Error(`Greeting service unavailable (${response.status}).`);
                  return (await response.json()).message;
                }''')
                imports.append("import {request} from './main.greeting.api.client.js';")
            if level >= 9:
                put('main.greeting.delivery.js', '''// consumes: main.greeting.api.response delivery input
                // produces: main.greeting.message delivered or cached greeting
                import {request} from './main.greeting.api.client.js';
                const cache = new Map();
                export async function deliver(name, locale) {
                  const key = JSON.stringify([name, locale]); let failure;
                  for (let attempt = 0; attempt < 2; attempt++) {
                    try { const message = await request(name, locale); cache.set(key, message); return {message, source:'network'}; }
                    catch (error) { failure = error; }
                  }
                  if (cache.has(key)) return {message:cache.get(key), source:'cache'};
                  throw failure;
                }''')
                imports.append("import {deliver} from './main.greeting.delivery.js';")
            if level >= 10:
                put('main.history.model.js', '''// defines: main.history.entries bounded recent greeting contract
                export function append(items, message) { return [...items, message].slice(-5); }''')
                put('main.history.view.js', '''// consumes: main.greeting.sent accepted greeting event
                // produces: main.history.entries rendered recent greetings
                import {append} from './main.history.model.js';
                export function mountHistory(bus) {
                  let items = [];
                  const draw = () => { document.querySelector('#history').replaceChildren(...items.map(text => {
                    const li = document.createElement('li'); li.textContent = text; return li;
                  })); };
                  bus.addEventListener('main.greeting.sent', event => { items = append(items, event.detail); draw(); });
                  document.querySelector('#clear-history').addEventListener('click', () => { items = []; draw(); });
                }''')
                imports.append("import {mountHistory} from './main.history.view.js';")
                setup.append('const bus = new EventTarget(); mountHistory(bus);')
            action = "const message = format(name, locale);"
            if level == 8:
                action = "const message = await request(name, locale); document.querySelector('#transport').textContent = 'network';"
            elif level >= 9:
                action = "const result = await deliver(name, locale); const message = result.message; document.querySelector('#transport').textContent = result.source;"
            if level >= 4:
                setup.append(f'''{ '// produces: main.greeting.sent parent publishes accepted greeting' if level >= 10 else '// Parent coordinates greeting input and output.' }
                const form = document.querySelector('#greet-form');
                form.addEventListener('submit', async event => {{
                  event.preventDefault(); const error = document.querySelector('#error'); error.textContent = '';
                  form.querySelector('button').disabled = true;
                  try {{
                    const name = normalize(document.querySelector('#name').value);
                    const locale = document.querySelector('#locale').value;
                    {action}
                    render(message);
                    {'save({name, locale});' if level >= 6 else ''}
                    {"bus.dispatchEvent(new CustomEvent('main.greeting.sent', {detail:message}));" if level >= 10 else ''}
                  }} catch (failure) {{ error.textContent = failure.message; }}
                  finally {{ form.querySelector('button').disabled = false; }}
                }});
                render(format(document.querySelector('#name').value, document.querySelector('#locale').value));''')
                if level == 4:
                    setup.append("document.querySelector('#locale').closest('label').hidden = true;")
            else:
                setup.append("render(format('World'));")
            put('main.js', '\n'.join(imports + setup))
        put('main.contract.md', f'''# main — {title}
        Level {level:02} is an independent project; the directory number is not part of its semantic identity.
        The parent owns visible composition. Child tests do not imply parent acceptance.
        Acceptance is executable in ../../main.test.py; each level inherits earlier relevant behavior.
        ''')
        if level >= 4:
            put('main.greeting.contract.md', '''# main.greeting
            Defines: main.greeting.name trimmed nonempty name, at most 40 characters.
            Defines: main.greeting.message plain text, localized at level 05 onward.
            Input must render as text, never executable markup.
            API errors become visible at level 08; level 09 retries once and caches per name and locale.
            ''')
        if level >= 10:
            put('main.history.contract.md', '''# main.history
            Consumes: main.greeting.sent a successful greeting string, dispatched by the parent.
            Keeps the latest five greetings in this tab; clear removes all entries.
            Feature-level append tests and parent event wiring require separate acceptance.
            ''')
    write(ROOT / 'main.levels.json', json.dumps(LEVELS, indent=2))


if __name__ == '__main__':
    build()
