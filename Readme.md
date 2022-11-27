# Kayak UI deserializer

Deserializer (loader) for [Kayak UI](https://github.com/StarArawn/kayak_ui) the premier UI engine for the [Bevy ECS](https://bevyengine.org/) game engine.

Currently supports:

- [RON](https://docs.rs/ron/latest/ron/) Rust Object Notation
- [JSON](https://www.json.org/json-en.html) JavaScript Object Notation

This project uses `nanoserde`, `ron` for deserialization.

<!-- vscode-markdown-toc -->

- [Kayak UI deserializer](#kayak-ui-deserializer)
  - [Project Goals](#project-goals)
  - [Serialization formats](#serialization-formats)
  - [Usage](#usage)
  - [API](#api)
  - [JSON structure](#json-structure)
  - [Ron structure (WIP)](#ron-structure-wip)
  - [Roadmap](#roadmap)
  - [Style example](#style-example)
  - [KStyle example](#kstyle-example)

<!-- vscode-markdown-toc-config
	numbering=false
	autoSave=true
	/vscode-markdown-toc-config -->
<!-- /vscode-markdown-toc -->

## <a name='Projectgoals'></a>Project Goals

The current goal of this project is to enable deserialization of a JSON file into Kayak UI components that are stored and can be referenced.

This will allow a Bev UI designer to externalize most of the UI definitions as Game assets that can be loaded from one or more UI asset files.

## <a name='Serializationformats'></a>Serialization formats

This deserializer currently targets the JSON format.
Support will likely be added for other structured formats like:

- [RON](https://docs.rs/ron/latest/ron/) (WIP)
- [YAML](https://yaml.org/)
- [KDL](https://kdl.dev/)

## <a name='Usage'></a>Usage

Use `nanoserde` to deserialize JSON into a `KayakUiData` object that is then fed into the `KayakBuilder`. The `store` on the `builder` will contain the built and registered UI components.

```rust
use kayak_ui_deserializer::*;

// loads kayak UI data map from a JSON string
let data: KayakUiData = DeJson::deserialize_json(json).unwrap();
// builds Kayak UI structs from the loaded data map.
// Stores the Kayak UI structs in HashMaps in the builder store
let builder = KayakBuilder::new(data).build();
// store contains the HashMaps with Kayak UI structs that were built
let store = builder.store;
```

## <a name='API'></a>API

Currently this library can be used to load and build the following Kayak UI constructs:

```rust
    let data: KayakData = DeJson::deserialize_json(json).unwrap();
    let builder = KayakBuilder::new(data).build();
    let store = builder.store;

    // kayak and bevy styles
    let kstyles = store.kstyles;// HashMap<String, KStyle>
    let styles = store.styles;// HashMap<String, Style>

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
    let np_bundles = bundles.nine_patch_bundles; // HashMap<String, ElementBundle>

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
    let npb = store.nine_patch_bundle("my np image");
```

## <a name='JSONstructure'></a>JSON structure

The builder populates `HashMap`s in the builder `store` that can be referenced when building a Kayak UI. This should greatly reduce the code footprint and make the UI definition more declarative and allow the UI to be managed independently of the code, similar to CSS for HTML.

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

## Ron structure (WIP)

Ron (Rust Object Notation) should translate to the following Kayak UI string format:

```rust
KayakUiData( // class name is optional
    bundles: ( // this is a map
        "text_widget_bundles": (
            name: "game-title",
            text: (
              extends: "base",
              content: "hello",
              size: 20,
              font: "roboto"
            )
        ),
    ),
)
```

Loading a `ron` string into a Kayak UI store

```rust
fn load_ron() {
    let str = ron_ui_data();
    let data: KayakUiData = DeRon::deserialize_ron(str).unwrap();
    let source_io = FileAssetIo::new("path", false);
    let asset_server = AssetServer::new(source_io);
    let builder = KayakBuilder::new(asset_server, data).build();
}
```

## Roadmap

Currently most of the serialization structs use `OptStr` which is a type alias for `Option<String>`. This will shortly be changed to proper primitive types like `f32`.

The project further needs to be partitioned into proper module folders and a good, clean file structure.

## <a name='Styleexample'></a>Style example

Bevy style properties.

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
  "position": "10 20 100 150",
  "margin_obj": {
    "top": 4,
    "left": 4,
    "right": 4,
    "bottom": 4
  },
  "padding": "2",
  "border": "2",
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

## <a name='KStyleexample'></a>KStyle example

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
  "offset-obj": {
    "left": "20px"
  },
  "padding-obj": {
    "bottom": "10px",
    "top": "10px"
  },
  "padding_left": "20px",
  "position_type": "self-directed",
  "right": "100px",
  "row_between": "50px",
  "top": "20px",
  "width": "20px",
  "z_index": 1
}
```
