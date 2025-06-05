mod binarysearch;
use binarysearch::binary_search;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use super::*;

    const ARRAY: [u16;20] = [
        3, 7, 12, 15, 18, 21, 25, 29, 34, 38,
        42, 47, 51, 55, 59, 63, 68, 72, 77, 81
    ];

    #[test]
    fn binary_correct() {
        let result = binary_search(&ARRAY, &63, 0, ARRAY.len() - 1);
        assert_eq!(result, Some(15)); 
    }

    #[test]
    fn binary_wrong() {
        let result = binary_search(&ARRAY, &19, 0, ARRAY.len() - 1);
        assert_eq!(result, None);
    }
}
