/// There are five kinds of ranges in Rust:
///
///      1..5: A (half-open) range. It includes all numbers from 1 to 4. It doesn't include the last value, 5.
///      1..=5: An inclusive range. It includes all numbers from 1 to 5. It includes the last value, 5.
///      1..: An open-ended range. It includes all numbers from 1 to infinity (well, until the maximum value of the integer type).
///      ..5: A range that starts at the minimum value for the integer type and ends at 4. It doesn't include the last value, 5.
///      ..=5: A range that starts at the minimum value for the integer type and ends at 5. It includes the last value, 5.
pub fn main() {
for i in 0..=5 {
        println!("iter {}", i)
    }
}
