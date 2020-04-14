use std::error::Error;

pub fn head_tail(L: String) -> Result<(char, Option<String>)> {
    match L.len() {
        0 => panic!("IndexError"),
        1 => Ok((L.chars().next().unwrap(), None)),
        _ => {
            let mut chars = L.chars();
            let head: char = chars.next().unwrap();
            let tail: String = chars.collect();
            Ok((head, Some(tail)))
        },
    }
}
