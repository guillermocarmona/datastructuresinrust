//TODO: Add partition method

pub fn quick_sort<T: Ord + Clone>(mut arr: Vec<T>) -> Vec<T> {
    let n = arr.len();
    recursion_quick_sort(&mut arr, 0, n-1)
}

fn recursion_quick_sort<T: Ord + Clone>(mut arr: &mut Vec<T>, low: usize, high: usize) -> Vec<T> {
    if low >= high { return arr.to_vec() }

    let pivot = arr[high];

    let mut i = low.clone();

    for j in low..high {
        if arr[j] < pivot {
            i = i + 1;
            arr.swap(i, j);
        }
    }

    arr = recursion_quick_sort(&mut arr, low, i);
    arr = recursion_quick_sort(&mut arr, i+1, high);
    
    arr.to_vec()
}
