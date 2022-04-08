use std::cmp;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 || key < array[0] || key > array[array.len() - 1] {
        return None;
    }

    let middle = array.len() / 2;

    match key.cmp(&array[middle]) {
        cmp::Ordering::Less => find(&array[..middle], key),
        cmp::Ordering::Greater => match find(&array[middle..], key) {
            Some(x) => Some(middle + x),
            None => None,
        },
        cmp::Ordering::Equal => Some(middle),
    }
}
