# Kayak UI deserializer

Deserializer for [kayak UI](https://github.com/StarArawn/kayak_ui), a Bevy ECS UI engine.

## Goal

The current goal is to deserialize a JSON file into Kayak UI structures that can be stored in a Hashmap for reference. This allows the Game UI designer to externalize some UI decisions as Game "assets" that can be loaded from file.

## Status

Still fleshing out the internal functioning and structures. The focus so far is on parsing and building up Kayak UI structs.

The essential builders should now be working.

Currently missing:

- link between loading the JSON into memory structures and calling the builders
- `HashMap` registration

## Serialization formats

Currently targeting JSON format as a POC. It should be easy to support additional structued formats like [YAML](https://yaml.org/), [KDL](https://kdl.dev/) etc. later on.

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
    ],
    "text-widgets": [
      {
        "name": "game-title",
        "type": "text-widget",
        "text": {
          "extends": "base",
          "content": "hello",
          "size": 20,
          "font-ref": "roboto"
        }
      }
    ],
    "image-bundles": [
      {
        "name": "my-image",
        "type": "image-bundle",
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
