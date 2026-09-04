/*
 * Copyright (C) 2026 Lix *

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */


use macroquad::prelude::*;


// Button
struct Button {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    label: String,
    is_hovered: bool,
    color: Color
}

impl Button {
    fn new(position_x: f32, position_y: f32, width: f32, height: f32, label: String, color: Color) -> Self {
        Button {
            position_x,
            position_y,
            width,
            height,
            label,
            is_hovered: false,
            color
        }
    }

    fn draw(&mut self) {
        let (mx, my) = mouse_position();
        let button: Rect = Rect::new(self.position_x, self.position_y, self.width, self.height);
        draw_rectangle(self.position_x, self.position_y, self.width, self.height, self.color);
        
        let font_size = 20;
        let label_data = measure_text(&self.label, None, font_size, 1.0);
        draw_text(&self.label, self.position_x + (self.width - label_data.width) / 2.0, self.position_y + (self.height + label_data.height) / 2.0, font_size as f32, BLACK);

        self.is_hovered = button.contains(vec2(mx, my));
    }

    fn pressed(&self) -> bool {
        if self.is_hovered {
            return is_mouse_button_pressed(MouseButton::Left);
        }
        false
    }
}


#[macroquad::main("test")]
async fn main() {
    let mut button1 = Button::new(100.0, 100.0, 100.0, 50.0, "Button1".to_string(), RED);

    loop {
        clear_background(BLACK);

        button1.draw();
        
        if button1.is_hovered {
            button1.color = Color::from_rgba(100, 0, 0, 255)
        } else {
            button1.color = RED;
        }

        button1.draw();

        if button1.pressed() {
            println!("button pressed");
        }

        next_frame().await;
    }
}