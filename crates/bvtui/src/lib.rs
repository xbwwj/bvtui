use bevy::app::plugin_group;

plugin_group! {
    /// This plugin group will add all the default plugins for a Bevy TUI application.
    #[derive(Debug)]
    pub struct BvtuiPlugins {
        bvtui_crossterm:::CrosstermPlugin,
    }
}
