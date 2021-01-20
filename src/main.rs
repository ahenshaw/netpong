mod ball;
mod player;
mod netpong;

use ball::Ball;
use player::Player;

use std::time::Duration;

use ggez::input::mouse;
use ggez::{
    event, 
    graphics, 
    conf,
    Context, 
    GameResult,
    ContextBuilder
};

use ggez::mint as na;

type Position = na::Point2<f32>;
type Velocity = na::Vector2<f32>;

const SCREEN_WIDTH:  f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct MainState {
    p1:   Player,
    p2:   Player,
    ball: Ball,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {

        MainState {
            p1: Player::new(true),
            p2: Player::new(false),
            ball: Ball::new(ctx),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        ggez::timer::sleep(Duration::from_secs_f32((0.016666 - dt).max(0.0)));

        if dt < 0.1 {
            self.p1.is_active =  self.ball.is_going_left();
            self.p2.is_active = !self.p1.is_active;

            self.p1.update(ctx);
            self.p2.update(ctx);

            let (s1, s2) = self.ball.update(dt, ctx);
            self.p1.score += s1;
            self.p2.score += s2;

            self.p1.check_for_hit(&mut self.ball, ctx);
            self.p2.check_for_hit(&mut self.ball, ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let mid_line = graphics::Mesh::new_line(
            ctx, 
            &[[SCREEN_WIDTH/2.0, 0.0], [SCREEN_WIDTH/2.0, SCREEN_HEIGHT]], 
            2.0, graphics::WHITE)?;
        graphics::draw(ctx, &mid_line, graphics::DrawParam::default())?;
        
        self.p1.draw(ctx)?;
        self.p2.draw(ctx)?;
        self.ball.draw(ctx)?;
        self.p1.draw_score(ctx)?;
        self.p2.draw_score(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("netpong", "ahenshaw")
            .window_mode(
                conf::WindowMode::default()
                    .resizable(true) 
                    .maximized(true)
                    .fullscreen_type(conf::FullscreenType::Windowed)
            )
            .build()?;

    let _ = ggez::timer::delta(&ctx);

    graphics::set_window_title(&ctx, "Net Pong");
    let state = MainState::new(&mut ctx);
    mouse::set_cursor_grabbed(&mut ctx, true)?;
    mouse::set_cursor_hidden(&mut ctx, true);
    event::run(ctx, event_loop, state)
}
