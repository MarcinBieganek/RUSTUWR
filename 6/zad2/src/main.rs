fn range_extraction(a: &[i32]) -> String {
    (1..a.len())
        .fold(vec![vec![a[0]]], |mut res: Vec<Vec<i32>>, i| {
            if a[i-1] + 1 == a[i] {
                let last = res.len() - 1;
                res[last].push(a[i]);
                return res;
            } else {
                res.push(vec![a[i]]);
                return res;
            }
        })
        .into_iter()
        .map(|range| {
            match range.len() {
                1 => range[0].to_string(),
                2 => format!("{},{}", range[0], range[1]),
                _ => format!("{}-{}", range[0], range[range.len()-1])
            }
        })
        .collect::<Vec<String>>()
        .join(",")
}

mod tests {
    use super::*;
    
    #[test]
    fn first_test() {
        assert_eq!(range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]), "-6,-3-1,3-5,7-11,14,15,17-20");	
    }

    #[test]
    fn second_test() {	
        assert_eq!(range_extraction(&[-3,-2,-1,2,10,15,16,17,19,20]), "-3--1,2,10,15-17,19,20");
    }

    #[test]
    fn third_test() {	
        assert_eq!(range_extraction(&[-3,-2,-1]), "-3--1");
    }

    #[test]
    fn none_test() {	
        assert_eq!(range_extraction(&[-3,0,10]), "-3,0,10");
    }

    #[test]
    fn simple_test() {	
        assert_eq!(range_extraction(&[-3,0,1,2,10]), "-3,0-2,10");
    }
}