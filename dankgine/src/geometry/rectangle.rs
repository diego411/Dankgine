use super::vector::Vec2;

#[derive(Debug)]
pub struct Rectangle {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x:f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle { position: Vec2::new(x, y), width: width, height: height }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        return point.x >= self.position.x && point.x <= self.position.x + self.width && 
            point.y >= self.position.y && point.y <= self.position.y + self.height
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        return self.position.x < other.position.x + other.width && 
            self.position.x + self.width > other.position.x &&
            self.position.y < other.position.y + other.height &&
            self.position.y + self.height > other.position.y;
    }
}