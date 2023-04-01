// cargo-deps: primal = "0.2.3"

use primal::Sieve;

println!("{}", Sieve::new(2_000_000).primes_from(0).sum::<usize>());
