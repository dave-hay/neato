pub fn bubble(arr: &mut Vec<usize>) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn insertion(arr: &mut Vec<usize>) {
    let len = arr.len();
    for i in 1..len {
        for j in 0..i {
            if arr[i] < arr[j] {
                arr.swap(j, i);
            }
        }
    }
}

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

pub fn quick(arr: &mut Vec<usize>) {
    fn partition(arr: &mut Vec<usize>, low: usize, hi: usize) -> usize {
        // right most el
        let pivot = arr[hi];

        // greater els
        let mut i = low;

        for j in low..hi {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, hi);
        i
    }
    fn qs(arr: &mut Vec<usize>, low: usize, hi: usize) {
        if low < hi {
            let pi = partition(arr, low, hi);

            // usize negative issue
            if pi > 0 {
                qs(arr, low, pi - 1);
            }
            qs(arr, pi + 1, hi);
        }
    }

    qs(arr, 0, arr.len() - 1);
}

pub fn mergesort(nums: &mut Vec<usize>) {
    fn sort(nums: &mut Vec<usize>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;
        sort(nums, left, mid);
        sort(nums, mid + 1, right);
        merge(nums, left, mid, right);
    }

    fn merge(nums: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
        let mut tmp = Vec::with_capacity(right - left + 1);
        let mut l_index = left;
        let mut r_index = mid + 1;

        while l_index <= mid && r_index <= right {
            if nums[l_index] < nums[r_index] {
                tmp.push(nums[l_index]);
                l_index += 1;
            } else {
                tmp.push(nums[r_index]);
                r_index += 1;
            }
        }

        while l_index <= mid {
            tmp.push(nums[l_index]);
            l_index += 1;
        }

        while r_index <= right {
            tmp.push(nums[r_index]);
            r_index += 1;
        }

        for i in 0..tmp.len() {
            nums[left + i] = tmp[i]
        }
    }

    sort(nums, 0, nums.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut v: Vec<usize> = vec![4, 1, 23, 0, 4, 3, 11, 49];
        bubble(&mut v);
        assert_eq!(v, [0, 1, 3, 4, 4, 11, 23, 49]);
    }

    #[test]
    fn test_insertion() {
        let mut v: Vec<usize> = vec![4, 1, 23, 0, 4, 3, 11, 49];
        insertion(&mut v);
        assert_eq!(v, [0, 1, 3, 4, 4, 11, 23, 49]);
    }

    #[test]
    fn test_selection() {
        let mut v: Vec<usize> = vec![4, 1, 23, 0, 4, 3, 11, 49];
        selection(&mut v);
        assert_eq!(v, [0, 1, 3, 4, 4, 11, 23, 49]);
    }

    #[test]
    fn test_quick() {
        let mut v: Vec<usize> = vec![4, 1, 23, 0, 4, 3, 11, 49];
        quick(&mut v);
        assert_eq!(v, [0, 1, 3, 4, 4, 11, 23, 49]);
    }

    #[test]
    fn test_mergesort() {
        let mut v: Vec<usize> = vec![4, 1, 23, 0, 4, 3, 11, 49];
        mergesort(&mut v);
        assert_eq!(v, [0, 1, 3, 4, 4, 11, 23, 49]);
    }
}
