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
