struct Sudoku{
    data: Vec<Vec<u32>>,
}


impl Sudoku{
    fn check_size(&self) -> bool {
        let rows_no = self.data.len();
        if rows_no == 0 || ((rows_no as f64).sqrt() as usize * (rows_no as f64).sqrt() as usize) != rows_no {
            return false;
        }

        for row in &self.data {
            if row.len() != rows_no {
                return false;
            }
        }
        true
    }

    fn get_size(&self) -> u32 {
        self.data.len() as u32
    }

    fn get_litte_squares_no(&self) -> u32 {
        (self.get_size() as f64).sqrt() as u32
    }

    fn get_row(&self, i: u32) -> Vec<u32> {
        self.data[i as usize].to_vec()
    }

    fn get_col(&self, i: u32) -> Vec<u32> {
        self.data.iter()
            .map(|row| {
                row[i as usize]
            })
            .collect::<Vec<u32>>()
    }

    fn get_little_square(&self, i: u32, j: u32) -> Vec<u32> {
        let squares_no = self.get_litte_squares_no();
        let mut res = Vec::new();

        for si in (i*squares_no)..((i*squares_no)+squares_no) {
            for sj in (j*squares_no)..((j*squares_no)+squares_no) {
                res.push(self.data[si as usize][sj as usize]);
            }
        }

        res
    }

    fn check_line(&self, mut line: Vec<u32>) -> bool {
        let size = self.get_size();
        let posible_values: Vec<u32> = (1..size+1).collect();

        line.sort();
        
        line == posible_values
    }

    fn is_valid(&self) -> bool {
        if !self.check_size() {
            return false;
        }

        let size = self.get_size();
        let litte_squares_no = self.get_litte_squares_no();

        for i in 0..size {
            if !self.check_line(self.get_row(i)) {
                println!("WRONG ROW {}", i);
                return false;
            }

            if !self.check_line(self.get_col(i)) {
                println!("WRONG COL {}", i);
                return false;
            }
        }

        for i in 0..litte_squares_no {
            for j in 0..litte_squares_no {
                if !self.check_line(self.get_little_square(i, j)) {
                    println!("WRONG SQUARE ({}, {})", i, j);
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn good_one_sudoku() {
        let good_sudoku_1 = Sudoku{
            data: vec![
                    vec![7,8,4, 1,5,9, 3,2,6],
                    vec![5,3,9, 6,7,2, 8,4,1],
                    vec![6,1,2, 4,3,8, 7,5,9],

                    vec![9,2,8, 7,1,5, 4,6,3],
                    vec![3,5,7, 8,4,6, 1,9,2],
                    vec![4,6,1, 9,2,3, 5,8,7],
                    
                    vec![8,7,6, 3,9,4, 2,1,5],
                    vec![2,4,3, 5,6,1, 9,7,8],
                    vec![1,9,5, 2,8,7, 6,3,4]
                ]
        };

        assert!(good_sudoku_1.is_valid());
    }

    #[test]
    fn good_two_sudoku() {
        let good_sudoku_2 = Sudoku{
            data: vec![
                    vec![1, 4,  2, 3],
                    vec![3, 2,  4, 1],
            
                    vec![4, 1,  3, 2],
                    vec![2, 3,  1, 4],
                ]
        };
        
        assert!(good_sudoku_2.is_valid());
    }

    #[test]
    fn bad_one_sudoku() {
        let bad_sudoku_1 = Sudoku{
            data: vec![
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],

                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                ]
        };

        assert!(!bad_sudoku_1.is_valid());
    }

    #[test]
    fn bad_two_sudoku() {
        let bad_sudoku_2 = Sudoku{
            data: vec![
                    vec![1,2,3,4,5],
                    vec![1,2,3,4],
                    vec![1,2,3,4],
                    vec![1],
                ]
        };

        assert!(!bad_sudoku_2.is_valid());
    }

    #[test]
    fn bad_empty_sudoku() {
        let bad_sudoku = Sudoku{
            data: vec![
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ]
        };

        assert!(!bad_sudoku.is_valid());
    }
}
