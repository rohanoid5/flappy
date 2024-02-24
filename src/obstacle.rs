use bracket_lib::prelude::*;

use super::SCREEN_HEIGHT;

pub struct Obstacle {
    pub x: i32,
    pub gap_y: i32,
    pub size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        Obstacle {
            x,
            gap_y: RandomNumberGenerator::new().range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    pub fn hit_obstacle(&mut self, player_x: i32, player_y: i32) -> bool {
        let half_size = self.size / 2;
        let does_match_x = self.x == player_x;
        let above_gap = player_y < self.gap_y - half_size;
        let below_gap = player_y > self.gap_y + half_size;

        does_match_x && (above_gap || below_gap)
    }
}
