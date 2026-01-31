use std::{
    sync::{OnceLock, mpsc},
    thread::{self, Thread},
    time::Duration,
};

use tray_item::{IconSource, TrayItem};

slint::include_modules!();

enum TrayCommand {
    Show,
    Hide,
    Quit,
}
fn ui() -> MainWindow {
    MainWindow::new().unwrap()
}

fn screen_as_string(screen: Screen) -> String {
    match screen {
        Screen::Jiggling => String::from("Jiggling"),
        Screen::Setting => String::from("Setting"),
        Screen::About => String::from("About"),
    }
}

fn config_as_string(config: Configuration) -> String {
    format!("delay: {}, delta: {}", config.delay, config.delta)
}

pub fn main() {
    #[cfg(target_os = "linux")]
    gtk::init().unwrap();

    let ui = ui();
    let ui_weak = ui.as_weak();
    let global_state = ui.global::<RigglerState>();

    global_state.on_onActiveScreenChanged(|screen| {
        println!("screen -> {}", screen_as_string(screen));
    });

    global_state.on_onIsJigglingChanged(|is_jiggling| {
        println!("is_jiggling -> {}", is_jiggling);
    });
    global_state.on_onConfigurationChanged(|config| {
        println!("config -> {}", config_as_string(config));
    });

    let ui_weak_to_tray = ui_weak.clone();
    global_state.on_minimizeToTray(move || {
        ui_weak_to_tray.unwrap().window().hide().unwrap();
    });

    let mut tray = TrayItem::new("Riggler Tray", IconSource::Resource("tray-default")).unwrap();

    let ui_weak_show = ui_weak.clone();
    tray.add_menu_item("Show", move || {
        ui_weak_show.unwrap().window().show().unwrap();
    })
    .unwrap();

    let ui_weak_hide = ui_weak.clone();
    tray.add_menu_item("Hide", move || {
        ui_weak_hide.unwrap().window().hide().unwrap();
    })
    .unwrap();

    let ui_weak_quit = ui_weak.clone();
    tray.add_menu_item("Quit", move || {
        ui_weak_quit.unwrap().window().hide().unwrap();
        slint::quit_event_loop().unwrap();
    })
    .unwrap();

    ui.window().show().unwrap();
    slint::run_event_loop_until_quit().unwrap();
}
