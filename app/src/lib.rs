#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let ui = ui();
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

    ui.run().unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    slint::android::init(android_app).unwrap();
    let ui = ui();
    MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    ui.run().unwrap();
}
