use std::net::UdpSocket;

use quick_protobuf::{Writer};
use ggez::{Context, GameResult, graphics};
use ggez::input::mouse;

use crate::netpong::mod_Update::{OneOfUpdateType};
use crate::netpong::{Update, Paddle};

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, Position, Ball};
use crate::wacky_tube_man::WackyTubeMan;
use num_traits::float::FloatConst;

pub enum PlayerMode {
    Active,
    Winner,
    Loser,
}

pub struct Player {
    me: PlayerType,
    pos: Position,
    width: f32,
    height: f32,
    is_left: bool,
    last_ball: Option<Position>,
    pub score: i32,
    t: f32,
    mode: PlayerMode,
    wacky: WackyTubeMan,
}

#[derive(PartialEq, Clone)]
pub enum PlayerType {
    Human(String),
    Computer,
    Network(Option<String>),
}

const PORT: u16 = 34521;

impl Player {
    pub fn new(is_left: bool, me: &PlayerType) -> Self {
        
        let padding = SCREEN_HEIGHT / 15.0;
        let x = if is_left {padding} else {SCREEN_WIDTH - padding};
        let height = SCREEN_HEIGHT / 15.0;
        let width  = SCREEN_HEIGHT / 100.0;
        Player{
            me: me.clone(),
            pos: Position{x, y: SCREEN_HEIGHT/30.0}, 
            width,
            height,
            is_left, 
            last_ball: None, 
            score: 0,
            t: 0.0,
            mode: PlayerMode::Active,
            wacky: WackyTubeMan::new(height, width, graphics::Color::from_rgb(255, 198, 41)),
        }
    }

    pub fn update_score(&mut self, increment: i32) -> i32 {
        self.score += increment;
        self.score
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32, extra: f32) {
        self.t += dt;
        self.wacky.update(dt);
        self.wacky.set_position(self.pos.x, self.pos.y);
        match self.me{
            PlayerType::Human(_) => {
                if self.is_left {
                    //self.pos.y = mouse::position(ctx).y;
                    self.pos.y += extra;
                } else { 
                    self.pos.y += extra;
                }
                self.pos.y = self.pos.y.max(self.height/2.0).min(SCREEN_HEIGHT - self.height/2.0);
            },
            _ => ()
        }

        // match &self.opponent {
        //     PlayerType::Network(Some(address)) => { 
        //         // send network message with paddle position
        //         let message = Update{UpdateType: OneOfUpdateType::paddle(Paddle{y:self.pos.y})};
        //         let mut out = Vec::new();
        //         let mut writer = Writer::new(&mut out);
        //         writer.write_message(&message).expect("Cannot write message!");
        //         if let Some(socket) = &self.out_sock {
        //             socket.send_to(&out, format!("{}:{}", address, self.out_port)).expect("Can't send to UDP socket");
        //         }
        //     },
        //     _ => ()
        // }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(),  
            graphics::Rect::new(
                self.pos.x-self.width/2.0, 
                self.pos.y-self.height/2.0, 
                self.width, 
                self.height), 
            graphics::Color::from_rgb(255, 198, 41))?;

        match self.mode {
            PlayerMode::Active => graphics::draw(ctx, &mesh, graphics::DrawParam::default())?,
            PlayerMode::Loser  => self.wacky.draw(ctx, false)?,
            PlayerMode::Winner => self.wacky.draw(ctx, true)?,
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: PlayerMode) {
        self.mode = mode;
    }
    
    pub fn draw_score(&self, ctx: &mut Context) -> GameResult {
        let font = graphics::Font::new(ctx, "/Orbitron-Bold.ttf")?;

        let fragment = graphics::TextFragment {
            text: format!("{}", self.score),
            color: Some(graphics::WHITE),
            font: Some(font),
            scale: Some(graphics::PxScale::from(64.0)),
        };
        let text = graphics::Text::new(fragment);

        let score_x = if self.is_left {SCREEN_WIDTH * 0.25} else {SCREEN_WIDTH * 0.75};
        let score_pos = [score_x, 40.0];
        graphics::draw(ctx, &text, graphics::DrawParam::default().dest(score_pos))?;
        Ok(())
    }

    pub fn check_for_hit(&mut self, ball: &mut Ball, ctx: &mut Context) { 
        if self.me == PlayerType::Computer {
            self.pos.y = ball.pos.y;
        }

        if let Some(prev) = &self.last_ball {

            let (edge, possible) = if self.is_left {
                let edge = self.pos.x + self.width/2.0 + ball.radius;
                (edge, (ball.pos.x <= edge) && (self.pos.x <= prev.x))
            } else {
                let edge = self.pos.x - self.width/2.0 - ball.radius;
                (edge, (ball.pos.x  >= edge) && (self.pos.x >= prev.x))
            };

            if possible {
                // find the time when the ball would have contacted the edge
                let dx    = ball.pos.x - prev.x;
                let dy    = ball.pos.y - prev.y;
                let frac = (edge - prev.x)/dx;
                let y    = prev.y + dy * frac;

                if self.me == PlayerType::Computer {
                    self.pos.y  = y; // demo mode
                }
                let top    = self.pos.y + self.height/2.0;
                let bottom = self.pos.y - self.height/2.0;
    
                if (y >= bottom - ball.radius ) && (y <= top + ball.radius) {
                    // collision!
                    ball.paddle_strike(edge, y, self.pos.y, self.height, self.is_left, ctx);
                    self.last_ball = None;
                    return;
                } 
            }
        }
        self.last_ball = Some(ball.pos);
    }

    
}
