import Data.Numbers.Primes (primes)

main :: IO ()
main = print $ sum $ takeWhile (<= 2 * 10^6) $ primes
