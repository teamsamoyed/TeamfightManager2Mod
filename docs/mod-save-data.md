# Mod Save Data

Native Rust mods can store small custom data inside the game's save file.

Use this for data that belongs to one save slot, such as a mod tutorial flag, custom progression, generated state, or settings that should travel with the saved league. Do not use it for large assets, logs, or global preferences that should apply to every save.

Mod save data is available through `ClientData::mod_save_*` helpers. The most common place to access it is a `ModExtension` while the scene is `Scene::InGame`.

Server extensions can also write authoritative save data through `ctx.database.mod_save_data` in `ModServerExtension` hooks or command handlers.

## Basic Example

```rust
use mod_api::*;

const MOD_ID: &str = "my_mod";

#[derive(Default)]
struct MyExtension;

impl ModExtension for MyExtension {
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        let Scene::InGame { data } = scene else {
            return;
        };

        if data.mod_save_get_string(MOD_ID, "initialized").is_none() {
            data.mod_save_set_version(MOD_ID, 1);
            data.mod_save_set_string(MOD_ID, "initialized", "true");
        }
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(MyExtension::default());
    reg
}

declare_mod!(init);
```

This writes to the namespace named by `MOD_ID`. Use your own mod id, and keep it equal to your mod folder name and `ModRegistration::new(...)` id.

## Available Helpers

```rust
data.can_write_mod_save();

data.mod_save_version(MOD_ID);
data.mod_save_set_version(MOD_ID, 1);

data.mod_save_keys(MOD_ID);
data.mod_save_contains_key(MOD_ID, "key");

data.mod_save_get_bytes(MOD_ID, "key");
data.mod_save_set_bytes(MOD_ID, "key", vec![1, 2, 3]);

data.mod_save_get_string(MOD_ID, "key");
data.mod_save_set_string(MOD_ID, "key", "value");

data.mod_save_remove_key(MOD_ID, "key");
data.mod_save_clear_namespace(MOD_ID);
```

Write helpers return `true` when the local request was accepted and queued. They return `false` when the mod id, key, value, or multiplayer write authority is invalid.

## Versions and Migration

Each mod namespace has a version number. Use it to migrate your own saved data:

```rust
let version = data.mod_save_version(MOD_ID);

if version < 1 {
    data.mod_save_set_string(MOD_ID, "initialized", "true");
    data.mod_save_set_version(MOD_ID, 1);
}
```

The namespace version is separate from `mod.mod_info`'s package version. It is only for your saved data format.

## Limits

Current limits:

- Mod id: non-empty, up to 128 bytes, no NUL bytes.
- Key: non-empty, up to 128 bytes, no NUL bytes.
- Value: up to 1 MiB per key.
- String helpers store UTF-8 bytes.

Use compact values. If you need structured data, serialize your own JSON or binary blob into bytes or a string.

## Multiplayer Behavior

Single-player saves can write mod save data normally.

In multiplayer league saves, only the host can write mod save data. Non-host clients should check `can_write_mod_save()` or handle a `false` return value from write helpers.

The server owns the final saved data. It validates writes, updates the save database, and broadcasts changes to other clients.

When a `ModServerExtension` writes `ctx.database.mod_save_data` directly, that change is already on the authoritative server database. Use `ctx.emit_event(...)` if the client UI should react immediately, or rely on the next normal data sync if immediate UI feedback is not needed.

## Practical Advice

- Do not write the same value every frame. Check whether the value is missing or changed first.
- Use one namespace: your own `MOD_ID`.
- Prefer small string values for flags and counters.
- Use namespace versions for migrations instead of renaming old keys immediately.
- Keep global mod configuration outside save data when it should apply to all saves.
