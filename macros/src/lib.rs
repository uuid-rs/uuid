use std;

#[path = "../../shared/error.rs"]
#[allow(dead_code)]
mod error;

#[path = "../../shared/parser.rs"]
#[allow(dead_code)]
mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
