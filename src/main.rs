use ggez::{
    event::{self, EventHandler},
    graphics, Context, GameResult,
};
mod sorting_algorithms;
use rand::Rng;
use sorting_algorithms::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 85;
const GRID_CELL_SIZE: usize = 20;

const SCREEN_SIZE: (f32, f32) = (
    GRID_WIDTH as f32 * GRID_CELL_SIZE as f32,
    GRID_HEIGHT as f32 * GRID_CELL_SIZE as f32,
);

fn main() -> GameResult {
    //Inspired™ by https://github.com/ggez/ggez/blob/master/examples/04_snake.rs
    let (ctx, events_loop) = ggez::ContextBuilder::new("Sort", "ALWW")
        .window_setup(ggez::conf::WindowSetup::default().title("Sort"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let app = AppState::new();

    // Run!
    event::run(ctx, events_loop, app);
}

struct AppState {
    // Your state here...
    fps: u32,
    counter: usize,
    sort_step: usize,
    stack: usize,
    hight: isize,
    sort_steps: [Vec<Vec<isize>>; 4],
}

impl AppState {
    /// Initialize new shuffled grid.
    pub fn new() -> AppState {
        let mut rng = rand::thread_rng();

        let insert_sort_steps =
            insertion_sort(&mut ((0..100).map(|_| rng.gen_range(-100..100)).collect()));
        let selection_sort_steps =
            selection_sort(&mut ((0..100).map(|_| rng.gen_range(0..100)).collect()));
        let merge_sort_steps = merge_sort(&mut ((0..100).map(|_| rng.gen_range(0..100)).collect()));
        let pancake_sort_steps =
            pancake_sort(&mut ((0..100).map(|_| rng.gen_range(0..100)).collect()));

        AppState {
            fps: 120,
            stack: 0,
            hight: 0,
            counter: 0,
            sort_step: 0,
            sort_steps: [
                insert_sort_steps,
                selection_sort_steps,
                merge_sort_steps,
                pancake_sort_steps,
            ],
        }
    }
    pub fn calc(&mut self) {
        let current_value = self.sort_steps[self.counter]
            .get(self.sort_step)
            .unwrap()
            .get(self.stack)
            .unwrap();
        let max = self.sort_steps[self.counter]
            .last()
            .unwrap()
            .to_owned()
            .last()
            .unwrap()
            .to_owned();
        let min = self.sort_steps[self.counter]
            .last()
            .unwrap()
            .to_owned()
            .first()
            .unwrap()
            .to_owned();
        let span = (max - min);
        let win = (GRID_HEIGHT * GRID_CELL_SIZE) as isize;
        let tmp = win / span;
        // as f32 * 0.89) as isize
        self.hight = (current_value + (min.abs())) * tmp;
    }
}

impl EventHandler for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut rng = rand::thread_rng();
        while ctx.time.check_update_time(self.fps) {
            if self.sort_step > self.sort_steps[self.counter].len() {
                self.counter += 1;
                self.sort_step = 0;
                if (self.counter == self.sort_steps.len()) {
                    self.counter = 0;
                    self.fps = 120;

                    self.sort_steps[0] =
                        insertion_sort(&mut ((0..100).map(|_| rng.gen_range(0..100)).collect()));
                    self.sort_steps[1] =
                        selection_sort(&mut ((0..100).map(|_| rng.gen_range(0..100)).collect()));
                    self.sort_steps[2] =
                        merge_sort(&mut ((0..100).map(|_| rng.gen_range(0..100)).collect()));
                    self.sort_steps[3] =
                        pancake_sort(&mut ((0..100).map(|_| rng.gen_range(0..100)).collect()));
                } else {
                    self.fps = 20;
                }
            } else {
                self.sort_step += 1;
            }
        }
        Ok(())
    }

    //Inspired™™ by https://github.com/ggez/ggez/blob/master/examples/04_snake.rs
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        if self.sort_step < self.sort_steps[self.counter].len() {
            for stack in 0..GRID_WIDTH {
                self.stack = stack;
                self.calc();
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            stack as i32 * GRID_CELL_SIZE as i32,
                            0,
                            GRID_CELL_SIZE as i32,
                            self.hight as i32,
                        ))
                        .color(graphics::Color::BLACK),
                )
            }
            canvas.finish(ctx)?;
            ggez::timer::yield_now();
        }
        Ok(())
    }
}
