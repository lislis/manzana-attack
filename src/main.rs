extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;
use rand::Rng;

struct Game {
    pub scene: usize,
    //pub folks: Vec<Folk>,
    //pub player: Player,
    last_folk: f64,
    folk_interval: f64,
    global_time: f64
}

impl Game {
    pub fn new(param_scene: usize) -> Game {
        Game {
            scene: param_scene,
            //folks: vec![],
            last_folk: 0.0,
            folk_interval: 3.0,
            global_time: 0.0
        }
    }
    pub fn set_scene(&mut self, scene: usize) {
        self.scene = scene;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        "Manzana Attack", [500, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3,3)
        .for_folder("assets").unwrap();

    let ref font = assets.join("Amatic-Bold.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone()).unwrap();

    let mut game = Game::new(1);

    while let Some(e) = window.next() {

        match e {
            Input::Release(Button::Keyboard(key)) => {
                match game.scene {
                    1 => {
                        if key == Key::M {
                            game.set_scene(2);
                        }
                    }
                    2 => {
                        // game play
                    }
                    _ => {}
                }
            }

            Input::Update(args) => {
                if game.scene == 2 {
                    // game.update();
                    // game.check_collision();
                }
            }

            Input::Render(_) => {
                match game.scene {
                    1 => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0; 4], g);

                            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 50)
                                .draw(
                                    &"Manzana Attack",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 100.0),
                                    g);

                            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 20)
                                .draw(
                                    &"Press <m> to start",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(200.0, 300.0),
                                    g);
                        });
                    }
                    2 => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0; 4], g);

                            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 50)
                                .draw(
                                    &"This is a game",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 100.0),
                                    g);

                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
