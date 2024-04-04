use voracious_radix_sort::RadixSort;

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);
    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

pub fn bubblesort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn radix_sor(arr: &mut Vec<i32>) {
    arr.voracious_sort()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut arr = [3, 2, 1, 5, 4];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubblesort() {
        let mut arr = [3, 2, 1, 5, 4];
        bubblesort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_radix_sort() {
        let mut arr = vec![3, 2, 1, 5, 4];
        radix_sor(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
