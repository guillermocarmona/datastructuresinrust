## Binary Search and Interpolation Search Exercise for Rust

This exercise aims to solidify your understanding of binary search and interpolation search algorithms by implementing them in Rust. You'll also analyze their performance.

### Part 1: Implement the Algorithms

Your task is to implement generic versions of binary search and interpolation search.

1.  **Binary Search Implementation:**
    *   Create a function `binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize>`
    *   This function should take a sorted slice `arr` and a `target` element.
    *   It should return `Some(index)` if the `target` is found, where `index` is the position of the `target` in the slice.
    *   It should return `None` if the `target` is not found.

2.  **Interpolation Search Implementation:**
    *   Create a function `interpolation_search<T>(arr: &[T], target: &T) -> Option<usize>`
    *   **Constraint:** For this implementation, assume `T` can be converted to a numerical type (e.g., `i32`, `f64`) for the interpolation formula. You'll need to use `num_traits` or similar to allow for generic numerical operations. A simpler approach for this exercise is to restrict `T` to types that implement `From<usize>` and allow casting to `f64` for the formula. Alternatively, focus on `i32` for a concrete implementation. For a more generic approach, let's assume `T` can be treated as a number that can be converted to `f64` for the interpolation step, and also implements `PartialOrd` and `Sub` (or similar traits for subtraction).

    *   This function should take a sorted slice `arr` and a `target` element.
    *   It should return `Some(index)` if the `target` is found.
    *   It should return `None` if the `target` is not found.

    *   **Interpolation Formula Reminder:**
        The `pos` (potential index) for interpolation search is calculated as:
        $$ pos = lo + \frac{(hi - lo) \times (\text{target} - \text{arr}[lo])}{\text{arr}[hi] - \text{arr}[lo]} $$
        Where `lo` and `hi` are the lower and upper bounds of the search space.
        You'll need to handle potential division by zero if `arr[hi] == arr[lo]`.

### Part 2: Testing

Write unit tests for both implementations.

1.  **Test Cases:**
    *   An empty array.
    *   An array with a single element (found and not found).
    *   An array with an even number of elements.
    *   An array with an odd number of elements.
    *   Target element at the beginning, middle, and end of the array.
    *   Target element not present in the array (smaller than min, larger than max, in between elements).
    *   Arrays with duplicate elements (ensure it finds *an* occurrence, not necessarily the first).

### Part 3: Performance Analysis

Conduct a simple performance comparison between your two implementations.

1.  **Setup:**
    *   Generate a large, sorted array of random numbers (e.g., 100,000 to 1,000,000 `i32`s).
    *   Generate a set of target elements to search for. Include elements that are present in the array and elements that are not.

2.  **Benchmarking (Approximate):**
    *   For each algorithm, measure the time it takes to search for all your target elements.
    *   Use `std::time::Instant` to measure execution time.

3.  **Analysis:**
    *   Based on your results, which algorithm performed better?
    *   Under what conditions do you expect one algorithm to outperform the other? (Think about the distribution of elements in the array and the target value.)
    *   Describe the time complexity of both algorithms in the best, average, and worst-case scenarios.

### Submission Guidelines

*   Provide all your Rust code in a single file (e.g., `search_exercise.rs`).
*   Include clear comments in your code.
*   Provide your performance analysis and answers to the questions in a separate section or as comments in the Rust file.

Good luck!
