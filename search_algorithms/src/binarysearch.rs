pub fn binary_search<T: Ord> (arr: &[T], target: &T, low:usize, high: usize) -> Option<usize> {

    if low > high { return None }
    
    let middle = (high +  low) / 2;
    let mid = &arr[middle];
    

    if mid == target { return Some(middle) }

    else if target < mid { return binary_search(arr, target, low, middle - 1) } 

    else { return binary_search(arr, target, middle+1, high) }
}
