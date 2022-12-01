use crate::observers::*;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use std::{thread, time};

pub fn quick_sort(mut slice: RefMut<'_, Vec<i32>>, subject: &Subject<'_, ConcreteObserver>) {
    if !slice.is_empty() {
        let len = slice.len();
        let partition_index = partition(&mut slice[0..len], subject);

        _quick_sort(&mut slice[0..partition_index], subject);
        _quick_sort(&mut slice[partition_index + 1..len], subject);
    }
}

fn _quick_sort(slice: &mut [i32], subject: &Subject<'_, ConcreteObserver>) {
    if !slice.is_empty() {
        let partition_index = partition(slice, subject);
        let len = slice.len();

        _quick_sort(&mut slice[0..partition_index], subject);
        _quick_sort(&mut slice[partition_index + 1..len], subject);
    }
}

fn partition(slice: &mut [i32], subject: &Subject<'_, ConcreteObserver>) -> usize {
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    let t = time::Duration::from_millis(250);
    thread::sleep(t);

    slice.swap(i, len - 1);

    subject.notify_observers();

    i
}

pub fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
    let n = x.len();
    let m = n / 2;

    if n <= 1 {
        return;
    }

    merge_sort(&mut x[0..m]);
    merge_sort(&mut x[m..n]);

    let mut y: Vec<T> = x.to_vec();

    merge(&x[0..m], &x[m..n], &mut y[..]);

    x.copy_from_slice(&y);
}

fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // already sorted
    }

    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}

pub fn gnome_sort<T: Ord>(s: &mut [T]) {
    let mut i = 0;
    while i < s.len() {
        if i == 0 || s[i - 1] < s[i] {
            i += 1;
        } else {
            s.swap(i - 1, i);
            i -= 1;
        }
    }
}
