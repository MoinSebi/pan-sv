use std::collections::HashSet;



/// HashSet to String
/// This is not always in the same order
pub fn hashset2string(input: &Vec<u32>, sep:  &str) -> String{
    let j:Vec<String> = input.iter().map(|i| i.to_string()).collect();
    j.join(sep)
}

#[cfg(test)]
mod helpertest {
    use crate::core::helper::{vec2string};

    #[test]
    fn helpers() {
        let k: Vec<u32> = vec![1,2,3,4];
        assert_eq!(vec2string(&k, "."), "1.2.3.4".to_string());
    }
}