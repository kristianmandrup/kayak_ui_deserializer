# Kayak UI deserializer

Deserializer for [kayak UI](https://github.com/StarArawn/kayak_ui) a [Bevy ECS](https://bevyengine.org/) UI engine.

## What is inside

Install `cargo-modules` crate and run:

`$ cargo modules generate tree --with-types`

Currently almost everything is public.

```rust
// loads kayak UI data map from a JSON string
let data: KayakData = DeJson::deserialize_json(json).unwrap();
// builds Kayak UI structs from the loaded data map.
// Stores the Kayak UI structs in HashMaps in the builder store
let builder = KayakBuilder::new(data).build();
// store contains the HashMaps with Kayak UI structs that were built
let store = builder.store;
```

## Goal

The current goal is to deserialize a JSON file into Kayak UI structures that can be stored in hashmaps. This will allow the Game UI designer to externalize parts of the UI as Game "assets" that can be loaded from one or more files.

## API

Currently this library can be used to load and build the following Kayak UI constructs:

- Kayak UI `KStyle` and Bevy `Style`

Widgets:

- `KButton`

Bundles:

- `TextWidgetBundle`
- `TextBoxBundle`
- `KButtonBundle`
- `WindowBundle`
- `ImageBundle`
- `BackgroundBundle`
- `TextureAtlasBundle`
- `ClipBundle`
- `ElementBundle`

```rust
    let data: KayakData = DeJson::deserialize_json(json).unwrap();
    let builder = KayakBuilder::new(data).build();
    let store = builder.store;

    let styles = store.styles;// HashMap<String, KStyle>
    let widgets = store.widgets; // individual widgets such as buttons

    // assets
    let assets = store.assets;
    let fonts = assets.fonts; // Vec<Handle<Font>>
    let images = assets.images; // Vec<Handle<Image>>

    // button widgets
    let buttons = widgets.buttons; // HashMap<String, KButton>

    let bundles = store.bundles;

    // bundle hashmap
    let tw_bundles = bundles.text_widget_bundles; // HashMap<String, TextWidgetBundle>
    let tb_bundles = bundles.text_box_bundles; // HashMap<String, TextBoxBundle>
    let img_bundles = bundles.image_bundles; // HashMap<String, ImageBundle>
    let button_bundles = bundles.button_bundles; // HashMap<String, KButtonBundle>
    let clip_bundles = bundles.button_bundles; // HashMap<String, ClipBundle>
    let elem_bundles = bundles.element_bundles; // HashMap<String, ElementBundle>

    // get reusable styles from store
    let base_image = store.style("base-image");
    // get individual widgets
    let menu_button = store.button("menu_button");
    // get bundles
    let title = store.text_widget_bundle("title");
    let sub_title = store.text_box_bundle("sub_title");

    let wb = store.window_bundle("main window");
    let tab = store.texture_atlas_bundle("my tab");
    let img_b = store.image_bundle("my image bundle");
    let buttons_b = store.button_bundle("my button bundle");
    let background_b = store.background_bundle("my background");
    let clip_b = store.clip_bundle("my clip");
    let elem_b = store.element_bundle("my elements");
```

## Status

Still fleshing out the internal functioning and structures. The focus so far is on parsing and building up Kayak UI structures.

The essential builders should now be working.

### Todo?

- `KayakAppBundle`
- `KayakApp`
- `RenderCommand` (partly done)

## Serialization formats

Currently targeting JSON format as a POC. It should be easy to support additional structured formats like [YAML](https://yaml.org/), [KDL](https://kdl.dev/) etc. later on.

## JSON structure

The current goal is to be able to load the following type of JSON structure into a `HashMap`.

The `HashMap` can then be referenced when building the Kayak UI to reduce the code footprint and make the UI definition more like an asset that can to a large degree be managed independently of the code, similar to CSS for HTML.

You can use `extends` to extend a `style` or `styles` property with an asset style by name as demonstrated for the `menu-button` button.

You can use `ref_id` for an `image` to reference an image asset by name as demonstrated for the `my-ref-image` image in the image bundle below.

```json
{
  "assets": {
    "images": [
      {
        "name": "profile-image",
        "path": "path/to/profile.png"
      }
    ],
    "fonts": [
      {
        "name": "roboto",
        "path": "path/to/roboto.tff"
      }
    ]
  },
  "styles": [
    {
      "name": "base",
      "color": "white",
      "background-color": "darkgray"
    },
    {
      "name": "base-image",
      "border-radius": "500",
      "position-type": "self-directed"
    }
  ],
  "widgets": {
    "buttons": [
      {
        "name": "menu-button",
        "type": "button",
        "style": {
          "extends": "base",
          "bottom": "20 px",
          "cursor": "hand"
        }
      }
    ]
  },
  "bundles": {
    "text_widget_bundles": [
      {
        "name": "game-title",
        "type": "text-widget",
        "text": {
          "extends": "base",
          "content": "hello",
          "size": 20,
          "font": "roboto"
        }
      }
    ],
    "image_bundles": [
      {
        "name": "my-ref-image",
        "type": "image-bundle",
        "image": {
          "ref_id": "profile-image",
        },
        "styles": {
          "extends": "base-image",
          "left": "10 px",
          "top": "10 px",
          "width": "200 px",
          "height": "182 px"
        }
      },
      {
        "name": "my-image",
        "type": "image-bundle",
        "image": {
          "path": "path/to/image.png",
        }
        "styles": {
          "left": "10 px",
          "top": "10 px",
          "width": "200 px",
          "height": "182 px"
        }
      }
    ]
  }
}
```

## Style

Bevy style properties.

Note that for `UiRect` and `Size` object properties such as `margin` and `size`, will be improved to support an `all` attribute and a string split, parsed similar to CSS rect props.

```json
{
  "display": "flex",
  "position_type": "relative",
  "direction": "rtl",
  "flex_direction": "row-reverse",
  "flex_wrap": "no-wrap",
  "align_items": "flex-start",
  "align_self": "baseline",
  "align_content": "space-between",
  "justify_content": "space-evenly",
  "position": {
    "top": 10,
    "left": 20,
    "right": 100,
    "bottom": 150
  },
  "margin": {
    "top": 4,
    "left": 4,
    "right": 4,
    "bottom": 4
  },
  "padding": {
    "top": 2,
    "left": 2,
    "right": 2,
    "bottom": 2
  },
  "border": {
    "top": 2,
    "left": 2,
    "right": 2,
    "bottom": 2
  },
  "flex_grow": 4,
  "flex_shrink": 4,
  "flex_basis": "2 px",
  "size": {
    "width": 10,
    "height": 10
  },
  "min_size": {
    "width": 6,
    "height": 6
  },
  "max_size": {
    "width": 20,
    "height": 20
  },
  "aspect_ratio": 4,
  "overflow": "visible"
}
```

## KStyle

`KStyle` object and properties.

```json
{
  "background_color": "blue",
  "border": 4,
  "border_color": "rgba(0.2, 0.5, 1, 1)",
  "border_radius": 2,
  "bottom": "2 %",
  "color": "hsla(0.4, 0.7, 0.8, 0.6)",
  "col_between": "4px",
  "cursor": "hand",
  "font": "roboto",
  "font_size": 12,
  "height": 100,
  "layout_type": "grid",
  "left": "10 em",
  "line_height": 14,
  "max_height": "120px",
  "max_width": "120px",
  "min_height": "120px",
  "min_width": "120px",
  "offset": {
    "left": "20px"
  },
  "padding": {
    "bottom": "10px",
    "top": "10px"
  },
  "padding_left": "20 px",
  "position_type": "self-directed",
  "right": "100px",
  "row_between": "50px",
  "top": "20px",
  "width": "20px",
  "z_index": 1
}
```
