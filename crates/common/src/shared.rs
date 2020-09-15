/// Bounds a number between 0 and 100.
pub fn bound_to_positive_hundred(number: i8) -> i8 {
    if number < 0 {
        0
    } else if number > 100 {
        100
    } else {
        number
    }
}