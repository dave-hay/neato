pub mod sort {
    pub fn selection(arr: &mut Vec<usize>) {
        let mut i = 0;

        while i < arr.len() - 1 {
            let mut min = i;
            let mut j = i + 1;

            while j < arr.len() {
                if arr[j] < arr[min] {
                    min = j
                }
                j += 1;
            }

            arr.swap(i, min);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection() {
        let mut v: Vec<usize> = vec![4, 1, 23, 0, 4, 3, 11, 49];
        sort::selection(&mut v);
        assert_eq!(v, [0, 1, 3, 4, 4, 11, 23, 49]);
    }
}
