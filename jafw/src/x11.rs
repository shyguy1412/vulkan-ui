use std::{num::NonZero, sync::Arc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use x11rb::{connection::Connection, protocol::xproto::ConnectionExt};

use crate::{Error::*, Result, XError::*};

pub struct Window {
    connection: Arc<x11rb::xcb_ffi::XCBConnection>,
    window: NonZero<u32>,
}

impl HasWindowHandle for Window {
    fn window_handle(
        &self,
    ) -> std::prelude::v1::Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError>
    {
        let window_handle = raw_window_handle::XcbWindowHandle::new(self.window);

        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(window_handle.into()) })
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(
        &self,
    ) -> std::prelude::v1::Result<
        raw_window_handle::DisplayHandle<'_>,
        raw_window_handle::HandleError,
    > {
        let connection = std::ptr::NonNull::new(self.connection.get_raw_xcb_connection());
        let handle = raw_window_handle::XcbDisplayHandle::new(connection, 0);

        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(handle.into())) }
    }
}

impl super::Window for Window {
    fn new() -> Result<Self> {
        let (connection, screen_num) = x11rb::xcb_ffi::XCBConnection::connect(None)
            .map_err(XConnectError)
            .map_err(XError)?;
        let connection = Arc::new(connection);

        let screen = connection
            .setup()
            .roots
            .get(screen_num as usize)
            .ok_or(OtherError)?;

        let window = connection
            .generate_id()
            .map_err(XReplyOrIdError)
            .map_err(XError)?;
        let window = NonZero::new(window).ok_or(OtherError)?;
        connection
            .create_window(
                x11rb::COPY_DEPTH_FROM_PARENT as u8,
                window.get(),
                screen.root,
                0,
                0,
                150,
                150,
                0,
                x11rb::protocol::xproto::WindowClass::INPUT_OUTPUT,
                screen.root_visual,
                &x11rb::protocol::xproto::CreateWindowAux::new()
                    .event_mask(Some((u32::MAX >> 7).into())),
            )
            .map_err(XConnectionError)
            .map_err(XError)?;

        connection
            .map_window(window.into())
            .map_err(XConnectionError)
            .map_err(XError)?;
        connection
            .flush()
            .map_err(XConnectionError)
            .map_err(XError)?;

        Ok(Self { connection, window })
    }

    fn title(&self, title: &dyn std::fmt::Display) -> Result<()> {
        let title = title.to_string();
        let data = title.as_bytes();
        self.connection
            .change_property(
                x11rb::protocol::xproto::PropMode::REPLACE,
                self.window.into(),
                x11rb::protocol::xproto::AtomEnum::WM_NAME,
                x11rb::protocol::xproto::AtomEnum::STRING,
                8,
                data.len() as u32,
                data,
            )
            .map_err(XConnectionError)
            .map_err(XError)?;

        self.connection
            .flush()
            .map_err(XConnectionError)
            .map_err(XError)?;

        Ok(())
    }

    fn next_event(&self) -> Option<crate::Event> {
        self.connection
            .wait_for_event()
            .map(crate::Event::XEvent)
            .ok()
    }
}
