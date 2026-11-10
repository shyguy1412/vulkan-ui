compile_error!("Wayland is not yet supported");

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct Window {}

impl HasWindowHandle for Window {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        todo!()
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        todo!()
    }
}

impl super::Window for Window {
    fn new() -> crate::Result<Self> {
        todo!()
    }

    fn title(&self, title: &dyn std::fmt::Display) -> crate::Result<()> {
        todo!()
    }

    fn next_event(&self) -> Option<crate::Event> {
        todo!()
    }
}
