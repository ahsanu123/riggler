use crate::mouse_mover::moveable_mouse::{MoveableMouseErr, MoveablePointer};
use windows::Win32::Foundation::POINT;
use windows::Win32::System::Power::{ES_CONTINUOUS, ES_DISPLAY_REQUIRED, SetThreadExecutionState};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos};

pub struct WindowsMouseMover;

impl WindowsMouseMover {
    pub fn new() -> Self {
        Self
    }
}

impl MoveablePointer for WindowsMouseMover {
    fn move_to_pos(&mut self, x: i32, y: i32) -> Result<(), MoveableMouseErr> {
        let (current_x, current_y) = self.get_pos()?;
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED);
            if current_x == x && current_y == y {
                return Ok(());
            }

            let result = SetCursorPos(x, y);

            if result.is_ok() {
                return Ok(());
            }

            return Err(MoveableMouseErr::Fail(
                "win32 error, cant move cursor".into(),
            ));
        }
    }

    fn get_pos(&mut self) -> Result<(i32, i32), MoveableMouseErr> {
        unsafe {
            let mut point = POINT::default();
            let result = GetCursorPos(&mut point);

            if result.is_ok() {
                return Ok((point.x, point.y));
            }

            return Err(MoveableMouseErr::Fail(
                "win32 error, cant get mouse posisition".into(),
            ));
        }
    }
}
