use macroquad::prelude::*;
use crate::types::Page;

pub struct MenuState {
    background_color: Color,
    button_color: Color,
    button_icon: Texture2D
}

impl MenuState {
    pub async fn new() -> MenuState {
        MenuState {
            background_color: Color::from_rgba(48, 52, 70, 255),

            button_color: Color::from_rgba(98, 104, 128, 255),
            button_icon: load_texture("res/wking.png").await.unwrap()
        }
    }

    fn draw_button(&self, x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
        draw_rectangle(x - w / 2., y, w, h, self.button_color);
        draw_texture_ex(self.button_icon, x - w / 2., y, WHITE, DrawTextureParams { dest_size: Some(vec2(h, h)), ..Default::default()});
        draw_text(text, x - w / 2. + h, y + h / 2. + h / 8., h / 2., WHITE);
        if mouse_position().0 > x - w/2. && mouse_position().0 < x - w / 2. + w && mouse_position().1 > y && mouse_position().1 < y + h {
            draw_rectangle(x - w / 2., y, w, h, Color::from_rgba(255, 255, 255, 20));
            if is_mouse_button_down(MouseButton::Left) {
                draw_rectangle(x - w / 2., y, w, h, Color::from_rgba(255, 255, 255, 50));
            }
            return true
        }
        false
    }
}

pub fn menu_loop(ms: &MenuState, mut page: &Page) {
    clear_background(ms.background_color);
    page = &Page::GameNormal;

    if ms.draw_button(screen_width() / 2., screen_height() / 2. - 110., 400., 100., "Normal") {
        page = &Page::GameNormal;
    }
    ms.draw_button(screen_width() / 2., screen_height() / 2. + 10., 400., 100., "Daily Board"); 
}
