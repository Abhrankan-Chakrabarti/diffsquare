use diffsquare::legendre::LEGENDRE_TABLES;

#[test]
fn test_legendre_table_lengths() {
    for &(p, ref table) in LEGENDRE_TABLES.iter() {
        assert_eq!(
            table.len(),
            p as usize,
            "Incorrect table length for p = {}",
            p
        );
    }
}

#[test]
fn test_legendre_quadratic_residue() {
    for &(p, ref table) in LEGENDRE_TABLES.iter() {
        for a in 1..p {
            let residue = (a * a) % p;
            let chi = table[residue as usize];
            assert_eq!(
                chi, 1,
                "a = {}, a^2 mod {} = {} should be quadratic residue (χ = {})",
                a, p, residue, chi
            );
        }
    }
}
