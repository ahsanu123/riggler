use crate::mouse_mover::moveable_mouse::{MoveableMouse, MoveableMouseErr};

pub struct LinuxMouseMover;

impl MoveableMouse for LinuxMouseMover {
    fn move_to_pos(x: i32, y: i32, time: f32) -> Result<(), MoveableMouseErr> {
        todo!()
    }
}
