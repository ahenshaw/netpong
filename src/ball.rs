use std::f32::consts::PI;

use ggez::{
    Context,
    graphics,
    audio,
    GameResult
};
use ggez::audio::SoundSource;
use rand::{self, thread_rng, Rng};

use crate::{Position, Velocity, SCREEN_WIDTH, SCREEN_HEIGHT};

const RADIUS: f32 = 6.0;
const BALL_SPEED: f32 = 360.0;

pub struct Ball {
    pub pos: Position,
    pub vel: Velocity,
    pub radius: f32,
    ping:  audio::Source,
    pong:  audio::Source,
    table: audio::Source,
    consecutive: i32,
}

impl Ball {
    pub fn new(ctx: &mut Context) -> Self {
        let mut ball = Ball{pos: Position{x:0.0, y:0.0}, 
                            vel: Velocity{x:0.0, y:0.0},
                            radius: RADIUS,
                            ping:  audio::Source::new(ctx, "/ping.wav").expect("Could load pong sound file"),
                            pong:  audio::Source::new(ctx, "/pong.wav").expect("Could load pong sound file"),
                            table: audio::Source::new(ctx, "/table.wav").expect("Could load table sound file"),
                            consecutive: 0,};
        ball.init();
        ball
    }

    fn init(&mut self) {
        self.pos.x = SCREEN_WIDTH / 2.0;

        let mut rng = thread_rng();
        self.vel.x = match rng.gen_bool(0.5) {
            true  =>  BALL_SPEED,
            false => -BALL_SPEED,
        };
        self.vel.y = match rng.gen_bool(0.5) {
            true  =>  BALL_SPEED,
            false => -BALL_SPEED,
        };
        self.consecutive = 0;
    }
    
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            self.radius,
            0.1,
            graphics::Color::from_rgb(255, 198, 41),
        )?;

        let draw_param = graphics::DrawParam::default();
        // draw_param.dest(self.pos);
        graphics::draw(ctx, &mesh, draw_param)?;
        //graphics::pop_transform(ctx);
        // graphics::apply_transformations(ctx)?;

        Ok(())
    }


    pub fn update(&mut self, dt: f32, ctx: &mut Context) -> (i32, i32) {
        let s = dt * 1.1f32.powi(self.consecutive/2);
        self.pos.x += self.vel.x * s; 
        self.pos.y += self.vel.y * s; 

        if self.pos.x <= 0.0 {
            self.init();
            return (0, 1);
        }
        if self.pos.x >= SCREEN_WIDTH {
            self.init();
            return (1, 0);
        }

        // floor or ceiling bounce
        if self.pos.y <= self.radius {
            self.pos.y = self.radius;
            self.vel.y = self.vel.y.abs();
            self.table.play_detached(ctx).unwrap();
        } else if self.pos.y >= SCREEN_HEIGHT - self.radius {
            self.pos.y = SCREEN_HEIGHT - self.radius;
            self.vel.y = -self.vel.y.abs();
            self.table.play_detached(ctx).unwrap();
        }
        (0, 0)
    }

    pub fn paddle_strike(&mut self, x: f32, y: f32, yp: f32, length: f32, is_left:bool, ctx: &mut Context) {
        self.consecutive = (self.consecutive + 1).min(44);
        if is_left {
            self.ping.play_detached(ctx).unwrap();
        } else {
            self.pong.play_detached(ctx).unwrap();
        }
        self.pos.x = x;
        self.pos.y = y;

        let dy = yp - y;
        let offset= if dy.abs() <= length/4.0 {0.0} else {PI * dy / (2.0 * length)};
        let mut theta = self.vel.y.atan2(-self.vel.x);
        let mut angle = (((theta + offset) * 180.0/PI) + 360.0) % 360.0;
        let speed = (self.vel.x * self.vel.x + self.vel.y * self.vel.y).sqrt();
        if is_left {
            if  angle > 60.0 && angle <= 180.0 {
                angle = 60.0;
            }
            if angle > 180.0 && angle <= 300.0 {
                angle = 300.0;
            }
        } else {
            angle = angle.max(120.0).min(240.0);
        }
        theta = angle * PI / 180.0;
        self.vel.x = theta.cos() * speed;
        self.vel.y = theta.sin() * speed;
    }
}