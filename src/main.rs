extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;
use rand::Rng;

struct Player {
    pub rows: i32,
    pub columns: i32,
    pub x: i32,
    pub y: i32,
    factor: f64
    //pub shots: Shots,
    //pub apples: Vec<Apples>
}

impl Player {
    pub fn new() -> Player {
        Player {
            rows: 4,
            columns: 3,
            x: 0,
            y: 0,
            factor: 110.0
            //shots: Shots::new(),
            // apples: vec![]
        }
    }

    fn calc_coord(&mut self, pos: f64) -> f64 {
        self.factor + (pos * self.factor)
    }

    pub fn throw(&mut self) {
        let x = self.x as f64;
        let y = self.y as f64;
        let x = self.calc_coord(x);
        let y = self.calc_coord(y);
        println!("{}, {}", x, y);
    }
    pub fn update(&mut self, dt: f64) {
        //
    }
    pub fn moving(&mut self, x: i32, y: i32) {
        self.x += x;
        if self.x > self.columns -1 {
            self.x = 0;
        } else if self.x < 0 {
            self.x = self.columns -1;
        }
        self.y += y;
        if self.y > self.rows -1 {
            self.y = 0;
        } else if self.y < 0 {
            self.y = self.rows -1;
        }
    }
}


struct Game {
    pub scene: usize,
    //pub folks: Vec<Folk>,
    pub player: Player,
    last_folk: f64,
    folk_interval: f64,
    global_time: f64
}

impl Game {
    pub fn new(param_scene: usize) -> Game {
        Game {
            scene: param_scene,
            //folks: vec![],
            player: Player::new(),
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
        "Manzana Attack", [500, 700])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3,3)
        .for_folder("assets").unwrap();

    let ref font = assets.join("Amatic-Bold.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone()).unwrap();

    let house = Texture::from_path(
        &mut window.factory,
        assets.join("house.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();


    let black = [0.0, 0.0, 0.0, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];

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
                        match key {
                            Key::W => {
                                game.player.moving(0, -1)
                            }
                            Key::S => {
                                game.player.moving(0, 1);
                            }
                            Key::A => {
                                game.player.moving(-1,0);
                            }
                            Key::D => {
                                game.player.moving(1, 0);
                            }
                            Key::M => {
                                game.player.throw();
                            }
                            _ => {}
                        }
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
                            clear(white, g);

                            text::Text::new_color(black, 50)
                                .draw(
                                    &"Manzana Attack",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 100.0),
                                    g);

                            text::Text::new_color(black, 20)
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
                            clear(white, g);

                            image(&house, c.transform.scale(0.5, 0.5), g);

                            text::Text::new_color(black, 50)
                                .draw(
                                    &"This is a game",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 100.0),
                                    g);

                            for (ci, cv) in (0..game.player.rows).enumerate() {
                                for (ri, rv) in (0..game.player.columns).enumerate() {
                                    if rv == game.player.x
                                        && cv == game.player.y {
                                        let square = rectangle::square(0.0, 0.0, 50.0);
                                        rectangle(black, square, c.transform.trans(
                                            game.player.calc_coord(rv as f64),
                                            game.player.calc_coord(cv as f64)), g);
                                    }

                                }
                            }

                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
