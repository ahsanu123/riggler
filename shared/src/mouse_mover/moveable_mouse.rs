pub enum MoveableMouseErr {
    Fail(String),
}

pub trait MoveableMouse {
    fn move_to_pos(x: i32, y: i32, time: f32) -> Result<(), MoveableMouseErr>;
}
