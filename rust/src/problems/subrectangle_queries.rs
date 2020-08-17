// https://leetcode.com/problems/subrectangle-queries
pub trait Query {
    fn new(rectangle: Vec<Vec<i32>>) -> Self;
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32);
    fn get_value(&self, row: i32, col: i32) -> i32;
}

#[derive(Debug)]
pub struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>
}

impl Query for SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        return SubrectangleQueries {
            rectangle
        };
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..=row2 {
            for j in col1..=col2 {
                self.rectangle[i as usize][j as usize] = new_value
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        return self.rectangle[row as usize][col as usize];
    }
}
