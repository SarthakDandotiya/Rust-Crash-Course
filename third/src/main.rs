// To draw a Red box to the screen
// The point of this is to know how to install a dependency, using the library and not about the functionality & working of the code here
extern crate piston_window; 

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480]).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0],[0.0, 0.0, 100.0, 100.0], context.transform, graphics);
        });
    }
}