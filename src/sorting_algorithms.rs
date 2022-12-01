pub fn selection_sort(arr: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut rvec: Vec<Vec<isize>> = vec![];
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
        rvec.push(arr.to_vec());
    }
    rvec
}

pub fn insertion_sort(arr: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut rvec: Vec<Vec<isize>> = vec![];
    rvec.push(arr.to_vec());
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            rvec.push(arr.to_vec());
            j = j - 1;
        }
    }
    rvec
}

//I could not for the life of me get it to work so this is jblomlof solution
pub fn merge_sort(_list: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut return_vec: Vec<Vec<isize>> = vec![];
    return_vec.push(_list.to_vec());
    return_vec.extend(merge_sort_l(_list));
    return_vec
}

fn merge_sort_l(_list: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut return_vec: Vec<Vec<isize>> = vec![];

    if _list.len() != 1 {
        let n = _list.len() / 2;
        let mut second_half = _list.split_off(n);
        let result = merge_sort_l(_list);
        for mut i in result {
            i.extend(second_half.as_slice());
            return_vec.push(i.to_vec());
        }
        let result2 = merge_sort_l(&mut second_half);
        for i in result2 {
            let mut temp_vec: Vec<isize> = vec![];
            temp_vec.extend(_list.as_slice());
            temp_vec.extend(i.as_slice());
            return_vec.push(temp_vec.to_vec());
        }

        let temp = merge(&_list, &second_half);
        *_list = temp.to_vec();
        return_vec.push(_list.to_vec());
    }
    return_vec
}

pub fn merge(_list_a: &Vec<isize>, _list_b: &Vec<isize>) -> Vec<isize> {
    let mut left_counter = 0;
    let mut right_counter = 0;
    let mut return_vec = vec![];
    while (left_counter < _list_a.len()) && (right_counter < _list_b.len()) {
        return_vec.push(
            if _list_a.get(left_counter).unwrap() < _list_b.get(right_counter).unwrap() {
                left_counter += 1;
                *_list_a.get(left_counter - 1).unwrap()
            } else {
                right_counter += 1;
                *_list_b.get(right_counter - 1).unwrap()
            },
        )
    }
    for i in left_counter.._list_a.len() {
        return_vec.push(*_list_a.get(i).unwrap());
    }
    for i in right_counter.._list_b.len() {
        return_vec.push(*_list_b.get(i).unwrap());
    }
    return_vec
}

pub fn pancake_sort(arr: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut rvec: Vec<Vec<isize>> = vec![];
    rvec.push(arr.to_vec());
    let len = arr.len();
    if len < 2 {
        return rvec;
    }
    for i in (0..len).rev() {
        // find index of the maximum element within `arr[0..i]` (inclusive)
        let max_index = arr
            .iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();

        if max_index != i {
            flip(arr, max_index);
            rvec.push(arr.to_vec());
            flip(arr, i);
            rvec.push(arr.to_vec());
        }
    }
    rvec
}

fn flip<E: PartialOrd>(arr: &mut [E], num: usize) {
    arr[0..num + 1].reverse();
}
