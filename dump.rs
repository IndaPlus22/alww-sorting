use std::collections::LinkedList;

use ggez::*;
use sorting_algorithms::*;
mod sorting_algorithms;
use rand::Rng;
const GRID_SIZE: (i16, i16) = (100, 100);
const GRID_CELL_SIZE: (i16, i16) = (10, 10);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
const DESIRED_FPS: u32 = 8;

struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}
impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

struct Stack {
    pos: GridPosition,
}

impl Stack {
    pub fn new(pos: GridPosition) -> Self {
        Stack { pos }
    }
}

struct SelectionSort {
    pos: GridPosition,
    list: LinkedList<Stack>,
}

impl SelectionSort {
    pub fn new(pos: GridPosition) -> Self {
        SelectionSort { pos, LinkedList::new }
    }
    fn update(&mut self) {
        self.pos = GridPosition::from(self.pos)
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("Sort", "ALWW")
        .window_setup(ggez::conf::WindowSetup::default().title("Sort!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let mut rng = rand::thread_rng();
    let mut state = State {
        random_arr: (0..100).map(|_| rng.gen_range(0..100)).collect(),
        sorted_arr: vec![0],
        counter: 0,
    };
    state.sorted_arr = (&state.random_arr).to_owned();
    state.sorted_arr.sort();

    event::run(ctx, events_loop, state);
}

struct State {
    random_arr: Vec<usize>,
    sorted_arr: Vec<usize>,
    counter: usize,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
            if self.random_arr != self.sorted_arr {
                selection_sort(&mut self.random_arr, self.counter);
            }
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);

        canvas.finish(ctx)?;

        ggez::timer::yield_now();

        let mut icounter = 0;
        for i in &self.random_arr {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new_i32(
                        icounter,
                        i.to_owned() as i32,
                        GRID_CELL_SIZE as i32,
                        SCREEN_SIZE.1 as i32 - start_y_reversed, // making em rectangles with colour all the way from bottom to value
                    ))
                    .color(Color::from_rgb_u32(0xDe3163)), //den är cerise, inte rosa
            );
            icounter += 1;
        }

        Ok(())
    }
}
