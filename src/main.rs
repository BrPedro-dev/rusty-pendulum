mod vector;
mod pendulum;
mod window_handler;
use speedy2d::Window;

fn main() {
    let window = Window::new_centered("Pedulum", (800, 480)).unwrap();
    let window_handler = window_handler::MyWindowHandler{ 
        p: pendulum::Pendulum::new(400.0, 0.0, 200.0),
        p2: pendulum::Pendulum::new(400.0, 0.0, 400.0),
    };

    window.run_loop(window_handler);

}




