extern crate dankgine;

use dankgine::bodies;
use dankgine::engine::Engine;
use dankgine::world::World;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape;
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

pub fn setup_and_draw_sketch() {
    let world = World::new(9.81);
    let engine = Engine { world: world };

    let window = Window::new_centered("Speedy2D: Animation", (800, 800)).unwrap();

    window.run_loop(MyWindowHandler {
        mouse_pos: Vector2::ZERO,
        mouse_button_down: false,
        engine: engine,
    })
}

struct MyWindowHandler {
    mouse_pos: Vector2<f32>,
    mouse_button_down: bool,
    engine: Engine,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);

        if self.mouse_button_down {
            let circle = bodies::Circle::new(self.mouse_pos.x, self.mouse_pos.y, 25.0);
            self.engine.world.push_circle(circle);
        }

        self.engine.update();

        for circle in &self.engine.world.circles {
            graphics.draw_circle(
                Vector2::new(circle.x, circle.y),
                circle.radius,
                Color::from_rgb(1.0, 0.4, 0.0),
            );
        }

        for rectangle in &self.engine.world.rectangles {
            graphics.draw_rectangle(
                shape::Rectangle::new(
                    Vector2::new(rectangle.x, rectangle.y + rectangle.height),
                    Vector2::new(rectangle.x + rectangle.width, rectangle.y),
                ),
                Color::from_rgb(1.0, 0.4, 0.0),
            );
        }

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vector2<f32>) {
        self.mouse_pos = position;

        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        if button == MouseButton::Left {
            self.mouse_button_down = true;
        }

        helper.request_redraw();
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        if button == MouseButton::Left {
            self.mouse_button_down = false;
        }

        helper.request_redraw();
    }
}
