use std::cmp;

pub fn find<A: AsRef<[T]> , T:Ord>(array: A, key: T) -> Option<usize>{
    let array = array.as_ref();
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

pub fn find_iter<A: AsRef<[T]> , T:Ord>(array: A, key: T) -> Option<usize>{
    let mut array = array.as_ref();
    if array.len() == 0 || key < array[0] || key > array[array.len() - 1] {
        return None;
    }

    let mut found:bool = false;
    let mut res:usize = 0;


    while !found && array.len()!=0 {

        let middle = array.len()/2;

        match key.cmp(&array[middle]) {
            cmp::Ordering::Less => {
                array = &array[..middle];
            },
            cmp::Ordering::Greater => {
                array = &array[middle..];
                res += middle;
            }
            cmp::Ordering::Equal => {
                res += middle;
                found = true;
            },
        }
    }

    if found {
        Some(res)
    }else{
        None
    }


}


