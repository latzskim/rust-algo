pub fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[arr.len() - 1];
    let mut left = 0;
    let mut right = arr.len() - 1;

    loop {
        while left < arr.len() && arr[left] < pivot {
            left += 1;
        }
        right -= 1; // This prevent from infinite loop when all elements are the same.
        while right > 0 && arr[right] > pivot {
            right -= 1;
        }
        if left >= right {
            break;
        }
        arr.swap(left, right);
    }
    arr.swap(left, arr.len() - 1);
    quicksort(&mut arr[..left]);
    quicksort(&mut arr[left + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty_array() {
        let mut arr: [i32; 0] = [];
        quicksort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn sort_single_element_array() {
        let mut arr = [1];
        quicksort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn sort_two_element_array() {
        let mut arr = [2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2]);
    }

    #[test]
    fn sort_n_element_array() {
        let mut arr = [5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_the_same_element_array() {
        let mut arr = [1, 1, 1, 1, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 1, 1, 1, 1]);
    }

    #[test]
    fn sort_sorted_array() {
        let mut arr = [1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_one_different_element() {
        let mut arr = [1, 1, 1, 4, 1, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 1, 1, 1, 1, 4]);
    }
}
