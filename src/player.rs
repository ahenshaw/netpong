use std::net::UdpSocket;

use quick_protobuf::{Writer};
use ggez::{Context, GameResult, graphics};
use ggez::input::mouse;

use crate::netpong::mod_Update::{OneOfUpdateType};
use crate::netpong::{Update, Paddle};

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, Position, Ball};


pub struct Player {
    me: PlayerType,
    opponent: PlayerType,
    pos: Position,
    width: f32,
    height: f32,
    is_left: bool,
    last_ball: Option<Position>,
    in_sock:   Option<UdpSocket>,
    out_sock:  Option<UdpSocket>,
    out_port:  u16,
    pub score: i32,
}
#[derive(PartialEq, Clone)]
pub enum PlayerType {
    Human(String),
    Computer,
    Network(Option<String>),
}

const PORT: u16 = 34521;

impl Player {
    pub fn new(
            is_left: bool, 
            me: &PlayerType, 
            opponent: &PlayerType) -> Self {
        
        let padding = SCREEN_HEIGHT / 15.0;
        let x = if is_left {padding} else {SCREEN_WIDTH - padding};

        let (in_port, out_port) = if is_left {(PORT, PORT+1)} else {(PORT + 1, PORT)};

        let in_socket = match me {
            PlayerType::Network(_) => {
                let socket = UdpSocket::bind(format!("0.0.0.0:{}", in_port)).expect("Failed to bind socket");
                socket.set_nonblocking(true).expect("Couldn't set inport to non-blocking");
                Some(socket)
            },
            _ => None,
        };
        let out_socket = match opponent {
            PlayerType::Network(_) => {
                Some(UdpSocket::bind("0.0.0.0:0").expect("Failed to create socket"))
            },
            _ => None,
        };

        Player{
            me: me.clone(),
            opponent: opponent.clone(),
            pos: Position{x, y: SCREEN_HEIGHT/30.0}, 
            width:  SCREEN_HEIGHT / 100.0,
            height: SCREEN_HEIGHT / 15.0,
            is_left, 
            in_sock: in_socket,
            out_sock: out_socket,
            out_port,
            last_ball: None, 
            score: 0
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        match self.me{
            PlayerType::Human(_) => {
                self.pos.y = mouse::position(ctx).y;
                self.pos.y = self.pos.y.max(self.height/2.0).min(SCREEN_HEIGHT - self.height/2.0);
            },
            PlayerType::Network(_) => {
                if let Some(socket) = &self.in_sock {
                    let mut buf = [0u8; 1024];
                    let result = socket.recv_from(&mut buf);
                    match result {
                        Ok((len, addr)) => {
                            dbg!(len, addr);
                        },
                        _ => ()
                    }
                }
            },
            _ => ()
        }

        match &self.opponent {
            PlayerType::Network(Some(address)) => { 
                // send network message with paddle position
                let message = Update{UpdateType: OneOfUpdateType::paddle(Paddle{y:self.pos.y})};
                let mut out = Vec::new();
                let mut writer = Writer::new(&mut out);
                writer.write_message(&message).expect("Cannot write message!");
                if let Some(socket) = &self.out_sock {
                    socket.send_to(&out, format!("{}:{}", address, self.out_port)).expect("Can't send to UDP socket");
                }
            },
            _ => ()
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
