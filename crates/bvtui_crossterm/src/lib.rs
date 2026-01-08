use std::{fmt::Debug, time::Duration};

use bevy::prelude::*;
use crossterm::event::{poll, read};

#[derive(Default, Debug)]
pub struct CrosstermPlugin;

impl Plugin for CrosstermPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<CrosstermSettings>()
            .set_runner(crossterm_runner);
    }
}

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

/// Settings for `CrosstermPlugin`.
///
/// TODO: this resource is currently placeholder, and does not affect runner.
#[derive(Resource, Clone, Debug)]
pub struct CrosstermSettings {
    /// Determines how frequently the application can update when it has focus.
    pub focused_mode: UpdateMode,
    /// Determines how frequently the application can update when it's out of focus.
    pub unfocused_mode: UpdateMode,
}

impl CrosstermSettings {
    pub fn game() -> Self {
        Self {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::reactive(Duration::from_secs_f64(1.0 / 60.0)), /* 60Hz */
        }
    }

    /// TUI game is rare, so we make `tui_app` the default rather than `game`.
    pub fn tui_app() -> Self {
        Self {
            focused_mode: UpdateMode::reactive(Duration::from_secs(5)),
            unfocused_mode: UpdateMode::reactive(Duration::from_secs(60)),
        }
    }

    pub fn continuous() -> Self {
        Self {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        }
    }

    pub fn update_mode(&self, focused: bool) -> UpdateMode {
        match focused {
            true => self.focused_mode,
            false => self.unfocused_mode,
        }
    }
}

impl Default for CrosstermSettings {
    fn default() -> Self {
        Self::tui_app()
    }
}

/// Determines how frequently an `App` should update.
#[derive(Copy, Clone, Debug)]
pub enum UpdateMode {
    /// The `App` will update over and over, as fast as it possibly can, until an `AppExit` event appears.
    Continuous,
    // TODO: doc comments
    Reactive {
        wait: Duration,
        // TODO: react_to_{device,user,window}_events
        // user event may be implemented with crossterm EventStream, futures block_on and join
    },
}

impl UpdateMode {
    pub fn reactive(wait: Duration) -> Self {
        Self::Reactive { wait }
    }

    // TODO: not sure whether this works for tui application.
    // pub fn reactive_low_power(wait: Duration) -> Self {
    //     todo!()
    // }
}
