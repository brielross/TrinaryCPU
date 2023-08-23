pub struct ALU {
    pub input_1: String,
    pub input_2: String,
    pub operation: String
}

impl ALU {
    fn add_trip_strings(&self) -> String {
        // Reverse the input strings
        let rev_in_1 = self.input_1.chars().rev().collect::<Vec<char>>();
        let mut rev_in_2 = self.input_2.chars().rev().collect::<Vec<char>>();
        
        // Loop through each index and add and adjust carry
        let mut carry = 0;
        let mut result = String::from("");
        for it in rev_in_1.iter().zip(rev_in_2.iter_mut()) {
            let (in_trip_1, in_trip_2) = it;
            let in_value_1 = *in_trip_1 as u32 - '0' as u32;
            let in_value_2 = *in_trip_2 as u32 - '0' as u32;
            let mut trip_sum = in_value_1 + in_value_2 + carry;
            if trip_sum > 2 {
                carry = 1;
                trip_sum = trip_sum - 3;
            } else {
                carry = 0;
            }
            result = format!("{}{}", trip_sum, result);
        }

        return String::from(result)
    }

    fn sub_trip_strings(&self) -> String {
        // Reverse the input strings
        let rev_in_1 = self.input_1.chars().rev().collect::<Vec<char>>();
        let mut rev_in_2 = self.input_2.chars().rev().collect::<Vec<char>>();
        
        // Loop through each index and subtract and adjust carry
        let mut carry = 0;
        let mut result = String::from("");
        for it in rev_in_1.iter().zip(rev_in_2.iter_mut()) {
            let (in_trip_1, in_trip_2) = it;
            let in_value_1 = *in_trip_1 as u32 - '0' as u32;
            let in_value_2 = *in_trip_2 as u32 - '0' as u32;
            let trip_sum: u32;
            if in_value_1 >= in_value_2 + carry {
                trip_sum = in_value_1 - in_value_2 - carry;
                carry = 0;
            } else {
                trip_sum = in_value_1 + 3 - in_value_2 - carry;
                carry = 1;
            };
            result = format!("{}{}", trip_sum, result);
        }

        return String::from(result)
    }

    fn check_less_than(&self) -> String {
        // Reverse the input strings
        let rev_in_1 = self.input_1.chars().rev().collect::<Vec<char>>();
        let mut rev_in_2 = self.input_2.chars().rev().collect::<Vec<char>>();
        
        // Loop through each index and subtract and adjust carry
        let mut carry = 0;
        let mut result = String::from("");
        for it in rev_in_1.iter().zip(rev_in_2.iter_mut()) {
            let (in_trip_1, in_trip_2) = it;
            let in_value_1 = *in_trip_1 as u32 - '0' as u32;
            let in_value_2 = *in_trip_2 as u32 - '0' as u32;
            let trip_sum: u32;
            if in_value_1 >= in_value_2 + carry {
                trip_sum = in_value_1 - in_value_2 - carry;
                carry = 0;
            } else {
                trip_sum = in_value_1 + 3 - in_value_2 - carry;
                carry = 1;
            };
            result = format!("{}{}", trip_sum, result);
        }

        if carry > 0 {
            return String::from("000000000002")
        } else {
            return String::from("000000000000")
        }
    }

    pub fn calc(&self) -> String {
        return match self.operation.as_str() {
            "0" => { self.add_trip_strings() },
            "1" => { self.sub_trip_strings() },
            "2" => { self.check_less_than() },
            _ => { String::from("000000000000") }
        }
    }
}