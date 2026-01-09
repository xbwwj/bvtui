use std::time::Duration;

use bevy::{
    app::{App, Update},
    ecs::system::Local,
};
use bvtui::BvtuiPlugins;
use bvtui_crossterm::{CrosstermSettings, UpdateMode};

fn main() {
    // update each two second
    let pt2s = Duration::from_secs(2);
    App::new()
        .add_plugins(BvtuiPlugins)
        .insert_resource(CrosstermSettings {
            focused_mode: UpdateMode::reactive(pt2s),
            unfocused_mode: UpdateMode::reactive(pt2s),
        })
        .add_systems(Update, print_when_update)
        .run();
}

fn print_when_update(mut inc: Local<u32>) {
    println!("inc = {}", *inc);
    *inc += 1;
}
