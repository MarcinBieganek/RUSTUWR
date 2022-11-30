fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut res_vec = Vec::new();
    
    (0..ints.len())
        .for_each(|i| {
            (i+1..ints.len())
                .for_each(|j| {
                    if (ints[i] + ints[j]) == s {
                        res_vec.push((i, j));
                    }
                })
        });
    
    let res = res_vec.iter()
                    .reduce(|acc, e| {
                        if acc.1 >= e.1 {
                            e
                        } else {
                            acc
                        }
                    });
    
    match res {
        None => None,
        _ => Some((ints[res.unwrap().0], ints[res.unwrap().1])),
    }
}


mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let l1 = [1, 4, 8, 7, 3, 15];
        assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    }

    #[test]
    fn second_test() {
        let l2 = [1, -2, 3, 0, -6, 1];
        assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    }

    #[test]
    fn third_test() {
        let l3 = [20, -13, 40];
        assert_eq!(sum_pairs(&l3, -7), None);

    }

    #[test]
    fn fourth_expected() {
        let l4 = [1, 2, 3, 4, 1, 0];
        assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    }

    #[test]
    fn five_expected() {
        let l8 = [5, 9, 13, -3];
        assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    }
}
