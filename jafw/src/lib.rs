#[cfg(target_os = "linux")]
#[cfg(feature = "wayland")]
#[path = "wayland.rs"]
mod window;

#[cfg(target_os = "linux")]
#[cfg(not(feature = "wayland"))]
#[path = "x11.rs"]
mod window;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod window;

use std::sync::Arc;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

#[derive(Debug)]
pub enum XError {
    XConnectError(x11rb::errors::ConnectError),
    XConnectionError(x11rb::errors::ConnectionError),
    XReplyOrIdError(x11rb::errors::ReplyOrIdError),
}

#[derive(Debug)]
pub enum Error {
    XError(XError),
    OtherError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Event {
    XEvent(x11rb::protocol::Event),
}

pub trait Window: HasWindowHandle + HasDisplayHandle + Send + Sync + Sized {
    fn new() -> Result<Self>;
    fn title(&self, title: &dyn std::fmt::Display) -> Result<()>;
    fn next_event(&self) -> Option<Event>;
}

pub fn open_window() -> Result<Arc<impl Window>> {
    window::Window::new().map(Arc::new)
}
