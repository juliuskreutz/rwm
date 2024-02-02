use std::ptr::NonNull;

use pangocairo::pango;
use xcb::{x, Xid};

pub struct Draw {
    surface: cairo::XCBSurface,
    context: cairo::Context,
    layout: pango::Layout,
}

impl Draw {
    pub fn new(
        connection: &xcb::Connection,
        window: x::Window,
        width: i32,
        height: i32,
        font: &str,
    ) -> Self {
        let screen = connection.get_setup().roots().next().unwrap();

        let cairo_conn =
            cairo::XCBConnection(NonNull::new((*connection).get_raw_conn() as *mut _).unwrap());
        let cairo_window = cairo::XCBDrawable(window.resource_id());
        let visual_type = screen
            .allowed_depths()
            .flat_map(|depth| depth.visuals())
            .find(|visual| screen.root_visual() == visual.visual_id())
            .unwrap();

        #[repr(C)]
        struct XcbVisualtypeT {
            visual_id: u32,
            _class: u8,
            bits_per_rgb_value: u8,
            colormap_entries: u16,
            red_mask: u32,
            green_mask: u32,
            blue_mask: u32,
            pad0: u8,
        }

        let mut visual_ptr = XcbVisualtypeT {
            visual_id: visual_type.visual_id(),
            _class: visual_type.class() as u8,
            bits_per_rgb_value: visual_type.bits_per_rgb_value(),
            colormap_entries: visual_type.colormap_entries(),
            red_mask: visual_type.red_mask(),
            green_mask: visual_type.green_mask(),
            blue_mask: visual_type.blue_mask(),
            pad0: 0,
        };

        let visual_ptr = &mut visual_ptr as *mut _ as *mut cairo::ffi::xcb_visualtype_t;
        let cairo_visual = cairo::XCBVisualType(std::ptr::NonNull::new(visual_ptr).unwrap());
        let surface =
            cairo::XCBSurface::create(&cairo_conn, &cairo_window, &cairo_visual, width, height)
                .unwrap();
        let context = cairo::Context::new(&surface).unwrap();

        let layout = pangocairo::create_layout(&context);
        layout.set_font_description(Some(&pango::FontDescription::from_string(font)));

        Draw {
            surface,
            context,
            layout,
        }
    }

    pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64, color: u32) {
        self.context.rectangle(x, y, width, height);
        self.color(color);
        self.context.fill().unwrap();
    }

    pub fn text_width(&self, text: &str) -> i32 {
        self.layout.set_text(text);
        self.layout.size().0 / pango::SCALE
    }

    pub fn text_centered(&self, text: &str, x: f64, height: f64, color: u32) {
        self.layout.set_text(text);

        let layout_height = self.layout.size().1 / pango::SCALE;
        let y = (height - layout_height as f64) / 2.0;

        self.context.move_to(x, y);
        self.color(color);
        pangocairo::show_layout(&self.context, &self.layout);
    }

    pub fn update(&self, x: f64, y: f64, width: i32, height: i32) {
        self.surface.set_size(width, height).unwrap();
        self.context
            .set_source_surface(&self.surface, x, y)
            .unwrap();
    }

    fn color(&self, color: u32) {
        self.context.set_source_rgb(
            (color >> 16) as f64 / 255.,
            (color >> 8 & 0x0000ff) as f64 / 255.,
            (color & 0x0000ff) as f64 / 255.,
        );
    }
}
