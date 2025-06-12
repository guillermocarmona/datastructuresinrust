pub fn quick_sort<T: Ord + Clone>(arr: Vec<T>) -> Vec<T> {
    let mut new_arr = arr.clone();

    let n = new_arr.len().saturating_sub(1);

    quick_recursive(&mut new_arr, 0, n);

    new_arr
}

fn quick_recursive<T: Ord + Clone>(arr: &mut Vec<T>, low: usize, high: usize) {
    if low >= high {
        return;
    }

    let index = partition(arr, low, high);

    quick_recursive(arr, low, index.saturating_sub(1));
    quick_recursive(arr, index + 1, high);
}

fn partition<T: Ord + Clone>(arr: &mut Vec<T>, low: usize, high: usize) -> usize {
    let pivot = arr[high].clone();

    let mut index: usize = low;

    for i in low..high {
        if &arr[i] < &pivot {
            arr.swap(i, index);
            index += 1;
        }
    }

    arr.swap(index, high);

    index
}
