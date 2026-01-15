use crate::mouse_mover::moveable_mouse::MoveablePointer;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64};
use std::thread;
use std::{sync::atomic::Ordering, time::Duration};

mod mouse_mover;

#[cfg(target_os = "linux")]
use mouse_mover::linux::LinuxMouseMover as MouseMover;

#[cfg(target_os = "windows")]
use mouse_mover::windows::WindowsMouseMover as MouseMover;

// #[cfg(target_os = "macos")]
// use mouse_mover::macos::MacosMouseMover as MouseMover;

pub static JIGGLING_ENABLE: AtomicBool = AtomicBool::new(false);
pub static JIGGLING_DELTA: AtomicI32 = AtomicI32::new(1);
pub static JIGGLING_DELAY: AtomicU64 = AtomicU64::new(1);

pub fn jiggling() {
    #[cfg(target_os = "linux")]
    {
        use x11::xlib::XInitThreads;
        thread::spawn(|| {
            unsafe { XInitThreads() };

            let mut linux_mouse = MouseMover::new();

            loop {
                let delay = JIGGLING_DELAY.load(Ordering::Relaxed);
                let delta = JIGGLING_DELTA.load(Ordering::Relaxed);

                if JIGGLING_ENABLE.load(Ordering::Relaxed) {
                    let current_pos = linux_mouse.get_pos().unwrap();
                    // TODO: add result to message queue??
                    let _ = linux_mouse.move_to_pos(current_pos.0 + delta, current_pos.1 + delta);
                }

                thread::sleep(Duration::from_secs(delay));

                if JIGGLING_ENABLE.load(Ordering::Relaxed) {
                    let current_pos = linux_mouse.get_pos().unwrap();
                    // TODO: add result to message queue??
                    let _ = linux_mouse.move_to_pos(current_pos.0 - delta, current_pos.1 - delta);
                }

                thread::sleep(Duration::from_secs(delay));
            }
        });
    }

    #[cfg(target_os = "windows")]
    {
        thread::spawn(|| {
            let mut windows_mouse_mover = MouseMover::new();

            loop {
                let delay = JIGGLING_DELAY.load(Ordering::Relaxed);
                let delta = JIGGLING_DELTA.load(Ordering::Relaxed);

                if JIGGLING_ENABLE.load(Ordering::Relaxed) {
                    let current_pos = windows_mouse_mover.get_pos().unwrap();
                    // TODO: add result to message queue??
                    let _ = windows_mouse_mover
                        .move_to_pos(current_pos.0 + delta, current_pos.1 + delta);
                }

                thread::sleep(Duration::from_secs(delay));

                if JIGGLING_ENABLE.load(Ordering::Relaxed) {
                    let current_pos = windows_mouse_mover.get_pos().unwrap();
                    // TODO: add result to message queue??
                    let _ = windows_mouse_mover
                        .move_to_pos(current_pos.0 - delta, current_pos.1 - delta);
                }

                thread::sleep(Duration::from_secs(delay));
            }
        });
    }
}
