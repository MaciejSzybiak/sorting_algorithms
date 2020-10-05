fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut l = 0;
    let mut r = 0;
    let mut array: Vec<i32> = Vec::new();
    
    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            array.push(left[l]);
            l += 1;
        } else {
            array.push(right[r]);
            r += 1;
        }
    }
    if l < left.len() {
        array.extend_from_slice(&left[l..]);
    }
    if r < right.len() {
        array.extend_from_slice(&right[r..]);
    }
    array
}

pub fn sort_top_down(array: &mut [i32]) {
    let middle = array.len() / 2;
    if middle == 0 {
        return;
    }

    sort_top_down(&mut array[..middle]);
    sort_top_down(&mut array[middle..]);

    //let mut intermediate = array.to_vec();
    let merged = merge(&array[..middle], &array[middle..]);
    array.copy_from_slice(&merged);
}

#[cfg(test)]
mod tests {
    use super::*;
    use sorting_algorithms::{UNSORTED_ARRAY, SORTED_ARRAY};
    #[test]
    fn test_merge_top_down_sort() {
        let mut arr = UNSORTED_ARRAY;
        sort_top_down(&mut arr);
        assert_eq!(arr, SORTED_ARRAY);
    }
}
