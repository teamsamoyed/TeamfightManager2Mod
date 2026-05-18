# Assets and Sprite Sheets

Most image problems in mods come from one small detail: the game does not always use an image file directly. A sprite, icon set, or animation is often a group of related assets that share the same base path.

For example, this file:

```text
mods/example/champions/fire_mage.aseprite
```

is loaded as:

```text
asset/example/champions/fire_mage
```

When the game prepares that Aseprite file for rendering, it may also create extra assets next to it:

```text
asset/example/champions/fire_mage#sheet
asset/example/champions/fire_mage#anim
asset/example/champions/fire_mage#data
```

The part after `#` is not a file extension. It is a named piece of the same asset.

## The Common Asset Parts

`#sheet` is the image that gets drawn. It is usually a PNG atlas.

`#anim` is a list of named animations. Each animation tag points to one or more rectangles inside `#sheet`, with a duration for each frame. Champion sprites and skill effects usually use this.

`#data` is a list of named still-image rectangles inside `#sheet`. UI icons, item icons, inline text icons, and simple image atlases usually use this.

`#lanim` is layered animation data. It is used by more complex layered animation rendering and is usually only needed when you are matching the base game's layered Aseprite setup.

Most mod files only need `#sheet`, `#anim`, and `#data`.

## What a Sprite Sheet Tag Means

A sprite sheet is one image that contains many smaller images. A tag is the name used to pick one of those smaller images or animations.

For a skill icon sheet, tags might look like this:

```text
fire_skill
fire_skill2
fire_ult
```

For a champion animation, tags usually look like this:

```text
idle
run
attack
skill
skill2
ult
dead
```

When you write `action_name: "skill"` in a champion action, the game looks for an animation tag named `skill` in that champion's `#anim` data.

When you write a skill icon tag such as `fire_skill`, the UI looks for a rectangle named `fire_skill` in the icon sheet's `#data` data.

## Aseprite Files

You can put `.aseprite` files directly in a mod. The game does not decide how to use an Aseprite file from the filename. It reads the sprite user data inside the file and uses that metadata to decide which related assets to prepare.

If an Aseprite file has no recognized `sheet_type`, it is loaded as an Aseprite source file, but it will not automatically provide `#sheet`, `#anim`, or `#data` for rendering.

The important part is the Aseprite sprite user data. In Aseprite, open the sprite properties and put JSON in the user data text field.

For a normal animated champion or effect:

```json
{
  "sheet_type": "Animation",
  "layers": ["body", "weapon"],
  "anchor_x": 0.5,
  "anchor_y": 0.5
}
```

This creates:

```text
asset/example/champions/fire_mage#sheet
asset/example/champions/fire_mage#anim
```

Timeline tags in Aseprite become animation tags in `#anim`. If the Aseprite timeline has a tag named `attack`, the game can play that animation with `action_name: "attack"`.

The frame timing also comes from Aseprite. If one frame lasts longer in the Aseprite timeline, it lasts longer in game.

`layers` tells the game which Aseprite layers to combine into the animation frames. Keep the names exactly the same as the layer names in Aseprite.

`anchor_x` and `anchor_y` are normalized positions inside the original canvas. The default is the center, `0.5, 0.5`. They help keep a character visually anchored when transparent space is trimmed from each frame.

Here is the short version:

| `sheet_type` | What it is for | Assets it provides |
| --- | --- | --- |
| `Animation` | Champion sprites and animated effects | `#sheet`, `#anim` |
| `Sheet` | Still-image sheets that keep full canvas size | `#sheet`, `#data` |
| `PackedSheet` | Still-image sheets that trim empty transparent space | `#sheet`, `#data` |
| `LayeredAnimation` | Layer-aware animation setups | `#sheet`, `#data`, `#anim`, `#lanim` |

For a still-image sheet, use:

```json
{
  "sheet_type": "Sheet"
}
```

or:

```json
{
  "sheet_type": "PackedSheet"
}
```

Both create:

```text
asset/example/ui/icons#sheet
asset/example/ui/icons#data
```

`Sheet` keeps each layer/frame image at its full canvas size. `PackedSheet` trims empty transparent space around each image before placing it in the atlas.

For sheet files, the tag names are made from the Aseprite layer name and frame number:

```text
<layer_name>_<frame_number>
```

If you have a layer named `fire_skill`, frame `0` becomes:

```text
fire_skill_0
```

For layered animations:

```json
{
  "sheet_type": "LayeredAnimation",
  "layers": ["body", "weapon"]
}
```

This creates `#sheet`, `#data`, `#anim`, and `#lanim`. Use this only when you specifically need layered rendering behavior.

## PNG and JSON Sprite Sheets

Aseprite is convenient, but it is not the only way to make a sprite sheet. You can also provide a PNG atlas and JSON data yourself.

Use this naming pattern:

```text
mods/example/icons/spells#sheet.png
mods/example/icons/spells#data.sprite_sheet
```

The game loads those as:

```text
asset/example/icons/spells#sheet
asset/example/icons/spells#data
```

Then you can refer to the shared base path:

```text
asset/example/icons/spells
```

The `.sprite_sheet` file stores named rectangles inside the sheet. Rectangles are normalized UV values, where `x: 0, y: 0` is the top-left of the image and `w: 1, h: 1` is the full image size.

```json
{
  "images": {
    "fire_skill": { "x": 0.0, "y": 0.0, "w": 0.25, "h": 0.5 },
    "fire_skill2": { "x": 0.25, "y": 0.0, "w": 0.25, "h": 0.5 },
    "fire_ult": { "x": 0.5, "y": 0.0, "w": 0.25, "h": 0.5 }
  }
}
```

This is useful for skill icons, item icons, UI icons, and inline text icons.

For champion skill icons, a data champion can use the sheet like this:

```json
{
  "skill_icon": {
    "source": "asset/example/icons/spells",
    "tags": ["fire_skill", "fire_skill2", "fire_ult"]
  }
}
```

If each icon is its own PNG, use `skill_icons` instead:

```json
{
  "skill_icons": [
    "asset/example/icons/fire_skill",
    "asset/example/icons/fire_skill2",
    "asset/example/icons/fire_ult"
  ]
}
```

## PNG and JSON Animations

You can also provide an animation sheet without Aseprite by pairing a PNG with a `.fanim` file.

Use this naming pattern:

```text
mods/example/champions/fire_mage#sheet.png
mods/example/champions/fire_mage#anim.fanim
```

The `.fanim` file stores named animations. Unlike `.sprite_sheet`, these rectangles are pixel coordinates inside the sheet image.

```json
{
  "anims": {
    "idle": {
      "frames": [
        { "duration": 0.12, "data": { "x": 0, "y": 0, "w": 64, "h": 64 } },
        { "duration": 0.12, "data": { "x": 64, "y": 0, "w": 64, "h": 64 } }
      ]
    },
    "attack": {
      "frames": [
        { "duration": 0.08, "data": { "x": 0, "y": 64, "w": 64, "h": 64 } },
        { "duration": 0.08, "data": { "x": 64, "y": 64, "w": 64, "h": 64 } }
      ]
    }
  }
}
```

A data champion can use that pair as its sprite:

```json
{
  "sprite": "asset/example/champions/fire_mage",
  "anim_prefix": ""
}
```

The empty `anim_prefix` means the animation tags are copied as-is. If your animation file has `idle`, `attack`, and `skill`, the champion gets those same tags.

## Reusing One Animation File

Sometimes one Aseprite file contains several characters or variants. In that case, prefix the tags in Aseprite:

```text
eagle_idle
eagle_run
eagle_attack
eagle_skill
```

Then set `anim_prefix` in the champion data:

```json
{
  "sprite": "asset/base/aseprite_resources/champions/druid",
  "anim_prefix": "eagle_"
}
```

The champion receives these tags:

```text
idle
run
attack
skill
```

Tags without that prefix are also kept. This makes it possible to share common effects or action tags in the same source file.

## Static PNG Sprites

For a quick test, `sprite` can point directly to a PNG:

```json
{
  "sprite": "asset/example/champions/fire_mage_idle"
}
```

Do not set `anim_prefix` for this case. The game uses the whole PNG as a one-frame sprite and creates simple fallback animations for the common champion tags.

This is good for checking that a champion loads. For a finished animated champion, use an Aseprite file or a `#sheet` plus `#anim` pair.
