use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
pub trait IObserver {
    fn update(&self);
}

pub trait ISubject<'a, T: IObserver> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, observer: &'a T);
    fn notify_observers(&self);
}

pub struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}
impl<'a, T: IObserver + PartialEq> Subject<'a, T> {
    pub fn new() -> Subject<'a, T> {
        Subject {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: IObserver + PartialEq> ISubject<'a, T> for Subject<'a, T> {
    fn attach(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }
    fn detach(&mut self, observer: &'a T) {
        if let Some(idx) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(idx);
        }
    }
    fn notify_observers(&self) {
        for item in self.observers.iter() {
            item.update();
        }
    }
}

#[derive(PartialEq)]
pub struct ConcreteObserver<'a> {
    pub id: i32,
    pub arr: &'a RefCell<Vec<i32>>,
}
impl IObserver for ConcreteObserver<'_> {
    fn update(&self) {
        println!("Observer id:{} received event! {:?}", self.id, self.arr);
    }
}
