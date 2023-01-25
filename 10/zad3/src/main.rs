fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a_copy = a.to_vec();

    for item in b {
        let check = a_copy.iter().position(|x| x*x == item);

        match check {
            Some(index) => { a_copy.remove(index); },
            None => { return false; }
        } 
    };

    true
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn tests_comp() {
    
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, true);
    
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);
    
}