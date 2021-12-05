#![feature(test)]

#[cfg(test)]
mod test;
mod bitwise_smallest;
mod bitwise_aligned;
mod array_smallest;
mod array_16bit;
mod array_32bit;
mod array_aligned;
mod array_aligned_16bit;

trait Board: Clone {
    /// Initialize a new Board from a list of numbers
    fn new(numbers: [u8; 25]) -> Self;

    /// Mark a number on this board, optionally returning the answer to the
    /// challenge if this board gets solved.
    ///
    /// This algorithm is O(1) since the board size is constant at 25 spaces.
    fn mark(&mut self, num: u8) -> Option<u32>;
}
