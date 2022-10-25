import Data.Numbers.Primes (primes)

main :: IO ()
main = print $ primes !! 10000
