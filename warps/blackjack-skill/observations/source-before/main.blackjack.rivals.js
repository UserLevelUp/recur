/* publish: demo.blackjack.rivals.browser.seats accessible rival presentation
 * consumer: demo.blackjack.rivals.table.projection public snapshots only
 * consumer: demo.blackjack.rivals.http.revision schema-selected transport version
 * Julia owns scores, actions and settlement. The integrator owns revision checks.
 */
(function () {
  'use strict';

  function accepts(schema) {
    return schema === 'blackjack-web-state-v2' || schema === 'blackjack-web-state-v3';
  }

  // This selects the transport version; it does not validate a whole snapshot.
  function protocolVersion(state) {
    if (!state || !accepts(state.schema)) throw new TypeError('Unsupported blackjack snapshot schema');
    return state.schema === 'blackjack-web-state-v3' ? 3 : 2;
  }

  const value = x => x === null || x === undefined || x === '' ? '—' : String(x);
  const signed = x => typeof x === 'number' && x > 0 ? '+' + x : value(x);

  // Decode public card IDs for display only; never assign blackjack values.
  function cardLabel(card) {
    if (card === null) return 'Face-down card';
    if (!Number.isInteger(card) || card < 1 || card > 52) return 'Unknown card';
    const ranks = ['A', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K'];
    return ranks[(card - 1) % 13] + ' of ' + ['spades', 'hearts', 'clubs', 'diamonds'][Math.floor((card - 1) / 13)];
  }

  function scoreLabel(score) {
    if (!score) return 'Score: —';
    return 'Score: ' + value(score.total) + (score.bust ? ' · Bust' : '') +
      (score.soft ? ' · Soft' : '') + (score.natural ? ' · Blackjack' : '');
  }

  function render(container, rivals) {
    const doc = container.ownerDocument || document;
    function node(tag, className, text) {
      const element = doc.createElement(tag);
      element.className = className;
      if (text !== undefined) element.textContent = text;
      return element;
    }
    const seats = Array.isArray(rivals) ? rivals : [];
    const panels = seats.map((rival, index) => {
      const name = rival.name || 'Rival ' + (index + 1);
      const panel = node('section', 'rival-seat');
      panel.setAttribute('aria-label', 'Computer rival: ' + name);
      panel.append(node('h3', 'rival-heading', name));
      panel.append(node('p', 'rival-detail', 'Bankroll: ' + value(rival.balance) +
        ' chips · Net: ' + signed(rival.net) + ' · In play: ' + value(rival.escrow)));
      panel.append(node('p', 'rival-detail', 'Policy: ' + value(rival.policy)));
      panel.append(node('p', 'rival-detail', 'Outcome: ' + value(rival.result) +
        ' · Round profit: ' + signed(rival.profit)));
      const hands = Array.isArray(rival.hands) ? rival.hands : [];
      if (!hands.length) panel.append(node('p', 'rival-detail', 'No hand in play.'));
      for (const hand of hands) {
        const handPanel = node('section', 'rival-hand');
        handPanel.setAttribute('aria-label', 'Rival hand ' + value(hand.id));
        handPanel.append(node('p', 'rival-detail', scoreLabel(hand.score)));
        const cards = node('ul', 'rival-cards');
        cards.setAttribute('aria-label', 'Rival hand ' + value(hand.id) + ' cards');
        for (const card of hand.cards || []) cards.append(node('li', 'rival-card', cardLabel(card)));
        if (!cards.children.length) cards.append(node('li', 'rival-card', 'No cards dealt.'));
        handPanel.append(cards, node('p', 'rival-detail', 'Bet: ' + value(hand.stake) +
          ' · Status: ' + value(hand.status) + ' · Outcome: ' + value(hand.result) +
          ' · Profit: ' + signed(hand.profit)));
        panel.append(handPanel);
      }
      return panel;
    });
    container.replaceChildren(...panels);
    container.hidden = panels.length === 0;
  }

  const view = { render, accepts, protocolVersion };
  if (typeof window !== 'undefined') window.BlackjackRivalsView = view;
  if (typeof module !== 'undefined' && module.exports) module.exports = view;
}());
