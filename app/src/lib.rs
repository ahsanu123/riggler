use dirs::config_dir;
use riggler_shared::{JIGGLING_DELAY, JIGGLING_DELTA, JIGGLING_ENABLE, jiggling};
use std::{fs::OpenOptions, sync::atomic::Ordering};
use tray_item::{IconSource, TrayItem};

slint::include_modules!();

fn init_configuration() -> Configuration {
    let config_path = config_dir().unwrap().join(".riggler");
    let config_file = OpenOptions::new()
        .read(true)
        .open(config_path.clone())
        .unwrap();

    let config = match serde_json::from_reader::<_, Configuration>(config_file) {
        Ok(config) => config,
        Err(_) => {
            let config_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(config_path.clone())
                .unwrap();

            let config = Configuration {
                delay: 1.0,
                delta: 2.0,
            };

            serde_json::to_writer_pretty::<_, Configuration>(config_file, &config).unwrap();
            config
        }
    };

    JIGGLING_DELAY.store(config.delay.round() as i32, Ordering::Relaxed);
    JIGGLING_DELTA.store(config.delta.round() as i32, Ordering::Relaxed);

    config
}

fn set_config(config: Configuration) {
    let config_path = config_dir().unwrap().join(".riggler");
    let config_file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(config_path.clone())
        .unwrap();
    serde_json::to_writer_pretty::<_, Configuration>(config_file, &config).unwrap();
}

pub fn main() {
    #[cfg(target_os = "linux")]
    gtk::init().unwrap();

    let initial_config = init_configuration();

    let ui = MainWindow::new().unwrap();

    let ui_weak = ui.as_weak();
    let global_state = ui.global::<RigglerState>();

    global_state.set_config(initial_config);

    global_state.on_onActiveScreenChanged(|screen| {
        #[cfg(feature = "debug")]
        println!("screen -> {:?}", screen);
    });

    global_state.on_onIsJigglingChanged(|is_jiggling| {
        #[cfg(feature = "debug")]
        println!("is_jiggling -> {}", is_jiggling);
        JIGGLING_ENABLE.store(is_jiggling, Ordering::Relaxed);
    });
    global_state.on_onConfigurationChanged(|config| {
        #[cfg(feature = "debug")]
        println!("config -> {:?}", config);

        JIGGLING_DELAY.store(config.delay.round() as i32, Ordering::Relaxed);
        JIGGLING_DELTA.store(config.delta.round() as i32, Ordering::Relaxed);

        set_config(config);
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

    jiggling();

    ui.window().show().unwrap();

    slint::run_event_loop_until_quit().unwrap();
}
