use bevy::{
    app::{App, Update},
    ecs::system::Local,
};
use bvtui::BvtuiPlugins;

fn main() {
    App::new()
        .add_plugins(BvtuiPlugins)
        .add_systems(Update, print_when_update)
        .run();
}

fn print_when_update(mut inc: Local<u32>) {
    println!("inc = {}", *inc);
    *inc += 1;
}
