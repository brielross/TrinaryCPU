pub fn trip_string_to_decimal(trip_string: String) -> u32 {
    let mut total = 0;
    let base: u32 = 3;
    let mut index = 0;
    for trip in trip_string.chars().rev() {
        total += base.pow(index) * (trip as u32 - '0' as u32);
        index += 1;
    }
    return total
} 