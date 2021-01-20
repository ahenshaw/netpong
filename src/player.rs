use std::net::UdpSocket;

use quick_protobuf::{Writer};
use ggez::{Context, GameResult, graphics};
use ggez::input::mouse;

use crate::netpong::mod_Update::{OneOfUpdateType};
use crate::netpong::{Update, Paddle};

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, Position, Ball};


pub struct Player {
    pos: Position,
    width: f32,
    height: f32,
    is_left: bool,
    last_ball: Option<Position>,
    pub is_active: bool,
    pub score: i32,
}

impl Player {
    pub fn new(is_left: bool) -> Self {
        let padding = SCREEN_HEIGHT / 15.0;
        let x = if is_left {padding} else {SCREEN_WIDTH - padding};

        Player{
            pos: Position{x, y: SCREEN_HEIGHT/30.0}, 
            width:  SCREEN_HEIGHT / 100.0,
            height: SCREEN_HEIGHT / 15.0,
            is_left, 
            last_ball: None, 
            is_active:false, 
            score: 0
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if !self.is_left {
            self.pos.y = mouse::position(ctx).y;
        }

        self.pos.y = self.pos.y.max(self.height/2.0).min(SCREEN_HEIGHT - self.height/2.0);

        if self.is_left {
            // send network message with paddle position
            let message = Update{UpdateType: OneOfUpdateType::paddle(Paddle{y:self.pos.y})};
            let mut out = Vec::new();
            let mut writer = Writer::new(&mut out);
            writer.write_message(&message).expect("Cannot write message!");

            let socket = UdpSocket::bind("127.0.0.1:0").expect("Can't open UDP connection!");
            socket.send_to(&out, "127.0.0.1:34254").expect("Can't send to UDP socket");
        }
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
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        
        Ok(())
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
        if self.is_left {
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

                if self.is_left {
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
