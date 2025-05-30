use malachite::Integer;
use diffsquare::utils::sqr_perf;

#[test]
fn test_perfect_squares() {
    let squares = [0u64, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
    for &n in &squares {
        assert!(sqr_perf(&Integer::from(n)), "Expected {} to be a perfect square", n);
    }
}

#[test]
fn test_non_perfect_squares() {
    let non_squares = [2u64, 3, 5, 6, 7, 8, 10, 11, 12, 14, 15];
    for &n in &non_squares {
        assert!(!sqr_perf(&Integer::from(n)), "Expected {} to not be a perfect square", n);
    }
}
