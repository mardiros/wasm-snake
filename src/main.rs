#[macro_use]
extern crate stdweb;

use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

#[derive(Clone, Copy, PartialEq)]
struct Item {
    x: u32,
    y: u32,
}

struct Snake {
    snake: Vec<Item>,
}

fn rand_32(max: u32) -> u32 {
    let v = js!(return Math.random());
    let v: f64 = v.try_into().unwrap();
    let v = (v * max as f64).ceil();
    let v: u32 = v as u32;
    v
}

impl Snake {
    fn new(max_x: u32, max_y: u32) -> Snake {
        let mut x: u32 = rand_32(max_x / 2);
        if x < 3 {
            x = 3;
        }
        let x2 = x - 1;
        let x3 = x - 2;
        let max_y = max_y / 2;
        let y: u32 = rand_32(max_y) + max_y - 1;
        let snake: Vec<Item> = vec![Item { x, y }, Item { x: x2, y }, Item { x: x3, y }];
        Snake { snake }
    }

    fn items(&self) -> &[Item] {
        &self.snake.as_slice()
    }

    fn grow(&mut self, direction: Direction) {
        let new_item = {
            let item = { self.snake.first().unwrap() };
            match direction {
                Direction::Up => Item::at_position(item.x, item.y - 1),
                Direction::Down => Item::at_position(item.x, item.y + 1),
                Direction::Left => Item::at_position(item.x - 1, item.y),
                Direction::Right => Item::at_position(item.x + 1, item.y),
            }
        };
        self.snake.insert(0, new_item);
    }

    fn move_(&mut self, direction: Direction, item: &Item) -> Result<bool, ()> {
        let snake_head = {
            self.snake.pop().unwrap();
            let item = { self.snake.first().unwrap() };
            match direction {
                Direction::Up => Item::at_position(item.x, item.y - 1),
                Direction::Down => Item::at_position(item.x, item.y + 1),
                Direction::Left => Item::at_position(item.x - 1, item.y),
                Direction::Right => Item::at_position(item.x + 1, item.y),
            }
        };
        let growing = &snake_head == item;
        if growing {
            self.grow(direction);
        } else if self.contains(&snake_head) {
            return Err(());
        }
        self.snake.insert(0, snake_head);
        Ok(growing)
    }

    fn contains(&self, item: &Item) -> bool {
        self.snake.contains(item)
    }

    fn validate(&self, width: u32, height: u32) -> bool {
        let head = self.snake.first().unwrap();
        return head.x >= 1 && head.x <= width && head.y >= 1 && head.y <= height;
    }
}

impl Item {
    fn new(max_x: u32, max_y: u32) -> Item {
        let x: u32 = rand_32(max_x);
        let y: u32 = rand_32(max_y);
        Item { x, y }
    }
    fn at_position(x: u32, y: u32) -> Item {
        Item { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Store {
    width: u32,
    height: u32,
    speed: f64,
    snake: Snake,
    item: Item,
    playing: bool,
    game_over: bool,
    direction: Direction,
}

impl Store {
    fn new(width: u32, height: u32) -> Store {
        let snake = Snake::new(width, height);
        let item = Item::new(width, height);
        Store {
            width,
            height,
            item,
            snake,
            speed: 150.0,
            playing: true,
            game_over: false,
            direction: Direction::Right,
        }
    }
    fn move_up(&mut self) {
        if self.direction == Direction::Down {
            return;
        }
        js! {
            console.log("move Up");
        };
        self.direction = Direction::Up
    }
    fn move_down(&mut self) {
        if self.direction == Direction::Up {
            return;
        }
        js! {
            console.log("move Down");
        };
        self.direction = Direction::Down
    }
    fn move_left(&mut self) {
        if self.direction == Direction::Right {
            return;
        }
        js! {
            console.log("move Left");
        };
        self.direction = Direction::Left
    }
    fn move_right(&mut self) {
        if self.direction == Direction::Left {
            return;
        }
        js! {
            console.log("move Right");
        };
        self.direction = Direction::Right
    }
    fn pause_toggle(&mut self) {
        self.playing = !self.playing
    }
    fn play(&mut self) {
        match self.snake.move_(self.direction, &self.item) {
            Ok(growing) => {
                if !self.snake.validate(self.width, self.height) {
                    js! {
                        console.log("Snake hit the border");
                    };
                    self.game_over = true;
                }
                if growing {
                    self.item = Item::new(self.width, self.height);
                }
            }
            Err(_) => {
                js! {
                    console.log("Snake bite its queue");
                };
                self.game_over = true;
            }
        }
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
            console.log("canvas initialized" );
        };
        Canvas { store, canvas }
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

        for item in self.store.snake.items() {
            context.fill_rect(
                f64::from(item.x),
                f64::from(item.y),
                f64::from(1),
                f64::from(1),
            );
        }
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
                c.store.play();
                c.repaint();
            }
        }

        window().request_animation_frame(|t| {
            self.play(t);
        });
    }
}

fn main() {
    let store = Store::new(40, 25);
    let canvas = Canvas::new("#game", store);
    Animation::new(canvas);
}
