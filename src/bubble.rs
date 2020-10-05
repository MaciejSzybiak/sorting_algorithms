pub fn sort(array: &mut [i32]) {
    let mut swapped;
    let length = array.len();

    for x in 0..length {
        swapped = false;
        for y in 0..length - x - 1 {
            if array[y] > array[y + 1] {
                array.swap(y, y + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{UNSORTED_ARRAY, SORTED_ARRAY};
    #[test]
    fn test_bubble_sort() {
        let mut arr = UNSORTED_ARRAY;
        sort(&mut arr);
        assert_eq!(arr, SORTED_ARRAY);
    }
}
