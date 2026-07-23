# Stable API Reference (Level 1)

The complete, per-method reference for the stable mod API. If you have not built a stable mod yet, start with [Stable Native Mods](stable-native-mods.md) — this page assumes you know how to build and install one.

Everything here is plain Rust: traits you implement and safe wrapper types you call. You never write `unsafe` or FFI yourself.

**Contents**: [Units & conventions](#1-units--conventions) · [Registration](#2-registration) · [Gameplay traits](#3-gameplay-traits) · [Simulation context](#4-simulation-context-stablesim) · [Match hook & map customizer](#5-match-hook--map-customizer) · [Client context](#6-client-context-stableclient) · [Server context](#7-server-context-stableserverctx) · [Draft hook](#8-draft-hook-stabledrafthook) · [Player AI](#9-player-input-ai-stableplayerai) · [Value types](#10-value-types) · [Enum codes](#11-enum-codes) · [Availability matrix](#12-availability-matrix) · [Debugging](#13-debugging) · [Pitfalls & FAQ](#14-pitfalls--faq)

Two rules make your DLL survive game updates:

1. **Ignore unknown enum codes.** Wrappers surface codes newer than your build as `None` — skip them.
2. **Write failure-tolerant code.** On an older game, calls into newer features fail gracefully (`None` / `false` / empty), never crash.

A panic inside any of your callbacks is caught by the game and **disables your mod** (the game keeps running) — so handle failures with `?`/defaults instead of `unwrap`.

Contexts (`StableSim`, `StableClient`, …) and borrowed strings (`StrV1`) are valid **only for the duration of the callback** that handed them to you. Never store them, never send them to another thread.

---

## 1. Units & conventions

| Thing | Value |
|---|---|
| Simulation tick | **60 ticks per second** (`tick_per_second` in GameSetting). Every "ticks" value — cooldowns, durations, respawn — uses this clock |
| World coordinates | `u64` fixed integers; the default map is **960000 × 960000**. Champion radius 10000, sight range 130000 |
| Map grid | Walls/bushes/regions are a **30×30 cell grid** (one cell = 32000 world units) |
| Screen coordinates | `f32`; render maps `"UI"`/`"Default"` are **1920×1080**, `"Game"` is match-world space |
| Colors | `u32` packed **0xRRGGBBAA** (opaque white = `0xffffffff`) |
| UI node paths | Dot-separated id selectors: `"body.popup.buttons.close"` |
| JSON document paths | Dot-separated field paths: `"melee_minion.stat.attack"`; numbers index arrays: `"towers.0.pos"`. **Empty path = the whole document** |
| Document writes | Validated by round-tripping the whole document through the game's schema — an invalid write changes **nothing** and returns `false` (atomic reject) |
| Typical magnitudes | Champion base stats run at real-game scale: `move_speed` ~900, `hp` ~900, `attack` ~80, basic-attack `range` ~60000, melee range ~12000. Copy magnitudes from `champion_brief` / the champion sheet, not from thin air |
| Sim determinism | Every callback that touches the simulation (effects, passives, items, match hooks, player AI) must be deterministic: same inputs → same result. Derive randomness only from the `rng_seed` you are handed (add `rand` to your own mod and use `StdRng::seed_from_u64(rng_seed)`). No clocks, no global state. The sim contract carries no floats (draft scoring is the one f32 exception — it lives outside the deterministic zone) |

---

## 2. Registration

### `StableHost` (received once, in your `init`)

| Method | Description |
|---|---|
| `abi_level() -> u32` | The game's contract level |
| `game_version() -> GameVersionV1 { major, minor, patch }` | Game version |
| `log(LogLevel::{Error, Warn, Info, Debug}, &str)` | Write to the game log |
| `register_service(id, ServiceVersionV1, ModServiceV1) -> bool` | Publish a service other native mods can consume |
| `query_service(provider_mod_id, service_id, semver_req) -> Option<ModServiceV1>` | Consume another mod's service (semver requirement string, e.g. `">=1.0.0, <2.0.0"`) |

### `StableMod` builder

| Method | Registers | Implement |
|---|---|---|
| `add_champion` | A champion; **reusing an existing id reworks that champion** | `StableChampion` |
| `add_item` | An item with runtime callbacks | `StableItem` |
| `add_native_effect(name, effect)` | A named effect — referenced by name from `.data_champion` files, `queue_effect`, and `spawn_projectile` | `StableEffectType` |
| `set_extension` | Client lifecycle hooks (§6) | `StableExtension` |
| `set_server_extension` | Management-server hooks (§7) | `StableServerExtension` |
| `add_draft_score_hook` | Ban/pick scoring + full decision override (§8) | `StableDraftHook` |
| `add_player_input_ai` | Per-tick player input override (§9) | `StablePlayerAi` |
| `set_map_customizer` | Match-map editing hook (§5) | `StableMapCustomizer` |
| `set_match_hook` | Match rule hook — the backbone of custom modes (§5) | `StableMatchHook` |

---

## 3. Gameplay traits

### 3-1. `StableChampion`

```rust
fn id(&self) -> String;                          // unique id; an existing id reworks that champion
fn name(&self) -> String;                        // display name (i18n key recommended); defaults to id
fn skill_icon(&self, skill_index: usize) -> (String, String);  // (sprite source, rect tag), index 0..=3
fn category(&self) -> ChampionCategoryV1;        // Melee / Range / Magician / Util / Assassin
fn tags(&self) -> Vec<ChampionTagV1>;            // Ad, Ap, Heal, Shield, Dot, Cc, Range, Melee, Tank, Magic
fn stat(&self) -> StatV1;                        // level-1 base stats
fn growth(&self) -> StatV1;                      // per-level growth
fn attack(&self) -> Box<dyn StableAction>;       // basic attack
fn skill(&self) -> Box<dyn StableAction>;        // Q
fn skill2(&self) -> Box<dyn StableAction>;       // W
fn ult(&self) -> Option<Box<dyn StableAction>>;  // R (optional)
fn passive(&self) -> Option<Box<dyn StablePassive>>;
```

A champion's visuals are asset-driven by its id: sprites, animation tags, and skill icons follow the same rules as data champions — see [Sprite Binding](data-champion.md#sprite-binding), [Animation Tags](data-champion.md#animation-tags), and [Skill Icons](data-champion.md#skill-icons). A rework (reusing an existing id) inherits that champion's existing assets automatically.

### 3-2. `StableAction`

```rust
fn clone_box(&self) -> Box<dyn StableAction>;
fn action_name(&self) -> String;         // animation tag: "attack" / "skill" / "skill2" / "ult"
fn duration(&self) -> usize;             // full cast animation length, in ticks
fn cancelable(&self) -> bool;            // can a move order cancel the cast
fn cooltime(&self, caster_stat: &StatV1, caster_level: usize) -> usize;  // cooldown in ticks
fn casting_target(&self) -> CastingTargetV1;   // what the AI targets (Enemy / AllyChampion / ...)
fn effect(&self) -> Option<StableEffectSpec>;  // what actually happens
fn cooltime_use_count(&self, caster_stat: &StatV1) -> usize;  // charges (default 1)
fn can_use_with_move(&self) -> bool;     // castable while moving
fn description(&self) -> String;         // tooltip (i18n key recommended)
```

`StableEffectSpec { range, growth_range, start_timing, casting: CastingTypeV1, target: CastingTargetV1, attack_type: AttackTypeV1, effect: Box<dyn StableEffectType> }`
— `range` in world units, `growth_range` per level, `start_timing` = tick offset inside the cast when the effect fires, `casting` = Targeting (unit) / Position (point) / Direction / None.

### 3-3. `StableEffectType`

```rust
fn apply(&self, sim: &mut StableSim<'_>, rng_seed: u64, caster_id: usize, input: InputTargetV1);
```

The execution body. Check `input.kind` for the target shape (`InputTargetKindV1::Target` → `input.target_id`, `Pos` → `input.x/y`, …).

Reporting methods for the AI (all have defaults — the more accurately you report, the better the AI uses your skill):

| Method | Meaning |
|---|---|
| `expected_damage(&StatV1) -> (usize, usize)` | Expected single-target raw (AD, AP) damage |
| `expected_heal / expected_shield(&StatV1) -> usize` | Expected heal / shield |
| `expected_cc_time() -> Option<usize>` | CC duration in ticks |
| `expected_buff(&StatV1) -> Option<BuffV1>` | Buff it grants |
| `expected_move_distance() -> Option<(usize, u64)>` | (travel ticks, distance) for dashes |
| `expected_rush_effect() -> bool` | Is it a rush (dash that hits) |
| `auto_target() / on_caster() / can_move() -> bool` | Auto-targeting / self-cast / cast while moving |
| `linear_move_speed() -> Option<usize>` | Linear travel speed |

Example — targeted damage + stun + slow:

```rust
#[derive(Debug)]
struct Shackle;
impl StableEffectType for Shackle {
    fn apply(&self, sim: &mut StableSim<'_>, _seed: u64, caster_id: usize, input: InputTargetV1) {
        if input.kind != InputTargetKindV1::Target.code() { return; }
        let Some(caster) = sim.get_entity(caster_id) else { return };
        let ap = caster.stat().magic_power;
        sim.deal_damage(caster_id, input.target_id, 0, ap, AttackTypeV1::Skill);
        sim.apply_cc(input.target_id, &CcV1::stun(45));            // 45 ticks = 0.75s
        let mut slow = BuffV1::timed("my_mod_slow", 120);          // 2s
        slow.move_speed_mult = -30;                                // -30%
        sim.add_buff(input.target_id, &slow);
    }
    fn expected_cc_time(&self) -> Option<usize> { Some(45) }
}
```

### 3-4. `StablePassive` — every hook receives `&mut StableSim`

| Hook | When |
|---|---|
| `on_spawn(sim, player, entity)` | Spawn / respawn |
| `on_attack(sim, player, entity, target, damage: &mut usize)` | Basic attack hit — mutate `damage` |
| `on_damaged(sim, player, entity, attacker, damage)` | After taking damage |
| `on_kill(sim, player, entity)` / `on_assist(sim, player, entity)` | Kill / assist |
| `on_update(sim, rng_seed, player, entity)` | Every tick |
| `on_base_attack(sim, rng_seed, player, entity)` | Basic attack cast |
| `on_dead(sim, player)` | Death |

### 3-5. `StableItem`

Definition: `key` (unique), `icon` (asset path), `price`, `tier`, `stat() -> BuffV1` (equip stats, expressed as a buff), `next_tier() / previous_tier() -> Vec<String>` (build tree), `tags() -> Vec<ItemTagV1>`, `category() -> ItemCategoryV1`.
Callbacks: `on_attack(damage: &mut usize, attack_type)`, `update(rng_seed, player)` (every tick), `on_spawn`, `on_healed(is_self, target, amount)`, `on_damaged`, `on_kill(rng_seed, …)`, `on_skill_hit(rng_seed, …)`.

---

## 4. Simulation context (`StableSim`)

Received by effect `apply`, passive/item callbacks, and match hooks. Player AI gets a **read-only** view via `ctx.sim()` (mutating calls are inert there).

### 4-1. Game state reads

| Method | Description |
|---|---|
| `tick() -> usize` | Current tick (60 = 1 second) |
| `seed() -> u64` | Match seed |
| `score_diff(team) -> i32` | Score difference for team 0/1 |
| `is_end() -> bool` | Match over |
| `distance_sq(id1, id2) -> u64` | Squared distance between two entities |
| `is_visible(team, entity_id) -> bool` | Visible to that team |
| `champion_count()` / `champion_id_at(i) -> usize` | Champion entity ids |
| `tower_count()` / `tower_id_at(i) -> usize` | Tower entity ids |
| `kill_log_count()` / `kill_log_at(i) -> Option<KillLogV1>` | Kill log |
| `projectile_count()` / `projectile_at(i) -> Option<ProjectileInfoV1>` | Live projectiles (position, caster, team) |

### 4-2. Entity view (`StableEntity`) — via `get_entity(id)`, `entity_at(index)`, `entity_count()`, or `entity(EntityHandleV1)`

| Method | Description |
|---|---|
| `id() -> usize` | Entity id — what combat/mutation calls take |
| `handle() -> EntityHandleV1` | Raw handle (valid within this callback; re-acquire next time) |
| `name() -> Option<String>` | Champion/unit name ("prisoner", "ghoul", your spawned unit's name, …) |
| `stat() -> StatV1` | Current effective stats (buffs applied) |
| `pos() -> (u64, u64)` | World position |
| `hp() -> (usize, usize)` | (current, max) |
| `team() -> usize` | 0 = blue, 1 = red |
| `level()`, `shield()`, `radius()` | Level / shield amount / collision radius |
| `is_alive / is_champion / is_tower / is_minion / is_targetable() -> bool` | Kind & state checks |
| `buff_count()` / `buff_at(i) -> Option<BuffV1>` | Active buffs |
| `cc_count()` / `cc_at(i) -> Option<CcV1>` | Active crowd control |

### 4-3. Player view (`StablePlayer`) — via `get_player(id)`, `player_at(index)`, `player_count()`, or `player(PlayerHandleV1)`

| Method | Description |
|---|---|
| `id()`, `handle()`, `team()`, `lane() -> Option<LaneV1>` | Identity |
| `champion() -> Option<StableEntity>` | The controlled champion entity |
| `level()`, `gold()`, `is_alive()`, `respawn_time()` | State |
| `kills() / deaths() / assists() / cs()` | This match's KDA / CS |
| `cooldowns() -> Option<(usize, usize, usize, usize)>` | Remaining ticks: (attack, skill, skill2, ult) |
| `item_count()` / `item_keys() -> Vec<String>` | Owned item keys |
| `statistics_json(path) -> Option<String>` | Cumulative statistics document. Fields: `level, solo_kill, solo_killed, rating, exp, cs, cs_jungle, gold, kill, death, assist, kill_bounty` |

### 4-4. Combat actions (mutating)

| Method | Description |
|---|---|
| `deal_damage(attacker, target, ad, ap, AttackTypeV1)` | Damage through the full pipeline — resistances, kill credit, logs |
| `heal(caster, target, amount)` | Heal (heal-reduction applies) |
| `add_buff(target, &BuffV1)` / `apply_cc(target, &CcV1)` | Grant a buff / CC |
| `entity_remove_buff(id, name) -> usize` | Remove every buff with that name (returns count) |
| `entity_clear_cc(id) -> usize` | Clear all crowd control (returns count) |

### 4-5. Direct mutation (custom-rule building blocks)

| Method | Description |
|---|---|
| `entity_set_hp(id, hp)` | Set HP directly (no kill log/stats — use `deal_damage` when it should count) |
| `entity_set_pos(id, x, y)` | Teleport |
| `entity_set_base_stat(id, &StatV1)` | Replace base stats (buffs still apply on top) |
| `player_set_gold(id, gold)` / `player_add_gold(id, delta)` | Gold (negative delta saturates at 0) |
| `force_end(blue_win: bool)` | End the match now with the winner you pick (standings update properly) |

### 4-6. Spawning, effect queue, team strategy

```rust
// Fire a registered native effect 30 ticks from now at a point
sim.queue_effect("my_mod:meteor", AttackTypeV1::Skill, caster_id,
    &InputTargetV1 { kind: InputTargetKindV1::Pos.code(), x: 480_000, y: 480_000,
                     ..Default::default() }, 30);

// Spawn a 20-second chasing combat unit (ghoul-style AI: hunts nearby enemies)
let stat = StatV1 { attack: 60, hp: 800, move_speed: 1200, ..StatV1::default() };
let unit_id = sim.spawn_unit("my_mod_zombie", caster_id, 1 /*team*/, x, y,
    1200 /*ticks*/, &stat, &UnitAttackV1::default());   // default = melee attack, range 12000

// Point-to-point projectile — applies a native effect on hit
sim.spawn_projectile("my_mod_fireball", "my_mod:burn", &ProjectileSpawnV1 {
    caster_id, team: 0, x, y,                       // default radius 10000 (one champion),
    move_kind: ProjectileMoveKindV1::Linear.code(), // default speed 5000/tick (like basic attacks)
    target_x, target_y, penetrate: true,
    ..ProjectileSpawnV1::default()
});
```

- `spawn_unit(name, summoner_id, team, x, y, duration_ticks, &StatV1, &UnitAttackV1) -> Option<entity_id>` — `summoner_id` receives kill credit. Use the returned id for later reads/mutation.
- `spawn_projectile(name, effect_name, &ProjectileSpawnV1) -> bool` — circular hit shape; movement `Target` (homes onto `target_id`) or `Linear` (flies to `target_x/y`, `penetrate` keeps going through hits); `casting_target` filters what it can hit (default Enemy).
- `queue_effect(effect_name, attack_type, caster_id, &InputTargetV1, delay_ticks) -> bool` — false if the name is not registered.
- `strategy_get_json(team, path)` / `strategy_set_json(team, path, json)` — the team's live macro-strategy document. Fields (all enums): `focused` (pressured lane: `"Top"` / `"Bottom"` / `"All"`), `early_jungle`, `early_serpen`, `early_serpen_top`, `object_buildup`, `object_battle`, `morgard_use`, `tower_press`, `morgard_defense`, `object_finish`, `minion_wave`, `game_finish`.

### 4-7. Debug drawing

`debug_draw_line(x1, y1, x2, y2, color)` and `debug_draw_circle(x, y, r, color)` — world coordinates, drawn over the match view. For visualizing your logic while developing.

---

## 5. Match hook & map customizer

### `StableMatchHook` — custom modes as "a variant on top of an existing mode"

```rust
fn on_match_start(&self, sim: &mut StableSim<'_>);                  // once, right after spawns
fn on_match_tick(&self, sim: &mut StableSim<'_>, rng_seed: u64);    // every tick
fn check_match_end(&self, sim: &mut StableSim<'_>) -> Option<bool>; // Some(true) = blue wins now
```

If no mod registers a match hook the game runs bit-identical to vanilla (no RNG consumed).

Example — a zombie wave for both sides every 2 minutes:

```rust
struct ZombieWave;
impl StableMatchHook for ZombieWave {
    fn on_match_tick(&self, sim: &mut StableSim<'_>, _seed: u64) {
        let t = sim.tick();
        if t > 0 && t % 7200 == 0 {                     // 7200 ticks = 2 min
            let stat = StatV1 { attack: 40, hp: 600, move_speed: 1100, ..StatV1::default() };
            for team in 0..2 {
                let (x, y) = if team == 0 { (200_000, 200_000) } else { (760_000, 760_000) };
                sim.spawn_unit("zombie", usize::MAX, 1 - team, x, y, 3600, &stat,
                    &UnitAttackV1::default());
            }
        }
    }
}
```

### `StableMapCustomizer`

```rust
fn customize(&self, mode: Option<GameModeKindV1>, doc: &mut StableJsonDoc<'_>);
```

Runs at match creation for every mode; edits the map document before the game captures it.

`StableJsonDoc` methods: `get_json(path)` / `set_json(path, json)` plus typed sugar — `get_string / set_string`, `get_i64 / set_i64`, `get_bool / set_bool`, `get_f64`, `is_null(path)`. Writes are atomically rejected on schema violations.

Document fields: `walls` / `bushes` / `regions` (30×30 integer grids), `nexus_pos` (per-team `[x, y]`), `fountains` (per-team `[lx, ly, rx, ry]`), `lanes`, `towers`, `camps`, and the region graph (`region_adj`, `region_dist`, `region_centers`, `is_line_region`, `lane_regions`).

**Limit**: pathfinding and vision are baked offline from the original terrain, so large wall/bush rework confuses movement AI — object placement (towers, camps, nexus, fountains, lanes) is fully live.

---

## 6. Client context (`StableClient`)

### 6-1. Lifecycle (`StableExtension`)

```rust
fn on_init(&self, ctx: &mut StableClient<'_>);                    // once after loading
fn pre_update / post_update(&self, ctx, dt_micros: u64);          // every frame (microseconds)
fn pre_render / post_render(&self, ctx: &mut StableClient<'_>);   // every frame, around rendering
fn on_end(&self);                                                 // shutdown
```

- **Render hooks are read-only**: every read (UI queries, data, scene) works, every mutation (UI writes, handler registration, save, net) returns `false`. In exchange, the `draw_*` surface is live only here. `pre_render` draws under the game's own commands, `post_render` over them (z permitting).
- **UI event handlers get a reduced context**: UI and assets work; scene/save/net/data do not. Do data work in update hooks.

### 6-2. Scene

`scene_kind() -> Option<SceneKindV1>` — Loading, Title, NewGame, DatabaseEdit, InGame, GameTest, Room, Lobby. `is_in_game()`.

**NewGame-scene accessors** (graceful failure elsewhere):

| Method | Description |
|---|---|
| `new_game_team_id()` / `new_game_set_team_id(id)` | Starting team |
| `new_game_with_tutorial()` / `new_game_set_with_tutorial(b)` | Tutorial on/off |
| `new_game_manager_name()` / `new_game_set_manager_name(s)` | Manager name |
| `new_game_option_get_json(path)` / `new_game_option_set_json(path, json)` | The play-option document |

Play-option document fields: `difficulty`, `simulation_intensity`, `patch_period`, `patch_intensity_setting`, `champion_pool`, `champion_add_count`, `banpick_style`, `banpick_time_limit`, `league_rule`, `cup_rule`, `group_stage_format`, `custom_champions`, `always_delegate_to_staff`, `condense_competition_matchdays`, `view_stat`, and the scout filter set.

### 6-3. UI

Paths are dot-separated selectors. The tree hangs off `"body"`; explore the real tree with `ui_child_names("")` (empty path = root) downward.

**Reads**

| Method | Description |
|---|---|
| `ui_exists(path)` | Node exists |
| `ui_visible(path) -> Option<bool>` / `ui_set_visible(path, b)` | Visibility |
| `ui_text(path) -> Option<String>` / `ui_set_text(path, s)` | Label text (label runners only) |
| `ui_runner_name(path) -> Option<String>` | Widget kind of the node ("label" / "checkbox" / …; empty for unregistered kinds) |
| `ui_node_rect(path)` / `ui_contents_rect(path) -> Option<(x, y, w, h)>` | Computed rect from the last layout pass (screen f32) |
| `ui_child_count(path) -> Option<usize>` / `ui_child_names(path) -> Vec<String>` | Direct children (empty path = root) |
| `ui_state_json(path) -> Option<String>` | Runtime widget state (table below) |

**Writes, spawning, removal**

| Method | Description |
|---|---|
| `ui_spawn_template(parent, "asset/<mod>/ui/...", visible)` | Spawn a prebuilt `.ui` template asset as a child |
| `ui_spawn_source(parent, ".ui source text")` | **Spawn from `.ui` syntax in a string** — every registered runner kind works |
| `ui_set_properties(path, "key: value; ...")` | Apply `.ui` property lines in bulk (layout, visible, disable, runner-specific settings) |
| `ui_set_state_json(path, json)` | Write runtime state — unknown keys/wrong types are atomically rejected |
| `ui_remove_node(path)` | Delete a subtree |

**`.ui` syntax** (shared by `ui_spawn_source`, `ui_set_properties`, and `.ui` asset files):

```text
my_popup:color {              // "id:runner_kind"
  width: 300px;  height: 120px;
  anchor_x: 0.5; anchor_y: 0.5; pivot_x: 0.5; pivot_y: 0.5;
  color: #1d1f2cff;           // #rrggbbaa
  rounding: Uniform { rounding: 12; }

  #title:label {              // children are prefixed with #
    text: "Hello";
    font_size: 18;
  }
}
```

**Runner kinds**: the 17 engine basics — `label, image, svg, color, button, checkbox, slider, text_edit, dropdown, selectable, color_selectable, scroll_view, tree_view, animation, canvas, empty, asset_selector` — plus ~50 game-specific runners (`icon_button`, `main_ui`, `schedule_ui`, `player_detail_ui`, …) share one registry. `ui_runner_name` is a registry reverse-lookup, so new runner kinds added in game updates are covered automatically.

**Runtime widget state** (`ui_state_json` / `ui_set_state_json` + typed sugar):

| Runner | Read | Write | Sugar |
|---|---|---|---|
| label | `{"text"}` | — (use set_text) | `ui_text` / `ui_set_text` |
| checkbox | `{"selected", "text"}` | `{"selected": bool}` | `ui_checkbox_selected` / `ui_set_checkbox_selected` |
| text_edit | `{"text", "is_editing"}` | `{"text": string}` | `ui_text_edit_text` / `ui_set_text_edit_text` |
| slider | `{"ratio"}` | `{"ratio": number}` | `ui_slider_ratio` / `ui_set_slider_ratio` |
| selectable | `{"selected", "text"}` | `{"selected": bool}` | `ui_selectable_selected` / `ui_set_selectable_selected` |
| dropdown | `{"selected_item"}` | — (read-only) | `ui_dropdown_selected_item` |
| others | `{}` | rejected | — |

**Events**

```rust
ctx.ui_register_click("body.my_popup.ok", "" /*item*/, |ctx| {
    ctx.ui_set_visible("body.my_popup", false);
});
ctx.ui_register_path_events("body.some_node", |ctx| {         // every event on that path
    if let Some(ev) = ctx.ui_current_event() {
        // ev.kind: Option<UiEventKindV1>, ev.path, ev.payload_json
    }
});
```

Payload JSON per event kind:

| UiEventKindV1 | payload |
|---|---|
| Click / RightClick | `{"item": string}` |
| CheckboxSelect | `{"selected": bool}` |
| TreeViewSelect | `{"item": string}` |
| TreeViewRightClick | `{"item": string, "x": f32, "y": f32}` |
| TreeViewMove | `{"item_from": string, "item_to": string}` |
| TextEditComplete | `{"text": string}` |
| Remove / Changed | `{}` |
| Custom | `{"data": string}` |

Handler registration is permanent (no removal API). Handler closures must be `'static` — they live until the process exits.

### 6-4. Render drawing (render hooks only — probe with `can_draw()`)

Draw onto a named map: `"UI"` / `"Default"` = 1920×1080 screen space, `"Game"` = match-world space (while spectating a match).

| Method | Description |
|---|---|
| `draw_map_size(map) -> Option<(w, h)>` | Map size (may be unset yet during `pre_render`) |
| `draw_set_camera(map, cx, cy, w, h)` | Camera (center + extent) |
| `draw_rect(map, x, y, w, h, z, rounding, color)` | Filled box (`rounding > 0` = rounded corners) |
| `draw_circle(map, cx, cy, r, z, color)` | Filled circle |
| `draw_line(map, x1, y1, x2, y2, width, z, color)` | Line segment |
| `draw_text(map, text, font, (x, y, w, h), z, size, color, TextAlignXV1, TextAlignYV1)` | Text in a rect. Fonts: `"asset/base/font/set/regular"`, `"asset/base/font/set/bold"` |
| `draw_svg(map, path, (x, y, w, h), z, color)` | SVG scaled into a rect, tinted |
| `draw_sprite(map, texture, &StableSpriteParams)` | Texture at native size — `{ x, y, z, rot, flip_x/y, pivot_x/y, uv: (x, y, w, h normalized), sample_nearest }` |

```rust
fn post_render(&self, ctx: &mut StableClient<'_>) {
    ctx.draw_rect("UI", 20., 20., 220., 60., 5000, 8., 0x101018cc);
    ctx.draw_text("UI", "My Mod Overlay", "asset/base/font/set/bold",
        (20., 20., 220., 60.), 5001, 16., 0xffffffff,
        TextAlignXV1::Center, TextAlignYV1::Center);
}
```

### 6-5. Audio (from any hook — the game's frame loop plays it)

`play_sound(path, volume 0.0..=1.0)`, `play_bgm(&["asset/my_mod/sound/bgm_a", ...])` (replaces the playlist), `stop_bgm()`. Your own sound files in `mods/<mod_id>/` load automatically.

### 6-6. Raw keyboard input (per-frame snapshot — hotkeys)

`input_events() -> Vec<StableInputEvent { kind: Option<InputEventKindV1>, key: String }>`, `key_pressed("F5") -> bool`.
Key names are the engine's key enum names: `"A"`–`"Z"`, `"F1"`–`"F12"`, `"Tab"`, `"Space"`, `"Enter"`, `"Escape"`, `"LShift"`, …. This is raw input (before the UI consumes it) — guard against firing while the player is typing (combine with `client_scene_kind` if needed).

### 6-7. Mod save data (InGame only; automatically namespaced per mod; stored in the save file)

`save_can_write()`, `save_version()` / `save_set_version(v)`, `save_contains_key(k)`, `save_get_bytes / save_get_string(k)`, `save_set_bytes / save_set_string(k, v)`, `save_remove_key(k)`, `save_keys()`.

### 6-8. Networking (client ↔ management server)

- `send_command(command, payload) -> bool` — delivered to your server extension's `handle_command`.
- `take_events() -> Vec<StableModEvent { event, payload }>` — drains events the server sent with `emit_event`.

### 6-9. Management data reads (InGame only)

**Basics**: `player_team_id()`, `team_ids()`, `team_name(id)`, `athlete_ids()`, `athlete_name(id)`.

**Record documents** — `record_ids(kind) -> Vec<usize>` + `record_get_json(kind, id, path)` (+ `record_get_string` / `record_get_i64`):

| RecordKindV1 | Contents | Client | Server (write) | Example paths |
|---|---|---|---|---|
| Team | Teams (name, finances, news, resale clauses, …) | ✅ | ✅ | `"name"`, `"transfer_budget"`, `"news"` |
| Athlete | Athletes (age, contract, stats, solo rank, …) | ✅ | ✅ | `"name"`, `"age"`, `"contract"`, `"stat"` |
| League / Tournament | League / tournament definitions | ✅ | ✅ | `"name"`, `"region_id"` |
| Staff | Staff | ✅ | ✅ | `"name"` |
| MatchNormal / MatchPractice / MatchTutorial / MatchSoloRank | Match records per category | ✅ | ❌ (server uses Match) | `"date"`, `"running_state"`, `"team1"`, `"need_win"` |
| LeagueCompetition / TournamentCompetition | Running/finished competitions (standings, brackets) | ✅ | ✅ | `"standings"`, `"matches"` |
| SoloRankMatch | Solo-rank matches | ✅ | ✅ | |
| KnowledgeBase | Team scouting knowledge (keyed by team id) | ✅ | ✅ | |
| YearSchedule | Season schedules (`year`, `recruits`, `schedules`) | ✅ | ✅ | `"year"` |
| Match | **The server's whole match table** (no category split) | ❌ | ✅ | |

**Typed globals**: `game_time() -> (year, month, day, hour, minute)` / `game_date()`, `head_to_head(opponent_team) -> (wins, losses)` (all-time official), `competition_result(is_tournament, comp_id) -> (champion_team, runner_up_team)`, `schedule_get_json(path)` (season schedule array — paths like `"0.year"`).

**Champion info**: `champion_names() -> Vec<String>` (selectable champions of the active, patched sheet), `champion_brief(name) -> Option<StableChampionBrief { name, category, tags, stat, growth }>`.

**Current screen**: `client_scene_kind() -> Option<ClientSceneKindV1>` (Main, Lineup, StadiumEntrance, Match, MatchResult, LockerRoom, InGame, Prologue, PrologueFirst, TutorialMorgad, TutorialSerpen, Tutorial5v5), `client_main_tab() -> Option<String>` (tab name on the Main screen: "Home" / "Squad" / "Schedule" / "PlayerDetail" / …).

**Settings reads**: `setting_get_json(SettingTargetV1::{GameSetting | ItemSetting}, path)` (+ `setting_get_i64` / `setting_get_string`).
GameSetting path examples: `"kill_gold"`, `"start_gold"`, `"gold_per_second"`, `"respawn_tick"`, `"visible_distance"`, `"return_tick"`, `"melee_minion.stat.hp"`. **Writes** happen server-side (§7-3) or via asset overrides.

---

## 7. Server context (`StableServerCtx`)

### 7-1. Lifecycle (`StableServerExtension`)

```rust
fn on_server_start(&self, ctx: &mut StableServerCtx<'_>);        // server boot
fn before_management_tick / after_management_tick(&self, ctx);   // around date progression
fn handle_command(&self, ctx, cmd: &StableCommand<'_>) -> CommandResultV1;  // client commands
```

`StableCommand { command: &str, payload: &[u8], sender_player_id, sender_team_id }` — `cmd.reply_target()` picks the reply target (`Player` > `Team` > `Broadcast`). Return `Handled` to stop propagation to other mods.

Command round trip:

```rust
// Server
fn handle_command(&self, ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> CommandResultV1 {
    if cmd.command != "remember_note" { return CommandResultV1::Pass; }
    ctx.save_set_bytes("note", cmd.payload);
    ctx.emit_event(cmd.reply_target(), "note_saved", b"ok");
    CommandResultV1::Handled
}
// Client (update hook)
ctx.send_command("remember_note", b"hello");
for ev in ctx.take_events() { if ev.event == "note_saved" { /* ... */ } }
```

### 7-2. Mod save data (authoritative) — same surface as the client

`save_version / save_set_version`, `save_contains_key`, `save_get_bytes / save_get_string`, `save_set_bytes / save_set_string`, `save_remove_key`, `save_keys`.

### 7-3. Settings documents (apply to later matches)

`setting_get_json(target, path)` / `setting_set_json(target, path, json)` plus sugar `setting_get / setting_set` for `i64`, `f64`, `bool`, `string`. Targets: `GameSetting`, `ItemSetting`. Atomic reject on violation.

### 7-4. Record documents (authoritative writes)

- Dedicated sugar: `team_get_json / team_set_json / team_get_string / team_set_string / team_get_i64 / team_set_i64` — and the same six for `athlete_*`.
- Generic: `record_ids(kind)`, `record_get_json / record_set_json(kind, id, path, …)`, `record_get_string / record_set_string` — every server-✅ kind in the table above.
- **Cross-record invariants are your responsibility** — this is raw document editing. Where a validated compound operation exists, prefer the typed API (news, transfers).

```rust
// Rename a league
let league_id = ctx.record_ids(RecordKindV1::League)[0];
ctx.record_set_string(RecordKindV1::League, league_id, "name", "Modded Super League");
```

### 7-5. News injection

`news_push(team_id, title, content, author) -> bool` — publishes a simple article to that team's news feed, dated with the in-game clock; syncs to the client through the normal pipeline. `title` / `content` accept literals or i18n keys (`"#asset/my_mod/text/news?my.key"`).

### 7-6. Management events (pull model)

```rust
fn after_management_tick(&self, ctx: &mut StableServerCtx<'_>) {
    let cursor = ctx.save_get_string("ev_cursor").and_then(|s| s.parse().ok()).unwrap_or(0);
    for ev in ctx.management_events_after(cursor) {
        match ev.kind {
            Some(ManagementEventKindV1::MatchFinished)     => { /* parse ev.payload_json */ }
            Some(ManagementEventKindV1::TransferCompleted) => { /* ... */ }
            Some(ManagementEventKindV1::SeasonRollover)    => { /* ... */ }
            None => {}                                   // future kind — skip
        }
        ctx.save_set_string("ev_cursor", &ev.seq.to_string());
    }
}
```

| Kind | When | payload_json |
|---|---|---|
| MatchFinished | **Every** match result lands (player matches and background sims share one path) | `{"match_id", "winner_team_id", "loser_team_id", "team1_id", "team2_id", "team1_score", "team2_score"}` |
| TransferCompleted | A transfer/signing completes | `{"athlete_id", "from_team_id" (null = free-agent signing), "to_team_id"}` |
| SeasonRollover | A season schedule is generated (including a new game's first season) | `{"year"}` |

The host keeps a bounded backlog (most recent 256) — polling every management tick loses nothing. Keep your cursor (`seq`) in mod save data.

### 7-7. Forced transfers

`force_transfer(athlete_id, to_team_id, transfer_fee: f64) -> bool`

- **Contracted athlete**: moves immediately. Past contract recorded; buyer pays the fee, seller receives it (finance reports included); transfer news on the seller's feed; `TransferCompleted` event; solo-rank region of the target league ensured. Contract terms (salary, end date) carry over.
- **Free agent**: signed on a fair-salary contract; the fee is ignored.
- Returns `false` for unknown athlete/team, or if the athlete is already on the target team.
- **Deliberately skipped** (this is a forced operation): budget checks (the buyer can go negative), installments, resale clauses, incentive negotiation.

### 7-8. Misc

`player_team_id(player_id) -> Option<usize>` — the team of a connected human player.

---

## 8. Draft hook (`StableDraftHook`)

```rust
fn id(&self) -> String;
fn priority(&self) -> i32;   // lower runs earlier in the score chain

fn score_ban / score_pick(&self, ctx: &StableDraftContext<'_>, candidate: usize, base_score: f32)
    -> StableDraftDecision;  // Pass | Add(delta) | Replace(score)

fn decide_ban / decide_pick(&self, ctx: &StableDraftContext<'_>) -> Option<usize>;  // full override
```

- **score**: called per candidate; adjusts the built-in AI's score. Final selection still uses difficulty-based top-k randomness.
- **decide**: return `Some(champion_id)` to **lock in that exact champion** — bypasses scoring and top-k randomness entirely. Ids outside the candidate pool (banned/taken/unavailable) are ignored and selection falls back to normal scoring. If several hooks decide, the last one wins.
- Scores are `f32` (draft runs outside the deterministic sim zone).

`StableDraftContext`:

| Method | Description |
|---|---|
| `phase() -> Option<DraftPhaseV1>` (Ban / Pick), `difficulty()`, `is_explore()` | Phase info |
| `available_champions()`, `ally_bans() / enemy_bans()`, `ally_picks() / enemy_picks()` | Candidate/ban/pick id slices |
| `champion_briefs() -> &[ChampionBriefV1]` | The identity array those ids index into |
| `champion_name(id)`, `champion_category(id)`, `champion_tags(id)`, `champion_stat / champion_growth(id)` | Candidate identity |

```rust
// "If an assassin is still available, ban it first."
fn decide_ban(&self, ctx: &StableDraftContext<'_>) -> Option<usize> {
    ctx.available_champions().iter().copied()
        .find(|&id| ctx.champion_category(id) == Some(ChampionCategoryV1::Assassin))
}
```

---

## 9. Player input AI (`StablePlayerAi`)

```rust
fn clone_box(&self) -> Box<dyn StablePlayerAi>;
fn id(&self) -> String;
fn priority(&self) -> i32;
fn matches(&self, init: &StableAiInit) -> bool;   // which players to attach to
fn think(&mut self, ctx: &mut StableAiContext<'_>, base_input: Option<InputV1>) -> Option<InputV1>;
```

`StableAiInit { player_id, athlete_id, team, lane: Option<LaneV1>, champion_name }`.
`think` runs every tick — `None` keeps the built-in input, `Some` replaces it. **It must be a pure decision function** (determinism).

`StableAiContext`:

| Method | Description |
|---|---|
| `player_id / athlete_id / team / lane / champion_name / tick` | Own identity & time |
| `hp() / max_hp() / hp_ratio_percent() / is_hp_below_percent(p)` | Own champion HP |
| `is_valid_input(&InputV1)` | Pre-validate an input (range, cooldowns, …) |
| `run_away_input() / run_away_without_skill_input() / recall_input() / is_safe_to_recall()` | Built-in behavior inputs |
| **`sim() -> Option<StableSim>`** | **Read-only view of the whole simulation** — every read from §4 works; mutations are inert. `None` on game versions older than this feature |

Building inputs: `InputV1::move_to(x, y)`, `InputV1::return_home()`, `InputV1::action(InputKindV1::{Attack, Skill, Skill2, Ult}, InputTargetV1 { ... })`.

```rust
fn think(&mut self, ctx: &mut StableAiContext<'_>, base: Option<InputV1>) -> Option<InputV1> {
    let my_id = ctx.player_id();
    if let Some(sim) = ctx.sim() {
        // Ult the lowest-HP enemy champion when the ult is ready.
        let player = sim.get_player(my_id)?;
        let me = player.champion()?;
        let ult_ready = player.cooldowns()?.3 == 0;
        if ult_ready {
            let target = (0..sim.champion_count())
                .map(|i| sim.champion_id_at(i))
                .filter_map(|id| sim.get_entity(id))
                .filter(|e| e.team() != me.team() && e.is_alive())
                .min_by_key(|e| e.hp().0)?;
            let input = InputV1::action(InputKindV1::Ult, InputTargetV1 {
                kind: InputTargetKindV1::Target.code(), target_id: target.id(),
                ..Default::default()
            });
            if ctx.is_valid_input(&input) { return Some(input); }
        }
    }
    base   // otherwise keep the built-in AI
}
```

---

## 10. Value types

### `StatV1` (all `usize`)
`attack, magic_power, hp, defence, magic_resistance, move_speed, hp_regen, stack, crit_chance`

### `BuffV1`
Constructors: `BuffV1::named(name)` (permanent), `BuffV1::timed(name, ticks)`. Names are capped at 64 bytes; read/write with `name()` / `set_name()`.
- Duration: `duration_kind` (BuffDurationV1: Permanent / Time / WithShield), `duration_tick`
- Flat (i32): `attack, magic_power, defence, hp, hp_regen, magic_resistance, vamp, crit_chance`
- Percent multipliers (i32): `attack_mult, magic_power_mult, defence_mult, magic_resistance_mult, hp_mult, move_speed_mult, attack_speed_mult, skill_cooldown_mult, ult_cooldown_mult, radius_mult`
- Special (usize): `damage_reflect, damaged_amplify, damaged_reduce, defence_penetration, magic_resistance_penetration, toughness, heal_reduce, range, base_attack_enemy_max_hp_damage, self_max_hp_damage, skill_enemy_max_hp_damage, dot_amplify, base_attack_damaged_reduce, skill_damaged_reduce`
- Flags (bool): `cc_immune, undying, ignore_wall`

### `CcV1`
`kind` (CcKindV1 code), `tick` (duration), `dx / dy / speed / target` (forced-movement kinds), 64-byte name (`name()` / `set_name()`).
Constructors: `CcV1::stun(ticks)`, `CcV1::of_kind(CcKindV1, ticks)`.

### `InputTargetV1`
`kind` (InputTargetKindV1: None / Target / Dir / Pos), `target_id`, `dir_x / dir_y` (i64), `x / y` (u64).

### `InputV1`
`kind` (InputKindV1: Move / Return / Attack / Skill / Skill2 / Ult), `target: InputTargetV1`, `x / y` (for Move). Constructors: `move_to`, `return_home`, `action`.

### `KillLogV1`
`tick, killer_team, killer_position, killed_position, assist_count, assist_positions[4]` (positions are lane indices).

### `ProjectileInfoV1`
`x, y, caster_id, team, is_end`.

### `UnitAttackV1` (spawned-unit attack profile; `Default` = a plain melee attack)
`attack_ratio` (% of the unit's attack stat, 100 = 1×), `attack` (flat bonus), `range`, `cooltime`, `duration`, `start_timing`, `cancelable`, `attack_type` (code).

### `ProjectileSpawnV1` (`Default` provided)
`caster_id, team, x, y, radius, speed, move_kind` (Target / Linear), `target_id` (Target), `target_x / target_y, penetrate` (Linear), `attack_type, casting_type, casting_target` (hit filter).

### `ChampionBriefV1`
`name: StrV1`, `category` (code; unknown = `u32::MAX`), `tag_count`, `tags[12]`, `stat`, `growth`.

### `StableSpriteParams` (draw wrapper; `Default` provided)
`x, y, z, rot, flip_x, flip_y, pivot_x, pivot_y, uv: (x, y, w, h normalized — (0,0,1,1) = whole texture), sample_nearest`.

### Wrapper-owned types
`StableUiEvent { kind, path, payload_json }` · `StableModEvent { event, payload }` · `StableManagementEvent { seq, kind, payload_json }` · `StableChampionBrief { name, category, tags, stat, growth }` · `StableInputEvent { kind, key }` · `StableDraftDecision { Pass | Add(f32) | Replace(f32) }` · `StableEventTarget { Broadcast | Player(id) | Team(id) }` · `StableCommand` · `StableEffectSpec`.

---

## 11. Enum codes

All enum codes are `u32` and append-only — treat unknown codes as "skip".

| Enum | Variants |
|---|---|
| `ChampionCategoryV1` | Melee, Range, Magician, Util, Assassin |
| `ChampionTagV1` | Ad, Ap, Heal, Shield, Dot, Cc, Range, Melee, Tank, Magic |
| `ItemTagV1` | Ad, Ap, AttackSpeed, Defense, MagicResistance, Hp, DefensePenetration, Vamp, HealReduce, ShieldBreak, MoveSpeed, AttackRange, Shield, HpPercentDamage, AsDebuff, ReflectDamage, Toughness, MrDebuff, RangeDamage, MrPenetration, CooltimeReduce, DotDamage, HpRegen, MyHpPercentDamage, ShareDamage, Range |
| `ItemCategoryV1` | Ad, AttackSpeed, Defense, MagicResistance, Magic, Hp |
| `AttackTypeV1` | BaseAttack, Skill, Dot, DotIgnoreShield, Item, Well |
| `DamageTypeV1` | Ad, Ap, Fixed |
| `CastingTypeV1` | Targeting, Position, Direction, None |
| `CastingTargetV1` | Ally, AllyChampion, AllyChampionInCc, AllyNotSelf, AllyOnlySelf, Enemy, EnemyWithoutTower, EnemyChampion, EnemyChampionInCc, EnemyChampionRecentlyAttacked, Both, BothWithoutTower, BothChampion, None |
| `LaneV1` | Top, Jungle, Mid, Bottom, Support |
| `CcKindV1` | Airborne, Stun, Bind, BlockAttack, BlockSkill, BlockMoveSkill, ForceMove, Taunt, Fear, Charm, Animation |
| `BuffDurationV1` | Permanent, Time, WithShield |
| `InputTargetKindV1` | None, Target, Dir, Pos |
| `InputKindV1` | Move, Return, Attack, Skill, Skill2, Ult |
| `EventTargetKindV1` | Broadcast, Player, Team |
| `CommandResultV1` | Pass, Handled |
| `SceneKindV1` | Loading, Title, NewGame, DatabaseEdit, InGame, GameTest, Room, Lobby |
| `UiEventKindV1` | Click, RightClick, CheckboxSelect, TreeViewSelect, TreeViewRightClick, TreeViewMove, TextEditComplete, Remove, Custom, Changed |
| `DraftPhaseV1` | Ban, Pick |
| `DifficultyV1` | Easy, Normal, Hard |
| `DraftDecisionKindV1` | Pass, Add, Replace |
| `AiDecisionKindV1` | Pass, Replace |
| `GameModeKindV1` | Moba, SingleLane, DeathMatch |
| `SettingTargetV1` | GameSetting, ItemSetting |
| `RecordKindV1` | Team, Athlete, League, Tournament, Staff, MatchNormal, MatchPractice, MatchTutorial, MatchSoloRank, LeagueCompetition, TournamentCompetition, SoloRankMatch, KnowledgeBase, YearSchedule, Match |
| `TextAlignXV1` / `TextAlignYV1` | Left, Center, Right / Top, Center, Bottom |
| `ProjectileMoveKindV1` | Target, Linear |
| `ManagementEventKindV1` | MatchFinished, TransferCompleted, SeasonRollover |
| `ClientSceneKindV1` | Main, Lineup, StadiumEntrance, Match, MatchResult, LockerRoom, InGame, Prologue, PrologueFirst, TutorialMorgad, TutorialSerpen, Tutorial5v5 |
| `InputEventKindV1` | KeyPressed, KeyReleased |

---

## 12. Availability matrix

| Surface | on_init / update hooks | UI event handlers | pre/post_render | AI think (`sim()`) | Match/effect callbacks |
|---|---|---|---|---|---|
| UI reads (exists/text/rect/tree/state) | ✅ | ✅ | ✅ | — | — |
| UI writes (spawn/props/state/remove/register) | ✅ | ✅ | ❌ returns false | — | — |
| `draw_*` | ❌ | ❌ | ✅ | — | — |
| Audio | ✅ | ✅ | ✅ | — | — |
| Keyboard snapshot | ✅ | ✅ | ✅ | — | — |
| Scene kind / NewGame accessors | ✅ (right scene) | ❌ (no scene) | reads only | — | — |
| Save / net / management data (client) | ✅ (InGame) | ❌ | reads ✅ / writes ❌ | — | — |
| Sim reads | — | — | — | ✅ | ✅ |
| Sim mutation / spawns / strategy writes | — | — | — | ❌ inert | ✅ |
| Server surface | (in server hooks) ✅ | — | — | — | — |

---

## 13. Debugging

- **Logs**: `host.log(LogLevel::..., msg)` writes to the game log — `%APPDATA%\TeamSamoyed\TeamfightManager2\data\log.log` (also the place to look when your mod got disabled by a panic; the panic message is recorded there).
- **Sim visualization**: `debug_draw_line` / `debug_draw_circle` from any sim callback draw over the match view in world coordinates.
- **UI exploration**: walk the live tree with `ui_child_names("")` downward, and identify nodes with `ui_runner_name` — faster than guessing paths.
- **Failure probes**: almost every call returns `Option`/`bool`. When something "does nothing", check the return value first — a `false`/`None` usually means wrong context (see the availability matrix), wrong path, or a schema-rejected write.

## 14. Pitfalls & FAQ

- **Handles and contexts are call-scoped.** Re-acquire entities/players in each callback; a stale handle is a graceful failure, not UB.
- **A panic disables your mod.** `unwrap`/`assert` in a hook kills your mod, not the game. Handle failures.
- **Determinism violations** (clocks, HashMap iteration order, mutable statics, threads inside sim callbacks) break replays and multiplayer sync. Derive an `StdRng` from the provided `rng_seed`.
- **Raw record writes** are schema-validated only. Editing match records directly can disagree with standings — for match outcomes use the match hook's `force_end` (updates standings properly); for transfers use `force_transfer`.
- **`entity_set_hp(0)` vs `deal_damage`**: the former kills silently (no stats/kill log); use the latter when it should count.
- **Draw order is all about z.** Some screens use high z for the game's own UI; if your overlay is invisible, raise z (5000+).
- **`Match*` record ids are per-category** (Normal #4 ≠ Practice #4). The server-side `Match` kind is one flat table.
- **Where is the tick clock?** Everything time-like in the sim is ticks at 60/s. Client hooks get wall-clock `dt_micros` instead — do not mix the two.
