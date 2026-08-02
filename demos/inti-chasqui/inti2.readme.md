# Inti 2 — What We Want to Build

## Purpose

Build a playable strategy-adventure about becoming a trusted Chasqui within a living road-and-information network.

The player begins as a modern visitor in Cusco, learns why the Qhapaq Ñan and khipus matter, then enters a historical game-fiction layer. They grow from a person who can recognize only a few clues into a master runner and network coordinator.

The game is inspired by the feeling of an Oil Tycoon loop:

~~~text
discover opportunity
  -> understand its value
  -> invest limited resources
  -> operate a route or delivery
  -> receive outcome
  -> unlock a stronger next choice
~~~

The resource being discovered is not oil. It is **knowledge**: route conditions, messages, needs, relationships, supplies, and the ability to interpret more of the khipu-inspired system.

## The desired player experience

We want a player to say:

- “I started as a curious tourist and gradually understood this world.”
- “The khipu tree makes my growth visible and feels unlike a normal skill menu.”
- “A message is not just a quest marker; I must understand it, plan for it, and live with the result.”
- “The capital is reacting to what I learned in the field.”
- “I can see why a road, a tambo, a bridge, food stores, and trusted people matter.”

## The game’s major parts

### 1. Modern tourist prologue

The player arrives in modern Cusco and is introduced through a small number of simple, human choices.

We want:

- airport or arrival framing without a bureaucracy simulation;
- hotel check-in as a quick transition;
- optional first-evening atmosphere;
- a concierge or guide who offers three first-day interests;
- Plaza / modern culture choice;
- museum / khipu-and-road-history choice;
- short Qhapaq Ñan walk / landscape choice;
- a graceful “end vacation” option that can become the chapter transition;
- a reason for the player to care about the road network before the historical layer begins.

The prologue teaches observation, dialogue, choice, and the first visual symbols. It should be brief.

### 2. Historical Chasqui chronicle

The main game opens as a clearly labeled historical-game-fiction chronicle, not as a claim that the player literally travelled through time.

We want:

- a first novice message;
- a mentor, archive keeper, or trusted guide;
- a capital where messages are interpreted and missions are issued;
- villages, tambos, passes, bridges, and routes on small readable maps;
- a player role that grows through reliable service and relationships;
- consequences for both the people waiting for a message and the network that carries it.

### 3. Field loop

The field loop must be fun before the world becomes large.

~~~text
receive a message or lead
  -> inspect the khipu patterns currently understood
  -> choose a route, runner, supplies, and risk posture
  -> travel or dispatch
  -> encounter a route condition, person, or choice
  -> deliver, investigate, or return
  -> bring verified information and consequences to the capital
~~~

We want field decisions involving:

- safe versus fast routes;
- weather and terrain;
- food and rest at tambos;
- bridge condition and repair needs;
- local knowledge that can be uncertain until verified;
- time-sensitive and routine messages;
- communities with needs, trust, and useful information;
- rewards that are knowledge, trust, capability, and network readiness—not only money.

### 4. Capital loop

The capital must turn field information into strategic choices.

~~~text
receive report
  -> interpret what is known
  -> inspect the network and regional needs
  -> allocate limited resources
  -> invest in capacity
  -> dispatch the next mission
~~~

We want an Imperial Ledger that shows:

- available runners and labor;
- food, textiles, tools, animals, and supplies;
- route readiness;
- tambo readiness;
- bridge and pass status;
- local trust and unmet needs;
- known versus uncertain intelligence;
- season, time pressure, and risk;
- ongoing missions and their dependencies.

We do **not** want a generic stock-market screen. The economy is logistics, obligations, care, capacity, and consequence.

### 5. Khipu-inspired message and technology tree

The khipu is both the message interface and the progression interface.

We want:

- cords that represent skill and knowledge families;
- knot, colour, placement, and branch motifs as game-specific symbols;
- locked knots, uncertain knots, understood knots, and mastered knots;
- English explanations that appear as the player learns a symbol;
- messages that initially reveal only the portions the player can read;
- a growing archive of previously understood patterns;
- a player-visible reason that a new capability is locked or available.

The system must be described honestly as **khipu-inspired game grammar**. It must not claim that every historical khipu is completely deciphered or that the game has a universal historical translation key.

The initial cords are:

~~~text
message literacy
route craft
field resilience
logistics
relationships
leadership
~~~

### 6. Player progression

We want the player to progress through these roles:

~~~text
Visitor
  -> Learner
  -> Apprentice Runner
  -> Chasqui
  -> Senior Chasqui
  -> Master Chasqui
~~~

Progress should come from reliable outcomes, responsible planning, verified discoveries, and earned trust.

We do not want pure grinding, faster-running numbers, or wealth alone to determine rank.

### 7. Hardness and heatmap

Every mission has a hardness value from 0 through 63.

~~~text
0..15   routine
16..31  bounded
32..47  demanding
48..63  exceptional
~~~

Hardness explains how demanding a mission is. It does not itself grant permission. Availability requires appropriate khipu unlocks, route access, supplies, network readiness, and player rank.

We want a heatmap that makes the next choices immediately understandable:

~~~text
green = runnable now
amber = one nearby unlock, investment, or verification away
red   = blocked by a clear requirement
~~~

The same model should work for travel routes, messages, capital projects, runner assignments, and later game expansions.

### 8. First playable chapter

We want one small, complete chapter before adding regions.

~~~text
Locations:
  capital
  one village
  one tambo
  one mountain pass

Routes:
  safe long route
  short route through an uncertain pass

Messages:
  food inventory report
  urgent bridge request
  community reply

Khipu patterns:
  destination
  urgency
  supplies

Capital choices:
  provision the tambo
  investigate the pass
  prioritize the bridge
  defer a request

Chapter win:
  complete the three-message chain with no community left unsupported
~~~

This first chapter must teach the actual game verbs:

1. Read part of a khipu.
2. Plan a route.
3. Invest one limited capital resource.
4. Travel or dispatch.
5. Resolve a consequence.
6. Unlock one new knot and one new route choice.

### 9. Future content, after the first chapter works

Only add these when the first chapter is genuinely fun:

- more regional maps;
- seasons and changing terrain;
- additional message categories;
- more runner and relay roles;
- richer tambo planning;
- bridge, storehouse, and route projects;
- community relationships and competing needs;
- optional historical archive material;
- present-day return scenes;
- higher-rank regional coordination;
- a full network map and long-term campaign.

Every new feature must answer: **what new meaningful choice does this add to the field-to-capital loop?**

## Historical and cultural care

We want the game to be exciting without pretending it is a museum exhibit.

- Separate documented history from game-fiction mechanics.
- Use a researched, culturally respectful presentation for places, people, textiles, languages, medicines, and beliefs.
- Do not reduce living traditions or real medicinal/spiritual practices to a “god mode” consumable.
- Use cultural and historical advisors before presenting specific traditions as fact.
- Make the costs of imperial administration visible: players manage people and obligations, not faceless extraction.

## Technology direction

This file maps the game; it does not select the engine yet.

Eventually we want:

- a normal game engine to own scenes, input, rules, saves, sound, and rendering;
- game data for locations, messages, routes, requirements, and outcomes;
- a simple first UI for the khipu, route board, capital ledger, and field map;
- deterministic test scenarios for routes, message decoding, and consequences;
- Recur used, if useful, to coordinate development work and inspect game data—not to replace the game engine.

## Build order

1. Keep refining this game map until the first chapter is unambiguous.
2. Write the first chapter’s exact scenes, choices, and outcomes.
3. Sketch the four essential screens: khipu, route board, field map, capital ledger.
4. Define a tiny data model for locations, routes, messages, requirements, and outcomes.
5. Build a text or simple visual prototype of the first chapter.
6. Test whether the field-to-capital loop is enjoyable.
7. Add artwork, sound, richer research, and larger content only after that proof.

## Definition of success

The first version succeeds when a new player can finish the three-message chapter, understand why the capital choices changed their route options, and want to unlock the next knot on the khipu tree.

