use macroquad::prelude::*;

mod blocks;
mod player;
#[macroquad::main("breakout")]
async fn main() {
    let mut player: player::Player = player::Player::new();
    let mut blocks = Vec::new();

    let padding = 5f32;
    let total_block_size = blocks::BLOCK_SIZE + vec2(padding, padding);

    let (width, height) = (6, 6);
    let board_start_pos = vec2(
        screen_width() - (total_block_size.x * width as f32),
        50f32,
    );

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;

        blocks.push(blocks::Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    loop {
        player.update(get_frame_time());
        clear_background(WHITE);
        player.draw();

        for block in blocks.iter() {
            block.draw();
        }

        next_frame().await;
    }
}
