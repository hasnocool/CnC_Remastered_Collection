# AI agents in the map editor

The map editor can author AI commanders (agents) as part of a scenario. An
agent is a small program written in **AgentScript**, a deterministic language
compiled by the cnc-ai crate. A scenario stores the program, the owning house
and a doctrine, so an edited map is self-contained.

The same crate provides the runtime that executes agents, the LLM/vision oracle
protocol, and the data the ranked ladder will need. This document covers the
authoring flow, the language, the determinism contract, and the integration
boundary with the game simulation.

## What works today

- **Agent Studio** in the map editor: create, duplicate, delete, rename, assign
  a house, pick a doctrine, edit the script, compile-check it, run a
  static-world dry run, and export an LLM request.
- **AgentScript compiler**: lexer, parser, diagnostics with line/column, class
  catalog checks against the active game plugin.
- **Deterministic runtime**: seeded random, bounded loops, fixed instruction
  budget, stateful variables, scheduled and event handlers, serializable
  commands.
- **Scenario storage**: [AIAgents] plus one [AIAgentN] section per agent, with
  scripts hex-encoded so semicolons and newlines survive the INI dialect.
- **Undo/redo**: every studio edit is one SetAgent command; add/delete are
  AddAgent/RemoveAgent commands.
- **LLM/vision bridge**: an oracle request/response JSON protocol, a text
  vision frame (32x24), a prompt builder, and a ready-to-run runner script.
- **Validation**: scenario validation reports script errors, unknown classes,
  duplicate agent ids, unknown owners and two agents sharing a house.
- **Ladder registration**: cnc-ai/src/ladder.rs builds the AgentCard body
  (owner, doctrine, SHA-1 script fingerprint, creator, scenario) for the
  directory's AI ladder. cnc-net-directory already separates human and ai
  controllers, validates the script hash, and filters queues and leaderboards
  by controller.

## What is not wired yet

The game simulation does not consume agent commands yet. The editor and the
runtime library are complete, but translating a command such as Attack(x, y)
into the TD/RA Game APIs, and feeding the resulting Observation back, is the
next milestone. See **Game integration** below.

The ranked AI ladder's registration and directory routing exist, but no
headless match runner feeds agent-vs-agent results into it yet. See **Ranked
ladder**.

## Editor workflow

1. Open the AI Studio with the **AI** toolbar button.
2. **New** creates an agent owned by the current player house with a Balanced
   starter script. **Dup** copies the selected agent under a new id.
3. Edit **Name**, cycle **Owner** (player house, enemy house, unassigned) and
   **Doctrine**. The doctrine only picks the starter template; the script is
   what runs.
4. Click the script area and type. Arrow keys, Home, End, Backspace and Delete
   move the caret; the line numbers turn red on compile errors.
5. **Check** compiles the buffer and warns about class names that are not in the
   active game catalog.
6. **Run** executes 300 ticks against a static observation built from the map:
   your objects are counted, enemy objects become contacts, credits grow by 15
   per tick. It prints commands, runtime diagnostics and missing intent keys.
   If ai_oracle_response.json exists next to the scenario it is loaded first.
7. **LLM** writes ai_oracle_request.json and ai_oracle_prompt.txt next to the
   scenario, including the vision frame; run scripts/ai_oracle.py to produce
   the response, then **Run** again.
8. **Close** or Escape commits the buffer as one undo step. Ctrl+S also commits
   before saving the map.

The studio edits a buffer, not the model. This keeps a typed script from
creating one undo entry per keystroke. Selecting another agent or closing the
studio commits the buffer with SetAgent.

## AgentScript

### Handlers

~~~text
on start { ... }                 # once, when the match begins
every 120 { ... }                # every 120 simulation ticks
on event unit_lost { ... }       # when the host raises the event
on event "attacked" { ... }      # quoted names also work
~~~

The every interval is an expression evaluated when the handler is due, so it
may depend on variables. It is clamped to 1..60000 ticks. A handler that runs
out of instruction budget reports a diagnostic and is skipped for that tick;
the rest of the simulation continues.

### Statements

- let name = expression
- name = expression
- if condition { } else if condition { } else { }
- while condition { } (bounded by the runtime limit)
- a bare call such as harvest()

### Expressions

Integers (i64), booleans, strings, variables, + - * / % == != < <= > >=,
&& || ! (and the words and, or, not), parentheses. Division by zero yields 0.
There is no floating point, no clock, no I/O and no recursion.

### World queries

| Function | Meaning |
|---|---|
| tick(), credits(), power(), power_ratio() | match state |
| threat(), enemies() | danger estimate and visible contacts |
| units(class), buildings(class) | own counts by class name |
| map_width(), map_height(), base_x(), base_y() | geometry |
| enemy_x(i), enemy_y(i) | the i-th visible contact |
| nearest_enemy_x(), nearest_enemy_y() | nearest contact to the base |
| nearest_threat_x(), nearest_threat_y() | nearest contact with threat > 0 |
| stance() | current stance |
| random(n) | seeded deterministic value in 0..n |
| abs(x), min(a,b), max(a,b), clamp(v,lo,hi) | arithmetic |
| intent(key, fallback), intent_text(key, fallback) | advisor values |

### Commands

| Function | Command |
|---|---|
| train("InfantryE1") | queue production |
| build("Refinery") | queue a structure |
| attack(x, y), defend(x, y), move(x, y) | movement orders |
| scout(x, y), retreat() | scouting and withdrawal |
| harvest() | economy order |
| set_stance("aggressive") | stance change |
| say("..."), log("...") | visible chat and runtime log |

Class names are quoted and checked against the active plugin catalog.

### Example

~~~text
# Balanced: defend early, expand, then push.
on start { set_stance("defensive") }

every 120 {
    if credits() < 800 { harvest() }
    if units("InfantryE1") < 4 { train("InfantryE1") }
    if credits() > 2000 { build("Refinery") }
    if threat() > 2 { defend(nearest_threat_x(), nearest_threat_y()) }
    if tick() > 900 { attack(nearest_enemy_x(), nearest_enemy_y()) }
}
~~~

## Determinism contract

The runtime is safe for lockstep multiplayer:

- No wall clock, no ambient randomness, no I/O. random(n) uses a splitmix-style
  generator seeded from the agent id and the match seed.
- Variables live in a BTreeMap, so iteration order cannot leak.
- Every run has a fuel budget (50,000 evaluated expressions) and a loop cap
  (10,000 iterations) per handler invocation.
- Commands are emitted in program order into a Vec.

LLM and vision are deliberately outside this boundary. The host asks the oracle
for values ahead of time, stores them in the match record, and supplies an
IntentCache. A missing value returns the script's fallback. This means a model
timeout, a hallucinated answer, or a different model on another client can
never desync the simulation. For ranked play, the recorded intent log is part
of the replay and must hash identically.

## Scenario format

~~~ini
[AIAgents]
Count=1

[AIAgent1]
Id=1
Name=Iron Fist
Owner=HouseNod
Doctrine=Aggressive
Script=hex:2320416767726573736976650a...
~~~

Count is the number of indexed [AIAgentN] sections. Id is the stable id used by
the editor and ladder. Name/Owner/Doctrine are stored as plain text unless they
contain a semicolon or newline, in which case the editor also hex-encodes them
with the hex: prefix. Script is always hex-encoded because scripts contain both
semicolons and newlines.

## LLM and vision

The oracle protocol is JSON and transport-agnostic. A request carries:

- request_id, agent name, owner, doctrine and objective
- the full Observation
- an optional Vision frame: 32x24 cells using own, enemy, resource and blocked
  markers, rendered as ASCII for text-only models
- the intent slots extracted from the compiled script (key, kind, fallback)

Three files are written next to the scenario:

- ai_oracle_request.json - the structured request
- ai_oracle_prompt.txt - observation + vision + answer schema
- ai_vision.png - the rendered 32x24 perception frame (8x8 pixels per cell)

Then run:

~~~bash
cd RUST
python3 scripts/ai_oracle.py --request maps/ai_oracle_request.json \
    --response maps/ai_oracle_response.json
~~~

The runner talks to any OpenAI-compatible /chat/completions endpoint:

~~~bash
export CNC_LLM_BASE_URL=http://localhost:11434/v1   # Ollama default
export CNC_LLM_MODEL=llama3.1
# or a hosted provider
export CNC_LLM_BASE_URL=https://api.example.com/v1
export CNC_LLM_API_KEY=...
~~~

API keys are read from the environment and never stored in the scenario or the
repository. The runner validates the answer against the requested keys and
types; unknown or mistyped answers are dropped, and omitted keys fall back
inside the script.

The request always carries the text grid. When the oracle runs with
CNC_LLM_VISION=1 it also attaches ai_vision.png as an image_url data part for a
multimodal endpoint. The text grid stays in the prompt so small local models
keep working and token cost stays predictable; the image is additive.

### Prompt-injection note

The observation is data, not instructions, but an LLM sees it as text. Do not
put untrusted player names or chat into the prompt without delimiting them, and
validate every answer against the requested schema. The runner already rejects
unknown keys and wrong value types; the runtime then clamps and validates the
commands.

## Game integration

The runtime API is intentionally small:

~~~rust
let program = cnc_ai::compile(&agent.script)?;
let mut runtime = cnc_ai::AgentRuntime::new(program, seed);
runtime.set_intents(cache);
let start = runtime.on_start(&observation);
let step = runtime.on_tick(&observation);
for command in step.commands { /* translate to Game APIs */ }
~~~

Wiring it into a match requires four pieces:

1. **Map loading.** cnc-tiberian-dawn/src/map.rs (and the RA equivalent) should
   read [AIAgents], compile each script and attach the runtime to the owning
   house. The editor already writes the sections.
2. **Observation builder.** Build Observation from the Game each tick: credits,
   power, stance, unit/building counts by ClassId name, visible contacts,
   threat. The editor's agent_observation is a template for the shape.
3. **Command translation.** Map Train/Build to production queues, Attack/
   Defend/Move/Scout/Retreat to unit orders, Harvest to the harvester order,
   SetStance to the house stance, Say to chat. Reject commands that the house
   cannot execute instead of panicking.
4. **Decision log.** Store the IntentCache used for the match in the replay
   header and include its hash in the handshake. Deterministic fallback values
   are already part of the script.

The current game AI (cnc-tiberian-dawn/src/ai.rs) is production planning plus
timers; an agent can replace it per house or run beside it with commands taking
priority. Keep the old AI as the fallback for houses without an agent.

## Ranked ladder

The ladder is split across two crates:

- cnc-ai/src/ladder.rs turns an authored AgentDefinition into an
  AiLadderRegistration: name, controller "ai", and an AgentCard carrying the
  map-local agent id, owner, doctrine, SHA-1 script fingerprint, creator and
  scenario. This is the JSON posted to the directory.
- cnc-net-directory/src/matchmaking.rs already models Controller (human/ai),
  AgentCard, per-title MMR, leagues, a widening queue and per-controller
  leaderboards. AI profiles never pair with human profiles, and a profile with
  controller "human" may not carry an agent card.

What is still missing is the ranked execution path:

- **Headless runner.** A worker that loads the scenario, attaches both agent
  runtimes, applies commands to the game simulation, records the replay and
  submits the result. The runtime is deterministic and has no window
  dependency, but the command-to-Game translation above has to exist first.
- **Intent log integrity.** Record the IntentCache and its hash in the replay
  header. Re-execute the match and reject the result when the script hash or
  the intent hash differs.
- **Ownership and pairing.** Rank by agent MMR, rate-limit per human owner, and
  avoid pairing two agents of the same owner unless explicitly allowed.
- **Provenance.** Publish model id, oracle version and script hash with every
  result so a run can be audited; in ranked mode an agent may only read the
  recorded cache, never call a model mid-match.
- **Result plumbing.** Reuse RatingChange and LeaderboardRow, and filter by
  Controller::Ai for the AI board. The human tables stay untouched.

Suggested next slice: run one exhibition AI-vs-AI match on a single node with
the static game state, submit the hashes and a replay, and only then connect
the real Game observation/command loop.

## Testing

~~~bash
cd RUST
cargo test -p cnc-ai
cargo test -p cnc-map-editor
cargo run -p cnc-map-editor -- --validate maps/your_scenario.ini
~~~

The cnc-ai tests cover parsing, diagnostics, seeded determinism, loop budgets,
intent fallbacks, vision encoding and oracle validation. The map editor tests
cover agent INI round-trips with semicolons/newlines, validation, undo/redo,
studio typing/navigation, compile checking, dry runs and frame rendering.
