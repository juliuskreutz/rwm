use xcb::x;
use xcb_util_cursor as cursor;

pub struct Cursors {
    left_ptr: x::Cursor,
    bottom_right_corner: x::Cursor,
    fleur: x::Cursor,
}

impl Cursors {
    pub fn new(connection: &xcb::Connection, screen: &x::Screen) -> Self {
        let context = cursor::CursorContext::new(connection, screen).unwrap();

        let left_ptr = context.load_cursor(cursor::Cursor::LeftPtr);
        let bottom_right_corner = context.load_cursor(cursor::Cursor::BottomRightCorner);
        let fleur = context.load_cursor(cursor::Cursor::Fleur);

        Cursors {
            left_ptr,
            bottom_right_corner,
            fleur,
        }
    }

    pub fn left_ptr(&self) -> x::Cursor {
        self.left_ptr
    }

    pub fn sizing(&self) -> x::Cursor {
        self.bottom_right_corner
    }

    pub fn fleur(&self) -> x::Cursor {
        self.fleur
    }
}
