use sctk::reexports::client::protocol::wl_seat::WlSeat;
use sctk::reexports::client::Attached;
use sctk::reexports::protocols::unstable::text_input::v3::client::zwp_text_input_manager_v3::ZwpTextInputManagerV3;
use sctk::reexports::protocols::unstable::text_input::v3::client::zwp_text_input_v3::{
    Event as TextInputEvent, ZwpTextInputV3,
};

pub struct TextInput {
    text_input: Attached<ZwpTextInputV3>,
}

impl TextInput {
    pub fn new(seat: &Attached<WlSeat>, text_input_manager: &ZwpTextInputManagerV3) -> Self {
        let text_input = text_input_manager.get_text_input(seat);
        text_input.quick_assign(move |text_input, event, _| {
            println!("Text event: {:?}", event);
            match event {
                TextInputEvent::Enter { surface } => {
                }
                _ => {
                    // This space intentionally left blank.
                }
            }
        });

        let text_input: Attached<ZwpTextInputV3> = text_input.into();

        text_input.enable();
        text_input.set_cursor_rectangle(10, 10, 100, 100);
        text_input.commit();

        Self { text_input }
    }
}
