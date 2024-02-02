use xcb::x;

pub struct Cursors {
    left_ptr: x::Cursor,
    sizing: x::Cursor,
    fleur: x::Cursor,
}

impl Cursors {
    pub fn new(connection: &xcb::Connection, screen: &x::Screen) -> Self {
        let context = xcb_util_cursor::CursorContext::new(connection, screen).unwrap();

        let left_ptr = context.load_cursor(xcb_util_cursor::Cursor::LeftPtr);
        let sizing = context.load_cursor(xcb_util_cursor::Cursor::Sizing);
        let fleur = context.load_cursor(xcb_util_cursor::Cursor::Fleur);

        Cursors {
            left_ptr,
            sizing,
            fleur,
        }
    }

    pub fn left_ptr(&self) -> x::Cursor {
        self.left_ptr
    }

    pub fn sizing(&self) -> x::Cursor {
        self.sizing
    }

    pub fn fleur(&self) -> x::Cursor {
        self.fleur
    }
}
