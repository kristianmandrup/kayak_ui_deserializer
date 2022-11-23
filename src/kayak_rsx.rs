use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.spawn(UICameraBundle::new());

    let mut widget_context = KayakRootContext::new();
    let parent_id = None;

    // The rsx! macro expects a parent_id, a widget_context from the user.
    // It also expects `Commands` from bevy.
    // This can be a little weird at first. 
    // See the rsx! docs for more info!
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    }
    
    commands.spawn(UICameraBundle::new(widget_context));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}