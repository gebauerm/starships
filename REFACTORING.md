# Refactoring Starships — A Resumable, Architecture-First Tutorial

> **What this is:** a self-paced tutorial that turns the current single-file
> `src/main.rs` (529 lines, everything in one place) into a set of small, focused
> modules — each owning one feature of the game. Along the way you will learn the
> Rust and Bevy *idioms* that make a program easy to read, change, and grow.
>
> **What this is not:** a performance guide. We will not optimize anything. We are
> laying the *foundation* (naming, structure, boundaries, types) on which clean
> optimizations and new features can later be built.

---

## Table of contents

- [How this tutorial teaches (the learning concept)](#how-this-tutorial-teaches)
- [The resumability system](#the-resumability-system)
- [Phase 0 · Baseline](#phase-0--baseline)
- [The Rust you will learn (concept map)](#the-rust-you-will-learn-concept-map)
- [Phase 1 · The module split](#phase-1--the-module-split)
  - [Step 1 · `timers.rs`](#step-1--timersrs)
  - [Step 2 · `movement.rs`](#step-2--movementrs)
  - [Step 3 · `combat.rs`](#step-3--combatrs)
  - [Step 4 · `projectiles.rs`](#step-4--projectilesrs)
  - [Step 5 · `ship.rs`](#step-5--shiprs)
  - [Step 6 · `score.rs`](#step-6--scorers)
  - [Step 7 · `main.rs` becomes a skeleton](#step-7--mainrs-becomes-a-skeleton)
- [Phase 2 · Idiomatic cleanup (R1–R8)](#phase-2--idiomatic-cleanup)
- [Phase 3 · Bevy plugins (the architecture layer)](#phase-3--bevy-plugins)
- [Phase 4 · Verification & review](#phase-4--verification--review)
- [Known issues to tackle after this refactor](#known-issues-to-tackle-after-this-refactor)
- [Recap — the lessons](#recap--the-lessons)

---

## How this tutorial teaches

Every step follows the **same shape**, so you always know what to expect:

```
Where you are   →  how to confirm you finished the previous step
Goal            →  what you are building
Concept         →  the Rust/Bevy idea behind it (read this even when it feels familiar)
Do              →  concrete tasks, in order, one feature at a time
Verify          →  commands + what you should see
Commit          →  the exact commit message (this is your progress record)
Done when       →  a checklist proving the step is actually finished
```

Three rules hold for the whole tutorial:

1. **Compile after every step.** You never move code into a half-working state. Each
   step ends in a green `cargo check`, committed to git.
2. **One topic = one commit.** Your git history becomes the tutorial's progress bar.
3. **Move by identifier, not by line number.** Line numbers in this guide are from the
   current snapshot and will drift as you edit. When a step says "move `Health`", use
   your editor's search, not the line number.

> **Python background?** Rust modules ≈ Python packages, but with real privacy
> enforced by the compiler (not a `_`-prefix convention). Rust `traits` ≈ duck
> typing, but checked at *compile time* instead of at runtime. `enum` ≈ Python's
> `Enum` but you can attach *data* to each variant. A "newtype" (`Position(Vec2)`)
> is like wrapping a value in a tiny class purely to get type safety.

---

## The resumability system

You will take breaks — days or weeks. The tutorial is designed so that a cold start
takes **less than 2 minutes**. Everything you need to resume is stored in git.

### Progress tracker

Check the box when a step is done. Keep this table up to date as you go (commit it
along with the code).

| Step | What | Commit message | Done |
|---|---|---|---|
| 0 | Baseline: commit + `cargo check/clippy/run` | `chore: baseline before refactor` | [ ] |
| 1 | `timers.rs` | `refactor(step 1): extract cooldown timers` | [ ] |
| 2 | `movement.rs` | `refactor(step 2): extract movement data + systems` | [ ] |
| 3 | `combat.rs` | `refactor(step 3): extract combat` | [ ] |
| 4 | `projectiles.rs` | `refactor(step 4): extract projectiles` | [ ] |
| 5 | `ship.rs` | `refactor(step 5): extract ship` | [ ] |
| 6 | `score.rs` | `refactor(step 6): extract scoring` | [ ] |
| 7 | `main.rs` skeleton + crate docs | `refactor(step 7): turn main.rs into a skeleton` | [ ] |
| R1–R8 | Idiomatic cleanup | `refactor(R<n>): <what>` | [ ] |
| 8 | Bevy plugins | `refactor(step 8): wrap modules in plugins` | [ ] |
| 9 | Final verification | — | [ ] |

### Resuming after a break

```bash
git status              # 1. uncommitted work? finish it or git stash
git log --oneline -5    # 2. find your last "refactor(step N)" commit → that's your position
cargo check             # 3. confirm a clean baseline before you edit
```

Then open this file at that step and re-read its **"Where you are"** and
**"Concept"** boxes (~2 min). If the step's **"Done when"** boxes are all checked,
move to the next step.

**Why this works:** because every step ends in a committed, compiling state, `git log`
is unambiguous. There is never any "what was I doing?" state to reconstruct.

---

## Phase 0 · Baseline

You only do this once, but it is also the template for every "Verify" later.

**Where you are:** fresh checkout, code as it is today.

**Do:**

1. Commit the current state so you can always undo:
   ```bash
   git add -A && git commit -m "chore: baseline before refactor"
   ```
2. Build and run your three health checks:
   ```bash
   cargo check      # does it compile?
   cargo clippy     # the Rust teacher: hints + warnings
   cargo run        # does it still play?
   ```
3. Record the baseline. `cargo clippy` currently reports **23 warnings**. The
   interesting ones, because they point at real design smells we will fix:

   | Warning | Where | What it means |
   |---|---|---|
   | `unused import` ×3 (`BoundingVolume`, `sprite`, `FRAC_PI_2`) | `main.rs:4-6` | dead imports |
   | `variants BACK and SIDE are never constructed` | `main.rs:64-65` | `Collision` is a fake enum |
   | `statement with no effect` ×2 | `main.rs:429,433` | a **real bug** — see Step 6 |
   | `associated function new is never used` | `main.rs:110` | three overlapping constructors |
   | `constant BOOST_THRUST is never used` | `config.rs:6` | dead constant |
   | `function detect_player_destruction / reset_game is never used` | `main.rs:421,451` | dead systems |
   | `redundant field names`, `needless_borrow`, `collapsible_if` | `main.rs:181,182,401` | small style issues |

4. **Read `main.rs` top to bottom and make a map.** Label each item with its role:
   *component*, *resource*, *event*, *spawning*, *movement*, *shooting*, *collision*,
   *scoring*, *app setup*. You cannot split a file cleanly until you can see its seams.
   Below is the map you should be able to reproduce — try on your own first.

   | Item | Lines | Kind |
   |---|---|---|
   | `Position`, `Facing` | 13–25 | components (movement) |
   | `Health` | 27–28 | component (combat) |
   | `Scored`, `Score` | 30–39 | event + resource (scoring) |
   | `Thrust`, `Velocity` | 41–55 | components (movement) |
   | `Shot` | 57–59 | component (projectiles) |
   | `Collision`, `Collider` | 61–74 | enum + component (combat) |
   | `Ship` | 76–79 | component (ship) |
   | `PlayerColor`, `Shipsprite`, `load_*` | 81–95 | ship look |
   | `PlayerControls` | 97–156 | ship input |
   | `PlayerBundle`, `spawn_player(s)` | 158–305 | ship spawning |
   | `Shoot` | 193–197 | event (shooting) |
   | `BoostDelayTimer`, `BoostDurationTimer`, `ShootDelayTimer` | 199–234 | components (timers) |
   | `vec_from_angle`, `project_positions`, `update_positions`, `enforce_movement_limits` | 236–331 | movement systems |
   | `handle_player_inputs`, `update_ship_velocity` | 333–380 | ship systems |
   | `spawn_shots` | 388–419 | shooting system |
   | `detect_player_destruction`, `update_score`, `reset_game` | 421–458 | scoring systems |
   | `collision_with_shot`, `handle_shot_hits`, `clear_dead_stuff` | 460–501 | combat systems |
   | `main()` | 503–529 | app setup + scheduling |

**Done when:** committed, and you can explain in one sentence what each of the four
"domain clusters" (movement, ship, combat+projectiles, scoring) is responsible for.

---

## The Rust you will learn (concept map)

Read this table once now, then let each step point you back to it. It is the learning
contract of the tutorial: every step teaches a *specific, named* Rust concept.

| Step | Rust concept | Bevy concept |
|---|---|---|
| 1 | modules, `pub` visibility, `mod` declarations | `#[require]` paths |
| 2 | newtype pattern, associated fns vs methods (`&self`) | components + systems co-location |
| 3 | tuple-struct field privacy, `pub(crate)` | queries filtering by component |
| 4 | `use` paths, cross-module imports | events + observers (`On<Shoot>`) |
| 5 | generics (`C: Component`), `impl` blocks | `Bundle`, spawning |
| 6 | `Option`/`Result` handling, dead code | events vs. statements |
| 7 | `//!` vs `///` docs, doc links | the `App` builder as narrative |
| R1 | honest enums (vs. fake enums) | — |
| R2 | `From` trait | — |
| R3 | methods that centralize logic | — |
| R4 | generics + `PhantomData` | one component, three jobs |
| R5 | single source of truth | `#[require]` vs. bundle fields |
| R6 | resources for shared assets | `Res<T>` |
| R7 | de-duplicating by restructuring | — |
| R8 | `clippy`, `rustfmt` | — |
| 8 | the `Plugin` trait | `App::add_plugins` |

---

## Phase 1 · The module split

**Target structure:**

```
src/
├── main.rs           # crate root: docs header, mod decls, App setup + scheduling (thin!)
├── config.rs         # (exists) tuning constants
├── player_config.rs  # (exists) controls, colors, roles
├── timers.rs         # NEW: the three cooldown components
├── movement.rs       # NEW: Position, Facing, Velocity, Thrust + angle math + motion systems
├── combat.rs         # NEW: Health, Collider, collision, damage, cleanup
├── projectiles.rs    # NEW: Shot, spawn_shots
├── ship.rs           # NEW: Ship, PlayerControls, PlayerBundle, spawning, input, physics
└── score.rs          # NEW: Score, Scored, scoring systems
```

A guiding principle for the whole phase:

> **Put data and the systems that act on that data in the same module.
> Expose the smallest possible public surface.**

This is the opposite of "layer by type" (all components in one file, all systems in
another). Features, not layers: when you want to change *shooting*, you open
`projectiles.rs`, and nothing else.

---

### Step 1 · `timers.rs`

**Status: [ ]**

**Where you are:** Phase 0 committed, baseline checks green.

**Goal:** move the three timer components into their own module and make them public.

**Concept — modules & visibility.** Every Rust program is a **crate**; its root is
`main.rs`. Other files are reached through a **module tree** hanging off that root. You
declare a file module in the root with `mod name;`, which pulls in `src/name.rs`.
Modules are **privacy walls**: by default everything is private to its own module. You
opt into sharing with `pub` (everyone), `pub(crate)` (everywhere inside this crate), or
leave it private. **The single biggest beginner trap:** the *field* of a tuple struct is
private even when the struct is `pub`. We'll hit this head-on in Steps 2–3.

**Do:**

1. Create `src/timers.rs` with a `//!` header at the very top:
   ```rust
   //! Cooldown timers shared between ship and projectile systems.
   ```
2. Move `ShootDelayTimer` (`main.rs:224`), `BoostDelayTimer` (`main.rs:199`), and
   `BoostDurationTimer` (`main.rs:212`) — including their `impl … { fn default … }`
   blocks — into it. Add `use bevy::prelude::*;` and `use crate::config;`.
3. Make them visible: each struct and each `timer` field becomes `pub` (for now; you
   will revisit field privacy later). Their `default()` functions become `pub` too.
4. In `main.rs`, add `mod timers;` next to the existing `mod config;`.
5. Fix the `#[require]` attribute on `Ship` (`main.rs:77-78`): those three names now
   live in another module, so they need a path:
   `ShootDelayTimer = timers::ShootDelayTimer::default()`, and the same for the two
   boost timers.
6. Move `tick_timers` (`main.rs:248`) into `timers.rs` as well — it is the "tick all
   timers" system and belongs with the timers. It queries `With<PlayerControls>`, which
   still lives in `main.rs`, so inside `timers.rs` write `use crate::PlayerControls;`.
   (This is the normal "transition phase" — paths get fixed as we move things around.)

**Concept — `#[require]`.** `#[require(Position, ...)]` on a component tells Bevy to
auto-insert those components when the marked one is spawned. It is a *path* like any
other `use`, so it must be updated when types move.

**Verify:**

```bash
cargo check      # compiles
cargo clippy     # same ~23 warnings, nothing new
```

**Commit:** `git add -A && git commit -m "refactor(step 1): extract cooldown timers"`

**Done when:**
- [ ] `src/timers.rs` exists with three `pub` components + `tick_timers`
- [ ] `mod timers;` is declared in `main.rs`
- [ ] `cargo check` is green
- [ ] committed

**If you get stuck:** *"cannot find type `ShootDelayTimer`"* → forgot `mod timers;` or
forgot the `timers::` prefix in `#[require]`. *"`timer` is private"* → forgot `pub` on
the field.

---

### Step 2 · `movement.rs`

**Status: [ ]**

**Where you are:** last commit is `refactor(step 1): …`.

**Goal:** give movement its own home: the components `Position`, `Facing`, `Thrust`,
`Velocity`, the angle math, and the pure motion systems.

**Concept — the newtype pattern.** `struct Position(Vec2);` wraps a `Vec2` in a new type
so the compiler can tell a *position* from a *velocity* from a *thrust*, even though all
three are `Vec2` underneath. This is cheap, idiomatic Rust type safety — it costs nothing
at runtime and stops you passing a velocity where a position is expected. Related
concept: **associated functions** (called `Type::fn()`, no `self`) vs **methods**
(called `value.fn()`, take `&self`). `Facing::to_angle(&self)` is a method because it
reads a value; `Velocity::from_facing(&Facing)` is an associated function because it
builds a new value.

**Do:**

1. Create `src/movement.rs` with a `//!` header explaining the angle convention
   (*"Facing is stored as a rotation quaternion; `to_angle()` gives the signed Z
   rotation in radians."*).
2. Move in the *data*: `Position` (`main.rs:13`), `Facing` + its `to_angle` impl
   (`main.rs:15-25`), `Thrust` (`main.rs:41`), `Velocity` + `from_facing` (`main.rs:45`).
3. Move the angle helper `vec_from_angle` (`main.rs:236`) here and give it a real name
   and doc:
   ```rust
   /// Unit vector pointing along the given angle (radians).
   pub fn direction_from_angle(angle: f32) -> Vec2 {
       Vec2::from_angle(angle).rotate(Vec2::Y).normalize()
   }
   ```
   (`Velocity::from_facing` already used the same expression — we will unify this in R3.)
4. Move the pure motion systems `project_positions` (`main.rs:240`) and
   `update_positions` (`main.rs:382`), plus `enforce_movement_limits` (`main.rs:311`).
   The last one queries `With<Ship>`, which is still in `main.rs`, so
   `use crate::Ship;` inside `movement.rs`.
5. Make the fields visible to other modules: `Position(pub Vec2)`, `Facing(pub Quat)`,
   `Velocity(pub Vec2)`, `Thrust(pub Vec2)`. (Your code already reads `.0` on these
   across systems, so this is the pragmatic choice; R3 will let you hide some of it.)
6. Fix all `#[require]` attributes (`Ship` and `Shot`) to use `movement::Position`, etc.
7. `main.rs`: add `mod movement;`, delete moved items, and add
   `use movement::{Position, Facing, Thrust, Velocity, …};` wherever the remaining code
   still names them. Remember: **a module does not inherit its parent's imports** — every
   file needs its own `use`.

**Verify:**

```bash
cargo check
cargo clippy
```

**Commit:** `git add -A && git commit -m "refactor(step 2): extract movement data + systems"`

**Done when:**
- [ ] `src/movement.rs` has the four components + `direction_from_angle` + three systems
- [ ] `#[require]` on `Ship` and `Shot` use `movement::…` paths
- [ ] `cargo check` green
- [ ] committed

**If you get stuck:** *"field `0` of struct `Position` is private"* → make the field
`pub` (see 1.2 in the concept). *"unresolved import `crate::Ship`"* → `Ship` is still in
`main.rs` (crate root), so it's reachable as `crate::Ship` — that's correct for now.

---

### Step 3 · `combat.rs`

**Status: [ ]**

**Where you are:** last commit is `refactor(step 2): …`.

**Goal:** move combat — health, colliders, collision, damage, cleanup — into one module.

**Concept — feature seam + field privacy.** Combat is a clean *seam*: everything that
reads or writes `Health` and `Collider` is combat. When you move `Health(f32)` here and
`ships`/`shots` construct it elsewhere, you must decide: **`pub` field, or private field
+ accessor method?** There is no universally right answer — the point is to decide
*deliberately and consistently*. Plain-data components often get a `pub` field;
components with invariants (e.g. "health never below 0") get a private field and a
method. Either way, `cargo check` will force you to make every accessor decision
explicitly — that's the value.

**Do:**

1. Create `src/combat.rs` with `//! Combat: hit points, colliders, collision detection.`
2. Move in: `Health` (`main.rs:27`), `Collider` + `half_size()` (`main.rs:68`),
   `Collision` (`main.rs:61`), `collision_with_shot` (`main.rs:460`),
   `handle_shot_hits` (`main.rs:467`), `clear_dead_stuff` (`main.rs:495`).
3. Decide the field visibility for `Health` and `Collider` (see Concept) and apply it
   everywhere they are constructed or read.
4. `handle_shot_hits` queries `With<PlayerControls>` and `With<Shot>` — both still in
   `main.rs`, so `use crate::{PlayerControls, Shot};` for now. It also needs
   `movement::Position`.
5. `main.rs`: `mod combat;`, delete moved items, add `use combat::{Health, Collider, …};`
   where needed. Watch the `#[require]` attributes again (`combat::Health`,
   `combat::Collider`).
6. Note (do not fix yet): `Collision`'s `BACK`/`SIDE` variants are never constructed and
   the three `match` arms in `handle_shot_hits` are identical. That's R1.

**Verify:**

```bash
cargo check
cargo clippy
```

**Commit:** `git add -A && git commit -m "refactor(step 3): extract combat"`

**Done when:**
- [ ] `src/combat.rs` owns `Health`, `Collider`, `Collision`, and the three combat systems
- [ ] you can state your field-visibility decision for `Health`/`Collider` in one sentence
- [ ] `cargo check` green
- [ ] committed

**If you get stuck:** *"field `0` of struct `Health` is private"* → you now understand
the tuple-struct privacy rule (Step 1 concept). Apply your chosen accessor pattern
everywhere, including the `#[require]` defaults.

---

### Step 4 · `projectiles.rs`

**Status: [ ]**

**Where you are:** last commit is `refactor(step 3): …`.

**Goal:** move the `Shot` entity and the shooting system into their own module.

**Concept — cross-module imports and events.** A feature module does *not* own every
type it touches; it imports them. `spawn_shots` reads `movement::Facing`,
`timers::ShootDelayTimer`, and `PlayerColor`/`PlayerControls` (still in `main.rs` for
now). That's normal — imports are how Rust modules communicate. Also note `Shoot` is an
**event** (`#[derive(EntityEvent)]`) and `spawn_shots` is an **observer**
(`On<Shoot>`): Bevy decouples "a player pressed fire" from "a shot spawned" through an
event bus, instead of calling the spawn code directly.

**Do:**

1. Create `src/projectiles.rs` with `//! Projectiles: the Shot entity and how shots spawn.`
2. Move `Shot` (`main.rs:57`), the `Shoot` event (`main.rs:193`), and `spawn_shots`
   (`main.rs:388`).
3. Fix imports inside `spawn_shots`: `use crate::{PlayerControls, PlayerColor};` (still
   in main.rs), `use crate::movement::{Position, Facing};`, `use crate::timers::ShootDelayTimer;`.
4. Update `Shot`'s `#[require]` to fully-qualified paths (`movement::Position`,
   `movement::Thrust`, `movement::Velocity`, `combat::Collider`, `combat::Health`).
5. **Do not** fix the per-shot `asset_server.load("shot.png")` yet — that is R6. But do
   collapse the `if let … { if … }` nesting Clippy flags in `spawn_shots` using the
   `let … && …` style it suggests.
6. `main.rs`: `mod projectiles;`, delete moved items.

**Verify:**

```bash
cargo check
cargo clippy   # warning count should be visibly *lower* now
```

**Commit:** `git add -A && git commit -m "refactor(step 4): extract projectiles"`

**Done when:**
- [ ] `src/projectiles.rs` owns `Shot`, `Shoot`, `spawn_shots`
- [ ] `Shot`'s `#[require]` uses module paths
- [ ] `cargo check` green
- [ ] committed

---

### Step 5 · `ship.rs`

**Status: [ ]**

**Where you are:** last commit is `refactor(step 4): …`.

**Goal:** move the biggest cohesive unit — the ship — into one module.

**Concept — the big cut, and generics.** The ship is one feature with many parts:
identity (`Ship`), input (`PlayerControls`), look (`PlayerColor`, `Shipsprite`), creation
(`PlayerBundle`, `spawn_player(s)`), and its physics (`update_ship_velocity`,
`handle_player_inputs`). Note `spawn_player` is **generic**:
`fn spawn_player<C: Component>(…, role_marker: C)` — the same function can attach either
the `Attacker` or `Defender` marker, so you write the spawn logic once. This is your
first taste of Rust generics (a type parameter constrained by the `Component` trait).

**Do:**

1. Create `src/ship.rs` with `//! The player ship: controls, spawning, and thrust physics.`
2. Move in: `Ship` (`main.rs:76`), `PlayerColor` (`main.rs:81`), `Shipsprite` +
   `load_ship_sprite` + `load_sprites` (`main.rs:84-95`), `PlayerControls` + its impls
   (`main.rs:97-156`), `PlayerBundle` + impl (`main.rs:158-191`), `spawn_player` +
   `spawn_players` (`main.rs:271-305`), `handle_player_inputs` (`main.rs:333`),
   `update_ship_velocity` (`main.rs:361`).
3. The `#[require]` on `Ship` now lists *every* dependency by full path —
   `movement::Position`, `movement::Thrust`, `movement::Velocity`, `combat::Health`,
   `combat::Collider`, `timers::ShootDelayTimer`, etc. This is where you feel the module
   map in your fingers.
4. `spawn_players` reads `crate::player_config::PLAYER_CONFIGS`; `PlayerBundle::new` uses
   `crate::player_config::PlayerConfig` and `window`.
5. Fix the small Clippy nits as you move: `&player_config` → `player_config` (the borrow
   is redundant), and the `controls: controls,` → `controls` field shorthand.
6. `main.rs`: `mod ship;`, delete moved items, and add `use ship::…` where the remaining
   code (score, combat, shooting) references `Ship`, `PlayerControls`, `PlayerColor`.

**Verify:**

```bash
cargo check
cargo clippy   # main.rs should now be *well* under 150 lines
```

**Commit:** `git add -A && git commit -m "refactor(step 5): extract ship"`

**Done when:**
- [ ] `src/ship.rs` owns ship identity, input, look, spawning, and physics
- [ ] `main.rs` is under ~150 lines
- [ ] `cargo check` green
- [ ] committed

**If you get stuck:** confusing "unresolved import" → a type moved but you left a `use`
in the old file, or forgot to add it in the new file. Confusing privacy errors on
`PlayerColor(Color)` → field visibility again (Step 3 concept).

---

### Step 6 · `score.rs`

**Status: [ ]**

**Where you are:** last commit is `refactor(step 5): …`.

**Goal:** move scoring — `Score`, `Scored`, and the scoring systems — into one module.

**Concept — events + resources travel with their systems, and a real bug.** `Score` (a
resource) and `Scored` (an event) and `update_score` (a system) are one *game-rule*
concern. They belong together. **But notice a real bug you are about to move:** in
`detect_player_destruction` (`main.rs:421`), the lines
`Scored { entity: attacker };` are *statements with no effect* — they build an event
value and immediately throw it away. Events must be **sent** (via a trigger/observer),
not constructed. Compounding this, `detect_player_destruction` is commented out of the
schedule, so `Scored` is **never emitted at all** — scoring silently doesn't work today.

**Do:**

1. Create `src/score.rs` with `//! Scoring and match reset.`
2. Move `Score` (`main.rs:35`), `Scored` (`main.rs:30`), `update_score` (`main.rs:437`),
   `reset_game` (`main.rs:451`), and `detect_player_destruction` (`main.rs:421`).
3. **Do not fix the bug in this step** — moving code faithfully first is a skill of its
   own. Instead, mark it honestly: leave a `// TODO:` comment on the no-effect lines
   noting "these construct but never send the event", and add a line to the `//!` header
   that scoring is currently disabled (see "Known issues" below). You will decide the fix
   there.
4. Observe the cross-module communication: `combat.rs` reduces `Health` to 0; `score.rs`
   reacts to that *separately*. Events are exactly the right tool for "combat shouldn't
   know about scoring".
5. `main.rs`: `mod score;`, delete moved items, keep the observers wired as they were.

**Verify:**

```bash
cargo check
cargo clippy   # the "statement with no effect" warnings now live in score.rs — expected
```

**Commit:** `git add -A && git commit -m "refactor(step 6): extract scoring"`

**Done when:**
- [ ] `src/score.rs` owns `Score`, `Scored`, and all three scoring systems
- [ ] the no-effect bug is documented (not silently dropped)
- [ ] `cargo check` green
- [ ] committed

---

### Step 7 · `main.rs` becomes a skeleton

**Status: [ ]**

**Where you are:** last commit is `refactor(step 6): …`.

**Goal:** shrink `main.rs` to its true job — declaring modules, setting up the app, and
scheduling — and document the crate.

**Concept — how thin should an entry point be?** In a well-factored Bevy app, `main.rs`
is a *narrative*: read `main()` top to bottom and you should be able to tell the game's
story. The details live in modules. Also learn the two doc comment kinds: `//!` documents
a **file/module** (top of file), `///` documents a single **item** (directly above it).
Both render into HTML via `cargo doc`.

**Do:**

1. Put a `//!` crate header at the very top of `main.rs`:
   ```rust
   //! A fast-paced 2-player spaceship battle arcade game built with Bevy.
   //!
   //! Crate layout:
   //! - [`ship`]: the player ship, its controls and physics
   //! - [`projectiles`]: shots and the shooting system
   //! - [`combat`]: hit points, colliders, collision resolution
   //! - [`movement`]: kinematics (`Position`, `Facing`, `Velocity`) and motion systems
   //! - [`timers`]: cooldown timers
   //! - [`score`]: scoring and match reset
   //! - [`config`] / [`player_config`]: tuning constants and player bindings
   ```
   The `[`mod`]` forms are **doc links** — rustdoc turns them clickable.
2. Add near the top:
   ```rust
   #![warn(missing_docs)]
   ```
   The compiler now lists every `pub` item that lacks a `///`. Use `warn`, not `deny`,
   so it never blocks a build while you catch up.
3. Read `main()` as a story (Startup actions → the FixedUpdate pipeline → observers).
   Rename any system whose name doesn't say what it does — this is the cheapest moment
   to do it, while the file is tiny.
4. Sweep dead code Clippy found at baseline: delete `BOOST_THRUST` (or leave a TODO to
   wire boost later — see "Known issues"), and remove stale `// TODO` comments that
   describe *features* rather than work you're doing (move them to the README roadmap).
5. Work through `cargo doc --document-private-items` and add the missing `///` docs so
   `missing_docs` goes quiet. Document behavior, not implementation: *"Applies SHOT_DMG
   for a colliding shot, then despawns it"* beats *"loops over shots"*.

**Verify:**

```bash
cargo check
cargo clippy
cargo doc --document-private-items --open   # read your generated docs; fix awkward wording
```

**Commit:** `git add -A && git commit -m "refactor(step 7): turn main.rs into a skeleton"`

**Done when:**
- [ ] `main.rs` is ~40–60 lines: header, mod decls, app setup, scheduling
- [ ] `#![warn(missing_docs)]` produces no *new* warnings for your `pub` items
- [ ] `cargo check` green
- [ ] committed

---

## Phase 2 · Idiomatic cleanup

Now that the code is modular, the duplication and "fake" patterns are easy to see. Each
item is a small, self-contained challenge that teaches one Rust idiom. Do them one at a
time; after each, `cargo check && cargo clippy` and commit with the `refactor(R<n>)`
prefix. **These are the "Rust-native solutions" you asked for.**

### R1 · `Collision` is a fake enum (`combat.rs`)

Clippy says `BACK`/`SIDE` are never constructed, and the `match` in `handle_shot_hits`
has three identical arms. The enum *pretends* richness for a value that is always
`FRONT`. Two honest outcomes — pick one:

- **If you only need "did it hit?":** make `collision_with_shot` return `bool`.
- **If you want directional behavior** (later: armor on the front?): compute it for real —
  the sign of the dot product between the player's facing direction and the vector from
  player to shot. Then the `match` arms legitimately differ.

Either way removes ~25 lines and a mislabeled concept. This is "honest types" — a core
Rust value.

### R2 · `PlayerControls` has three constructors (`ship.rs`)

`new`, `from_config`, and `Default` build the same shape. Idiomatic Rust keeps at most
two, and prefers the `From` trait:

```rust
impl From<&PlayerConfig> for PlayerControls {
    fn from(cfg: &PlayerConfig) -> Self {
        Self { acc: cfg.acc, reverse: cfg.reverse, /* … */ thrust_input: 0., rotation_input: 0. }
    }
}
```

Then `PlayerBundle::new` calls `PlayerControls::from(player_config)` and `new`/`from_config`
disappear. `From` is Rust's standard conversion trait — everything that takes "convert this
into that" can use it. Extra credit: make the input fields private and expose small
accessors, so only `handle_player_inputs` mutates them.

### R3 · The angle→vector math is written three times (`movement.rs`)

`direction_from_angle` duplicates `Vec2::from_angle(angle).rotate(Vec2::Y)` inside
`update_ship_velocity` and `Velocity::from_facing`. Centralize it as a method:

```rust
impl Facing {
    /// Unit vector in the direction the ship points.
    pub fn direction(&self) -> Vec2 {
        direction_from_angle(self.to_angle())
    }
}
```

Then replace all three copies with `facing.direction()` (ship thrust uses it too). Note
this *also* removes the duplicated quaternion decomposition. Methods that centralize
logic are how Rust programs stay honest.

### R4 · Three timers that differ only by a constant (`timers.rs`)

All three are `struct X { timer: Timer }` with a hardcoded duration. Options, easiest →
cleverest:

- **A. Keep three types, share the shape** — give each a `from_seconds(secs)` constructor.
  Minimal gain.
- **B. One generic component** (teaches generics + `PhantomData`):
  ```rust
  #[derive(Component)]
  pub struct Cooldown<M> {
      pub timer: Timer,
      _marker: PhantomData<M>,
  }
  ```
  with marker types `ShootDelay`, `BoostDelay`, `BoostDuration`. The `#[require]` becomes
  `Cooldown<ShootDelay>` etc. This makes the "why are there three identical structs?"
  question disappear, at the cost of a slightly noisier `#[require]`.
- **C. Data-driven** — a single component plus a resource describing the durations.

**If you want the least moving parts, pick B.** Whatever you choose, this removes ~30
lines of near-identical code, and `PhantomData` is worth meeting once.

### R5 · `PlayerBundle` re-declares what `#[require]` already provides (`ship.rs`)

`Ship`'s `#[require]` already auto-inserts `Position`, `Thrust`, `Velocity`, `Health`,
`Collider`, and the timers — but `PlayerBundle` also lists `health`, `shoot_delay_timer`,
`position`, `facing`, etc. The same default is specified twice = a maintenance hazard.
Decide the **single source of truth**: keep only what genuinely varies per player
(`position`, `facing`, `color`) in the bundle, and let `#[require]` fill the rest. If a
value must differ per player, move it into `PlayerConfig` (the `// TODO` already points
this way).

### R6 · Shot texture loaded on every shot (`projectiles.rs`)

`spawn_shots` calls `asset_server.load("shot.png")` on every trigger. The ship sprite,
by contrast, is loaded once into `Shipsprite`. Apply the same pattern:

- Create a `ShotSprite(Sprite)` resource, load it in a Startup system next to
  `load_sprites`, and have `spawn_shots` read `Res<ShotSprite>`.

Resources are Bevy's "shared, loaded-once" data — exactly right for assets.

### R7 · `spawn_players` duplicates the two arms (`ship.rs`)

The two `match` arms differ only in the marker type. Restructure so spawning isn't
copy-pasted — build the bundle once, branch only on the marker. (Advanced alternative:
replace `Attacker`/`Defender` with a single `Role(PlayerRole)` component; see "Known
issues".)

### R8 · Sweep with Clippy & friends

```bash
cargo clippy --all-targets
cargo clippy -- -W clippy::pedantic   # loud but educational; read each, apply what makes sense
cargo fmt
cargo test --doc                      # your doc examples still run
```

Fix what the sweep surfaces: unused imports left behind by the split, `let_and_return`
in `update_ship_velocity`, `too_many_arguments` on `spawn_player`, and the
`unused_assignments` in `player_config::starting_positions` (assign the `match` result
directly instead of the `let (mut pos, mut facing)` dance).

---

## Phase 3 · Bevy plugins

**Status: [ ]**

**Where you are:** Phase 2 complete (`git log --grep="refactor(R"` shows all of R1–R8).

**Goal:** the last architectural move — package each module as a Bevy `Plugin` so
`main.rs` becomes a registration sheet.

**Concept — the `Plugin` trait.** A `Plugin` is Bevy's unit of packaging: a struct that
owns some systems/resources and registers them in one place. It is Rust's answer to
"how do I bundle this feature so another app could reuse it?" Implement the `Plugin`
trait (`fn build(&self, app: &mut App)`) and call `app.add_plugins(MyPlugin)`.

**Do:**

1. For each feature module, add a plugin. Example (`projectiles.rs`):
   ```rust
   pub struct ProjectilesPlugin;

   impl Plugin for ProjectilesPlugin {
       fn build(&self, app: &mut App) {
           app.add_systems(Startup, …)
              .add_observer(spawn_shots);
       }
   }
   ```
2. Think about **where system ordering lives**. Some ordering constraints are local to a
   plugin (register them there with `.before(…)`/`.after(…)`); only cross-plugin order
   (e.g. "movement before combat cleanup") stays in `main.rs` or in a dedicated
   `GamePlugin`.
3. Often a top-level `GamePlugin` registers all the feature plugins in order, so `main.rs`
   is just `app.add_plugins((DefaultPlugins, GamePlugin))`. Do this if it feels natural.
4. Run `cargo check` after each plugin is extracted (keep it compiling).

**Verify:**

```bash
cargo check
cargo run   # identical behavior
```

**Commit:** `git add -A && git commit -m "refactor(step 8): wrap modules in plugins"`

**Done when:**
- [ ] each feature module exposes a `Plugin`
- [ ] `main.rs` is a list of plugin registrations
- [ ] the game plays exactly as before
- [ ] committed

---

## Phase 4 · Verification & review

Final checklist:

```bash
cargo check
cargo clippy                     # ideally 0 warnings
cargo fmt --check
cargo test --doc
cargo doc --open                 # read your generated docs; fix awkward wording
cargo run                        # still plays like before?
cargo run --release
```

**The win condition:** the game behaves *identically*. Refactoring changes shape, not
behavior. If movement, shooting, collisions and scoring feel the same, you preserved
semantics while reorganizing — that's the whole job.

Review questions (this *is* the learning objective — answer them in your commit message
or a note):

1. Can you find every place `Health` is touched by reading one module?
2. Which `pub` items extend beyond the crate by accident? Could any be `pub(crate)`?
3. Did any *behavior* change while you moved code? (It shouldn't have.)
4. Is every public item documented? (`cargo doc` warning-free with `missing_docs` on.)

---

## Known issues to tackle after this refactor

These are *feature* concerns discovered while organizing, not part of this tutorial. They
are deliberately left for later — they're the roadmap your clean foundation enables:

- **Scoring never fires.** `detect_player_destruction` is commented out and its `Scored`
  statements have no effect. Fix: send the event properly (an observer or a trigger from
  the combat cleanup), and order it *before* `clear_dead_stuff` despawns the bodies.
- **`reset_game` is dead.** Wire it to a score event, or delete it.
- **Boost is half-implemented.** `handle_player_inputs` has an empty boost branch and
  `BOOST_THRUST` is unused; the timers exist but nothing applies boost thrust.
- **`detect_player_destruction` uses `Single<…>`** — fine for exactly two players, but it
  would panic if a ship is ever despawned; revisit when you add AI or more players.
- **`handle_shot_hits` is O(players × shots)** nested loops — fine at this scale; optimize
  only if/when you have many entities (the classic "measure before optimizing" trap).

---

## Recap — the lessons

**Organizing Rust code**
- *Features, not layers:* each concern (`ship`, `combat`, `score`, …) in one module with
  its data + logic. `main.rs` tells the story in ~40 lines.
- *Privacy is the tool:* `pub` deliberately, `pub(crate)` by default for game internals;
  tuple-struct *fields* need `pub` too.
- *`crate::` for all cross-module paths;* every module imports what it uses.
- *Newtypes* (`Position(Vec2)`) give compile-time safety for free.
- *Honest types:* don't fake an enum that's always one variant; use `bool` or compute the
  real value.

**Documenting Rust code**
- `///` documents items; `//!` documents files/modules; `//` narrates *why* inside bodies.
- `#![warn(missing_docs)]` turns the compiler into a doc checklist.
- Doc tests (`cargo test --doc`) keep examples honest.

**Architecture in Bevy**
- Reach for `Plugin` when a module feels like a standalone unit; `main.rs` becomes a
  registration sheet.
- Use events/observers for cross-module communication (combat shouldn't know about scoring).
- Use resources for shared, loaded-once data (sprites).

**Further reading** (beginner-friendly, exactly on-topic):
- *Rust Book* ch. 7 "Managing Growing Projects with Packages, Crates, and Modules",
  ch. 14.2, and the rustdoc guide at <https://doc.rust-lang.org/rustdoc/>
- *The Rustdoc Book:* <https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html>
- *The Bevy Book* (systems, plugins, patterns): <https://bevy.org/learn/>
- *Rust by Example* (comments, enums, generics): <https://doc.rust-lang.org/rust-by-example/>
