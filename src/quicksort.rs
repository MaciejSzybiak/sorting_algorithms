fn partition(array: &mut [i32]) -> usize {
    let mut left = 0;
    let mut right = array.len() - 1;
    let pivot = array[right];
    loop {
        while array[left] < pivot {
            left += 1;
        }
        while array[right] > pivot {
            right -= 1;
            if right == 0 {
                break;
            }
        }
        if left >= right {
            break;
        }
        array.swap(left, right);
    }
    array.swap(left, right);
    left
}

pub fn sort(mut array: &mut [i32]) {
    let length = array.len();
    
    if length < 2 {
        return;
    }

    let partition_index = partition(&mut array);
    sort(&mut array[..partition_index]);
    sort(&mut array[(partition_index + 1)..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{UNSORTED_ARRAY, SORTED_ARRAY};
    #[test]
    fn test_quicksort() {
        let mut arr = UNSORTED_ARRAY;
        sort(&mut arr);
        assert_eq!(arr, SORTED_ARRAY);
    }
}
