use bracket_lib::prelude::*;

use self::obstacle::Obstacle;

#[path = "player.rs"]
pub mod player;

#[path = "obstacle.rs"]
pub mod obstacle;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

pub struct State {
    mode: GameMode,
    player: player::Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: player::Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "Press (P) to Play Game");
        ctx.print_centered(11, "Press (Q) to Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_fall();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);

        ctx.print(0, 0, "Press Space to Flap!");
        ctx.print(0, 1, format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(self.player.x, self.player.y)
        {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, format!("Your final score: {}", self.score));
        ctx.print_centered(8, "Press (R) to Restart");
        ctx.print_centered(11, "Press (Q) to Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = player::Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
