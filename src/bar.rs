use xcb::x;

use crate::{config, draw::Draw};

pub struct Bar {
    window: x::Window,
    width: u16,
    draw: Draw,
}

impl Bar {
    pub fn new(connection: &xcb::Connection, x: i16, y: i16, width: u16) -> Self {
        let screen = connection.get_setup().roots().next().unwrap();

        let window = connection.generate_id();
        connection.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: window,
            parent: screen.root(),
            x,
            y,
            width,
            height: config::BAR_HEIGHT,
            border_width: 0,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            value_list: &[],
        });
        connection.send_request(&x::MapWindow { window });

        let draw = Draw::new(
            connection,
            window,
            width as i32,
            config::BAR_HEIGHT as i32,
            config::FONT,
        );

        Bar {
            window,
            width,
            draw,
        }
    }

    pub fn init(&self) {
        self.draw(0, Vec::new(), "", "");
    }

    pub fn draw(&self, selected: usize, full_tags: Vec<usize>, name: &str, status: &str) {
        let mut x = 0;

        for (i, tag) in config::TAGS.iter().enumerate() {
            let box_color;
            let text_color;

            if i == selected {
                box_color = config::BAR_HL_COLOR;
                text_color = config::BAR_TEXT_HL_COLOR;
            } else {
                box_color = config::BAR_COLOR;
                text_color = config::BAR_TEXT_COLOR;
            }

            let tag_width = self.draw.text_width(tag) as u16 + 2 * config::BAR_TEXT_PADDING;

            self.draw.rectangle(
                x as f64,
                0.0,
                tag_width as f64,
                config::BAR_HEIGHT as f64,
                box_color,
            );
            self.draw.text_centered(
                tag,
                (x + config::BAR_TEXT_PADDING) as f64,
                config::BAR_HEIGHT as f64,
                text_color,
            );

            if full_tags.contains(&i) {
                let margin = config::BAR_HEIGHT as f64 / 16.;
                let size = config::BAR_HEIGHT as f64 / 4.;

                let x = x as f64 + margin;

                self.draw.rectangle(x, margin, size, size, text_color);
            }

            x += tag_width;
        }

        let status_width = self.draw.text_width(status) as u16 + 2 * config::BAR_TEXT_PADDING;
        let name_width = self.width - x - status_width;

        self.draw.rectangle(
            x as f64,
            0.0,
            name_width as f64,
            config::BAR_HEIGHT as f64,
            config::BAR_HL_COLOR,
        );
        self.draw.text_centered(
            name,
            (x + config::BAR_TEXT_PADDING) as f64,
            config::BAR_HEIGHT as f64,
            config::BAR_TEXT_HL_COLOR,
        );

        x += name_width;

        self.draw.rectangle(
            x as f64,
            0.0,
            status_width as f64,
            config::BAR_HEIGHT as f64,
            config::BAR_COLOR,
        );
        self.draw.text_centered(
            status,
            (x + config::BAR_TEXT_PADDING) as f64,
            config::BAR_HEIGHT as f64,
            config::BAR_TEXT_COLOR,
        );
    }

    pub fn update(&mut self, connection: &xcb::Connection, x: i16, y: i16, width: u16) {
        self.width = width;

        connection.send_request(&x::ConfigureWindow {
            window: self.window,
            value_list: &[
                x::ConfigWindow::X(x as i32),
                x::ConfigWindow::Y(y as i32),
                x::ConfigWindow::Width(width as u32),
            ],
        });

        self.draw
            .update(x as f64, y as f64, width as i32, config::BAR_HEIGHT as i32);
    }

    pub fn clean(self, connection: &xcb::Connection) {
        connection.send_request(&x::UnmapWindow {
            window: self.window,
        });
    }
}
