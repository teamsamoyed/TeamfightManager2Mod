# Override Existing Champions

You can rework a built-in champion instead of adding a new one. The trick is the
**id**: give your content the id of an existing base champion, and the game uses
your version in its place.

Because the id stays the same:

- saves, ban/pick records (including draft candidate positions), active champion
  pools, internal roster ordering, and patch data keep pointing at the same
  champion,
- the rework is non-destructive — the base definition is untouched, your version
  just sits on top while the mod is enabled,
- disabling the mod restores the original champion.

There are two ways to do it, matching the two kinds of champion content:

- **Data-only** — a `.data_champion` whose `id` is an existing champion id.
- **Native** — a `ModChampionInfo` registered with `replace_champion`.

Start with the data-only path. It needs no SDK, and it integrates with the
automatic balance/patch system.

## Data-Only Rework

A rework is just a normal data champion whose `id` matches a base champion. There
is no separate "override" flag and no `mod.override_info` entry — same id is the
whole mechanism.

```text
mods/my_mod/champion/fighter.data_champion
```

```json
{
  "id": "fighter",
  "category": "Range",
  "tags": ["AD", "Range"],
  "stat": {
    "attack": 300,
    "magic_power": 0,
    "hp": 3000,
    "defence": 80,
    "magic_resistance": 50,
    "move_speed": 1250,
    "hp_regen": 5,
    "stack": 0,
    "crit_chance": 0
  },
  "growth": {
    "attack": 15,
    "magic_power": 0,
    "hp": 150,
    "defence": 6,
    "magic_resistance": 4,
    "move_speed": 14,
    "hp_regen": 1,
    "stack": 0,
    "crit_chance": 0
  },
  "attack": { "action_name": "attack" },
  "skill": {
    "action_name": "skill",
    "description": "#asset/my_mod/text/champion?rework_fighter.skill"
  },
  "skill2": {
    "action_name": "skill2",
    "description": "#asset/my_mod/text/champion?rework_fighter.skill2"
  },
  "ult": {
    "action_name": "ult",
    "description": "#asset/my_mod/text/champion?rework_fighter.ult"
  }
}
```

Everything in the file replaces the built-in: stats, growth, category, tags,
actions, sprite, icons, and descriptions. The schema is identical to a new
champion — see [Data-Only Champions](data-champion.md) and the
[Data Champion Schema](data-champion-schema/index.md). The only thing that makes
this a rework is the matching `id`.

### Keeping the original look

The `sprite` field is optional. If you omit it, the champion keeps its original
base sprite and animations, which is exactly what you want for a stats- or
skills-only rework. Set `sprite` only when you also want to change the visual.

The same is true for `skill_icons` / `skill_icon`: omit them to fall back to no
custom icons, or set them to replace the icons.

### Descriptions

Point skill descriptions at your own i18n namespace:

```text
#asset/my_mod/text/champion?rework_fighter.skill
```

Ship `mods/my_mod/text/champion.i18n` with those keys. You do **not** need a
`mod.override_info` merge for this, because you are referencing your own asset
path, not overwriting base text. Only use an i18n `merge` if you want to change
the champion's **name** or other base text keyed by the champion id. See
[Asset Overrides and i18n](asset-overrides-and-i18n.md).

### Auto-balance

Data champion reworks plug into the automatic balance/patch system, because their
action numbers are real data fields. See
[Patchable Fields](data-champion-schema/patchable-fields.md).

## Native Rework

A native rework is a `ModChampionInfo` whose `id()` returns the existing champion
id. Register it with `replace_champion`:

```rust
fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new("my_mod");
    reg.replace_champion(MyFighterRework);
    reg
}

impl ModChampionInfo for MyFighterRework {
    fn id(&self) -> &str { "fighter" } // existing base champion id
    // ...stats, actions, passive, etc.
}
```

`replace_champion` is semantically the same as `add_champion`; it just documents
intent. If the id does not exist in the base game, it behaves like
`add_champion` (you get a new champion instead of a rework).

On the [stable mod API](stable-native-mods.md) there is no separate replace
call — `add_champion` with an existing base champion id reworks that champion
the same way.

Use a native rework when the champion needs custom runtime logic that JSON cannot
express. One trade-off: native action numbers are compiled code, so a native
rework does **not** expose patchable skill numbers to the automatic balance
system. If you want auto-balance to tune the numbers, prefer a data rework. See
[Native Rust Mods](native-rust-mods.md).

## Finding a Champion's Id

Champion ids are the keys defined in the game's champion data
(`ChampionInfoSheet`). They are the authority — not the sprite/aseprite path or
any other rendering asset, which only happen to be named after the champion. The
id is the same string used by saves, ban/pick data, and lookups such as
`db.champion_info("fighter")`.

From a native mod, read them straight from the sheet rather than from a fixed
list, so it always reflects the current roster:

```rust
// every champion id (base + any mod champions)
for id in db.champion_info_sheet.get_champions_list().keys() {
    println!("{id}");
}

// confirm a single id; returns None if it does not exist
let exists = db.champion_info("fighter").is_some();
```

## Precedence and Compatibility

- **Native beats data.** If a native rework and a data rework target the same id,
  the native mechanic wins.
- **One rework per id.** If two enabled mods rework the same champion, only one
  wins, and which one can depend on mod load order. Treat reworking the same
  champion from two mods as a conflict, the same as any other asset override.
- **Existing saves.** Enabling a rework applies it to saves created before the mod
  was added. Disabling the mod reverts to the base champion.
- **Edited/patched values are preserved.** Balance-patched or database-edited
  numbers for a reworked champion survive save/load; reloading the mod re-binds
  behavior without overwriting those values.
- **Multiplayer.** The host's champion sheet is authoritative and syncs to clients.
  Native runtime behavior re-binds on each client that has the same mod enabled.

## Verifying a Rework

1. Enable the mod and start or load a game.
2. Open the champion in the champion info UI. Its stats and skill tooltips should
   show your reworked values while the id and name stay the same.
3. Pick the champion in a match to confirm the new behavior.
4. Disable the mod and confirm the champion returns to its base form.
