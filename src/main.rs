use crate::euler::*;
use crate::aoc::*;

mod euler;
mod aoc;

fn main() {
    println!("Euler one: {0}", p001::find_multiples_sum());
    println!("Euler two: {0}", p002::sum_of_even_fibonacci_numbers());
    println!("AOC 1:1 {0}", aoc1::compute());
    println!("AOC 1:2 {0}", aoc1::compute2());
    println!("AOC 2: {0}", aoc2::compute());
}
