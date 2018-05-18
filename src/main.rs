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


fn rand_32(max: u32) -> u32 {
    let v = js!(return Math.random());
    let v: f64 = v.try_into().unwrap();
    let v = (v * max as f64).ceil();
    let v: u32 = v as u32;
    v
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Board {
    width: u32,
    height: u32,
}

impl Board {
    fn new(width: u32, height: u32) -> Self {
        Board { width, height }
    }
    fn paint(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style_color("#333");

        // Borders
        context.fill_rect(
            0f64,
            0f64,
            (self.width + 2) as f64,
            (self.height + 2) as f64,
        );

        context.set_fill_style_color("#ffe");

        context.fill_rect(
            1f64,
            1f64,
            (self.width) as f64,
            (self.height) as f64,
        );
    }
}



struct Score {
    score: u32,
}

impl Score {
    fn new() -> Self {
        Score { score: 0 }
    }
    fn up(&mut self) {
        self.score = self.score + 1;
    }
    fn paint(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style_color("#fff");

        context.fill_text("█████", 5., 0., None);

        let score = self.score.to_string();
        context.set_fill_style_color("#333");
        context.fill_text(score.as_str(), 5., 0., None);
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Item {
    x: u32,
    y: u32,
}

impl Item {
    fn new(board: &Board) -> Item {
        let x: u32 = rand_32(board.width);
        let y: u32 = rand_32(board.height);
        Item { x, y }
    }
    fn at_position(x: u32, y: u32) -> Item {
        Item { x, y }
    }
    fn paint(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style_color("#333");
        context.fill_rect(
            f64::from(self.x),
            f64::from(self.y),
            f64::from(1),
            f64::from(1),
        );
    }
}

struct Snake {
    snake: Vec<Item>,
}

impl Snake {
    fn new(board: &Board) -> Snake {
        let mut x: u32 = rand_32(board.width / 2);
        if x < 3 {
            x = 3;
        }
        let x2 = x - 1;
        let x3 = x - 2;
        let max_y = board.height / 2;
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

    fn validate(&self, board: &Board) -> bool {
        let head = self.snake.first().unwrap();
        return head.x >= 1 && head.x <= board.width && head.y >= 1 && head.y <= board.height;
    }

    fn paint(&self, context: &CanvasRenderingContext2d) {
        for item in self.items() {
            context.fill_rect(
                f64::from(item.x),
                f64::from(item.y),
                f64::from(1),
                f64::from(1),
            );
        }

    }
}

struct Store {
    speed: f64,
    play_time_stamp: f64,
    paint_time_stamp: f64,

    board: Board,
    snake: Snake,
    item: Item,
    score: Score,
    playing: bool,
    game_over: bool,
    direction: Direction,
}

impl Store {
    fn new(width: u32, height: u32) -> Store {
        let board = Board::new(width, height);
        let snake = Snake::new(&board);
        let item = Item::new(&board);
        let score = Score::new();
        Store {
            board,
            snake,
            item,
            score,
            speed: 150.0,
            play_time_stamp: 0.0,
            paint_time_stamp: 0.0,
            playing: true,
            game_over: false,
            direction: Direction::Right,
        }
    }
    fn move_up(&mut self) {
        if self.direction == Direction::Down {
            return;
        }
        if self.play_time_stamp >= self.paint_time_stamp {
            return;
        }
        js! {
            console.log("move Up");
        };
        self.play_time_stamp = self.paint_time_stamp;
        self.direction = Direction::Up
    }
    fn move_down(&mut self) {
        if self.direction == Direction::Up {
            return;
        }
        if self.play_time_stamp >= self.paint_time_stamp {
            return;
        }
        js! {
            console.log("move Down");
        };
        self.play_time_stamp = self.paint_time_stamp;
        self.direction = Direction::Down
    }
    fn move_left(&mut self) {
        if self.direction == Direction::Right {
            return;
        }
        if self.play_time_stamp >= self.paint_time_stamp {
            return;
        }
        js! {
            console.log("move Left");
        };
        self.play_time_stamp = self.paint_time_stamp;
        self.direction = Direction::Left
    }
    fn move_right(&mut self) {
        if self.direction == Direction::Left {
            return;
        }
        if self.play_time_stamp >= self.paint_time_stamp {
            return;
        }
        js! {
            console.log("move Right");
        };
        self.play_time_stamp = self.paint_time_stamp;
        self.direction = Direction::Right
    }
    fn pause_toggle(&mut self) {
        self.playing = !self.playing
    }
    fn play(&mut self) {
        match self.snake.move_(self.direction, &self.item) {
            Ok(growing) => {
                if !self.snake.validate(&self.board) {
                    js! {
                        console.log("Snake hit the border");
                    };
                    self.game_over = true;
                }
                if growing {
                    self.item = Item::new(&self.board);
                    self.score.up();
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
    fn paint(&self, context: &CanvasRenderingContext2d) {
        let scaling: f64 = 10.;
        context.set_transform(scaling, 0f64, 0f64, scaling, 0f64, 0f64);

        self.board.paint(&context);
        self.item.paint(&context);
        self.snake.paint(&context);

        let moving_h = ((self.board.width + 2) as f64) * scaling;
        let moving_v = ((self.board.height) as f64) * scaling;
        context.set_transform(2f64, 0f64, 0f64, 2f64, moving_h, moving_v);
        self.score.paint(&context);
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

        let score_width = 10;  // should be in the store.
        let scaling = 10;

        let canvas_width = store.board.width * scaling + score_width * scaling;
        let canvas_height = store.board.height * scaling;

        canvas.set_width(canvas_width + 2 * scaling);
        canvas.set_height(canvas_height + 2 * scaling);

        Canvas { store, canvas }
    }

    fn repaint(&mut self) {
        let context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
        self.store.paint(&context);
    }
}

struct Animation {
    canvas: Rc<RefCell<Canvas>>,
}

impl Animation {
    fn new(canvas: Canvas) {
        let canvas_rc = Rc::new(RefCell::new(canvas));
        let animation = Animation {
            canvas: canvas_rc.clone(),
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
        });

        animation.play(120.0);
    }

    fn play(self, time: f64) {
        if time - self.canvas.borrow().store.paint_time_stamp > self.canvas.borrow().store.speed {
            let mut c = self.canvas.borrow_mut();
            c.store.paint_time_stamp = time;
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
