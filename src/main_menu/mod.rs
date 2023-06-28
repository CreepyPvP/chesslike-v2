use bevy::prelude::Plugin;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_main_menu);
    }
}

fn setup_main_menu() {
    println!("main menu now active");
}
