use macroquad::prelude::*;

const PLAYERSIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYERSPEED: f32 = 500f32;

struct Player {
    m_playerRect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            m_playerRect: Rect::new(
                screen_width() * 0.5f32 - PLAYERSIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYERSIZE.x,
                PLAYERSIZE.y,
            ),
        }
    }

    pub fn update(&mut self, deltaTime: f32) {
        let playerMove = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.m_playerRect.x += playerMove * deltaTime * PLAYERSPEED;

        if self.m_playerRect.x < 0f32 {
            self.m_playerRect.x = 0f32;
        }

        if self.m_playerRect.x > screen_width() - self.m_playerRect.w {
            self.m_playerRect.x = screen_width() - self.m_playerRect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.m_playerRect.x,
            self.m_playerRect.y,
            self.m_playerRect.w,
            self.m_playerRect.h,
            RED,
        )
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player: Player = Player::new();

    loop {
        player.update(get_frame_time());
        clear_background(WHITE);
        player.draw();
        next_frame().await;
    }
}
