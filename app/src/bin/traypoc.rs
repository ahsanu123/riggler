use tray_item::{IconSource, TrayItem};

fn main() {
    #[cfg(target_os = "linux")]
    gtk::init().unwrap();
    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("riggler-icon")).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    })
    .unwrap();

    tray.add_menu_item("Quit", || {
        #[cfg(target_os = "linux")]
        gtk::main_quit();
    })
    .unwrap();

    #[cfg(target_os = "linux")]
    gtk::main();
}
