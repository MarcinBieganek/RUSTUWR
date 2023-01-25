fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    array
        .into_iter()
        .filter(|e| {
            (*e % 2) == 0
        })
        .map(|e| {
            *e
        })
        .rev()
        .take(number)
        .collect::<Vec<i32>>()
        .into_iter()
        .rev()
        .collect::<Vec<i32>>()
}

mod tests {
    use super::*;
    
    #[test]
    fn first_test() {
        assert_eq!(even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3), vec!(4, 6, 8));
    }

    #[test]
    fn second_test() {
        assert_eq!(even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2), vec!(-8, 26));
    }

    #[test]
    fn third_test() {
        assert_eq!(even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1), vec!(6));
    }

    #[test]
    fn short_test() {
        assert_eq!(even_numbers(&vec!(6, 2, 4), 2), vec!(2, 4));
    }

    #[test]
    fn last_test() {
        assert_eq!(even_numbers(&vec!(1, 2, 4, 1, 4), 3), vec!(2, 4, 4));
    }
}