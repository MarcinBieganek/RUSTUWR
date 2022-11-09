fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_worths = [1, 2, 3, 3, 4, 10];
    let evil_worths = [1, 2, 2, 2, 3, 5, 10];
    
    let good_force: i32 = good.split_whitespace()
                        .enumerate()
                        .map(|(i, x)| {
                            (x.parse::<i32>().unwrap()) * good_worths[i]
                        })
                        .sum();

    let evil_force: i32 = evil.split_whitespace()
                        .enumerate()
                        .map(|(i, x)| {
                            (x.parse::<i32>().unwrap()) * evil_worths[i]
                        })
                        .sum();

    if good_force > evil_force {
        String::from("Battle Result: Good triumphs over Evil")
    } else if good_force < evil_force {
        String::from("Battle Result: Evil eradicates all trace of Good")
    } else {
        String::from("Battle Result: No victor on this battle field")
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn good_win_test() {
        assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    }

    #[test]
    fn evil_win_test() {
        assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    }

    #[test]
    fn tie_test() {
        assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
    }

    #[test]
    fn one_of_type_test() {
        assert_eq!(good_vs_evil("1 1 1 1 1 1", "1 1 1 1 1 1 1"), "Battle Result: Evil eradicates all trace of Good");
    }

    #[test]
    fn zero_forces_test() {
        assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 0"), "Battle Result: No victor on this battle field");
    }

}