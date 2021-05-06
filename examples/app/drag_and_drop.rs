use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins_with(DefaultPlugins, |group| {
            group.disable::<bevy::audio::AudioPlugin>()
        })
        .add_system(file_drag_and_drop_system.system())
        .run();
}

fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
    for event in events.iter() {
        info!("{:?}", event);
    }
}
