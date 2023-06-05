use crate::vector::Vector;
use speedy2d::color::Color;
use speedy2d::Graphics2D;

pub struct Pendulum {
    origin: Vector,

    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_acceleretion: f32,

    length: f32,
    gravity: f32,
}

impl Pendulum {
    pub fn new(x: f32, y: f32, length: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleretion: 0.0,
            length,
            gravity: 1.5,
        }
    }

    pub fn update(&mut self) {
        self.angular_acceleretion = -1.0 * self.gravity * self.angle.sin() / self.length;

        self.angular_velocity += self.angular_acceleretion;

        self.angle += self.angular_velocity;

        self.position.set(
            self.length * self.angle.sin(),
            self.length * self.angle.cos(),
        );

        self.position.add(&self.origin);
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::BLACK,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::BLACK)

    }
}
