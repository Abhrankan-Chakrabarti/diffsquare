use malachite::Integer;
use malachite::base::num::basic::traits::Zero;

/// Efficiently checks if an integer is a perfect square using quadratic residues.
pub fn sqr_perf(n: &Integer) -> bool {
    static PRIMES: [u64; 10] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    
    for &p in PRIMES.iter() {
        let q = n % Integer::from(p);
        if q == Integer::ZERO {
            continue;
        }
        let mut is_non_residue = true;
        for j in 1..p {
            if Integer::from((j * j) % p) == q {
                is_non_residue = false;
                break;
            }
        }
        if is_non_residue {
            return false;
        }
    }
    true
}
