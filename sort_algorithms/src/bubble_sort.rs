pub fn bubble_sort<T: Ord + Clone>(mut arr: Vec<T>) -> Vec<T> {
    let n = arr.len()-1;

    for i in 0..n {
        let mut swapped: bool = false;
        for j in 0..n-i{
            if arr[j] > arr[j+1] {
                let temp = arr[j+1].clone();
                arr[j+1] = arr[j].clone();
                arr[j] = temp;
                swapped = true;
            }
        }
        if !swapped { break } 
    }

    arr
}
