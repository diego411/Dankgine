pub trait Body {
    fn pos(&self) -> (f32, f32);
    fn apply_force(&mut self, vec: (f32, f32));
    fn body_type(&self) -> BodyType;
}

pub enum BodyType {
    Rectangle,
    Circle,
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Body for Rectangle {
    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn apply_force(&mut self, vec: (f32, f32)) {
        self.x += vec.0;
        self.y += vec.1;
    }

    fn body_type(&self) -> BodyType {
        BodyType::Rectangle
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Body for Circle {
    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn apply_force(&mut self, vec: (f32, f32)) {
        self.x += vec.0;
        self.y += vec.1;
    }

    fn body_type(&self) -> BodyType {
        BodyType::Circle
    }
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Circle {
        Circle { x, y, radius }
    }
}
