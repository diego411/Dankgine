extern crate dankgine;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let window = Window::new_centered("Speedy2D: Animation", (800, 800)).unwrap();

    window.run_loop(MyWindowHandler {
        circles: Vec::new(),
        mouse_pos: Vector2::ZERO,
        mouse_button_down: false,
    })
}

struct MyWindowHandler {
    circles: Vec<(f32, f32)>,
    mouse_pos: Vector2<f32>,
    mouse_button_down: bool,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);

        if self.mouse_button_down {
            self.circles.push((self.mouse_pos.x, self.mouse_pos.y));
        }

        for vec in &mut self.circles {
            let v = Vector2::new(vec.0, vec.1);
            graphics.draw_circle(v, 20.0, Color::from_rgb(1.0, 0.4, 0.0));
            vec.1 = vec.1 + 9.81;
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
