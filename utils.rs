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

pub fn trip_string_is_zero(trip_string: String) -> bool {
    for trip in trip_string.chars() {
        if trip != '0' {
            return false
        }
    }
    return true
}

pub fn substring(trip_string: &String, start_index: usize, end_index: usize) -> String {
    let slice = &trip_string.as_str()[start_index..end_index];
    return slice.to_string();
}