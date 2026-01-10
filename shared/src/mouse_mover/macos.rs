// NOTE: dont have macos, need to wait

use crate::mouse_mover::moveable_mouse::{MoveableMouseErr, MoveablePointer};
// use core_graphics::{
//     event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, ScrollEventUnit},
//     event_source::{CGEventSource, CGEventSourceStateID},
//     geometry::CGPoint,
// };

pub struct MacosMouseMover;

impl MoveablePointer for MacosMouseMover {
    fn move_to_pos(&mut self, x: i32, y: i32) -> Result<(), MoveableMouseErr> {
        todo!()
    }

    fn get_pos(&mut self) -> Result<(i32, i32), MoveableMouseErr> {
        todo!()
    }
}

impl MacosMouseMover {
    // fn move_mouse(x: i32, y: i32) -> Result<(), MoveableMouseErr> {
    //     let gc_event_source =
    //         CGEventSource::new(CGEventSourceStateID::HIDSystemState).map_err(|_| {
    //             AutoGuiError::OSFailure(
    //                 "Error creating CGEventSource on mouse movement".to_string(),
    //             )
    //         })?;
    //
    //     let event = CGEvent::new_mouse_event(
    //         gc_event_source,
    //         CGEventType::MouseMoved,
    //         CGPoint::new(x as f64, y as f64),
    //         CGMouseButton::Left,
    //     )
    //     .map_err(|_| AutoGuiError::OSFailure("Failed creating CGEvent".to_string()))?;
    //     event.post(CGEventTapLocation::HID);
    //
    //     sleep(Duration::from_millis(20));
    //     Ok(())
    // }
    //
    //   /// Gets the current mouse position.
    //   pub fn get_mouse_position() -> Result<(i32, i32), AutoGuiError> {
    //       let gc_event_source =
    //           CGEventSource::new(CGEventSourceStateID::HIDSystemState).map_err(|_| {
    //               AutoGuiError::OSFailure(
    //                   "Error creating CGEventSource on mouse movement".to_string(),
    //               )
    //           })?;
    //       let event = CGEvent::new(gc_event_source)
    //           .map_err(|_| AutoGuiError::OSFailure("Failed creating CGevent".to_string()))?;
    //       let point = event.location();
    //       Ok((point.x as i32, point.y as i32))
    //   }
}
