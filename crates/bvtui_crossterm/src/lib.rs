use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct CrosstermPlugin;

impl Plugin for CrosstermPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.set_runner(crossterm_runner);
    }
}

// TODO: implementation
fn crossterm_runner(mut app: App) -> AppExit {
    loop {
        println!("In main loop");
        app.update();
        if let Some(exit) = app.should_exit() {
            return exit;
        }
    }
}
