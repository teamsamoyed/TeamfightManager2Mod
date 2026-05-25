# Native Rust Mods

Native Rust mods are for projects that need custom code. Use them when data-only champions or asset overrides are not enough.

You do not need this path for normal JSON, image, text, or data-only champion mods. Native Rust mods require the Mod SDK.

## Before You Start

Native DLLs are tied to the game version and SDK they were built with.

- Build with the Mod SDK that matches the game version you want to support.
- Rebuild your DLL after game updates when the SDK changes.
- Make sure the mod id returned by your DLL matches the mod folder name.
- If the DLL is incompatible, the game will show a diagnostics message on the title screen.

## Minimal DLL

```rust
use mod_api::*;

const MOD_ID: &str = "my_mod";

fn init(_ctx: &GameCtx) -> ModRegistration {
    ModRegistration::new(MOD_ID)
}

declare_mod!(init);
```

The `declare_mod!` macro exports the entry point the game needs in order to load the DLL.

## Registering Content

Most native mods create a `ModRegistration`, add content to it, and return it:

```rust
fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new("my_mod");
    reg.add_champion(MyChampion);
    reg.add_item(MyItem::default());
    reg.add_draft_score_hook(MyDraftScoreHook);
    reg.add_player_input_ai(MyPlayerInputAi::default());
    reg.set_extension(MyExtension::default());
    reg.set_server_extension(MyServerExtension);
    reg
}
```

You can register:

- `ModChampionInfo`: a champion with custom runtime logic.
- `ModItemInfo`: an item with metadata and runtime callbacks.
- `ModDraftScoreHook`: ban/pick AI score adjustment hooks.
- `ModPlayerInputAi`: final player input replacement hooks.
- `ModExtension`: lifecycle hooks for UI, scene, and asset behavior.
- `ModServerExtension`: server-side management hooks and client command handling.

For AI-specific examples, see [Native AI Hooks](native-ai-hooks.md).
For the full exported Rust API surface, see [Native Mod API Reference](native-mod-api-reference.md).
For save-slot data owned by your mod, see [Mod Save Data](mod-save-data.md).

## Reading Current Save Data

`Scene::InGame` gives extensions a `ClientData` handle. It exposes the current client-side game data directly, using the game's internal data types such as `Athlete`, `Team`, `MatchInfo`, `League`, and `ChampionInfo`.

For quick ID-based lookups, call the helper methods on `ClientData`:

```rust
impl ModExtension for MyExtension {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else {
            return;
        };

        let player_team_id = data.player_team_id();

        if let Some(team) = data.team(player_team_id) {
            println!("My team: {}", team.name);
        }

        for athlete_id in data.athlete_ids() {
            if let Some(athlete) = data.athlete(athlete_id) {
                println!("Athlete {}: {}", athlete.id, athlete.name);
            }
        }
    }
}
```

These helpers return borrowed internal data. They are not DTOs or copied view models. The borrow type is Rust's normal `Ref<'_, T>` guard from the client's internal `RefCell`; use it like `&T` and let it drop before taking another mutable borrow.

If you want to keep one database borrow and do several lookups, use `data.db()`:

```rust
let db = data.db();

let team = db.team(team_id);
let athlete = db.athlete(athlete_id);
let match_info = db.normal_match(match_id);
let champion = db.champion_info("fighter");
```

Available lookup helpers include:

- `player_team_id`, `player_team`, `team`, `team_ids`
- `athlete`, `athlete_ids`
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

## Champion Example

`ModChampionInfo` defines the champion's identity, stats, actions, and optional passive:

```rust
#[derive(Debug)]
struct MyChampion;

impl ModChampionInfo for MyChampion {
    fn id(&self) -> &str { "my_mod_fire_mage" }
    fn name(&self) -> &str { "my_mod_fire_mage" }
    fn category(&self) -> ChampionCategory { ChampionCategory::Magician }
    fn tags(&self) -> Vec<ChampionTag> { vec![ChampionTag::AP, ChampionTag::Range] }

    fn stat(&self) -> EntityStat {
        EntityStat {
            attack: 40,
            magic_power: 65,
            hp: 620,
            defence: 20,
            magic_resistance: 30,
            move_speed: 1050,
            hp_regen: 2,
            stack: 0,
            crit_chance: 0,
        }
    }

    fn growth(&self) -> EntityStat {
        EntityStat {
            attack: 3,
            magic_power: 7,
            hp: 75,
            defence: 3,
            magic_resistance: 3,
            move_speed: 0,
            hp_regen: 1,
            stack: 0,
            crit_chance: 0,
        }
    }

    fn attack(&self) -> Box<dyn ModAction> { Box::new(MyAttack) }
    fn skill(&self) -> Box<dyn ModAction> { Box::new(MySkill) }
    fn skill2(&self) -> Box<dyn ModAction> { Box::new(MySkill2) }
}
```

Each action can return a `ModEffect`. The effect uses `GameCtx` to read game state and apply damage, healing, buffs, crowd control, or debug drawing.

## Action and Effect Example

```rust
#[derive(Clone, Debug)]
struct MySkill;

impl ModAction for MySkill {
    fn clone_box(&self) -> Box<dyn ModAction> { Box::new(self.clone()) }
    fn action_name(&self) -> &str { "skill" }
    fn duration(&self) -> usize { 18 }
    fn cooltime(&self, _stat: &EntityStat, _level: usize) -> usize { 240 }
    fn casting_target(&self) -> CastingTarget { CastingTarget::Enemy }

    fn effect(&self) -> Option<ModEffect> {
        Some(ModEffect {
            range: 65000,
            growth_range: 0,
            start_timing: 10,
            casting: CastingType::Targeting,
            target: CastingTarget::Enemy,
            attack_type: AttackType::Skill,
            effect_type: Box::new(MySkillEffect),
        })
    }
}

#[derive(Debug)]
struct MySkillEffect;

impl ModEffectType for MySkillEffect {
    fn apply(&self, ctx: &mut GameCtx, _rng_seed: u64, caster_id: usize, input: InputTarget) {
        let InputTarget::Target { target_id } = input else {
            return;
        };

        let damage = ctx.get_entity(caster_id)
            .map(|caster| 50 + caster.stat().magic_power)
            .unwrap_or(50);

        ctx.deal_damage(caster_id, target_id, 0, damage, AttackType::Skill);
    }

    fn expected_damage(&self, caster_stat: &EntityStat) -> (usize, usize) {
        (0, 50 + caster_stat.magic_power)
    }
}
```

Actions, passives, and items that are cloned by the game need `clone_box`. Deriving `Clone` and returning `Box::new(self.clone())` is usually enough.

## Item Example

`ModItemInfo` defines a new item and any callbacks it needs:

```rust
#[derive(Clone, Debug, Default)]
struct MyItem {
    hit_count: usize,
}

impl ModItemInfo for MyItem {
    fn clone_box(&self) -> Box<dyn ModItemInfo> { Box::new(self.clone()) }
    fn key(&self) -> &str { "my_mod_claw" }
    fn icon(&self) -> &str { "t3_0" }
    fn price(&self) -> usize { 650 }
    fn tier(&self) -> usize { 2 }

    fn stat(&self) -> BuffState {
        BuffState {
            duration: BuffType::Permanent,
            attack: 40,
            attack_speed_mult: 10,
            ..Default::default()
        }
    }

    fn previous_tier(&self) -> Vec<String> {
        vec!["soldiers_longsword".to_string()]
    }

    fn next_tier(&self) -> Vec<String> {
        vec!["conquerors_greatsword".to_string()]
    }

    fn tags(&self) -> Vec<ItemTag> {
        vec![ItemTag::AD, ItemTag::AS]
    }

    fn category(&self) -> ItemCategory {
        ItemCategory::AD
    }
}
```

Use `previous_tier` to make the item reachable from an existing base item without replacing the full item setting file.

## Runtime Services Between Native Mods

Native DLL mods can expose small runtime services for other native DLL mods. Use this when you want to publish a reusable developer/library mod that other mods depend on.

This is not direct DLL linking. The game owns the service registry, loads dependencies first, and lets a consumer mod query a provider mod by mod id, service id, and version requirement.

The dependency still belongs in the consumer's `mod.mod_info`:

```json
{
  "dependencies": [
    {
      "mod_id": "base",
      "version": ">=0.1.0"
    },
    {
      "mod_id": "service_provider",
      "version": ">=1.0.0, <2.0.0"
    }
  ]
}
```

When the consumer is enabled, an installed dependency is included automatically and loaded first. This works the same for local folders and Workshop-installed folders because `mod.mod_info` is the game-side source of truth.

The `base` entry in that dependency list is special: it means the Teamfight Manager 2 game version, not a base asset package. Use it to declare the game version line your native DLL was built and tested against.

Provider example:

```rust
use std::ffi::c_void;

use mod_api::*;

const MOD_ID: &str = "service_provider";
const SERVICE_ID: &str = "math.v1";

#[repr(C)]
pub struct MathServiceV1 {
    pub bonus: unsafe extern "C" fn(value: u32) -> u32,
}

unsafe extern "C" fn bonus(value: u32) -> u32 {
    value + 77
}

static MATH_SERVICE: MathServiceV1 = MathServiceV1 {
    bonus,
};

fn init(ctx: &GameCtx) -> ModRegistration {
    ctx.register_service(
        SERVICE_ID,
        ModServiceVersion::new(1, 0, 0),
        ModService::from_raw(
            std::ptr::null_mut(),
            &MATH_SERVICE as *const MathServiceV1 as *const c_void,
        ),
    );

    ModRegistration::new(MOD_ID)
}

declare_mod!(init);
```

Consumer example:

```rust
use mod_api::*;

const MOD_ID: &str = "service_consumer";
const PROVIDER_MOD_ID: &str = "service_provider";
const SERVICE_ID: &str = "math.v1";

#[repr(C)]
struct MathServiceV1 {
    bonus: unsafe extern "C" fn(value: u32) -> u32,
}

fn init(ctx: &GameCtx) -> ModRegistration {
    let provider_bonus = ctx
        .query_service(PROVIDER_MOD_ID, SERVICE_ID, ">=1.0.0, <2.0.0")
        .and_then(|service| unsafe {
            service
                .vtable::<MathServiceV1>()
                .map(|vtable| (vtable.bonus)(23) as usize)
        })
        .unwrap_or(0);

    let mut reg = ModRegistration::new(MOD_ID);
    reg.add_item(MyItem::with_bonus(provider_bonus));
    reg
}

declare_mod!(init);
```

Keep service boundaries simple:

- Use `#[repr(C)]` service structs.
- Keep provider and consumer layouts exactly matched.
- Prefer primitive values, opaque handles, and explicit function pointers.
- Do not pass Rust trait objects, `String`, `Vec`, or provider-owned allocations unless you also define clear ownership rules.
- Keep the provider service data and vtable valid for as long as the provider DLL is loaded.
- Change the service id or major version when you break the service layout.

## Extension Hooks

`ModExtension` lets a native mod react to the scene, UI, and asset lifecycle:

```rust
#[derive(Default)]
struct MyExtension;

impl ModExtension for MyExtension {
    fn on_init(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets) {}

    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        // UI or scene logic here.
    }
}
```

These hooks are powerful, so keep them focused:

- Check that UI nodes exist before changing them.
- Add small UI pieces instead of replacing full base layouts.
- Register event handlers once, or guard setup so it does not run repeatedly.

## Server Extension Hooks

`ModServerExtension` runs on the game server side. Use it for management/save logic that must be authoritative, not for direct UI work.

```rust
use mod_api::*;

const MOD_ID: &str = "my_mod";

struct MyServerExtension;

impl ModServerExtension for MyServerExtension {
    fn on_server_start(&self, ctx: &mut ServerModContext) {
        // Runs once after the server initializes the save for this session.
        ctx.database.mod_save_data.set_string(MOD_ID, "server_started_at", ctx.database.time.to_string());
    }

    fn before_management_tick(&self, ctx: &mut ServerModContext) {
        // Runs before a server-side management time step.
        let current_time = ctx.database.time;
        ctx.database.mod_save_data.set_string(MOD_ID, "last_seen_before_tick", current_time.to_string());
    }

    fn after_management_tick(&self, ctx: &mut ServerModContext) {
        // Runs after the server-side management systems for a time step.
        ctx.emit_event("tick_finished", ctx.database.time.to_string().into_bytes());
    }
}
```

Register it separately from the client/UI extension:

```rust
fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(MyClientExtension::default());
    reg.set_server_extension(MyServerExtension);
    reg
}
```

The server context exposes the actual `Database` and `ServerState` used by the server. Changes made here are authoritative. Keep edits narrow and prefer existing game methods when they exist, because bypassing server rules can desync multiplayer saves or skip required side effects.

## Mod Commands and Events

Client/UI code can send a small command packet to the server:

```rust
impl ModExtension for MyClientExtension {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else {
            return;
        };

        data.send_mod_command(MOD_ID, "mark_seen", b"hello".to_vec());

        for event in data.take_mod_events(MOD_ID) {
            if event.event == "mark_seen_done" {
                println!("server replied: {:?}", event.payload);
            }
        }
    }
}
```

The matching server extension handles the command and can emit events back to clients:

```rust
impl ModServerExtension for MyServerExtension {
    fn handle_command(&self, ctx: &mut ServerModContext, command: &ModServerCommand) -> ModServerCommandResult {
        if command.command != "mark_seen" {
            return ModServerCommandResult::Pass;
        }

        if let Some(team_id) = command.sender_team_id {
            ctx.database.mod_save_data.set_string(MOD_ID, "last_command_team", team_id.to_string());
        }
        ctx.emit_event_to_command_sender(command, "mark_seen_done", command.payload.clone());
        ModServerCommandResult::Handled
    }
}
```

Commands and events are namespaced by mod id. Names are limited to 128 bytes, payloads are limited to 1 MiB, and invalid packets are rejected.

Server events can be sent to different targets:

```rust
ctx.emit_event("broadcast_event", vec![]);
ctx.emit_event_to_player(PlayerId(0), "player_event", vec![]);
ctx.emit_event_to_team(team_id, "team_event", vec![]);
ctx.emit_event_to_command_sender(command, "reply_event", vec![]);
```

Use targeted events for UI replies or team-private information in multiplayer. Use broadcast events only for information every connected client can receive.

## Per-Save Mod Data

Native extensions can store custom data in the current save through `ClientData::mod_save_*` helpers when the scene is `Scene::InGame`.

```rust
impl ModExtension for MyExtension {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else {
            return;
        };

        if data.mod_save_get_string("my_mod", "initialized").is_none() {
            data.mod_save_set_version("my_mod", 1);
            data.mod_save_set_string("my_mod", "initialized", "true");
        }
    }
}
```

The data is namespaced by mod id and saved with the game database. Multiplayer league writes are host-only, and write helpers return `false` if the request cannot be queued. See [Mod Save Data](mod-save-data.md) for limits, migration patterns, and the full helper list.

## Building With the Mod SDK

The SDK contains the prebuilt `mod-api` files and build helpers:

```text
mod-sdk/
  deps/
  native/
  build_mod.bat
  build_mod_cargo.ps1
  rust-toolchain.toml
  toolchain_version.txt
  template/
    Cargo.toml
    src/lib.rs
```

For simple one-file mods, a folder with only `src/lib.rs` still works. The SDK calls `rustc` directly and injects the matching `mod_api` crate:

```bat
cd mod-sdk
build_mod.bat path\to\your_mod\src\lib.rs
```

For mods that need external Rust crates, use a normal Cargo project in the mod folder:

```text
my_mod/
  mod.mod_info
  Cargo.toml
  src/
    lib.rs
  preview.png
```

Example `Cargo.toml`:

```toml
[package]
name = "my_mod"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
rand = "0.8"
serde_json = "1.0"
```

Do not add `mod-api` to `[dependencies]` in the public SDK workflow. The uploader and SDK build script inject the matching prebuilt `mod_api` crate automatically so the DLL matches the game SDK.

Manual Cargo build through the SDK:

```bat
cd mod-sdk
build_mod.bat path\to\your_mod
```

The build copies the produced DLL back to the mod folder as `my_mod.dll`, using the mod folder name. Keep the folder name and the `ModRegistration::new("my_mod")` id aligned.

If you develop inside another Cargo workspace and Cargo says the package is unexpectedly inside a workspace, add an empty `[workspace]` table to the mod's `Cargo.toml` or move the mod folder outside that workspace.

## Uploading Native Rust Mods

`TFM2ModUploader.exe` can build and upload a native Rust mod when the SDK is installed next to it.

Recommended layout:

```text
TeamfightManager2.exe
TFM2ModUploader.exe
steam_api64.dll
bundle.game_data
mod-sdk/
  deps/
  native/
  build_mod.bat
  build_mod_cargo.ps1
  rust-toolchain.toml
```

The uploader checks the selected mod folder in this order:

1. If `Cargo.toml` exists, it builds with Cargo. This supports external crates from crates.io or other normal Cargo dependency sources.
2. If there is no `Cargo.toml` but `src/lib.rs` exists, it uses the older direct `rustc` build.
3. If neither exists, no native build step is shown.

When **Build native Rust code before uploading** is checked, the uploader uses the SDK's `mod-api` files to build the DLL first. It then uploads the compiled DLL and the rest of the runtime mod assets. Build-only files and source folders such as `src/`, `target/`, `Cargo.toml`, and `Cargo.lock` are skipped during upload, so your Rust source code is not sent to Workshop.

If you already built the DLL yourself, leave the DLL in the mod folder and uncheck the build option before uploading.

The SDK and game version should match. After a game update that changes the SDK, rebuild the DLL and upload a new Workshop update.
## Common Native Load Problems

- The DLL is missing from the mod folder.
- The DLL was built with a different SDK.
- The DLL registers a mod id that does not match the folder name.
- The DLL does not export the expected entry point because `declare_mod!` was not used.
- The DLL does not export the expected API version symbol because it was built with an old SDK.
- The Rust toolchain or SDK artifacts do not match the target game version.
- The entry function panicked or returned an invalid registration.
