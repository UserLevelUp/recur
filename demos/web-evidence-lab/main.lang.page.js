'use strict';
const el = id => document.getElementById(id);
const json = value => JSON.stringify(value, null, 2);
let catalog = [];
function option(value, label) {
  const node = document.createElement('option');
  node.value = value;
  node.textContent = label;
  return node;
}
function busy(value) {
  el('capability').disabled = value;
  el('scope').disabled = value || !el('capability').value;
  el('inspect').disabled = value || !el('scope').value;
}
function clearReport() { el('report').hidden = true; }
el('capability').addEventListener('change', () => {
  clearReport();
  const entry = catalog.find(c => c.id === el('capability').value);
  el('scope').replaceChildren(option('', 'Choose a scope'));
  for (const scope of entry?.scopes || []) el('scope').append(option(scope, scope));
  el('status').textContent = 'Choose a capability and scope.';
  busy(false);
});
el('scope').addEventListener('change', () => { clearReport(); busy(false); });
function alias(local, canonical) { return local === canonical ? local : `${local} = ${canonical}`; }
function render(data) {
  const packet = data.packet;
  el('source').textContent = `${packet.source} · ${packet.source_hash}`;
  el('header').replaceChildren();
  for (const card of data.cards) {
    const node = document.createElement('div'); node.className = 'card';
    const title = document.createElement('strong'); title.textContent = card.identity;
    const meaning = document.createElement('p'); meaning.textContent = card.meaning;
    const ports = document.createElement('pre');
    ports.textContent = `${alias(card.input_local, card.input_canonical)} → ${alias(card.output_local, card.output_canonical)}`;
    node.append(title, meaning, ports); el('header').append(node);
  }
  if (!data.cards.length) el('header').textContent = 'No functions selected';
  el('contracts').textContent = packet.contracts.map(c => `${c.canonical_identity} := (${c.fields.map(f => `${f.name}: ${f.type_name}`).join(', ')})`).join('\n') || 'No bundle declarations selected.';
  el('body').textContent = [
    ...(packet.body.flows || []).map(f => `${f.scope} ${f.flow.mode} : ${f.flow.expression}`),
    ...packet.body.boundary_edges.map(e => `${e.producer} → ${e.consumer}`)
  ].join('\n') || 'No composition selected.';
  el('footer').textContent = `Execution: ${packet.footer.execution}\n` + packet.footer.events.map(e => {
    const events = (e.declared_events || []).map(d => `  ${d.edge} ${d.identifier}`).join('\n');
    const t = e.requested_transition;
    const transition = t ? `\n  E0(${t.current}) → dE(${t.slice}) → Ef(${t.desired})` : '';
    return `${e.scope}\n${events}${transition}`;
  }).join('\n\n');
  el('recorded').textContent = 'Recorded state (not acceptance)\n' + packet.footer.events.map(e => `${e.scope}: ${json(e.recorded || [])}`).join('\n');
  el('findings').textContent = `Validation: ${packet.footer.validation}\nWhole source validated: ${packet.coverage.whole_source_validated}\nFindings: ${json(packet.footer.findings)}\nOutside coverage: ${json(packet.coverage.excluded)}`;
  el('observed').textContent = data.observed_evidence.length ? json(data.observed_evidence) : 'No observed test evidence supplied by this API.';
  el('packet').textContent = json(packet);
  el('report').hidden = false;
  el('status').textContent = data.cards.length ? 'Report loaded.' : 'No functions selected.';
}
el('selection').addEventListener('submit', async event => {
  event.preventDefault(); clearReport(); busy(true);
  el('status').textContent = 'Loading report…';
  try {
    const params = new URLSearchParams({id: el('capability').value, scope: el('scope').value});
    const response = await fetch(`/api/lang?${params}`);
    const data = await response.json();
    if (!response.ok) throw new Error(data.error?.message || 'Query failed.');
    render(data);
  } catch (error) {
    clearReport(); el('status').textContent = `Unable to load report: ${error.message}`;
  } finally { busy(false); }
});
(async () => {
  try {
    const response = await fetch('/api/lang');
    if (!response.ok) throw new Error('Catalog unavailable.');
    catalog = (await response.json()).catalog;
    for (const entry of catalog) el('capability').append(option(entry.id, entry.label));
    el('status').textContent = 'Choose a capability and scope.'; busy(false);
  } catch (error) { el('status').textContent = `Unable to load catalog: ${error.message}`; }
})();
