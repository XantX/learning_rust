fn main() {
    println!("Test")
}

fn thuthy() -> bool {
    return false; 
}

#[cfg(test)]
mod test {
    use super::thuthy;

    #[test]
    fn test_something(){
        assert_ne!(thuthy(), false);
    }
}
