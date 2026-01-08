use std::time::Duration;

use bevy::prelude::*;
use crossterm::event::{poll, read};

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
        app.update();

        if let Some(exit) = app.should_exit() {
            return exit;
        }

        // TODO: wait time should be configurable, like winit
        let available = poll(Duration::from_secs_f32(1. / 30.)).expect("fail to poll io event");
        if available {
            let _event = read().expect("fail to read io event");
            // TODO: what to do with event
        }
    }
}
