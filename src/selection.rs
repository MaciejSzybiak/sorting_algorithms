pub fn sort(array: &mut [i32]) {
    let length = array.len();

    for i in 0..(length - 1) {
        let mut min_index = i;

        for j in (i + 1)..length {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            array.swap(min_index, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sorting_algorithms::{UNSORTED_ARRAY, SORTED_ARRAY};
    #[test]
    fn test_selection_sort() {
        let mut arr = UNSORTED_ARRAY;
        sort(&mut arr);
        assert_eq!(arr, SORTED_ARRAY);
    }
}
