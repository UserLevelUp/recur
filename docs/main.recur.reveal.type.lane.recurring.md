# Reveal type: lane

defines: main.recur.reveal.type.lane project-defined classification for general work-context capsules

`artifact.type = lane` classifies a Reveal capsule whose primary purpose is to
recover a named work area's intent, scope, attention, evidence and next useful
queries. This is a project-defined flat type, not a built-in execution mode.

A lane's readable hierarchy is its address. Eventness files expand or collapse
attention around that subject; their suffixes record context rather than prove
completion. A lane capsule can retain current work or historical guidance.

Use `warp` when the capsule primarily recovers a declared Warp with a map,
slices and acceptance gates. Use `agent`, `persona` or `skill` when the capsule
primarily describes that artifact. Mentioning an agent or persona in a work
capsule does not change its purpose.

The CLI's `recur lane` scaffolding/listing of named sub-roots is a separate
capability. Assigning this Reveal type does not create a sub-root, configure
execution, establish acceptance or authorize actions. Dotted type names have
no subtype inheritance; `--type lane` matches the exact resolved type.

Each capsule using this convention records an explicit consumer:

```text
artifact.type = lane
consumes: main.recur.reveal.type.lane general work-context capsule classification
```

Rediscover the classification and its users from the repository root:

```powershell
recur reveal --type lane -d .
recur tree main.recur.reveal.type -d docs --sep .
recur trace-id main.recur.reveal.type.lane --scope 'main.**' -d docs --format full
recur trace-id main.recur.reveal.type.lane --scope 'warp-merge.**' -d .recur/warp-merge --format full
```

The separate trace roots recover shared docs and the private warp-merge consumer
without scanning unrelated repository content.

consumes: recur.reveal.artifact-types shared artifact classification and discovery
