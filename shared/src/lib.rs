use crate::mouse_mover::moveable_mouse::{MoveableMouseErr, MoveablePointer};

mod mouse_mover;

#[cfg(target_os = "linux")]
use mouse_mover::linux::LinuxMouseMover as MouseMover;

#[cfg(target_os = "windows")]
use mouse_mover::windows::WindowsMouseMover as MouseMover;

// #[cfg(target_os = "macos")]
// use mouse_mover::macos::MacosMouseMover as MouseMover;

// pub struct Mover<T = MouseMover>
// where
//     T: MoveablePointer,
// {
//     pub mouse: T,
// }
// static MOUSE_MOVER_INSTANCE: OnceLock<Mover> = OnceLock::new();

// pub fn get_mouse_mover_instance
