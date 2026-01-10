use crate::mouse_mover::moveable_mouse::{MoveableMouseErr, MoveablePointer};
use x11::xlib::{_XDisplay, XFlush, XQueryPointer, XWarpPointer};

pub struct LinuxMouseMover {
    screen: *mut _XDisplay,
    root_window: u64,
}

impl LinuxMouseMover {
    pub fn new(screen: *mut _XDisplay, root_window: u64) -> Self {
        Self {
            screen,
            root_window,
        }
    }
}

impl MoveablePointer for LinuxMouseMover {
    fn move_to_pos(&mut self, x: i32, y: i32) -> Result<(), MoveableMouseErr> {
        let (current_x, current_y) = self.get_pos()?;

        // only move if position is not same
        if !(current_x == x && current_y == y) {
            unsafe {
                XWarpPointer(self.screen, 0, self.root_window, 0, 0, 0, 0, x, y);
                XFlush(self.screen);
            }
        }

        Ok(())
    }

    fn get_pos(&mut self) -> Result<(i32, i32), MoveableMouseErr> {
        // copied from
        // https://github.com/DavorMar/rustautogui/blob/main/src/core/mouse/linux/mod.rs
        unsafe {
            let mut root_return = 0;

            let mut child_return = 0;
            let mut root_x = 0;
            let mut root_y = 0;
            let mut win_x = 0;
            let mut win_y = 0;
            let mut mask_return = 0;

            let status = XQueryPointer(
                self.screen,
                self.root_window,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );

            if status == 0 {
                return Err(MoveableMouseErr::Fail(
                    "x11 error, Unable to query pointer position".to_string(),
                ));
            }

            Ok((root_x, root_y))
        }
    }
}
