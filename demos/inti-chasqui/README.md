# Inti Chasqui

## Words-first game map

**Working premise:** the player begins as a present-day visitor in Cusco. A short, approachable first day introduces the living landscape, the Qhapaq Ñan, and the idea of khipus as information objects. The game then opens a historical, game-fiction layer in which the player grows from a novice learner to a master Chasqui who keeps a road-and-information network working.

This is a compact strategy-adventure game, not an open-world tourism simulator, a historical accounting simulator, or a claim to decode every historical khipu.

## Player promise

The player should feel all three of these things:

1. "I understand where I am and why this road network matters."
2. "I can read more of the world each time I learn a khipu pattern."
3. "My decisions at the capital make later journeys and messages possible."

The central fantasy is not accumulating gold. It is becoming trusted with a larger and more difficult communication network.

## The two connected game loops

### Field loop: discovery, delivery, and route judgment

The field loop is the equivalent of prospecting in a tycoon game. The valuable resource is **usable knowledge**, not oil.

```text
receive a message or lead
  -> inspect its known khipu patterns
  -> choose a route and preparation
  -> travel or dispatch through a small connected map
  -> meet a person, assess a condition, or deliver a report
  -> return with verified route intelligence, supplies, trust, or a new pattern
```

Examples of field discoveries:

- a pass is unsafe after rain;
- a tambo has food for one more relay;
- a bridge needs materials before it can carry a cargo;
- a community can support a new relay if its needs are met;
- a knot and colour combination identifies a message destination or urgency;
- a new route is shorter but requires a trained runner or special preparation.

### Capital loop: interpret, invest, and dispatch

At the capital, the player sees the consequence of field information and chooses the next response.

```text
receive report
  -> interpret / archive its known meaning
  -> inspect regional needs and network status
  -> allocate people, supplies, and time
  -> improve a route, tambo, relay, skill, or decoding knowledge
  -> issue the next bounded mission
```

The capital is an **Imperial Ledger** screen, not a modern cash register. Its primary resources are labor availability, food and textile stores, tools, animals, route readiness, local trust, and verified information. Seasonal pressure and shortfalls replace generic stock-price speculation.

## Scene switching

The game alternates among a small number of scene types. Every transition has a clear player reason and a visible result.

```text
Modern arrival
  -> hotel / concierge choice
  -> plaza, museum, or short road walk
  -> transition to Chasqui chronicle
  -> capital ledger
  -> khipu reading
  -> route board
  -> field journey
  -> delivery / discovery
  -> capital ledger
```

### Scene types

| Scene | Player does | It produces |
|---|---|---|
| Modern Cusco prologue | talks, observes, chooses first interest | emotional context and first symbols |
| Transition scene | accepts the historical-game premise | first novice khipu and starting role |
| Capital ledger | reads reports, allocates limited resources, selects mission | a Work Order for the field loop |
| Khipu reading | matches patterns already learned; requests help when uncertain | message meaning, uncertainty, or a blocked clue |
| Route board | selects destination, route, runner, supplies, and risk posture | planned journey |
| Field map | travels, encounters a route condition, delivers or investigates | report, items, trust, and route knowledge |
| Delivery / consequence | resolves the request and shows who benefited or was strained | reward, cost, and next lead |
| Chronicle / archive | reviews learned history and prior decisions | optional learning and save-state clarity |

There is no need to simulate every hotel room, meal, passport form, or road mile. A scene is present only when it teaches a system, expresses the setting, or asks the player for a meaningful decision.

## The khipu-style tech tree

The khipu is the player-facing progression board. It is a **game-specific, khipu-inspired interface**: it visualizes learned meanings and capabilities; it does not pretend that every historical knot sequence has a settled English translation.

Each main cord is a progression family. A knot can be locked, visible but uncertain, understood, or mastered. A new knot becomes available when the player has the prerequisite knowledge, evidence, and capital support.

```text
central cord: player trust and rank
  |- message literacy       destination -> urgency -> category -> compound messages
  |- route craft            known road -> relay route -> pass judgment -> network routing
  |- field resilience       prepare -> highland travel -> hazard response -> master expedition
  |- logistics              supply pack -> tambo support -> relay scheduling -> regional provisioning
  |- relationships          introductions -> community trust -> mutual support -> regional cooperation
  `- leadership             assist -> dispatch -> coordinate -> master Chasqui
```

Unlocks must have gameplay effects. For example:

```text
learn "urgency" pattern
  -> distinguish routine and time-critical messages

fund tambo provisioning
  -> a longer relay route becomes feasible

earn community trust
  -> local route information becomes reliable

master relay scheduling
  -> deliver two dependent messages before a seasonal deadline
```

## Progression: tourist to master Chasqui

| Stage | Role | What the player can do |
|---|---|---|
| 0 | Visitor | observe, ask questions, recognize a few symbols, choose an interest |
| 1 | Learner | read simple destination and supply patterns; make guided local trips |
| 2 | Apprentice runner | choose between safe and fast routes; deliver basic messages; use a tambo |
| 3 | Chasqui | handle relay timing, route hazards, and mixed message categories |
| 4 | Senior Chasqui | allocate small capital support; coordinate dependent deliveries; interpret compound patterns |
| 5 | Master Chasqui | balance the network across regions and seasons; solve high-consequence communication problems |

Rank is earned from reliable delivery, responsible decisions, verified route knowledge, and relationships—not simply from running faster or accumulating currency.

## Difficulty and capability

Every mission has a `hardness` from `0` through `63`. This is a visible planning aid, not a replacement for the player's judgment.

```text
0..15   routine: known route, known message, low consequence
16..31  bounded: one uncertainty or resource decision
32..47  demanding: hazards, dependencies, or partial message knowledge
48..63  exceptional: regional consequence, hard deadline, or multiple joins
```

A mission is available only when its requirements are met. Hardness tells the player how demanding it is; the khipu tech tree tells the player which skills, route permissions, and preparations are available. The heatmap is a view over these facts: green is runnable, amber needs one nearby unlock or investment, and red is currently blocked.

## A first playable slice

Build one satisfying chapter before expanding the world.

```text
Map:       capital, one village, one tambo, one mountain pass
Routes:    safe long road; short pass that is initially uncertain
Messages:  food inventory report; urgent bridge request; community reply
Khipu:     destination, urgency, and supplies pattern families
Choices:   provision tambo, investigate pass, prioritize bridge, defer a request
Win:       complete the three-message chain with no community left unsupported
```

The first chapter teaches all major verbs: inspect a khipu, select a route, make a capital allocation, travel, receive a consequence, and unlock one new capability. It can end with the player becoming an Apprentice Runner; the larger empire remains a promise, not an initial implementation burden.

## Design boundaries

- The modern tourist material is a prologue and framing device, not a travel bureaucracy simulator.
- The historical layer must distinguish documented practice from game fiction.
- Khipu-inspired puzzle grammar and English UI translations are game rules; they must not claim to be complete historical decipherment.
- The game should represent administration as people, obligations, resources, and consequences—not as an uncritical empire-power fantasy.
- Powerful preparations, spiritual practices, and living cultural traditions require research and cultural consultation; they must not be reduced to a generic "drug gives god mode" buff.
- Recur may help coordinate development work and later inspect game data, but it is not the game engine. A normal game engine owns player state, rules, rendering, saves, and scene transitions.

## Future expansions

After the first slice is enjoyable, expand by adding one dimension at a time:

1. more regional maps and seasonal conditions;
2. richer khipu-inspired message families;
3. relay and tambo network planning;
4. community relationships and competing needs;
5. optional historical archive material and present-day return scenes.

The test for every addition is simple: does it create a new meaningful player choice in the field-to-capital loop? If not, it belongs in the archive or waits.
