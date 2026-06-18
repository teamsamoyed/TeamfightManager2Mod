# Native Mod API Reference

This page lists the Rust API exported by the `mod_api` crate for native DLL mods.
Most mods start with:

```rust
use mod_api::*;
```

The API version described here is `API_VERSION == (0, 8)`. Native DLLs are SDK-version-bound: rebuild the DLL when the game SDK changes.

## Top-Level Export Map

`mod_api` re-exports these modules:

- Registration and entry points from `registration`.
- Native content traits from `traits`.
- Simulation access wrappers from `game_ctx`.
- Opaque handles from `handles`.
- Client lifecycle hooks and UI/scene types from `extension`.
- Selected game value types from `game-core`.

For large game data structures such as `Team`, `Athlete`, `MatchInfo`, and `ChampionInfoSheet`, the exported type is the real game type from the matching SDK. Use the SDK version that matches the target game.

## Registration

### Constants and Types

| Item | Signature |
| --- | --- |
| `API_VERSION` | `(u32, u32)`, currently `(0, 8)` |
| `API_VERSION_ENCODED` | `u64` encoded as `major << 32 | minor` |
| `MOD_ENTRY_SYMBOL` | `"tfm2_mod_entry"` |
| `MOD_API_VERSION_SYMBOL` | `"tfm2_mod_api_version"` |
| `ModEntryFn` | `unsafe extern "C" fn(api: *const GameCtx) -> *mut ModRegistration` |
| `ModApiVersionFn` | `unsafe extern "C" fn() -> u64` |
| `decode_api_version(raw)` | `fn decode_api_version(raw: u64) -> (u32, u32)` |

Use `declare_mod!(init_fn)` to export both DLL symbols:

```rust
fn init(ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new("my_mod");
    reg.add_item(MyItem::default());
    reg
}

declare_mod!(init);
```

### `ModRegistration`

Fields:

| Field | Type |
| --- | --- |
| `mod_id` | `String` |
| `api_version` | `(u32, u32)` |
| `champions` | `Vec<Box<dyn ModChampionInfo>>` |
| `items` | `Vec<Box<dyn ModItemInfo>>` |
| `draft_score_hooks` | `Vec<Box<dyn ModDraftScoreHook>>` |
| `player_input_ai` | `Vec<Box<dyn ModPlayerInputAi>>` |
| `extension` | `Option<Box<dyn ModExtension>>` |
| `server_extension` | `Option<Box<dyn ModServerExtension>>` |

Methods:

| Method | Signature |
| --- | --- |
| `new` | `fn new(mod_id: impl Into<String>) -> Self` |
| `add_champion` | `fn add_champion(&mut self, info: impl ModChampionInfo + 'static)` |
| `replace_champion` | `fn replace_champion(&mut self, info: impl ModChampionInfo + 'static)` |
| `add_item` | `fn add_item(&mut self, info: impl ModItemInfo + 'static)` |
| `add_draft_score_hook` | `fn add_draft_score_hook(&mut self, hook: impl ModDraftScoreHook + 'static)` |
| `add_player_input_ai` | `fn add_player_input_ai(&mut self, ai: impl ModPlayerInputAi + 'static)` |
| `set_extension` | `fn set_extension(&mut self, ext: impl ModExtension + 'static)` |
| `set_server_extension` | `fn set_server_extension(&mut self, ext: impl ModServerExtension + 'static)` |

## Champion and Combat Traits

### `ModChampionInfo`

Defines one champion.

Use `add_champion` with a new unique id to add a champion. Use `replace_champion` with an existing base champion id to rework that champion while preserving saved champion ids, ban/pick references, and patch references. If `replace_champion` receives an id that is not present in the base game, it behaves like `add_champion`.

| Method | Signature | Default |
| --- | --- | --- |
| `id` | `fn id(&self) -> &str` | required. For `replace_champion`, this must be the exact base champion id. |
| `name` | `fn name(&self) -> &str` | `self.id()` |
| `skill_icon` | `fn skill_icon(&self, skill_index: usize) -> (String, String)` | `skill_icon` sheet and `{id}_{index}` tag |
| `category` | `fn category(&self) -> ChampionCategory` | required |
| `tags` | `fn tags(&self) -> Vec<ChampionTag>` | required |
| `stat` | `fn stat(&self) -> EntityStat` | required |
| `growth` | `fn growth(&self) -> EntityStat` | required |
| `attack` | `fn attack(&self) -> Box<dyn ModAction>` | required |
| `skill` | `fn skill(&self) -> Box<dyn ModAction>` | required |
| `skill2` | `fn skill2(&self) -> Box<dyn ModAction>` | required |
| `ult` | `fn ult(&self) -> Option<Box<dyn ModAction>>` | `None` |
| `passive` | `fn passive(&self) -> Option<Box<dyn ModPassive>>` | `None` |

### `ModAction`

Defines an action used by a champion: attack, skill, skill2, or ult.

| Method | Signature | Default |
| --- | --- | --- |
| `clone_box` | `fn clone_box(&self) -> Box<dyn ModAction>` | required |
| `action_name` | `fn action_name(&self) -> &str` | `"attack"` |
| `duration` | `fn duration(&self) -> usize` | required |
| `cancelable` | `fn cancelable(&self) -> bool` | `false` |
| `cooltime` | `fn cooltime(&self, caster_stat: &EntityStat, caster_level: usize) -> usize` | required |
| `casting_target` | `fn casting_target(&self) -> CastingTarget` | required |
| `effect` | `fn effect(&self) -> Option<ModEffect>` | required |
| `cooltime_use_count` | `fn cooltime_use_count(&self, caster_stat: &EntityStat) -> usize` | `1` |
| `can_use_with_move` | `fn can_use_with_move(&self) -> bool` | `false` |
| `description` | `fn description(&self) -> String` | empty string |

### `ModEffect`

Returned by `ModAction::effect()`.

| Field | Type |
| --- | --- |
| `range` | `u64` |
| `growth_range` | `u64` |
| `start_timing` | `usize` |
| `casting` | `CastingType` |
| `target` | `CastingTarget` |
| `attack_type` | `AttackType` |
| `effect_type` | `Box<dyn ModEffectType>` |

### `ModEffectType`

Defines what an effect does when it fires.

| Method | Signature | Default |
| --- | --- | --- |
| `apply` | `fn apply(&self, ctx: &mut GameCtx, rng_seed: u64, caster_id: usize, input: InputTarget)` | required |
| `expected_damage` | `fn expected_damage(&self, caster_stat: &EntityStat) -> (usize, usize)` | `(0, 0)` |
| `expected_heal` | `fn expected_heal(&self, caster_stat: &EntityStat) -> usize` | `0` |
| `expected_shield` | `fn expected_shield(&self, caster_stat: &EntityStat) -> usize` | `0` |
| `expected_cc_time` | `fn expected_cc_time(&self) -> Option<usize>` | `None` |
| `expected_buff` | `fn expected_buff(&self, caster_stat: &EntityStat) -> Option<BuffState>` | `None` |
| `expected_move_distance` | `fn expected_move_distance(&self) -> Option<(usize, u64)>` | `None` |
| `expected_rush_effect` | `fn expected_rush_effect(&self) -> bool` | `false` |
| `auto_target` | `fn auto_target(&self) -> bool` | `false` |
| `on_caster` | `fn on_caster(&self) -> bool` | `false` |
| `can_move` | `fn can_move(&self) -> bool` | `false` |
| `linear_move_speed` | `fn linear_move_speed(&self) -> Option<usize>` | `None` |

### `ModPassive`

Optional champion passive. Implement `clone_box` because the engine clones runtime instances.

| Method | Signature | Default |
| --- | --- | --- |
| `clone_box` | `fn clone_box(&self) -> Box<dyn ModPassive>` | required |
| `on_spawn` | `fn on_spawn(&mut self, ctx: &mut GameCtx, player: usize, entity: usize)` | no-op |
| `on_attack` | `fn on_attack(&mut self, ctx: &mut GameCtx, player: usize, entity: usize, target: usize, damage: &mut usize)` | no-op |
| `on_damaged` | `fn on_damaged(&mut self, ctx: &mut GameCtx, player: usize, entity: usize, attacker: usize, damage: usize)` | no-op |
| `on_kill` | `fn on_kill(&mut self, ctx: &mut GameCtx, player: usize, entity: usize)` | no-op |
| `on_update` | `fn on_update(&mut self, ctx: &mut GameCtx, rng_seed: u64, player: usize, entity: usize)` | no-op |
| `on_base_attack` | `fn on_base_attack(&mut self, ctx: &mut GameCtx, rng_seed: u64, player: usize, entity: usize)` | no-op |
| `on_assist` | `fn on_assist(&mut self, ctx: &mut GameCtx, player: usize, entity: usize)` | no-op |
| `on_dead` | `fn on_dead(&mut self, ctx: &mut GameCtx, player: usize)` | no-op |

### `ModEffectBuff`

Custom persistent buff logic type. It is part of the API surface, but it is not registered directly through `ModRegistration`.

| Method | Signature | Default |
| --- | --- | --- |
| `on_damaged` | `fn on_damaged(&self, ctx: &mut GameCtx, attacker: usize, target: usize, damage: &mut usize, attack_type: AttackType)` | no-op |
| `update` | `fn update(&mut self, ctx: &mut GameCtx, rng_seed: u64)` | required |
| `is_end` | `fn is_end(&self, ctx: &GameCtx) -> bool` | required |

## Item Trait

### `ModItemInfo`

Defines an item and its runtime callbacks. Implement `clone_box` because the game creates independent runtime instances.

| Method | Signature | Default |
| --- | --- | --- |
| `clone_box` | `fn clone_box(&self) -> Box<dyn ModItemInfo>` | required |
| `key` | `fn key(&self) -> &str` | required |
| `icon` | `fn icon(&self) -> &str` | `self.key()` |
| `price` | `fn price(&self) -> usize` | required |
| `tier` | `fn tier(&self) -> usize` | required |
| `stat` | `fn stat(&self) -> BuffState` | required |
| `next_tier` | `fn next_tier(&self) -> Vec<String>` | empty |
| `previous_tier` | `fn previous_tier(&self) -> Vec<String>` | empty |
| `tags` | `fn tags(&self) -> Vec<ItemTag>` | empty |
| `category` | `fn category(&self) -> ItemCategory` | `ItemCategory::default()` |
| `on_attack` | `fn on_attack(&mut self, ctx: &mut GameCtx, caster: usize, target: usize, damage: &mut usize, damage_type: DamageType)` | no-op |
| `update` | `fn update(&mut self, ctx: &mut GameCtx, rng_seed: u64, player: usize)` | no-op |
| `on_spawn` | `fn on_spawn(&mut self, ctx: &mut GameCtx, player: usize)` | no-op |
| `on_healed` | `fn on_healed(&mut self, ctx: &mut GameCtx, caster: Option<usize>, entity: usize, heal: usize)` | no-op |
| `on_damaged` | `fn on_damaged(&mut self, ctx: &mut GameCtx, player: usize, entity: usize, attacker: usize, damage: usize)` | no-op |
| `on_kill` | `fn on_kill(&mut self, ctx: &mut GameCtx, rng_seed: u64, player: usize, entity: usize)` | no-op |
| `on_skill_hit` | `fn on_skill_hit(&mut self, ctx: &mut GameCtx, rng_seed: u64, caster: usize, target: usize)` | no-op |

## Simulation Context

### Handles

These are opaque pointer wrappers. Handles are only valid during the callback in which they were obtained. Do not store them.

| Type | Methods |
| --- | --- |
| `EntityHandle` | `null()`, `is_null()`, `unsafe from_ptr(ptr)`, `as_ptr()` |
| `PlayerHandle` | `null()`, `is_null()`, `unsafe from_ptr(ptr)`, `as_ptr()` |
| `ProjectileHandle` | `null()`, `is_null()`, `unsafe from_ptr(ptr)`, `as_ptr()` |

### `GameCtx`

`GameCtx` is the main handle passed to native callbacks.

Runtime service methods:

| Method | Signature |
| --- | --- |
| `register_service` | `fn register_service(&self, service_id: &str, version: ModServiceVersion, service: ModService) -> bool` |
| `query_service` | `fn query_service(&self, provider_mod_id: &str, service_id: &str, version_req: &str) -> Option<ModService>` |

Simulation query methods:

| Method | Signature |
| --- | --- |
| `tick` | `fn tick(&self) -> usize` |
| `seed` | `fn seed(&self) -> u64` |
| `score_diff` | `fn score_diff(&self, team: usize) -> i32` |
| `is_end` | `fn is_end(&self) -> bool` |
| `get_entity` | `fn get_entity(&self, id: usize) -> Option<EntityRef<'_>>` |
| `entity_count` | `fn entity_count(&self) -> usize` |
| `entity_at` | `fn entity_at(&self, index: usize) -> Option<EntityRef<'_>>` |
| `get_player` | `fn get_player(&self, id: usize) -> Option<PlayerRef<'_>>` |
| `champion_count` | `fn champion_count(&self) -> usize` |
| `champion_id_at` | `fn champion_id_at(&self, index: usize) -> usize` |
| `tower_count` | `fn tower_count(&self) -> usize` |
| `tower_id_at` | `fn tower_id_at(&self, index: usize) -> usize` |
| `player_count` | `fn player_count(&self) -> usize` |
| `player_at` | `fn player_at(&self, index: usize) -> Option<PlayerRef<'_>>` |
| `projectile_count` | `fn projectile_count(&self) -> usize` |
| `projectile_at` | `fn projectile_at(&self, index: usize) -> Option<ProjectileRef<'_>>` |
| `kill_log_count` | `fn kill_log_count(&self) -> usize` |
| `kill_log_at` | `fn kill_log_at(&self, index: usize) -> KillLogEntry` |
| `distance_sq` | `fn distance_sq(&self, id1: usize, id2: usize) -> u64` |
| `is_visible` | `fn is_visible(&self, team: usize, id: usize) -> bool` |

Simulation mutation methods:

| Method | Signature |
| --- | --- |
| `deal_damage` | `fn deal_damage(&mut self, attacker: usize, target: usize, ad: usize, ap: usize, attack_type: AttackType)` |
| `heal` | `fn heal(&mut self, caster: usize, target: usize, amount: usize)` |
| `add_buff` | `fn add_buff(&mut self, target: usize, buff: BuffState)` |
| `apply_cc` | `fn apply_cc(&mut self, target: usize, cc: CCState)` |

Debug draw methods:

| Method | Signature |
| --- | --- |
| `debug_draw_line` | `fn debug_draw_line(&mut self, x1: u64, y1: u64, x2: u64, y2: u64, color: u32)` |
| `debug_draw_circle` | `fn debug_draw_circle(&mut self, x: u64, y: u64, r: u64, color: u32)` |

Colors are packed `u32` values. Existing examples use values such as `0xff66ccff`.

### `EntityRef`

| Method | Signature |
| --- | --- |
| `handle` | `fn handle(&self) -> EntityHandle` |
| `id` | `fn id(&self) -> usize` |
| `stat` | `fn stat(&self) -> EntityStat` |
| `pos` | `fn pos(&self) -> EntityPos` |
| `hp` | `fn hp(&self) -> EntityHp` |
| `team` | `fn team(&self) -> usize` |
| `level` | `fn level(&self) -> usize` |
| `is_alive` | `fn is_alive(&self) -> bool` |
| `is_champion` | `fn is_champion(&self) -> bool` |
| `is_tower` | `fn is_tower(&self) -> bool` |
| `is_minion` | `fn is_minion(&self) -> bool` |
| `shield` | `fn shield(&self) -> usize` |
| `buff_count` | `fn buff_count(&self) -> usize` |
| `buff_at` | `fn buff_at(&self, index: usize) -> BuffState` |
| `cc_count` | `fn cc_count(&self) -> usize` |
| `cc_at` | `fn cc_at(&self, index: usize) -> CCInfo` |
| `radius` | `fn radius(&self) -> usize` |
| `is_targetable` | `fn is_targetable(&self) -> bool` |

### `PlayerRef`

| Method | Signature |
| --- | --- |
| `handle` | `fn handle(&self) -> PlayerHandle` |
| `champion` | `fn champion(&self) -> Option<EntityRef<'_>>` |
| `level` | `fn level(&self) -> usize` |
| `gold` | `fn gold(&self) -> usize` |
| `position` | `fn position(&self) -> Position` |
| `team` | `fn team(&self) -> usize` |
| `is_alive` | `fn is_alive(&self) -> bool` |
| `respawn_time` | `fn respawn_time(&self) -> usize` |
| `kills` | `fn kills(&self) -> usize` |
| `deaths` | `fn deaths(&self) -> usize` |
| `assists` | `fn assists(&self) -> usize` |
| `cs` | `fn cs(&self) -> usize` |

### `ProjectileRef`

| Method | Signature |
| --- | --- |
| `handle` | `fn handle(&self) -> ProjectileHandle` |
| `info` | `fn info(&self) -> ProjectileInfo` |

### Return Structs

| Type | Fields |
| --- | --- |
| `EntityPos` | `x: u64`, `y: u64` |
| `EntityHp` | `current: usize`, `max: usize` |
| `CCInfo` | `cc_type: u32`, `tick: u64` |
| `ProjectileInfo` | `x: u64`, `y: u64`, `caster_id: usize`, `team: usize`, `is_end: bool` |
| `KillLogEntry` | `tick: usize`, `killer_team: usize`, `killer_position: u32`, `killed_position: u32`, `assist_count: u32`, `assist_positions: [u32; 4]` |

`CCInfo::cc_type` currently uses: `0=Airborne`, `1=Stun`, `2=Bind`, `3=BlockAttack`, `4=BlockSkill`, `5=BlockMoveSkill`, `6=ForceMove`, `7=Taunt`, `8=Fear`, `9=Animation`, `255=Invalid`.

## Runtime Services

Runtime services let one native mod publish an opaque vtable and another native mod query it.

### `ModServiceVersion`

Fields: `major: u32`, `minor: u32`, `patch: u32`.

Method:

```rust
pub const fn new(major: u32, minor: u32, patch: u32) -> Self
```

### `ModService`

Fields:

| Field | Type |
| --- | --- |
| `data` | `*mut c_void` |
| `vtable` | `*const c_void` |

Methods:

| Method | Signature |
| --- | --- |
| `null` | `pub const fn null() -> Self` |
| `from_raw` | `pub const fn from_raw(data: *mut c_void, vtable: *const c_void) -> Self` |
| `is_null` | `pub fn is_null(&self) -> bool` |
| `vtable` | `pub unsafe fn vtable<T>(&self) -> Option<&T>` |

Use a shared `#[repr(C)]` vtable contract between provider and consumer mods. `query_service` accepts semver requirement strings such as `">=1.0.0, <2.0.0"`.

## AI Hooks

### `ModDraftScoreHook`

Adjusts ban/pick candidate scores after the base draft AI scores them.

| Method | Signature | Default |
| --- | --- | --- |
| `id` | `fn id(&self) -> &str` | required |
| `priority` | `fn priority(&self) -> i32` | `0` |
| `score_ban` | `fn score_ban(&self, ctx: &DraftScoreContext, candidate: usize, base_score: f32) -> DraftScoreDecision` | `Pass` |
| `score_pick` | `fn score_pick(&self, ctx: &DraftScoreContext, candidate: usize, base_score: f32) -> DraftScoreDecision` | `Pass` |

Lower priorities run first; higher priorities run later and see the score after earlier hooks.

### `DraftScoreContext`

Fields:

| Field | Type |
| --- | --- |
| `phase` | `DraftScorePhase` |
| `available_champions` | `&[usize]` |
| `ally_ban` | `&[usize]` |
| `enemy_ban` | `&[usize]` |
| `ally_pick` | `&[usize]` |
| `enemy_pick` | `&[usize]` |
| `is_explore` | `bool` |
| `difficulty` | `Difficulty` |

Related enums:

```rust
pub enum DraftScorePhase {
    Ban,
    Pick,
}

pub enum DraftScoreDecision {
    Pass,
    Add(f32),
    Replace(f32),
}
```

### `ModPlayerInputAi`

Replaces the final per-tick `Input` produced by the built-in player AI.

| Method | Signature | Default |
| --- | --- | --- |
| `clone_box` | `fn clone_box(&self) -> Box<dyn ModPlayerInputAi>` | required |
| `id` | `fn id(&self) -> &str` | required |
| `priority` | `fn priority(&self) -> i32` | `0` |
| `matches` | `fn matches(&self, ctx: &PlayerAiInitContext) -> bool` | `true` |
| `think` | `fn think(&mut self, ctx: &mut PlayerAiContext<'_, '_, '_>, base_input: Option<Input>) -> PlayerInputDecision` | required |

`PlayerInputDecision`:

```rust
pub enum PlayerInputDecision {
    Pass,
    Replace(Input),
}
```

### `PlayerAiInitContext`

Fields:

| Field | Type |
| --- | --- |
| `player_id` | `usize` |
| `athlete_id` | `usize` |
| `team` | `usize` |
| `position` | `Position` |
| `champion_name` | `String` |

### `PlayerAiContext`

| Method | Signature |
| --- | --- |
| `player_id` | `fn player_id(&self) -> usize` |
| `athlete_id` | `fn athlete_id(&self) -> usize` |
| `team` | `fn team(&self) -> usize` |
| `position` | `fn position(&self) -> Position` |
| `champion_name` | `fn champion_name(&self) -> &str` |
| `tick` | `fn tick(&self) -> usize` |
| `hp` | `fn hp(&self) -> Option<usize>` |
| `max_hp` | `fn max_hp(&self) -> Option<usize>` |
| `hp_ratio_percent` | `fn hp_ratio_percent(&self) -> Option<usize>` |
| `is_hp_below_percent` | `fn is_hp_below_percent(&self, threshold: usize) -> bool` |
| `is_valid_input` | `fn is_valid_input(&self, input: &Input) -> bool` |
| `get_run_away_input` | `fn get_run_away_input(&mut self) -> Option<Input>` |
| `get_run_away_without_skill_input` | `fn get_run_away_without_skill_input(&mut self) -> Option<Input>` |
| `get_recall_input` | `fn get_recall_input(&mut self) -> Option<Input>` |
| `is_safe_to_recall` | `fn is_safe_to_recall(&mut self) -> bool` |

## Client Extension API

### Re-Exported Client and UI Types

`mod_api` re-exports:

- `Assets`
- `RenderState`
- `Node`
- `NodeTemplate`
- `UI`
- `UIEvent`
- `UIEventHandlerContext`
- `Scene`
- `UIOutEvent`
- `ClientData`
- `ClientDatabase`

Alias:

```rust
pub type GameUI = UI<(), UIOutEvent>;
```

### `ModExtension`

Lifecycle hooks called on the client/game-loop side.

| Method | Signature | Default |
| --- | --- | --- |
| `on_init` | `fn on_init(&self, scene: &mut Scene, ui: &mut GameUI, assets: &mut Assets)` | no-op |
| `pre_update` | `fn pre_update(&self, scene: &mut Scene, ui: &mut GameUI, assets: &mut Assets, dt: f32)` | no-op |
| `post_update` | `fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, assets: &mut Assets, dt: f32)` | no-op |
| `pre_render` | `fn pre_render(&self, scene: &Scene, ui: &GameUI, assets: &Assets, state: &mut RenderState)` | no-op |
| `post_render` | `fn post_render(&self, scene: &Scene, ui: &GameUI, assets: &Assets, state: &mut RenderState)` | no-op |
| `on_end` | `fn on_end(&self, assets: &Assets)` | no-op |

Use `Scene::InGame { data }` to access `ClientData` during normal management gameplay.

### `ClientData`

Fields:

| Field | Type |
| --- | --- |
| `db` | `Rc<RefCell<ClientDatabase>>` |
| `main_tutorial` | `Option<MainTutorial>` |

Borrow helpers:

| Method | Signature |
| --- | --- |
| `db` | `fn db(&self) -> Ref<'_, ClientDatabase>` |
| `db_mut` | `fn db_mut(&self) -> RefMut<'_, ClientDatabase>` |

Common read helpers:

| Method | Signature |
| --- | --- |
| `player_team_id` | `fn player_team_id(&self) -> usize` |
| `player_team` | `fn player_team(&self) -> Option<Ref<'_, Team>>` |
| `team` | `fn team(&self, team_id: usize) -> Option<Ref<'_, Team>>` |
| `team_ids` | `fn team_ids(&self) -> Vec<usize>` |
| `athlete` | `fn athlete(&self, athlete_id: usize) -> Option<Ref<'_, Athlete>>` |
| `athlete_ids` | `fn athlete_ids(&self) -> Vec<usize>` |
| `staff` | `fn staff(&self, staff_id: usize) -> Option<Ref<'_, Staff>>` |
| `staff_ids` | `fn staff_ids(&self) -> Vec<usize>` |
| `knowledge_base` | `fn knowledge_base(&self, team_id: usize) -> Option<Ref<'_, KnowledgeBase>>` |
| `league` | `fn league(&self, league_id: usize) -> Option<Ref<'_, League>>` |
| `league_ids` | `fn league_ids(&self) -> Vec<usize>` |
| `tournament` | `fn tournament(&self, tournament_id: usize) -> Option<Ref<'_, Tournament>>` |
| `tournament_ids` | `fn tournament_ids(&self) -> Vec<usize>` |
| `match_info` | `fn match_info(&self, match_type: MatchType) -> Option<Ref<'_, MatchInfo>>` |
| `normal_match` | `fn normal_match(&self, match_id: usize) -> Option<Ref<'_, MatchInfo>>` |
| `practice_match` | `fn practice_match(&self, match_id: usize) -> Option<Ref<'_, MatchInfo>>` |
| `tutorial_match` | `fn tutorial_match(&self, match_id: usize) -> Option<Ref<'_, MatchInfo>>` |
| `solo_rank_match_info` | `fn solo_rank_match_info(&self, match_id: usize) -> Option<Ref<'_, MatchInfo>>` |
| `match_replay` | `fn match_replay(&self, match_id: usize) -> Option<Ref<'_, MatchReplayData>>` |
| `match_replay_ids` | `fn match_replay_ids(&self) -> Vec<usize>` |
| `league_competition` | `fn league_competition(&self, competition_id: usize) -> Option<Ref<'_, LeagueCompetition>>` |
| `league_competition_ids` | `fn league_competition_ids(&self) -> Vec<usize>` |
| `tournament_competition` | `fn tournament_competition(&self, competition_id: usize) -> Option<Ref<'_, TournamentCompetition>>` |
| `tournament_competition_ids` | `fn tournament_competition_ids(&self) -> Vec<usize>` |
| `solo_rank_match` | `fn solo_rank_match(&self, match_id: usize) -> Option<Ref<'_, SoloRankMatch>>` |
| `solo_rank_match_ids` | `fn solo_rank_match_ids(&self) -> Vec<usize>` |
| `champion_info` | `fn champion_info(&self, champion_name: &str) -> Option<Arc<dyn ChampionInfo>>` |

Client/server mod messaging:

| Method | Signature |
| --- | --- |
| `send_mod_command` | `fn send_mod_command(&self, mod_id: &str, command: &str, payload: impl Into<Vec<u8>>) -> bool` |
| `mod_events` | `fn mod_events(&self, mod_id: &str) -> Vec<ModClientEvent>` |
| `take_mod_events` | `fn take_mod_events(&self, mod_id: &str) -> Vec<ModClientEvent>` |

Save-data helpers:

| Method | Signature |
| --- | --- |
| `can_write_mod_save` | `fn can_write_mod_save(&self) -> bool` |
| `mod_save_version` | `fn mod_save_version(&self, mod_id: &str) -> usize` |
| `mod_save_set_version` | `fn mod_save_set_version(&self, mod_id: &str, version: usize) -> bool` |
| `mod_save_keys` | `fn mod_save_keys(&self, mod_id: &str) -> Vec<String>` |
| `mod_save_contains_key` | `fn mod_save_contains_key(&self, mod_id: &str, key: &str) -> bool` |
| `mod_save_get_bytes` | `fn mod_save_get_bytes(&self, mod_id: &str, key: &str) -> Option<Vec<u8>>` |
| `mod_save_set_bytes` | `fn mod_save_set_bytes(&self, mod_id: &str, key: &str, value: impl Into<Vec<u8>>) -> bool` |
| `mod_save_get_string` | `fn mod_save_get_string(&self, mod_id: &str, key: &str) -> Option<String>` |
| `mod_save_set_string` | `fn mod_save_set_string(&self, mod_id: &str, key: &str, value: impl Into<String>) -> bool` |
| `mod_save_remove_key` | `fn mod_save_remove_key(&self, mod_id: &str, key: &str) -> bool` |
| `mod_save_clear_namespace` | `fn mod_save_clear_namespace(&self, mod_id: &str) -> bool` |

### `ClientDatabase`

`ClientDatabase` is the broad client-side save/game snapshot. Public fields include:

```rust
pub scene: ClientScene,
pub id: usize,
pub time: NaiveDateTime,
pub teams: HashMap<usize, Team>,
pub knowledge_bases: HashMap<usize, KnowledgeBase>,
pub matches: HashMap<MatchType, MatchInfo>,
pub match_replays: HashMap<usize, MatchReplayData>,
pub leagues: HashMap<usize, League>,
pub tournaments: HashMap<usize, Tournament>,
pub athletes: HashMap<usize, Athlete>,
pub staffs: HashMap<usize, Staff>,
pub league_competitions: HashMap<usize, LeagueCompetition>,
pub tournament_competitions: HashMap<usize, TournamentCompetition>,
pub solo_rank_matches: Vec<SoloRankMatch>,
pub champion_info_sheet: ChampionInfoSheet,
pub game_setting: GameSetting,
pub map_setting: MapSetting,
pub item_setting: ItemSetting,
pub mod_ai_registry: ModAiRegistry,
pub mod_save_data: ModSaveData,
pub mod_events: Vec<ModClientEvent>,
pub pre_patch_data: HashMap<String, GamePatchState>,
pub server_mode: ServerMode,
pub training_plan: TeamTrainingPlan,
pub research_data: TeamResearchData,
pub champion_patch_statistics: HashMap<String, ChampionPatchStatistics>,
pub available_champions: Vec<String>,
pub recruit_done_athletes: Vec<RecruitDoneAthlete>,
pub scout_dispatch: Option<ScoutDispatchInfo>,
pub champion_positions: HashMap<String, Vec<usize>>,
pub head_to_head: HashMap<usize, (usize, usize)>,
```

Common `ClientDatabase` helper methods mirror the `ClientData` read helpers, but return direct references instead of `Ref` guards:

- `player_team_id`, `try_player_team`, `team`, `team_ids`
- `athlete`, `athlete_ids`, `athlete_current_region_id`, `athlete_has_visible_solo_rank_in_region`, `visible_solo_rank_athletes`
- `staff`, `staff_ids`
- `knowledge_base`
- `league`, `league_ids`
- `tournament`, `tournament_ids`
- `match_info`, `normal_match`, `practice_match`, `tutorial_match`, `solo_rank_match_info`
- `match_replay`, `match_replay_ids`
- `league_competition`, `league_competition_ids`
- `tournament_competition`, `tournament_competition_ids`
- `solo_rank_match`, `solo_rank_match_ids`
- `champion_info`
- `mod_events`
- `team_display_name`, `multiplayer_chat_sender_display`
- `team_salary_total`, `player_team`, `can_pause_save`, `is_player_vs_player_match`, `match_has_player_team`, `due_player_match`
- `current_intl_break_target_date`, `version_at_date`, `get_historical_sheet`, `get_historical_game_setting`

## Server Extension API

### `ModServerExtension`

Runs on the server/management side.

| Method | Signature | Default |
| --- | --- | --- |
| `on_server_start` | `fn on_server_start(&self, ctx: &mut ServerModContext)` | no-op |
| `before_management_tick` | `fn before_management_tick(&self, ctx: &mut ServerModContext)` | no-op |
| `after_management_tick` | `fn after_management_tick(&self, ctx: &mut ServerModContext)` | no-op |
| `handle_command` | `fn handle_command(&self, ctx: &mut ServerModContext, command: &ModServerCommand) -> ModServerCommandResult` | `Pass` |

### `ServerModContext`

Fields:

| Field | Type |
| --- | --- |
| `mod_id` | `&str` |
| `database` | `&mut Database` |
| `server_state` | `&mut ServerState` |

Methods:

| Method | Signature |
| --- | --- |
| `emit_event` | `fn emit_event(&mut self, event: impl Into<String>, payload: impl Into<Vec<u8>>) -> bool` |
| `emit_event_to_player` | `fn emit_event_to_player(&mut self, player_id: PlayerId, event: impl Into<String>, payload: impl Into<Vec<u8>>) -> bool` |
| `emit_event_to_team` | `fn emit_event_to_team(&mut self, team_id: usize, event: impl Into<String>, payload: impl Into<Vec<u8>>) -> bool` |
| `emit_event_to_command_sender` | `fn emit_event_to_command_sender(&mut self, command: &ModServerCommand, event: impl Into<String>, payload: impl Into<Vec<u8>>) -> bool` |
| `player_team_id` | `fn player_team_id(&self, player_id: PlayerId) -> Option<usize>` |
| `team_player_ids` | `fn team_player_ids(&self, team_id: usize) -> Vec<PlayerId>` |

### Client/Server Message Types

`ModClientEvent` fields:

| Field | Type |
| --- | --- |
| `mod_id` | `String` |
| `event` | `String` |
| `payload` | `Vec<u8>` |

`ModClientEvent` constants and methods:

| Item | Signature |
| --- | --- |
| `MAX_MOD_ID_LEN` | `usize = 128` |
| `MAX_NAME_LEN` | `usize = 128` |
| `MAX_PAYLOAD_LEN` | `usize = 1024 * 1024` |
| `new` | `fn new(mod_id: impl Into<String>, event: impl Into<String>, payload: impl Into<Vec<u8>>) -> Option<Self>` |
| `is_valid` | `fn is_valid(&self) -> bool` |

`ModClientEventTarget`:

```rust
pub enum ModClientEventTarget {
    Broadcast,
    Player(PlayerId),
    Team(usize),
}
```

`ModServerCommand` fields:

| Field | Type |
| --- | --- |
| `mod_id` | `String` |
| `command` | `String` |
| `payload` | `Vec<u8>` |
| `sender_player_id` | `Option<PlayerId>` |
| `sender_team_id` | `Option<usize>` |

`ModServerCommand` methods:

| Method | Signature |
| --- | --- |
| `new` | `fn new(mod_id: impl Into<String>, command: impl Into<String>, payload: impl Into<Vec<u8>>, sender_player_id: Option<PlayerId>, sender_team_id: Option<usize>) -> Option<Self>` |
| `is_valid` | `fn is_valid(&self) -> bool` |

`ModServerCommandResult`:

```rust
pub enum ModServerCommandResult {
    Pass,
    Handled,
}
```

## Mod Save Data

`ModSaveData` stores per-save bytes owned by mod id namespaces.

Limits:

| Constant | Value |
| --- | --- |
| `MAX_ID_LEN` | `128` |
| `MAX_KEY_LEN` | `128` |
| `MAX_VALUE_LEN` | `1024 * 1024` |

`ModSaveData` methods:

| Method | Signature |
| --- | --- |
| `namespace_count` | `fn namespace_count(&self) -> usize` |
| `namespace_ids` | `fn namespace_ids(&self) -> Vec<String>` |
| `has_namespace` | `fn has_namespace(&self, mod_id: &str) -> bool` |
| `namespace` | `fn namespace(&self, mod_id: &str) -> Option<&ModSaveNamespace>` |
| `save_version` | `fn save_version(&self, mod_id: &str) -> usize` |
| `set_version` | `fn set_version(&mut self, mod_id: &str, version: usize) -> bool` |
| `keys` | `fn keys(&self, mod_id: &str) -> Vec<String>` |
| `contains_key` | `fn contains_key(&self, mod_id: &str, key: &str) -> bool` |
| `get_bytes` | `fn get_bytes(&self, mod_id: &str, key: &str) -> Option<Vec<u8>>` |
| `set_bytes` | `fn set_bytes(&mut self, mod_id: &str, key: &str, value: Vec<u8>) -> bool` |
| `get_string` | `fn get_string(&self, mod_id: &str, key: &str) -> Option<String>` |
| `set_string` | `fn set_string(&mut self, mod_id: &str, key: &str, value: impl Into<String>) -> bool` |
| `remove_key` | `fn remove_key(&mut self, mod_id: &str, key: &str) -> bool` |
| `clear_namespace` | `fn clear_namespace(&mut self, mod_id: &str) -> bool` |

`ModSaveNamespace` methods:

| Method | Signature |
| --- | --- |
| `save_version` | `fn save_version(&self) -> usize` |
| `keys` | `fn keys(&self) -> Vec<String>` |

When called through `ClientData`, the helper also queues the matching save mutation packet for the server when writes are allowed.

## Core Value Types

### Data and Settings Types

These are re-exported directly from `game-core`:

- `Athlete`
- `Team`
- `Staff`
- `League`
- `Tournament`
- `LeagueCompetition`
- `TournamentCompetition`
- `KnowledgeBase`
- `MatchInfo`
- `MatchType`
- `MatchReplayData`
- `SoloRankMatch`
- `ChampionInfo`
- `ChampionInfoSheet`
- `GameSetting`
- `MapSetting`
- `ItemSetting`
- `TeamTrainingPlan`
- `TeamResearchData`
- `ChampionPatchStatistics`
- `GamePatchState`
- `ServerMode`
- `ScoutDispatchInfo`
- `RecruitDoneAthlete`

### Simulation and Content Types

Also re-exported from `game-core`:

- `EntityStat`
- `ChampionCategory`
- `ChampionSubCategory`
- `ChampionTag`
- `AttackType`
- `DamageType`
- `CastingType`
- `CastingTarget`
- `Position`
- `Input`
- `InputTarget`
- `ItemTag`
- `ItemCategory`
- `BuffState`
- `BuffType`
- `CCState`

Common field and variant reference:

```rust
pub struct EntityStat {
    pub attack: usize,
    pub magic_power: usize,
    pub hp: usize,
    pub defence: usize,
    pub magic_resistance: usize,
    pub move_speed: usize,
    pub hp_regen: usize,
    pub stack: usize,
    pub crit_chance: usize,
}

pub enum ChampionCategory {
    Melee,
    Range,
    Magician,
    Util,
    Assassin,
}

pub enum ChampionSubCategory {
    Rush,
    Sub,
    CC,
    Tank,
    Single,
    Range,
    Poking,
    Assassin,
}

pub enum ChampionTag {
    AD,
    AP,
    Heal,
    Shield,
    Dot,
    CC,
    Range,
    Melee,
    Tank,
    Magic,
}

pub enum ItemTag {
    AD,
    AP,
    AS,
    Defense,
    MagicResistance,
    HP,
    DefensePenetration,
    Vamp,
    HealReduce,
    ShieldBreak,
    MoveSpeed,
    AttackRange,
    Shield,
    HpPercentDamage,
    ASDebuff,
    ReflectDamage,
    Toughness,
    MRDebuff,
    RangeDamage,
    MRPenetration,
    CooltimeReduce,
    DotDamage,
    HPRegen,
    MyHpPercentDamage,
    ShareDamage,
    Range,
}

pub enum ItemCategory {
    AD,
    AttackSpeed,
    Defense,
    MagicResistance,
    Magic,
    Hp,
}

pub enum DamageType {
    AD,
    AP,
    Fixed,
}

pub enum AttackType {
    BaseAttack,
    Skill,
    Dot,
    DotIgnoreShield,
    Item,
    Well,
}

pub enum CastingType {
    Targeting,
    Position,
    Direction,
    None,
}

pub enum CastingTarget {
    Ally,
    AllyChampion,
    AllyChampionInCC,
    AllyNotSelf,
    AllyOnlySelf,
    Enemy,
    EnemyWithoutTower,
    EnemyChampion,
    EnemyChampionInCC,
    EnemyChampionRecentlyAttacked,
    Both,
    BothWithoutTower,
    BothChampion,
    None,
}

pub enum Position {
    Top,
    Jungle,
    Mid,
    Bottom,
    Support,
}

pub enum Input {
    Move { x: u64, y: u64 },
    Return,
    Attack { target: InputTarget },
    Skill { target: InputTarget },
    Skill2 { target: InputTarget },
    Ult { target: InputTarget },
}

pub enum InputTarget {
    Target { target_id: usize },
    Dir { dir_x: i64, dir_y: i64 },
    Pos { x: u64, y: u64 },
    None,
}

pub enum BuffType {
    Permanent,
    Time { tick: usize },
    WithShield,
}

pub enum CCState {
    Airborne { tick: u64 },
    Stun { tick: u64 },
    Bind { tick: u64 },
    BlockAttack { tick: usize },
    BlockSkill { tick: usize },
    BlockMoveSkill { tick: usize },
    ForceMove { tick: u64, dx: i64, dy: i64, speed: u64 },
    Taunt { tick: u64, target: usize },
    Fear { tick: u64, dx: i64, dy: i64 },
    Charm { tick: u64, dx: i64, dy: i64 },
    Animation { name: String, tick: u64 },
}
```

`BuffState` fields:

```rust
pub struct BuffState {
    pub name: ArrayString<64>,
    pub duration: BuffType,
    pub attack: i32,
    pub attack_mult: i32,
    pub magic_power: i32,
    pub magic_power_mult: i32,
    pub defence: i32,
    pub defence_mult: i32,
    pub hp: i32,
    pub hp_regen: i32,
    pub magic_resistance: i32,
    pub magic_resistance_mult: i32,
    pub vamp: i32,
    pub hp_mult: i32,
    pub move_speed_mult: i32,
    pub attack_speed_mult: i32,
    pub skill_cooldown_mult: i32,
    pub damage_reflect: usize,
    pub damaged_amplify: usize,
    pub defence_penetration: usize,
    pub magic_resistance_penetration: usize,
    pub toughness: usize,
    pub heal_reduce: usize,
    pub range: usize,
    pub base_attack_enemy_max_hp_damage: usize,
    pub self_max_hp_damage: usize,
    pub skill_enemy_max_hp_damage: usize,
    pub damaged_reduce: usize,
    pub dot_amplify: usize,
    pub cc_immune: bool,
    pub ult_cooldown_mult: i32,
    pub radius_mult: i32,
    pub crit_chance: i32,
    pub base_attack_damaged_reduce: usize,
    pub skill_damaged_reduce: usize,
    pub undying: bool,
    pub ignore_wall: bool,
}
```

Common helper methods on these primitive types:

| Type | Methods |
| --- | --- |
| `EntityStat` | `zero() -> Self`, `add_stat(&mut self, added: EntityStat)` |
| `ChampionCategory` | `to_text_key(&self) -> String` |
| `ChampionSubCategory` | `to_text_key(&self) -> String` |
| `AttackType` | `is_dot(&self) -> bool` |
| `CastingType` | `is_nontarget(&self) -> bool`, `convert(&self, caster: &Entity, entity: &Entity) -> InputTarget` |
| `CastingTarget` | `to_enemy(&self) -> bool`, `can_use_to_jungle(&self) -> bool`, `check(&self, caster: &Entity, target: &Entity) -> bool`, `check_projectile(&self, projectile: &Projectile, target: &Entity) -> bool` |
| `Position` | `as_index(&self) -> usize`, `as_index_with_rule(self, rule: &GameRule) -> usize`, `from_index(index: usize) -> Position`, `from_index_with_rule(index: usize, rule: &GameRule) -> Position`, `to_string(&self) -> String`, `line_priority(&self, line_type: LineType) -> usize` |
| `Input` | `is_act(&self) -> bool` |
| `InputTarget` | `adjust(&self, from_x: u64, from_y: u64, range: u64) -> Self` |
| `BuffType` | `is_end(&self, has_shield: bool) -> bool` |
| `BuffState` | `merge(&mut self, other: &BuffState)`, `apply(&self, stat: EntityStat) -> EntityStat` |
| `CCState` | `block_move(&self) -> bool`, `block_input(&self) -> bool`, `is_cc(&self) -> bool`, `tick(&self) -> u64` |

Some helper methods mention internal game types such as `Entity`, `Projectile`, `GameRule`, or `LineType`. They are listed because they are inherent methods on exported primitives in the matching SDK, but normal native mods should usually prefer the safe `GameCtx`/`EntityRef` wrappers unless they already have those internal values through another exported object.

### AI, Messaging, and Save Types

These are also exported from `game-core` and documented above:

- `DraftScoreContext`
- `DraftScoreDecision`
- `PlayerAiContext`
- `PlayerAiInitContext`
- `PlayerInputDecision`
- `ModClientEvent`
- `ModClientEventTarget`
- `ModServerCommand`
- `ModServerCommandResult`
- `ModServerExtension`
- `ServerModContext`
- `ModSaveData`
- `ModSaveNamespace`

## Low-Level ABI Types

These structs are public because the loader and SDK need an ABI boundary. Normal mods should prefer the safe wrappers on `GameCtx`, `EntityRef`, `PlayerRef`, and `ProjectileRef`.

- `SimulationVtable`
- `FrameVtable`
- `ModServiceVtable`

`FrameVtable` also contains a low-level `debug_text` function pointer. There is no safe `GameCtx::debug_text` wrapper in API version `(0, 8)`.

## Related Guides

- [Native Rust Mods](native-rust-mods.md)
- [Native AI Hooks](native-ai-hooks.md)
- [Mod Save Data](mod-save-data.md)
