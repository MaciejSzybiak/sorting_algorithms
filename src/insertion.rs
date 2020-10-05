pub fn sort(array: &mut [i32]) {
    let length = array.len();
    let mut j: usize;
    
    for i in 1..length {
        j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{UNSORTED_ARRAY, SORTED_ARRAY};
    #[test]
    fn test_insertion_sort() {
        let mut arr = UNSORTED_ARRAY;
        sort(&mut arr);
        assert_eq!(arr, SORTED_ARRAY);
    }
}
