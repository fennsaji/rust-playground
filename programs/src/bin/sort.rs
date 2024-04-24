fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in i..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort() {
        let mut arr = vec![4, 6, 3, 1, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 6]);

        let mut arr = vec![4, 6, 3, 1, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 6]);
    }
}