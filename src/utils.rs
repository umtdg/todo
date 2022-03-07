pub fn remove_first(text: &str) -> &str {
    &text[1..text.len()]
}

pub trait NumberOfDigits {
    fn number_of_digits(&self) -> usize;
}

impl NumberOfDigits for usize {
    fn number_of_digits(&self) -> usize {
        if self < &10 { 1 }
        else if self < &100 { 2 }
        else if self < &1000 { 3 }
        else if self < &10000 { 4 }
        else if self < &100000 { 5 }
        else if self < &1000000 { 6 }
        else if self < &10000000 { 7 }
        else if self < &100000000 { 8 }
        else if self < &1000000000 { 9 }
        else if self < &10000000000 { 10 }
        else { self.to_string().len() }
    }
}
