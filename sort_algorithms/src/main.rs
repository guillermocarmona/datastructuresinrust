mod bubble_sort;
mod quick_sort;

use bubble_sort::bubble_sort;
use quick_sort::quick_sort;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    use super::*;
    
    const UNSORTED_ARRAY: [u16; 50] = [
        42, 7, 88, 15, 23, 9, 51, 3, 77, 68,
        2, 44, 31, 56, 84, 10, 65, 28, 19, 90,
        5, 39, 71, 12, 50, 60, 8, 35, 24, 47,
        17, 38, 66, 4, 83, 1, 26, 55, 34, 14,
        49, 20, 6, 58, 80, 29, 13, 53, 41, 11,
    ];

    const SORTED_ARRAY: [u16; 50] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 17, 19, 20, 23, 24, 26, 28,
        29, 31, 34, 35, 38, 39, 41, 42, 44, 47,
        49, 50, 51, 53, 55, 56, 58, 60, 65, 66,
        68, 71, 77, 80, 83, 84, 88, 90
     ];

    #[test]
    fn bubble_test (){
        let result = bubble_sort(UNSORTED_ARRAY.to_vec());

        assert_eq!(result, SORTED_ARRAY.to_vec())
    }

    #[test]
    fn quick_test (){
        let result = quick_sort(UNSORTED_ARRAY.to_vec());

        assert_eq!(result, SORTED_ARRAY.to_vec())
    }
}
