pub enum MoveableMouseErr {
    Fail(String),
}

pub trait MoveablePointer {
    fn move_to_pos(&mut self, x: i32, y: i32) -> Result<(), MoveableMouseErr>;
    fn get_pos(&mut self) -> Result<(i32, i32), MoveableMouseErr>;
}
