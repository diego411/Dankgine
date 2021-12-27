extern crate dankgine;

use dankgine::bodies;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape;
use speedy2d::Graphics2D;

pub trait Drawable {
    fn draw(&self, graphics: &mut Graphics2D);
    fn update(&mut self);
}

pub struct Rectangle<'a> {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    pub engine_body: &'a bodies::Rectangle,
}

impl<'a> Rectangle<'a> {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        engine_body_ref: &bodies::Rectangle,
    ) -> Rectangle {
        //let engine_body = bodies::Rectangle::new(x, y, width, height);
        Rectangle {
            x: x,
            y: y,
            width: width,
            height: height,
            engine_body: engine_body_ref,
        }
    }
}

impl<'a> Drawable for Rectangle<'a> {
    fn update(&mut self) {
        self.x = self.engine_body.x;
        self.y = self.engine_body.y;
        self.width = self.engine_body.width;
        self.height = self.engine_body.height;
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_rectangle(
            shape::Rectangle::new(
                Vector2::new(self.x, self.y + self.height),
                Vector2::new(self.x + self.width, self.y),
            ),
            Color::from_rgb(1.0, 0.4, 0.0),
        );
    }
}

pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    pub engine_body: bodies::Circle,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Circle {
        let engine_body = bodies::Circle::new(x, y, radius);
        Circle {
            x,
            y,
            radius,
            engine_body: engine_body,
        }
    }
}

impl Drawable for Circle {
    fn update(&mut self) {
        self.x = self.engine_body.x;
        self.y = self.engine_body.y;
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle(
            Vector2::new(self.x, self.y),
            self.radius,
            Color::from_rgb(1.0, 0.4, 0.0),
        );
    }
}
