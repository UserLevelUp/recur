# Warp 10 Sub-2: Two-Scene Foundation

## Scope

Consolidate top-level player navigation into one persistent map scene and one
persistent site scene.  Treat dialogue, settings, details, inventory, help,
and pause as overlays.

## Required behavior

- One explicit navigation state owns `map` or `site`, selected site, transition
  state, map transform, and return focus.
- Repeated navigation requests cannot stack scenes or start competing
  transitions.
- Site content is data-driven or composed below the site destination rather
  than multiplying top-level navigation scenes.
- Existing player progress survives the migration or receives a declared save
  migration.

## Acceptance evidence

- Scene/navigation ownership diagram before and after the change.
- Structural checks proving only the two intended top-level destinations.
- Godot runtime receipt entering, leaving, repeating, pausing, and restoring
  both destinations without errors or lost state.
