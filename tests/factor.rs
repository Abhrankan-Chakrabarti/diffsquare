use malachite::Integer;
use diffsquare::factor::difference_of_squares;

#[test]
fn test_difference_of_squares() {
    let n = Integer::from(5959);
    let expected = (Integer::from(59), Integer::from(101));
    let expected_rev = (Integer::from(101), Integer::from(59));
    let mut iter = Integer::from(1);
    let precision = 6;

    let result = difference_of_squares(&n, &mut iter, precision);

    assert!(
        result == Some(expected) || result == Some(expected_rev),
        "Unexpected factors for {}",
        n
    );
}

#[test]
fn test_difference_of_squares_prime() {
    let n = Integer::from(101);
    let mut iter = Integer::from(1);
    let precision = 6;

    let result = difference_of_squares(&n, &mut iter, precision);

    assert!(
        result.is_none(),
        "Expected no factorization for prime {}",
        n
    );
}

#[test]
fn test_difference_of_squares_one() {
    let n = Integer::from(1);
    let mut iter = Integer::from(1);
    let precision = 6;

    let result = difference_of_squares(&n, &mut iter, precision);

    assert!(
        result.is_none(),
        "Expected no factorization for {}",
        n
    );
}
