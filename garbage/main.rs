use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use std::{thread, time};
mod observers;
mod sorting_algorithms;

use ggez::*;

use crate::{observers::*, sorting_algorithms::*};
// use sorting_algorithms::*;
// mod sorting_algorithms;

fn main() {
    let thingy = RefCell::new(vec![3, 2, 6, 4]);
    let mut subject = Subject::new();
    let observer_a = ConcreteObserver {
        id: 1,
        arr: &thingy,
    };
    subject.attach(&observer_a);

    quick_sort(thingy.borrow_mut(), &subject);
    println!("{:?}", thingy);
    subject.notify_observers();
    // subject.detach(&observer_a);
}

// fn main() {
//     let mut subject = Subject::new();
//     let observer_a = ConcreteObserver { id: 1 };

//     // let state = State {
//     //     dt: std::time::Duration::new(0, 0),
//     // };
//     // let c = conf::Conf::new();
//     // let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
//     //     .default_conf(c)
//     //     .build()
//     //     .unwrap();
//     // event::run(ctx, event_loop, state);
// }

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello ggez! dt = {}ns", self.dt.as_nanos());
        Ok(())
    }
}

// use crate::ConcreteObserver;
