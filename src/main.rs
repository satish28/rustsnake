extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game{
    fn render(&mut self, arg: &RenderArgs){
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl|{
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }
}

struct Snake {
    pos_x: f64,
    pos_y: f64,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs){
        use graphics::*;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(self.pos_x, self.pos_y, 20_f64);
        gl.draw(arg.viewport(), |c, gl|{
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl)
        })
    }
}

fn main(){
    let opengl = OpenGL::V3_2;


    let mut window: Window = WindowSettings::new(
        "snake",
            [400, 400]
        ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake {pos_x: 50.0, pos_y: 100.0}
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
    }
}