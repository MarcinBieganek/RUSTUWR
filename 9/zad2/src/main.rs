use std::collections::BTreeMap;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();

    let letters:String = input.to_lowercase()
        .chars()
        .filter(|c| {
            c.is_alphabetic()
        })
        .collect();
    
    for c in letters.chars() { 
        if res.contains_key(&c) {
            let old = res.get(&c).unwrap();
            res.insert(c, old + 1);
        } else {
            res.insert(c, 1);
        }
    }
    
    res   
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::letter_frequency;
    
    #[test]
    fn simpleword() {
        let answer: BTreeMap<char, i32> =
        [('a', 2),
         ('c', 1),
         ('l', 1),
         ('t', 1),
         ('u', 1)]
         .iter().cloned().collect();
         
      assert_eq!(letter_frequency("actual"), answer);
    }
    
    #[test]
    fn sequence() {
        let answer: BTreeMap<char, i32> =
        [('a', 3),
         ('b', 2),
         ('f', 1),
         ('p', 1),
         ('s', 1),
         ('t', 2),
         ('u', 1),
         ('x', 5)]
         .iter().cloned().collect();
         
      assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
    }

    #[test]
    fn simple() {
        let answer: BTreeMap<char, i32> =
        [('a', 3),
         ('b', 3)]
         .iter().cloned().collect();
         
      assert_eq!(letter_frequency("BbAAba"), answer);
    }

    #[test]
    fn alpha() {
        let answer: BTreeMap<char, i32> =
        [('a', 1),
         ('b', 1),
         ('c', 1)]
         .iter().cloned().collect();
         
      assert_eq!(letter_frequency("cba"), answer);
    }

    #[test]
    fn one() {
        let answer: BTreeMap<char, i32> =
        [('c', 4)]
         .iter().cloned().collect();
         
      assert_eq!(letter_frequency("C _ +213312   ccc"), answer);
    }
}