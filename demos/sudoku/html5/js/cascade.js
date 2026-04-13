/**
 * cascade.js — Right panel renderer.
 *
 * Three modes:
 *   1. Cascade view — show define/produce/consume/trigger (correct placement)
 *   2. Conflict view — explain why a value is wrong (incorrect placement)
 *   3. Hint view — progressive hints for a selected cell
 */

export class CascadePanel {
  constructor(container) {
    this.container = container;
    this._renderEmpty();
  }

  _renderEmpty() {
    this.container.innerHTML = `
      <div class="cascade-placeholder">
        <p>Click a cell and type a digit to play.</p>
        <p>Wrong answers are allowed — the panel will explain why.</p>
        <p class="cascade-hint">Press <kbd>H</kbd> on a selected cell for a hint.<br>
        Each press reveals more of the reasoning.</p>
      </div>
    `;
  }

  // ── Cascade (correct placement) ──────────────────────────────────

  renderCascade(entry) {
    const { cell, value, cascade } = entry;
    const define  = cascade.define  ?? [];
    const produce = cascade.produce ?? [];
    const consume = cascade.consume ?? [];
    const trigger = cascade.trigger ?? [];

    this.container.innerHTML = `
      <div class="panel-section panel-success">
        <div class="cascade-header">
          <span class="cascade-cell">${cell}</span>
          <span class="cascade-eq">=</span>
          <span class="cascade-value">${value}</span>
          <span class="cascade-correct">Correct</span>
        </div>
        <div class="cascade-divider"></div>
        <div class="cascade-roles">
          ${this._roleBlock('define',  define,  'Definition sites')}
          ${this._roleBlock('produce', produce, 'Constraints published')}
          ${this._roleBlock('consume', consume, 'Peers notified')}
          ${this._roleBlock('trigger', trigger, 'Solvers activated')}
        </div>
        ${this._recurPipelineBlock(cell)}
        <div class="cascade-independence-note">
          What you see above was discovered by <code>recur trace-id</code>.<br>
          This game just reads the JSON. That's the independence principle.
        </div>
      </div>
    `;
  }

  _roleBlock(role, items, label) {
    const count = items.length;
    const lines = items.slice(0, 8).map(item =>
      `<li class="cascade-line cascade-line-${role}" title="${esc(item.path)}:${item.line_number}">${esc(item.line)}</li>`
    ).join('');
    const more = count > 8 ? `<li class="cascade-line cascade-more">... and ${count - 8} more</li>` : '';

    return `
      <div class="cascade-role cascade-role-${role}">
        <div class="cascade-role-header">
          <span class="cascade-role-name">${role}</span>
          <span class="cascade-role-count">${count}</span>
          <span class="cascade-role-label">${label}</span>
        </div>
        ${count > 0 ? `<ul class="cascade-lines">${lines}${more}</ul>` : ''}
      </div>
    `;
  }

  // ── Recur pipeline (expandable) ─────────────────────────────────

  _recurPipelineBlock(cellId) {
    const cmd = `recur trace-id "${cellId}" --scope "sudoku.**" --json`;
    return `
      <details class="recur-pipeline">
        <summary class="recur-pipeline-toggle">How recur built this cascade</summary>
        <div class="recur-pipeline-body">
          <div class="pipeline-step">
            <span class="pipeline-step-num">1</span>
            <div class="pipeline-step-content">
              <div class="pipeline-step-title">Generator writes flow events</div>
              <div class="pipeline-step-detail">
                Julia's <code>Generator.jl</code> knows Sudoku rules — row, column, and box constraints.
                For each cell placement, it writes a <code>.flow.txt</code> file with lines like:
              </div>
              <pre class="pipeline-code">${cellId} = [value]
${cellId} publish row.N.constraint
${cellId} publish col.N.constraint
${cellId} publish box.N.constraint
[peer] subscribe ${cellId}
${cellId} trigger solve</pre>
            </div>
          </div>
          <div class="pipeline-step">
            <span class="pipeline-step-num">2</span>
            <div class="pipeline-step-content">
              <div class="pipeline-step-title">recur classifies the content</div>
              <div class="pipeline-step-detail">
                recur reads the flow files and classifies each line by its vocabulary
                (<code>publish</code> → produce, <code>subscribe</code> → consume, <code>=</code> → define, <code>trigger</code> → trigger).
                recur knows nothing about Sudoku — only the vocabulary in <code>.recur/config.toml</code>.
              </div>
              <div class="pipeline-command">
                <code class="pipeline-cmd">${esc(cmd)}</code>
                <button class="pipeline-copy" onclick="navigator.clipboard.writeText('${esc(cmd)}')">copy</button>
              </div>
            </div>
          </div>
          <div class="pipeline-step">
            <span class="pipeline-step-num">3</span>
            <div class="pipeline-step-content">
              <div class="pipeline-step-title">JSON stored, game reads it</div>
              <div class="pipeline-step-detail">
                The cascade JSON captures recur's output. This HTML5 game reads that JSON at page load.
                Zero recur at runtime — the classification was done once, up front.
              </div>
            </div>
          </div>
          <div class="pipeline-independence">
            <strong>The independence principle:</strong> Generator taught the flow structure.
            recur discovered the patterns. Now you can read the cascade without either tool.
            The patterns (define/produce/consume/trigger) are yours to apply anywhere.
          </div>
          <div class="pipeline-recreate">
            <div class="pipeline-step-title">Recreate or build your own</div>
            <div class="pipeline-step-detail">
              <code>julia demos/sudoku/html5/generate.jl</code> regenerates all cascades.<br>
              To build your own puzzle: write flow files with your vocabulary, run <code>recur init</code>,
              configure your verbs in <code>.recur/config.toml</code>, then <code>recur trace-id</code>.
            </div>
          </div>
        </div>
      </details>
    `;
  }

  // ── Conflict (wrong placement) ───────────────────────────────────

  renderConflict(row, col, value, conflicts) {
    const id = `sudoku.r${row}.c${col}`;

    const explanations = conflicts.map(c => `
      <div class="conflict-item conflict-${c.type}">
        <div class="conflict-message">${esc(c.message)}</div>
        <div class="conflict-law">${esc(c.law)}</div>
      </div>
    `).join('');

    this.container.innerHTML = `
      <div class="panel-section panel-error">
        <div class="cascade-header">
          <span class="cascade-cell">${id}</span>
          <span class="cascade-eq">=</span>
          <span class="cascade-value cascade-wrong-value">${value}</span>
          <span class="cascade-incorrect">Conflict</span>
        </div>
        <div class="cascade-divider"></div>
        <div class="conflict-explanation">
          <div class="conflict-title">${value} cannot go here because:</div>
          ${explanations}
        </div>
        <div class="conflict-tip">
          Press <kbd>Backspace</kbd> to clear, or type a different digit.
          <br>Press <kbd>H</kbd> for a hint.
        </div>
      </div>
    `;
  }

  // ── Hint (progressive) ───────────────────────────────────────────

  renderCellInfo(row, col, cands, eyeball) {
    const id = `sudoku.r${row}.c${col}`;
    const countLine = cands.length === 1
      ? 'Naked Single - only one value is still legal here.'
      : `${cands.length} possible values remain.`;

    this.container.innerHTML = `
      <div class="panel-section panel-info">
        <div class="cascade-header">
          <span class="cascade-cell">${id}</span>
          <span class="hint-level-label">Selected</span>
        </div>
        <div class="cascade-divider"></div>
        <div class="hint-lines">
          <div class="hint-line">Candidates: ${esc(cands.join(', '))}</div>
          <div class="hint-line">${esc(countLine)}</div>
        </div>
        ${this._eyeballOrderBlock(eyeball)}
        <div class="hint-tip">
          Press <kbd>H</kbd> to light up the scan order, then keep pressing for deeper hints.
        </div>
      </div>
    `;
  }

  _eyeballOrderBlock(eyeball) {
    if (!eyeball || !eyeball.items || eyeball.items.length === 0) return '';

    let summary = 'These are the cleanest places to scan before asking for a deeper hint.';
    if (eyeball.selectedSummary?.rank && !eyeball.selectedInTop) {
      summary = `Your selected cell currently ranks #${eyeball.selectedSummary.rank}. Scan the cleaner patterns first, then come back here.`;
    } else if (eyeball.selectedInTop) {
      summary = 'Your selected cell is already in the current best scan order. Stay with the pattern, not the answer.';
    }

    const steps = eyeball.items.map(item => `
      <li class="eyeball-step ${item.selected ? 'eyeball-step-selected' : ''}">
        <div class="eyeball-step-row">
          <div class="eyeball-rank">${item.rank}</div>
          <div class="eyeball-main">
            <div class="eyeball-title-row">
              <div class="eyeball-title">${esc(item.title)}</div>
              ${item.selected ? '<span class="eyeball-selected-chip">Selected</span>' : ''}
            </div>
            <div class="eyeball-focus">${esc(item.focusLabel)}</div>
            <div class="eyeball-preview">${esc(item.preview)}</div>
          </div>
          <details class="eyeball-bubble">
            <summary class="eyeball-bubble-toggle" aria-label="Why this step matters">?</summary>
            <div class="eyeball-bubble-body">
              ${this._eyeballBubbleField('Why this first', item.whyThisFirst)}
              ${this._eyeballBubbleField('What to notice', item.whatToNotice)}
              ${this._eyeballBubbleField('Why not elsewhere', item.whyNotElsewhere)}
              ${this._eyeballBubbleField('Next escalation', item.nextEscalation)}
            </div>
          </details>
        </div>
      </li>
    `).join('');

    return `
      <div class="eyeball-order">
        <div class="eyeball-order-header">
          <div class="eyeball-order-title">Eyeball Order</div>
          <div class="eyeball-order-subtitle">Train the scan, not the answer.</div>
        </div>
        <div class="eyeball-summary">${esc(summary)}</div>
        <ol class="eyeball-steps">${steps}</ol>
      </div>
    `;
  }

  _eyeballBubbleField(label, text) {
    return `
      <div class="eyeball-bubble-field">
        <div class="eyeball-bubble-label">${esc(label)}</div>
        <div class="eyeball-bubble-text">${esc(text)}</div>
      </div>
    `;
  }

  renderHint(hintData) {
    const { level, title, lines, strategy, cell, overlayCells, eyeball } = hintData;
    const id = `sudoku.r${cell.row}.c${cell.col}`;
    const hintTip = level >= 4
      ? 'Press <kbd>H</kbd> again to clear hints and restart from Eyeball Order.'
      : 'Press <kbd>H</kbd> again for the next level of detail.';

    const levelLabel = ['Nudge', 'Elimination', 'Candidates', 'Strategy', 'Answer'][level] ?? `Level ${level}`;
    const dots = [0,1,2,3,4].map(i =>
      `<span class="hint-dot ${i <= level ? 'hint-dot-active' : ''}"></span>`
    ).join('');

    // Build overlay legend when strategy-specific cells are highlighted
    let overlayLegend = '';
    if (overlayCells) {
      const parts = [];
      if (overlayCells.strategy && overlayCells.strategy.length > 0) {
        const ids = overlayCells.strategy.map(c => `r${c.row}.c${c.col}`).join(', ');
        parts.push(`<div class="overlay-legend-item overlay-legend-strategy">Pattern cells: ${esc(ids)}</div>`);
      }
      if (overlayCells.eliminates && overlayCells.eliminates.length > 0) {
        const ids = overlayCells.eliminates.map(c => `r${c.row}.c${c.col}`).join(', ');
        parts.push(`<div class="overlay-legend-item overlay-legend-eliminates">Eliminates from: ${esc(ids)}</div>`);
      }
      if (parts.length > 0) {
        overlayLegend = `
          <div class="overlay-legend">
            <div class="overlay-legend-label">Grid overlay</div>
            ${parts.join('')}
          </div>
        `;
      }
    }

    this.container.innerHTML = `
      <div class="panel-section panel-hint">
        <div class="cascade-header">
          <span class="cascade-cell">${id}</span>
          <span class="hint-level-label">${levelLabel}</span>
        </div>
        <div class="hint-progress">${dots}</div>
        <div class="cascade-divider"></div>
        <div class="hint-title">${esc(title)}</div>
        <div class="hint-lines">
          ${lines.map(l => l === '' ? '<br>' : `<div class="hint-line">${esc(l)}</div>`).join('')}
        </div>
        ${strategy ? `
          <div class="hint-strategy">
            <div class="hint-strategy-label">Strategy</div>
            <div class="hint-strategy-text">${esc(strategy)}</div>
          </div>
        ` : ''}
        ${this._eyeballOrderBlock(eyeball)}
        ${overlayLegend}
        <div class="hint-tip">
          ${hintTip}
        </div>
      </div>
    `;
  }

  // ── Win ──────────────────────────────────────────────────────────

  showWin() {
    this.container.innerHTML = `
      <div class="cascade-win">
        <div class="cascade-win-title">Puzzle Solved!</div>
        <div class="cascade-win-subtitle">The cascade is complete.</div>
        <div class="cascade-win-note">
          Every placement was classified by recur<br>
          into define / produce / consume / trigger.<br><br>
          The strategies you used — scanning, elimination,<br>
          naked singles — those are yours now.<br>
          You don't need the hints anymore.
        </div>
      </div>
    `;
  }
}

function esc(str) {
  return String(str)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}
