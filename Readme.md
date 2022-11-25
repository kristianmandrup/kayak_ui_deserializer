# Kayak UI deserializer

Deserializer for [kayak UI](https://github.com/StarArawn/kayak_ui), a Bevy ECS UI engine.

## What is inside

## Goal

The current goal is to deserialize a JSON file into Kayak UI structures that can be stored in a Hashmap for reference. This allows the Game UI designer to externalize some UI decisions as Game "assets" that can be loaded from file.

## API

Currently this library can be used to load and build the following Kayak UI constructs:

- `KStyle`
- `KButton`
- `TextWidgetBundle`
- `WindowBundle`
- `TextureAtlasBundle`

```rust
    let data: KayakData = DeJson::deserialize_json(json).unwrap();
    let mut builder = KayakBuilder::new(data).build();
    let store = builder.store;
    // accessing hashmaps
    let styles = store.styles;// HashMap<String, KStyle>
    let widgets = store.widgets;
    let buttons = widgets.buttons; // HashMap<String, KButton>
    let tw_bundles = widgets.text_widget_bundles; // HashMap<String, TextWidgetBundle>

// get widgets and styles from store
    let base_image = store.style("base-image");
    let menu_button = store.button("menu_button");
    let title = store.text_widget_bundle("title");
    let sub_title = store.text_widget_bundle("sub_title");
    let wb = store.window_bundle("main window");
    let tab = store.texture_atlas_bundle("my tab");
```

## Status

Still fleshing out the internal functioning and structures. The focus so far is on parsing and building up Kayak UI structures.

The essential builders should now be working.

Currently missing:

- link between loading the JSON into memory structures and calling the builders
- `HashMap` registration

## Serialization formats

Currently targeting JSON format as a POC. It should be easy to support additional structured formats like [YAML](https://yaml.org/), [KDL](https://kdl.dev/) etc. later on.

## JSON structure

The current goal is to be able to load the following type of JSON structure into a `HashMap`.

The HashMap can then be referenced when building the Kayak UI to reduce the code footprint and make the UI definition more like an asset that can to a large degree be managed independently of the code, similar to CSS for HTML.

```json
{
  "assets": {
    "images": [
      {
        "name": "profile-image",
        "type": "image",
        "path": "path/to/profile.png"
      }
    ],
    "fonts": [
      {
        "name": "roboto",
        "type": "font",
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
    "texture_atlas_bundles": [{
      "name": "my-atlas",
      "atlas": {
        "handle": {
          "path": "path/to/atlas"
        },
        "position": [20, 40],
        "tile_size": [16, 16]
      },
      "styles": {
          "extends": "base",
          "bottom": "20 px"
      }
    }],
    "window_bundles": [
      {
        "name": "main window",
        "window": {
          "draggable": true,
          "initial_position": [50, 100],
          "size": [40, 40]
          "title": "Game menu"
        },
        "styles": "base"
      }
    ],
    "buttons": [
      {
        "name": "menu-button",
        "style": {
          "extends": "base",
          "bottom": "20 px",
          "cursor": "hand"
        }
      }
    ],
    "text_widget_bundles": [
      {
        "name": "game-title",
        "text": {
          "extends": "base",
          "content": "hello",
          "size": 20,
          "font-ref": "roboto"
        }
      }
    ],
    "image_bundles": [
      {
        "name": "my-image",
        "image-ref": "profile-image",
        "styles": {
          "extends": "base-image",
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
