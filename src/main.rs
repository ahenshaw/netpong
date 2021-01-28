mod ball;
mod player;
mod flexcontrol;
mod netpong;

// custom modules
use ball::Ball;
use player::{Player, PlayerType, PlayerMode};
use flexcontrol::FlexControl;

use std::time::Duration;
use structopt::StructOpt;

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

const WINNING_SCORE:i32 = 15;

struct MainState {
    p1:   Player,
    p2:   Player,
    ball: Ball,
    mode: GameMode,
    controller: FlexControl,
}


#[derive(Copy, Clone)]
enum GameMode {
    Paused,
    WaitingForNetwork,
    Active,
    GameOver,
}

fn to_player_type(s: &str) -> PlayerType {
    match s {
            "network"   => PlayerType::Network(None),
            "computer"  => PlayerType::Computer,
            "me"        => PlayerType::Human("player".to_string()),
            _           => PlayerType::Network(Some(s.to_string())),
        }
}

fn message(ctx: &mut Context, s: &str) -> GameResult {
    let text = graphics::Text::new(s);
    let r = text.dimensions(ctx);

    graphics::draw(ctx, &text, graphics::DrawParam::default().dest([(SCREEN_WIDTH - r.w)/2.0, (SCREEN_HEIGHT - r.h)/2.0]))?;
    // graphics::present(ctx)?;
    Ok(())
}

impl MainState {
    pub fn new(ctx: &mut Context, opt: Opt) -> Self {
        let left = to_player_type(&opt.left);
        let right = to_player_type(&opt.right);
        
        let mode = match (&left, &right) {
            (PlayerType::Network(None), _) => GameMode::WaitingForNetwork,
            (_, PlayerType::Network(None)) => GameMode::WaitingForNetwork,
            _ => GameMode::Paused,
        };
        MainState {
            p1: Player::new(true,  &left, &right),
            p2: Player::new(false, &right, &left),
            ball: Ball::new(ctx),
            mode,
            controller: FlexControl::new(),
        }
    }
}

impl event::EventHandler for MainState {
    fn key_up_event(&mut self, 
        ctx: &mut Context, 
        keycode: event::KeyCode, _keymods: event::KeyMods) {
        match (keycode, self.mode) {
            (event::KeyCode::Space, GameMode::Paused) => {
                self.mode = GameMode::Active;
                mouse::set_cursor_grabbed(ctx, true).unwrap();
                mouse::set_cursor_hidden(ctx, true);
            },
            (event::KeyCode::Space, GameMode::Active) => {
                self.mode = GameMode::Paused;
                mouse::set_cursor_grabbed(ctx, false).unwrap();
                mouse::set_cursor_hidden(ctx, false);
            },
        _ => (),
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let dt = ggez::timer::delta(ctx).as_secs_f32();
        ggez::timer::sleep(Duration::from_secs_f32((0.016666 - dt).max(0.0)));

        match self.mode {
            GameMode::Paused => {return Ok(())},
            GameMode::GameOver => {
                self.p1.update(ctx, dt, 0.0);
                self.p2.update(ctx, dt, 0.0);
                return Ok(())
            },
            GameMode::WaitingForNetwork => {
                self.mode = GameMode::Paused;
                return Ok(())},
            _ => ()
        };

        if dt < 0.1 {
            let motion = {
                let x = self.controller.read();
                (3 * x.signum() * x * x) as f32
            };
    
            self.p1.update(ctx, dt, 0.0);
            self.p2.update(ctx, dt, motion);

            let (s1, s2) = self.ball.update(dt, ctx);

            if self.p1.update_score(s1) == WINNING_SCORE {
                self.ball.game_over();
                self.mode = GameMode::GameOver;
                self.p1.set_mode(PlayerMode::Winner);
                self.p2.set_mode(PlayerMode::Loser);
            }
            if self.p2.update_score(s2) == WINNING_SCORE {
                self.ball.game_over();
                self.mode = GameMode::GameOver;
                self.p2.set_mode(PlayerMode::Winner);
                self.p1.set_mode(PlayerMode::Loser);
            }

            self.p1.check_for_hit(&mut self.ball, ctx);
            self.p2.check_for_hit(&mut self.ball, ctx);
        }
        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        match self.mode {
            GameMode::Paused => {
                message(ctx, "Game paused. Hit [space] to continue.\n[Esc] to quit.")?;
                graphics::present(ctx)?;
                return Ok(())
            },
            GameMode::WaitingForNetwork => {
                message(ctx, "Waiting for network player")?;
                graphics::present(ctx)?;
                return Ok(())
            },
            GameMode::GameOver => {
                self.p1.draw(ctx)?;
                self.p2.draw(ctx)?;
                self.p1.draw_score(ctx)?;
                self.p2.draw_score(ctx)?;
                message(ctx, "Game Over")?;
                graphics::present(ctx)?;
                return Ok(())
            },
            _ => ()
        }; 

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

#[derive(StructOpt, Debug)]
#[structopt(name = "netpong")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short, long, default_value = "360")]
    speed: f64,
    #[structopt(default_value = "me")]
    left: String,
    #[structopt(default_value = "computer")]
    right: String,
}

fn main() -> GameResult {
    let opt = Opt::from_args();

    let (mut ctx, event_loop) = ContextBuilder::new("netpong", "ahenshaw")
            .window_mode(
                conf::WindowMode::default()
                    .resizable(true) 
                    .maximized(true)
                    .fullscreen_type(conf::FullscreenType::Windowed)
            )
            .build()?;

    graphics::set_window_title(&ctx, "Net Pong");
    let state = MainState::new(&mut ctx, opt);

    event::run(ctx, event_loop, state)
}
