# Warp 10 Sub-5: Conversation and Idea Discovery

## Scope

Complete talking to a person as a gameplay loop whose meaningful outcome is a
discoverable, understandable, persistent idea.

## Required behavior

- Conversations declare nodes, speaker, lines, choices, conditions, effects,
  repeatability, and safe exit behavior.
- Person state remembers meeting, relevant prior topics, branch exhaustion,
  and declared relationship or story flags.
- Ideas have stable identity, player-facing meaning, source, discovery or
  refinement state, and an explicit consequence or use.
- Idea acquisition explains why the conversation produced it, gives immediate
  feedback, persists through save/load, and does not duplicate on replay.
- Dialogue supports readable pacing, fast reveal, advance/skip, choice focus,
  history where appropriate, keyboard/controller navigation, and clean return
  to site control.
- Dialogue retains the person and site as spatial context wherever readability
  permits; idea acquisition is one continuous interaction rather than a chain
  of dialogue, reward, confirmation, and collection screens.
- Any generated dialogue is bounded by authored state transitions and has a
  responsive authored fallback with a declared timeout and recovery signal.

## Acceptance evidence

- Versioned conversation, person-state, and idea contracts with migration
  behavior.
- Focused branching, replay/idempotency, invalid-condition, and save/load
  checks.
- Runtime receipt for one authored conversation with a meaningful choice, one
  idea payoff, repeat interaction, exit/re-entry, and focus restoration.
- Journey trace proving one committed idea grant, bounded presentation depth,
  authored fallback behavior, and return to a usable exploration state.
- Content review showing that the interaction is informative or delightful,
  not only structurally valid.
