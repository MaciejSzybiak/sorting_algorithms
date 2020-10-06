fn sift_down(slice: &mut [i32]) {
    let mut root = 0;
    let length = slice.len();

    loop {
        let mut child = root * 2 + 1;
        if child > length - 1 {
            break;
        }
        if child + 1 <= length - 1 && slice[child] < slice[child + 1] {
            child += 1;
        }

        if slice[root] < slice[child] {
            slice.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

fn heapify(array: &mut [i32]) {
    let end = array.len() - 1;

    for n in (0..(end / 2)).rev() {
        sift_down(&mut array[n..end]);
    }
}

pub fn sort(mut array: &mut [i32]) {
    heapify(&mut array);

    for end in (1..array.len()).rev() {
        array.swap(end, 0);
        sift_down(&mut array[0..end]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{UNSORTED_ARRAY, SORTED_ARRAY};
    #[test]
    fn test_heap_sort() {
        let mut arr = UNSORTED_ARRAY;
        sort(&mut arr);
        assert_eq!(arr, SORTED_ARRAY);
    }
}
