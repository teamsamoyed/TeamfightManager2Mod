# Ban/Pick Illustration Packs

Ban/Pick illustration packs are optional data-only mods. They replace the large draft presentation without replacing champion sprites in matches, the champion pool grid, or small ban icons.

They work for local mods and Steam Workshop mods and do not require the Mod SDK, a native DLL, or `mod.override_info`.

## Folder Layout

```text
mods/my_portrait_pack/
  mod.mod_info
  banpick_illustrations/
    fighter.png
    knight.png
    pyromancer.png
```

Each PNG filename is the champion id. The asset loader removes the extension, so `fighter.png` maps to champion id `fighter`.

Use `mod_type: "banpick_illustration"` for a presentation-only pack:

```json
{
  "name": "My Ban/Pick Portrait Pack",
  "author": "Author Name",
  "version": "1.0.0",
  "mod_type": "banpick_illustration",
  "dependencies": [
    { "mod_id": "base", "version": ">=0.5.2" }
  ]
}
```

The game validates this type before treating it as presentation-only. A pack containing executable code, `mod.override_info`, champion data, settings, UI layouts, or other functional assets remains a normal session mod. Its portraits still work, but its normal multiplayer and save-session compatibility rules still apply.

## Image Rules

- Format: sRGB PNG.
- Recommended size: `512 x 640` pixels.
- Maximum accepted width or height: `1024` pixels.
- Composition: keep the face and important silhouette near the center.
- Naming: exactly `<champion_id>.png`; nested folders and variants are not part of this contract.

The game center-crops images to fit each portrait surface. Accepted images appear in:

- the large pick/ban confirmation showcase;
- the portrait flying toward a confirmed pick slot;
- confirmed side pick slots.

The champion pool grid, compact ban slots, battle sprites, records, tooltips, and other champion portraits keep using base-game assets.

If an image is missing, corrupt, or too large, the UI renders the current base-game pixel sprite. There is no separate built-in illustration fallback.

## Priority and Multiplayer

Enabled mods are processed in resolved mod order. If several mods provide the same champion id, the later mod wins. Explicit `mod.override_info` remaps are applied after the automatic illustration mapping and keep final priority.

A validated presentation-only illustration pack is omitted from the gameplay/save/lobby mod signature. Players in the same multiplayer session may use different validated portrait packs. The pack adds no save fields, gameplay data, network packets, simulation work, or date-progression state.

## Native SDK Helpers

A portrait pack does not need native code. Native SDK users who need the same stable paths and crop logic can import these `mod_api` re-exports:

- `BANPICK_ILLUSTRATION_DIRECTORY`
- `BANPICK_ILLUSTRATION_VIRTUAL_DIRECTORY`
- `BANPICK_ILLUSTRATION_MAX_DIMENSION`
- `banpick_illustration_asset_path(...)`
- `banpick_illustration_source_asset_path(...)`
- `banpick_illustration_cover_rect(...)`
- `resolve_banpick_illustration(...)`

These helpers do not change `ModRegistration` or the native `API_VERSION`, so existing native mods do not need to rebuild for this data-only feature.

## Memory Note

The current asset pipeline decodes PNG files during startup. Prefer `512 x 640` instead of publishing a full roster of unnecessarily large images. Dedicated Ban/Pick images over the hard dimension limit are ignored and removed from the loaded asset map after validation.
