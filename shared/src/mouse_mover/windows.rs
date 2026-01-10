use crate::mouse_mover::moveable_mouse::{MoveableMouse, MoveableMouseErr};

pub struct WindowsMouseMover;

impl MoveableMouse for WindowsMouseMover {
    fn move_to_pos(x: i32, y: i32, time: f32) -> Result<(), MoveableMouseErr> {
        todo!()
    }
}
