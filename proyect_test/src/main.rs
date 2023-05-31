fn main() {
    println!("Test")
}

fn thuthy() -> bool {
    return true; 
}

fn result() -> char {
    return 'b'
}

#[cfg(test)]
mod test {
    use super::thuthy;
    use super::result;

    #[test]
    fn test_something(){
        assert_ne!(thuthy(), false);
    }

    #[test]
    fn result_test() {
        assert_eq!(result(), 'b');
    }
}
