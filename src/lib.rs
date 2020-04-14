mod util;


#[cfg(test)]
mod tests {
    use crate::util::head_tail;

    #[test]
    fn it_works() {
        assert_eq!(head_tail("नमस्ते".to_string()), Ok(('न',Some("मस्ते".to_string()))));
    }
}
