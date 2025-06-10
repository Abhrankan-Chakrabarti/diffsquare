use diffsquare::factor::difference_of_squares;
use malachite::Integer;

#[test]
fn test_difference_of_squares() {
    let n = Integer::from(5959);
    let expected = (Integer::from(59), Integer::from(101));
    let expected_rev = (Integer::from(101), Integer::from(59));
    let mut iter = Integer::from(1);
    let precision = 6;
    let quiet = true;

    let result = difference_of_squares(&n, &mut iter, precision, quiet);

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
    let quiet = true;

    let result = difference_of_squares(&n, &mut iter, precision, quiet);

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
    let quiet = true;

    let result = difference_of_squares(&n, &mut iter, precision, quiet);

    assert!(result.is_none(), "Expected no factorization for {}", n);
}

#[test]
fn big_numbers() {
    // Each of these is a product of two 31-bit primes
    let numbers: Vec<u64> = vec![
        2761929023323646159,
        3189046231347719467,
        3234246546378360389,
        3869305776707280953,
        3167208188639390813,
        3088042782711408869,
        3628455596280801323,
        2953787574901819241,
        3909561575378030219,
        4357328471891213977,
        2824368080144930999,
        3348680054093203003,
        2704267100962222513,
        2916169237307181179,
        3669851121098875703,
    ];

    for &n_u64 in &numbers {
        let n = Integer::from(n_u64);
        let mut iter = Integer::from(1);
        let precision = 6;
        let quiet = true;

        let result = difference_of_squares(&n, &mut iter, precision, quiet);

        assert!(
            matches!(result, Some((ref p, ref q)) if p * q == n),
            "Failed to factor {} correctly",
            n
        );
    }
}
