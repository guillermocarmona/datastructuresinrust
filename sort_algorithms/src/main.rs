mod bubble_sort;
mod quick_sort;

use bubble_sort::bubble_sort;
use quick_sort::quick_sort;

use std::time::Instant;

fn main() {
    let array = [
        50, 936, 983, 696, 372, 126, 66, 624, 107, 515, 242, 103, 92, 298, 580, 565, 524, 118, 890,
        202, 101, 935, 585, 358, 33, 333, 434, 105, 756, 374, 241, 508, 529, 483, 693, 444, 931,
        132, 174, 297, 546, 338, 874, 310, 212, 992, 175, 380, 704, 840, 473, 295, 424, 954, 973,
        142, 881, 570, 294, 807, 147, 124, 267, 368, 248, 698, 993, 824, 35, 766, 560, 247, 18,
        314, 384, 587, 723, 872, 996, 14, 868, 428, 787, 168, 719, 227, 361, 194, 908, 650, 8, 571,
        443, 520, 638, 475, 590, 465, 112, 349,
    ];

    let start = Instant::now();

    let bubble_result = bubble_sort(array.to_vec());

    let bubble_time = start.elapsed();

    let start = Instant::now();

    let quick_result = quick_sort(array.to_vec());

    let quick_time = start.elapsed();

    println!("Bubble sort tooks: {:?}.", bubble_time);

    println!("Quick sort tooks: {:?}.", quick_time);

    println!("=========================================");

    println!("Bubble sort result: {:?}", bubble_result);

    println!("Quick sort result: {:?}", quick_result);
}

#[cfg(test)]
mod test {

    use super::*;

    const UNSORTED_ARRAY: [u16; 50] = [
        42, 7, 88, 15, 23, 9, 51, 3, 77, 68, 2, 44, 31, 56, 84, 10, 65, 28, 19, 90, 5, 39, 71, 12,
        50, 60, 8, 35, 24, 47, 17, 38, 66, 4, 83, 1, 26, 55, 34, 14, 49, 20, 6, 58, 80, 29, 13, 53,
        41, 11,
    ];

    const SORTED_ARRAY: [u16; 50] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 19, 20, 23, 24, 26, 28, 29, 31, 34,
        35, 38, 39, 41, 42, 44, 47, 49, 50, 51, 53, 55, 56, 58, 60, 65, 66, 68, 71, 77, 80, 83, 84,
        88, 90,
    ];

    #[test]
    fn bubble_test() {
        let result = bubble_sort(UNSORTED_ARRAY.to_vec());

        assert_eq!(result, SORTED_ARRAY.to_vec())
    }

    #[test]
    fn quick_test() {
        let result = quick_sort(UNSORTED_ARRAY.to_vec());

        assert_eq!(result, SORTED_ARRAY.to_vec())
    }
}
