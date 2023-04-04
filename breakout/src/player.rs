use macroquad::prelude::*;

const PLAYERSIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYERSPEED: f32 = 500f32;

pub struct Player {
    m_player_rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            m_player_rect: Rect::new(
                screen_width() * 0.5f32 - PLAYERSIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYERSIZE.x,
                PLAYERSIZE.y,
            ),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let player_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.m_player_rect.x += player_move * delta_time * PLAYERSPEED;

        if self.m_player_rect.x < 0f32 {
            self.m_player_rect.x = 0f32;
        }

        if self.m_player_rect.x > screen_width() - self.m_player_rect.w {
            self.m_player_rect.x = screen_width() - self.m_player_rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.m_player_rect.x,
            self.m_player_rect.y,
            self.m_player_rect.w,
            self.m_player_rect.h,
            RED,
        )
    }
}
