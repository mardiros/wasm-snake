
#[macro_use]
extern crate stdweb;


use std::rc::Rc;
use std::cell::RefCell;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};


struct Item {
    x: u32,
    y: u32,
}


fn rand_32(max: u32) -> u32{
    let v = js!(return Math.random());
    let v: f64 = v.try_into().unwrap();
    let v = (v * max as f64).ceil();
    let v: u32 = v as u32;
    v
}

impl Item {
    fn new(max_x: u32, max_y: u32) -> Item {
        let x: u32 = rand_32(max_x);
        let y: u32 = rand_32(max_y);
        Item { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up, Down, Left, Right
}

struct Store {
    width: u32,
    height: u32,
    speed: f64,
    item: Item,
    playing: bool,
    game_over: bool,
    direction: Direction,
}

impl Store {
    fn new(width: u32, height: u32) -> Store {
        Store {
            width,
            height,
            item: Item::new(width, height),
            speed: 300.0,
            playing: true,
            game_over: false,
            direction: Direction::Right,
        }
    }
    fn move_up(&mut self) {
        self.direction = Direction::Up
    }
    fn move_down(&mut self) {
        self.direction = Direction::Down
    }
    fn move_left(&mut self) {
        self.direction = Direction::Left
    }
    fn move_right(&mut self) {
        self.direction = Direction::Right
    }
    fn pause_toggle(&mut self) {
        self.playing = !self.playing        
    }

}

struct Canvas {
    store: Store,
    canvas: CanvasElement,
}

impl Canvas {
    fn new(selector: &str, store: Store) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(selector)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let scaling = 10;

        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let border = scaling as f64;
        let canvas_width = store.width * scaling;
        let canvas_height = store.height * scaling;

        canvas.set_width(canvas_width + 2 * scaling);
        canvas.set_height(canvas_height + 2 * scaling);

        context.set_transform(border, 0f64, 0f64, border, 0f64, 0f64);

        js! {
            console.log(" canvas initialized" );
        };
        Canvas {
            store,
            canvas,
        }
    }

    fn repaint(&mut self) {
        let context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
        context.set_fill_style_color("#333");

        // Borders
        context.fill_rect(
            f64::from(0),
            f64::from(0),
            f64::from(self.store.width + 2),
            f64::from(self.store.height + 2),
        );

        context.set_fill_style_color("#ffe");

        context.fill_rect(
            f64::from(1),
            f64::from(1),
            f64::from(self.store.width),
            f64::from(self.store.height),
        );

        context.set_fill_style_color("#333");

        context.fill_rect(
            f64::from(self.store.item.x),
            f64::from(self.store.item.y),
            f64::from(1),
            f64::from(1),
        );

        //js! {
        //    console.log("canvas repaint" );
        //};

    }
}



struct Animation {
    canvas: Rc<RefCell<Canvas>>,
    time_stamp: f64,
}

impl Animation {
    fn new(canvas: Canvas) {
        let canvas_rc = Rc::new(RefCell::new(canvas));
        let animation = Animation {
            canvas: canvas_rc.clone(),
            time_stamp: 0.0,
        };
        let canvas_for_action = canvas_rc.clone();

        window().add_event_listener(move |e: KeyDownEvent| {
            let mut c = canvas_for_action.borrow_mut();
            match e.key().as_str() {
                "ArrowUp" | "w" | "i" => c.store.move_up(),
                "ArrowRight" | "d" | "l" => c.store.move_right(),
                "ArrowLeft" | "a" | "j" => c.store.move_left(),
                "ArrowDown" | "s" | "k" => c.store.move_down(),
                "p" => {
                    c.store.pause_toggle();
                    return;
                }
                &_ => (),
            }
            if !c.store.playing {
                c.store.pause_toggle()
            }
            c.repaint();
        });

        animation.play(400.0);
    }

    fn play(mut self, time: f64) {
        if time - self.time_stamp > self.canvas.borrow().store.speed {
            self.time_stamp = time;
            let mut c = self.canvas.borrow_mut();
            if c.store.playing && !c.store.game_over {
                c.repaint();
            }
        }

        window().request_animation_frame(|t| {
            self.play(t);
        });
    }
}


fn main() {
    let store = Store::new(30, 20);
    let canvas = Canvas::new("#game", store);
    Animation::new(canvas);
}
