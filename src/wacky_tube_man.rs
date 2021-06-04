
use ggez::graphics::{Color, Mesh, DrawMode, Rect, DrawParam};
use ggez::graphics;
use ggez::{Context, GameResult};
use num_traits::float::FloatConst;

//graphics::Color::from_rgb(255, 198, 41)
//let h = SCREEN_HEIGHT / 100.0;

struct Orientation {
    x:   f32,
    y:   f32,
    phi: f32,    
}

struct Chain {
    lengths: Vec<f32>,
}

impl Chain {
    pub fn new(lengths: Vec<f32>) -> Chain {
        Chain{lengths}
    }

    fn solve(&self, initial: &Orientation, angles: Vec<f32>) -> Vec<(Orientation, f32)> {
        let mut x   = initial.x;
        let mut y   = initial.y;
        let mut phi = initial.phi;
        let mut chain: Vec<(Orientation, f32)> = vec![];

        for (theta, &length) in angles.iter().zip(&self.lengths) {
            phi += theta;
            chain.push((Orientation{x, y, phi}, length));
            x += phi.cos() * length;
            y += phi.sin() * length;
        }        
        chain
    }
}

pub struct WackyTubeMan {
    time:   f32,
    color:  Color,
    x:      f32,
    y:      f32,
    body:   Chain,
    left:   Chain,
    right:  Chain,
    height: f32,
    thickness: f32,
}

impl WackyTubeMan {
    pub fn new(height: f32, thickness:f32, color: Color) -> WackyTubeMan {
        let bsegment = height / 4.0;
        let asegment = bsegment / 2.0;
        WackyTubeMan {
            time: 0.0,
            color,
            x: 0.0,
            y: 0.0,
            body:  Chain::new(vec![bsegment, bsegment, bsegment*0.6, bsegment*0.4, bsegment]),
            left:  Chain::new(vec![asegment;4]),
            right: Chain::new(vec![asegment;4]),
            height,
            thickness,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self, ctx: &mut Context, is_happy: bool) -> GameResult {
        let t = self.time;
        
        let angles = if is_happy {
            vec![-f32::PI()/2.0, (3.0*t).sin()/2.0, (5.0*t).cos()/2.0, 0.0, (7.0*t).sin()/5.0]
        } else {
            vec![-f32::PI()/2.0, 0.0, 0.4, 0.0, 0.7+(t).sin()/5.0]
            // vec![-f32::PI()/2.0, (t/3.0).sin()/2.0, (t/5.0).cos()/2.0, 0.0, (t/11.0).sin()/5.0]
        };

        let left_angles = if is_happy {
            vec![ -f32::PI()/4.0, (5.0*t).sin()/2.0, (7.0*t).cos()/2.0, (11.0*t).sin()/2.0]
        } else {
            // vec![ -f32::PI()*0.8, (t/3.0).sin()/4.0, (t/5.0).cos()/4.0, (t/1.0).sin()/4.0]
            vec![ f32::PI()*0.8, ( -t/3.0).cos()/4.0, ( -t/5.0).sin()/4.0, (-t/1.0).cos()/4.0]
        };

        let right_angles = if is_happy {
            vec![ f32::PI()/4.0, ( 5.0 * t).cos()/2.0, ( 7.0 * t).sin()/2.0, (11.0 * t).cos()/2.0]
        } else {
            vec![ f32::PI()*0.8, ( -t/3.0).cos()/4.0, ( -t/5.0).sin()/4.0, (-t/1.0).cos()/4.0]
        };

        let base = Orientation{x:self.x, y:self.y + self.height/2.0, phi: 0.0};
        let tube = self.body.solve(&base, angles);
        let (shoulder, _) = &tube[3];
        let left_arm = self.left.solve(&shoulder, left_angles);
        let right_arm = self.right.solve(&shoulder, right_angles);

        for (o, l) in &tube {
            let part = Mesh::new_rectangle(
                ctx, 
                DrawMode::fill(), 
                Rect::new(0.0, 0.0, *l, self.thickness), 
                self.color
            )?;
            graphics::draw(
                ctx, 
                &part, 
                DrawParam::default().dest([o.x, o.y]).rotation(o.phi).offset([0.0, self.thickness/2.0])
            )?;
        }
        for (o, l) in &left_arm {
            let part = Mesh::new_rectangle(
                ctx, 
                DrawMode::fill(), 
                Rect::new(0.0, 0.0, *l, self.thickness/2.0), 
                self.color
            )?;
            graphics::draw(
                ctx, 
                &part, 
                DrawParam::default().dest([o.x, o.y]).rotation(o.phi).offset([0.0, self.thickness/4.0])
            )?;
        }
        for (o, l) in &right_arm {
            let part = Mesh::new_rectangle(
                ctx, 
                DrawMode::fill(), 
                Rect::new(0.0, 0.0, *l, self.thickness/2.0), 
                self.color
            )?;
            graphics::draw(
                ctx, 
                &part, 
                DrawParam::default().dest([o.x, o.y]).rotation(o.phi).offset([0.0, self.thickness/4.0])
            )?;
        }

        Ok(())
    }
}

