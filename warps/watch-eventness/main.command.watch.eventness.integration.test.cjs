'use strict';
// trigger: main.command.watch.eventness.integration explicitly selected core coverage
// Standalone: node --test warps/watch-eventness/main.command.watch.eventness.integration.test.cjs
// Fault cases require a sibling test build with --features watch-test-hooks; override WATCH_TEST_BIN.
const test = require('node:test');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const {
  fs, path, assert, ROOT, CORE, WATCH, WARP, UUID, TOPIC, SUB,
  fixture, invoke, success, create, subscribe, notifications, publish, snapshot,
} = require('./main.command.watch.eventness.helpers.cjs');
const FAULT_WATCH = process.env.WATCH_TEST_BIN || path.join(ROOT, 'target', 'watch-fault-tests',
  process.env.RECUR_PROFILE || 'release-safe', 'recur-watch' + (process.platform === 'win32' ? '.exe' : ''));
const hash = value => crypto.createHash('sha256').update(value).digest('hex');
const journal = root => path.join(root, '.recur', 'watch', 'topics', hash(TOPIC), `subscription-${hash(SUB)}.jsonl`);
const setup = t => { const root = fixture(t); create(root); subscribe(root); return root; };
const drainArgs = (max = 10, confirm = true) => ['topic', 'drain', TOPIC, '--id', SUB,
  '--max-events', String(max), ...(confirm ? ['--confirm'] : [])];
const drain = (root, max = 10) => invoke(WATCH, root, drainArgs(max));
function failed(result) {
  assert.notEqual(result.status, 0, `Command unexpectedly succeeded: ${result.stdout}`);
  assert.ok(!result.stdout.trim(), `Failed operation emitted successful output: ${result.stdout}`);
}
function fault(root, point) {
  assert.ok(fs.existsSync(FAULT_WATCH), `Build watch-test-hooks binary or set WATCH_TEST_BIN: ${FAULT_WATCH}`);
  const r = cp.spawnSync(FAULT_WATCH, [...drainArgs(), '-d', root, '--json'], {
    encoding: 'utf8', timeout: 10000, windowsHide: true,
    env: { ...process.env, RECUR_WATCH_TEST_FAULT: point },
  });
  if (r.error) throw r.error;
  return r;
}
function replay(root, sequence) {
  return invoke(WATCH, root, ['topic', 'replay', TOPIC, '--id', SUB, '--sequence', String(sequence)]);
}
function rawPublication(root, name, declaration, body = 'Persisted result') {
  const file = path.join(root, 'eventness', `${name}.txt`);
  fs.writeFileSync(file, `${declaration}\n${body}\n`);
  return file;
}

test('bounded-backlog: max N acknowledges only N publications across process restarts', t => {
  const root = setup(t), expected = Array.from({ length: 7 }, (_, i) => publish(root, `item${i}`).id);
  const observed = [];
  for (const count of [2, 2, 2, 1, 0]) {
    const result = success(drain(root, 2));
    assert.equal(result.length, count);
    observed.push(...notifications({ status: 0, stdout: JSON.stringify(result), stderr: '' }));
  }
  assert.deepEqual(observed.sort(), expected.sort());
  assert.equal(new Set(observed).size, 7);
});

test('bounded-signatures: max-events counts publications and preserves distinct signatures together', t => {
  const root = setup(t), a = `${WARP}.first.ready`, b = `${WARP}.second.ready`;
  rawPublication(root, 'a-grouped', `publish: ${a},${b},${a}`);
  const other = publish(root, 'third');
  const first = success(drain(root, 1));
  assert.equal(first.length, 1);
  assert.deepEqual(first[0].trace_ids, [a, b]);
  assert.deepEqual(notifications(drain(root, 1)), [other.id]);
  assert.deepEqual(notifications(drain(root, 1)), []);
});

test('bounds: invalid max-events fails before changing subscription state', t => {
  const root = setup(t); publish(root);
  for (const max of ['0', '1001', '-1', 'not-a-number']) {
    const before = snapshot(root);
    failed(invoke(WATCH, root, drainArgs(max)));
    assert.deepEqual(snapshot(root), before);
  }
  assert.equal(notifications(drain(root, 1)).length, 1);
});

test('existing-publications: each subscriber begins with existing durable artifacts', t => {
  const root = fixture(t), p = publish(root); create(root); subscribe(root);
  assert.deepEqual(notifications(drain(root)), [p.id]);
  success(invoke(WATCH, root, ['topic', 'subscribe', TOPIC, '--id', 'second', '--confirm']));
  assert.deepEqual(notifications(invoke(WATCH, root, ['topic', 'drain', TOPIC, '--id', 'second',
    '--max-events', '1', '--confirm'])), [p.id]);
  subscribe(root);
  assert.deepEqual(notifications(drain(root)), []);
});

test('binding-drift: recreating a topic cannot replace existing interest or acknowledged history', t => {
  const root = setup(t), p = publish(root);
  assert.deepEqual(notifications(drain(root)), [p.id]);
  fs.mkdirSync(path.join(root, 'other-eventness'));
  for (const [filter, dir] of [[`${WARP}.**`, 'eventness'], [`${WARP}.**.ready`, 'other-eventness']]) {
    const before = snapshot(root);
    failed(invoke(WATCH, root, ['topic', 'create', TOPIC, '--warp', WARP, '--filter', filter,
      '--eventness-dir', dir, '--confirm']));
    assert.deepEqual(snapshot(root), before);
  }
  create(root); subscribe(root);
  assert.deepEqual(notifications(drain(root)), []);
  assert.deepEqual(notifications(replay(root, 1)), [p.id]);
});

test('content-change: same publication and referenced bytes wake without revision metadata', t => {
  const root = setup(t), id = `${WARP}.content.ready`;
  const file = rawPublication(root, 'content', `publish: ${id}\nartifact.refs = ["eventness/blob.bin"]`);
  const attachment = path.join(root, 'eventness', 'blob.bin');
  fs.writeFileSync(attachment, Buffer.from([0, 255, 1]));
  assert.deepEqual(notifications(drain(root)), [id]);
  fs.appendFileSync(file, 'More persisted intelligence\n');
  assert.deepEqual(notifications(drain(root)), [id]);
  fs.writeFileSync(attachment, Buffer.from([0, 255, 2]));
  assert.deepEqual(notifications(drain(root)), [id]);
  assert.deepEqual(notifications(drain(root)), []);
});

test('self-updates: private status and subscription journal never wake their own topic', t => {
  const root = setup(t), p = publish(root);
  assert.deepEqual(notifications(drain(root)), [p.id]);
  const watchDir = path.join(root, '.recur', 'watch');
  fs.writeFileSync(path.join(watchDir, 'recur-watch.coordinator.status.current.md'),
    `state = stopped\nack = accepted\npublish: ${WARP}.private.ready\n`);
  for (let pass = 0; pass < 4; pass++) {
    subscribe(root);
    assert.deepEqual(notifications(drain(root)), []);
  }
});

test('previews: create, subscribe and drain leave all bytes and directories unchanged', t => {
  const root = fixture(t), before = snapshot(root);
  success(invoke(WATCH, root, ['topic', 'create', TOPIC, '--warp', WARP, '--filter', `${WARP}.**.ready`,
    '--eventness-dir', 'eventness']));
  assert.deepEqual(snapshot(root), before);
  assert.ok(!fs.existsSync(path.join(root, '.recur')));
  create(root);
  const created = snapshot(root);
  success(invoke(WATCH, root, ['topic', 'subscribe', TOPIC, '--id', SUB]));
  assert.deepEqual(snapshot(root), created);
  subscribe(root); const p = publish(root), registered = snapshot(root);
  success(invoke(WATCH, root, drainArgs(1, false)));
  assert.deepEqual(snapshot(root), registered);
  assert.deepEqual(notifications(drain(root, 1)), [p.id]);
});

test('pure-query: core watch/warp and topic replay never write state', t => {
  const root = setup(t); publish(root); notifications(drain(root));
  const before = snapshot(root);
  for (const args of [['watch', 'list'], ['watch', 'explain'], ['warp', 'show', WARP],
    ['warp', 'dispatch', WARP]]) success(invoke(CORE, root, args));
  notifications(replay(root, 1));
  assert.deepEqual(snapshot(root), before);
});

test('configured-producer: explicit producer roles control publication discovery', t => {
  const root = fixture(t);
  fs.mkdirSync(path.join(root, '.recur'));
  fs.writeFileSync(path.join(root, '.recur', 'config.toml'),
    '[traits.trace_id]\nenabled = true\nproducer_keywords = "announce"\n');
  create(root); subscribe(root);
  const id = `${WARP}.configured.ready`;
  rawPublication(root, 'ignored', `publish: ${WARP}.ignored.ready`);
  rawPublication(root, 'configured', `announce: ${id}`);
  assert.deepEqual(notifications(drain(root)), [id]);
});

test('invalid-ids: malformed producer declarations fail without acknowledging valid siblings', t => {
  const root = setup(t), p = publish(root);
  for (const id of [`${WARP}..ready`, `${WARP}.bad/name.ready`, `${WARP}.bad-name.ready`]) {
    const file = rawPublication(root, 'invalid', `publish: ${id}`);
    const before = fs.readFileSync(journal(root));
    failed(drain(root));
    assert.deepEqual(fs.readFileSync(journal(root)), before);
    fs.unlinkSync(file);
  }
  assert.deepEqual(notifications(drain(root)), [p.id]);
});

test('foreign-warp: filter match does not turn foreign scope into publication', t => {
  const root = setup(t), p = publish(root);
  publish(root, 'foreign', { warp: 'demo.other', uuid: '01a11724-8143-7837-85b1-5a94bc2a2c0e' });
  assert.deepEqual(notifications(drain(root)), [p.id]);
  assert.deepEqual(notifications(drain(root)), []);
});

test('conflicting-warp: same Warp ID with conflicting UUID cannot advance the cursor', t => {
  const root = setup(t), p = publish(root);
  const conflicting = publish(root, 'conflict', { uuid: '01a11724-8143-7837-85b1-5a94bc2a2c0e' });
  const before = fs.readFileSync(journal(root));
  failed(drain(root));
  assert.deepEqual(fs.readFileSync(journal(root)), before);
  fs.unlinkSync(conflicting.file);
  assert.deepEqual(notifications(drain(root)), [p.id]);
});

test('scope-boundaries: root, private runtime and traversal cannot be Eventness directories', t => {
  const root = fixture(t);
  fs.mkdirSync(path.join(root, '.recur', 'watch'), { recursive: true });
  for (const dir of ['.', '.recur', '.recur/watch', '../', 'eventness/../']) {
    const before = snapshot(root);
    failed(invoke(WATCH, root, ['topic', 'create', TOPIC, '--filter', `${WARP}.**.ready`,
      '--eventness-dir', dir, '--confirm']));
    assert.deepEqual(snapshot(root), before);
  }
});

test('private-reference: publication cannot fingerprint private watch runtime', t => {
  const root = setup(t);
  const privateFile = path.join(root, '.recur', 'watch', 'internal.txt');
  fs.writeFileSync(privateFile, 'Private runtime bookkeeping');
  const p = publish(root, 'private', { refs: ['.recur/watch/internal.txt'] });
  const before = fs.readFileSync(journal(root));
  failed(drain(root));
  assert.deepEqual(fs.readFileSync(journal(root)), before);
  fs.writeFileSync(p.file, fs.readFileSync(p.file, 'utf8').replace('.recur/watch/internal.txt', 'eventness/public.txt'));
  fs.writeFileSync(path.join(root, 'eventness', 'public.txt'), 'Public attachment');
  assert.deepEqual(notifications(drain(root)), [p.id]);
});

test('symlink-escape: linked producer directories and references cannot escape the root', t => {
  const root = setup(t), outside = fixture(t), link = path.join(root, 'eventness', 'linked');
  try { fs.symlinkSync(path.join(outside, 'eventness'), link, process.platform === 'win32' ? 'junction' : 'dir'); }
  catch (error) {
    if (['EPERM', 'EACCES', 'ENOTSUP'].includes(error.code)) return t.skip(`Symlink permission unavailable: ${error.code}`);
    throw error;
  }
  fs.writeFileSync(path.join(outside, 'eventness', 'outside.txt'), 'Outside attachment');
  const p = publish(root, 'linked', { refs: ['eventness/linked/outside.txt'] });
  const before = fs.readFileSync(journal(root));
  failed(drain(root));
  assert.deepEqual(fs.readFileSync(journal(root)), before);
  fs.unlinkSync(p.file);
  failed(invoke(WATCH, root, ['topic', 'create', `${TOPIC}.escape`, '--filter', `${WARP}.**.ready`,
    '--eventness-dir', 'eventness/linked', '--confirm']));
});

for (const point of ['before-commit', 'partial-commit', 'write-failure']) {
  test(`recovery-${point}: unsuccessful drain leaves publication recoverable`, t => {
    const root = setup(t), p = publish(root), before = fs.readFileSync(journal(root));
    const result = fault(root, point); failed(result);
    if (point !== 'write-failure') assert.equal(result.status, 86);
    if (point !== 'partial-commit') assert.deepEqual(fs.readFileSync(journal(root)), before);
    assert.deepEqual(notifications(drain(root)), [p.id]);
    assert.deepEqual(notifications(drain(root)), []);
    assert.deepEqual(notifications(replay(root, 1)), [p.id]);
  });
}

test('recovery-after-commit: durable replay recovers lost stdout without repeating reconciliation', t => {
  const root = setup(t), p = publish(root), result = fault(root, 'after-commit');
  failed(result); assert.equal(result.status, 86);
  const committed = fs.readFileSync(journal(root));
  assert.deepEqual(notifications(replay(root, 1)), [p.id]);
  assert.deepEqual(fs.readFileSync(journal(root)), committed);
  assert.deepEqual(notifications(drain(root)), []);
  assert.deepEqual(notifications(replay(root, 1)), [p.id]);
});

test('journal-history: committed observations append and remain replayable after later revisions', t => {
  const root = setup(t), p = publish(root);
  assert.deepEqual(notifications(drain(root)), [p.id]);
  const first = fs.readFileSync(journal(root));
  const next = publish(root, 'beta');
  assert.deepEqual(notifications(drain(root)), [next.id]);
  const second = fs.readFileSync(journal(root));
  assert.deepEqual(second.subarray(0, first.length), first);
  assert.ok(second.length > first.length);
  assert.deepEqual(notifications(replay(root, 1)), [p.id]);
  assert.deepEqual(notifications(replay(root, 2)), [next.id]);
  const before = snapshot(root); failed(replay(root, 999)); assert.deepEqual(snapshot(root), before);
});

test('torn-tail: read-only replay preserves tail and confirmed drain repairs it before append', t => {
  const root = setup(t), first = publish(root);
  assert.deepEqual(notifications(drain(root)), [first.id]);
  const committed = fs.readFileSync(journal(root));
  fs.appendFileSync(journal(root), '{"interrupted":');
  const torn = fs.readFileSync(journal(root));
  assert.deepEqual(notifications(replay(root, 1)), [first.id]);
  assert.deepEqual(fs.readFileSync(journal(root)), torn);
  const second = publish(root, 'beta');
  assert.deepEqual(notifications(drain(root)), [second.id]);
  const recovered = fs.readFileSync(journal(root));
  assert.deepEqual(recovered.subarray(0, committed.length), committed);
  assert.ok(!recovered.includes(Buffer.from('"interrupted"')));
  assert.deepEqual(notifications(replay(root, 2)), [second.id]);
});

test('corrupt-committed-line: complete journal corruption fails closed', t => {
  const root = setup(t); publish(root); notifications(drain(root));
  fs.appendFileSync(journal(root), '{"broken":"complete line"}\n');
  const before = fs.readFileSync(journal(root));
  failed(drain(root)); failed(replay(root, 1));
  assert.deepEqual(fs.readFileSync(journal(root)), before);
});

test('production-binary: fault injection variable cannot interrupt normal publication', t => {
  const root = setup(t), p = publish(root);
  const result = cp.spawnSync(WATCH, [...drainArgs(), '-d', root, '--json'], {
    encoding: 'utf8', timeout: 10000, windowsHide: true,
    env: { ...process.env, RECUR_WATCH_TEST_FAULT: 'before-commit' },
  });
  if (result.error) throw result.error;
  assert.deepEqual(notifications(result), [p.id]);
});

test('concurrent-drains: OS ownership fence prevents duplicate acknowledgements', async t => {
  const root = setup(t), expected = Array.from({ length: 6 }, (_, i) => publish(root, `parallel${i}`).id);
  const run = () => new Promise((resolve, reject) => {
    const child = cp.spawn(WATCH, [...drainArgs(2), '-d', root, '--json'], { windowsHide: true });
    let stdout = '', stderr = '';
    child.stdout.on('data', chunk => { stdout += chunk; });
    child.stderr.on('data', chunk => { stderr += chunk; });
    child.once('error', reject);
    child.once('close', status => resolve({ status, stdout, stderr }));
  });
  const results = await Promise.all([run(), run(), run()]), observed = [];
  for (const result of results) {
    if (result.status === 0) {
      assert.ok(success(result).length <= 2); observed.push(...notifications(result));
    } else failed(result);
  }
  for (let pass = 0; pass < 4; pass++) observed.push(...notifications(drain(root, 2)));
  assert.deepEqual(observed.sort(), expected.sort());
  assert.equal(new Set(observed).size, observed.length);
});

test('legacy-subscription: file polling still signals unannotated artifacts and exposes query status',
  { timeout: 10000 }, async t => {
    const root = fixture(t);
    const child = cp.spawn(WATCH, ['--id', 'legacy', '--filter', 'main.legacy.**',
      '--poll-framing', '1', '--format', 'json', '-d', root], { windowsHide: true });
    let stdout = '', stderr = '', spawnError;
    const closed = new Promise(resolve => child.once('close', resolve));
    child.once('error', error => { spawnError = error; });
    child.stdout.on('data', chunk => { stdout += chunk; });
    child.stderr.on('data', chunk => { stderr += chunk; });
    try {
      for (let pass = 0; pass < 50 && !stderr.includes('ready (poll mode'); pass++) {
        if (spawnError) throw spawnError;
        assert.equal(child.exitCode, null, stderr);
        await new Promise(resolve => setTimeout(resolve, 20));
      }
      assert.ok(stderr.includes('ready (poll mode'), stderr);
      fs.writeFileSync(path.join(root, 'main.legacy.result.complete.md'), 'Unannotated useful artifact');
      for (let pass = 0; pass < 100 && !stdout.includes('\n'); pass++) {
        if (spawnError) throw spawnError;
        await new Promise(resolve => setTimeout(resolve, 20));
      }
      const event = JSON.parse(stdout.trim().split(/\r?\n/)[0]);
      assert.equal(event.event_type, 'created');
      assert.ok(event.path.endsWith('main.legacy.result.complete.md'));
      assert.ok(!Object.hasOwn(event, 'trace_ids'));
    } finally {
      if (child.exitCode === null) child.kill();
      await closed;
    }
    const before = snapshot(root), status = success(invoke(CORE, root, ['watch', 'status', 'legacy']));
    assert.equal(status[0].id, 'legacy');
    assert.equal(status[0].ack, 'accepted');
    assert.ok(Number(status[0].events_seen) >= 1);
    assert.deepEqual(snapshot(root), before);
  });

test('bounded-coordinator: real verified worker signals Eventness while review remains pending',
  { timeout: 35000 }, async t => {
    const root = setup(t), workspace = path.join(root, 'worker'); fs.mkdirSync(workspace);
    fs.writeFileSync(path.join(workspace, 'input.txt'), 'stable input');
    const id = `${WARP}.work.ready`, output = path.join(root, 'eventness', 'main.coordinator.work.complete.md');
    const verifier = path.join(workspace, 'verify.cjs');
    fs.writeFileSync(verifier, `const fs=require('node:fs'),assert=require('node:assert/strict');\n` +
      `assert.equal(fs.readFileSync('input.txt','utf8'),'stable input');\n` +
      `fs.writeFileSync(${JSON.stringify(output)},${JSON.stringify(`publish: ${id}\nVerification completed; acceptance remains separate.\n`)});\n`);
    const q = JSON.stringify;
    fs.writeFileSync(path.join(root, '.recur', 'config.toml'), `
[warp.dispatch]
enabled = true
default_host = "fixture"
max_parallel = 1
timeout_seconds = 10
max_attempts = 1
failure_threshold = 1
easy_success_threshold = 2
easy_seconds = 10
[warp.dispatch.hosts.fixture]
enabled = true
program = ${q(process.execPath)}
args = ["-e", "process.stdout.write('deterministic fixture host')"]
reasoning_levels = []
baseline_index = 0
max_parallel = 1
[warp.dispatch.assignments.${q(`${WARP}.work`)}]
workspace = "worker"
context = ["worker/input.txt", "worker/verify.cjs"]
inputs = ["worker/input.txt", "worker/verify.cjs"]
tests = [{program = ${q(process.execPath)}, args = ["verify.cjs"]}]
[watch.dispatch]
poll_seconds = 1
`);
    const baseline = success(invoke(CORE, root, ['warp', 'show', WARP]));
    assert.deepEqual(baseline.completed_slices, []);
    const result = invoke(WATCH, root, ['dispatch', WARP, '--cycles', '2', '--confirm']);
    assert.equal(result.status, 0, result.stderr);
    const passes = result.stdout.trim().split(/\r?\n/).map(line => JSON.parse(line));
    assert.ok(passes.length >= 1 && passes.length <= 2);
    assert.equal(passes[0].pass, 1);
    let observed;
    for (let pass = 0; pass < 80; pass++) {
      observed = success(invoke(CORE, root, ['warp', 'dispatch', WARP]));
      if (observed.attempts.length && observed.attempts.every(a => !['claimed', 'running'].includes(a.state))) break;
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    assert.equal(observed.attempts.length, 1);
    assert.equal(observed.attempts[0].state, 'produced', JSON.stringify(observed));
    assert.ok(observed.attempts[0].test_results.every(r => r.success === true));
    const producer = success(invoke(CORE, root, ['trace-id', id, '--scope', 'main.coordinator.**']));
    assert.equal(producer.identifier, id);
    assert.ok(producer.produce.some(site => site.path.includes('main.coordinator.work.complete.md')));
    const wakes = [];
    for (let cycle = 0; cycle < 3; cycle++) wakes.push(...notifications(drain(root, 1)));
    assert.deepEqual(wakes, [id]);
    const beforeQuery = snapshot(root), progress = success(invoke(CORE, root, ['warp', 'show', WARP]));
    assert.deepEqual(progress.completed_slices, []);
    assert.deepEqual(snapshot(root), beforeQuery);
    assert.ok(!Object.keys(snapshot(root)).some(name => name.endsWith('.warp-layer.json')));
  });
