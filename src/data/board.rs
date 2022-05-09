use crate::data::cage::{Cage, CageColor};
use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    grid: Vec<Vec<i32>>,
    solved_grid: Vec<Vec<i32>>,
    cage_grid: Vec<Vec<Cage>>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: vec![vec![0; 9]; 9],
            solved_grid: vec![vec![0; 9]; 9],
            cage_grid: vec![vec![Cage::default(); 9]; 9],
        }
    }
}

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Board {
    pub fn get_from_archive() -> Self {
        let mut rnd = rand::thread_rng();
        let mut board = Board::default();
        let mut rdr = csv::Reader::from_path("./assets/sudoku.csv").unwrap();
        let index = rnd.gen_range(0..1000000);
        let random_board = rdr.records().nth(index).unwrap();
        let record = random_board.unwrap();

        let values = &record[0];
        let solution = &record[1];

        for (i, num) in values.chars().enumerate() {
            let num = num.to_digit(10).unwrap() as i32;
            let (x, y) = (i % 9, i / 9);
            if num != 0 {
                board.unchecked_set_num(x, y, num);
            }

            let solution_num = solution.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            board.solved_grid[y][x] = solution_num;
            //TODO polyomio's are difficult :-) this probably wouldn't be a valid way
            // board.set_color(x,y,CageColor::from(rnd.gen_range(0..4)));
        }

        board
    }
    pub fn gen_random(difficulty: Difficulty) -> Self {
        let mut board = Board::default();
        let mut rnd = rand::thread_rng();

        let amt = match difficulty {
            Difficulty::Easy => 32,
            Difficulty::Normal => 24,
            Difficulty::Hard => 17,
        };

        for _ in 0..amt {
            let mut i = rnd.gen_range(0..81);
            let (mut x, mut y) = (i % 9, i / 9);
            let mut num = rnd.gen_range(1..=9);
            while !board.is_possible(x, y, num) {
                i = rnd.gen_range(0..81);
                x = i % 9;
                y = i / 9;
                num = rnd.gen_range(1..=9);
            }
            board.unchecked_set_num(x, y, num);
        }
        board
    }
    pub fn set_num(&mut self, x: usize, y: usize, num: i32) {
        if !self.is_possible(x, y, num) {
            return;
        }

        self.unchecked_set_num(x, y, num);
    }
    pub fn is_possible(&self, x: usize, y: usize, num: i32) -> bool {
        let temp = self;
        let temp_grid = &temp.grid;

        let cage = &temp.cage_grid[y][x];
        if cage.sum > 0 {
            let mut cage_indices = vec![];
            temp.cage_grid.iter().enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, c)| {
                    if c.index == cage.index {
                        cage_indices.push((x, y));
                    }
                })
            });
            let current_sum_list = cage_indices
                .iter()
                .map(|(x, y)| temp_grid[*y][*x])
                .collect::<Vec<_>>();
            let current_sum = current_sum_list.iter().sum::<i32>();

            if temp.get_cage_size_left(x, y) == 1 && num != (cage.sum - current_sum) {
                return false;
            }

            if num + current_sum > cage.sum {
                // println!("sum would be too big on y {} x {} ", y, x);
                return false;
            }
        }

        if temp_grid[y][x] != 0 {
            // debug!("square not empty {}", temp_grid[y][x]);
            return false;
        }

        let (square_x, square_y) = (((x) / 3) * 3, ((y) / 3) * 3);
        debug!("square {} {}", square_y, square_x);
        for y_add in 0..3 {
            for x_add in 0..3 {
                let (new_x, new_y) = (square_x + x_add, square_y + y_add);
                if temp_grid[new_y][new_x] == num {
                    debug!("Value not possible in square y {} x {}", new_y, new_x);
                    return false; // value not possible
                }
            }
        }

        for o in 0..9 {
            if !(square_y..square_y + 3).contains(&o) && temp_grid[o][x] == num {
                debug!("Value not possible on vertical line");
                return false; // value not possible
            }

            if !(square_x..square_x + 3).contains(&o) && temp_grid[y][o] == num {
                debug!("Value not possible on horizontal line");
                return false; // value not possible
            }
        }

        true
    }
    pub fn unchecked_set_num(&mut self, x: usize, y: usize, num: i32) {
        self.grid[y][x] = num;
    }
    pub fn get_value(&self, x: usize, y: usize) -> i32 {
        self.grid[y][x]
    }
    pub fn get_sum(&self, x: usize, y: usize) -> i32 {
        self.cage_grid[y][x].sum
    }
    pub fn get_color(&self, x: usize, y: usize) -> CageColor {
        match self.cage_grid[y][x].index % 4 {
            0 => CageColor::Yellow,
            1 => CageColor::Green,
            2 => CageColor::Red,
            3 => CageColor::Blue,
            _ => unimplemented!(),
        }
    }
    pub fn get_cage_size(&self, x: usize, y: usize) -> usize {
        let index = self.cage_grid[y][x].index;
        self.cage_grid
            .iter()
            .flat_map(|row| row.iter().filter(|x| x.index == index))
            .count()
    }
    pub fn get_cage_size_left(&self, x: usize, y: usize) -> usize {
        let index = self.cage_grid[y][x].index;
        let mut cage_positions = vec![];
        self.cage_grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, c)| {
                if c.index == index {
                    cage_positions.push((y, x))
                }
            })
        });
        cage_positions
            .iter()
            .filter(|(y, x)| self.grid[*y][*x] == 0)
            .count()
    }
    pub fn print(&self) {
        for row in &self.grid {
            print!("| ");
            for num in row {
                print!("{} | ", num);
            }
            println!();
        }
        println!();
    }
    pub fn is_finished(&self) -> bool {
        for o in 0..9 {
            let mut x_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            for x in 0..9 {
                let i = self.get_value(x, o);
                if i == 0 {
                    return false;
                }
                if let Some(pos) = x_list.iter().position(|x| *x == i) {
                    x_list.remove(pos);
                } else {
                    return false;
                }
            }
            let mut y_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            for y in 0..9 {
                let i = self.get_value(o, y);
                if i == 0 {
                    return false;
                }
                if let Some(pos) = y_list.iter().position(|x| *x == i) {
                    y_list.remove(pos);
                } else {
                    return false;
                }
            }

            let offset = ((o % 3) * 3, (o / 3) * 3);
            let mut square_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            for y in offset.1..offset.1 + 3 {
                for x in offset.0..offset.0 + 3 {
                    let i = self.get_value(x, y);
                    if i == 0 {
                        return false;
                    }
                    if let Some(pos) = square_list.iter().position(|x| *x == i) {
                        square_list.remove(pos);
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
    // pub fn get_row(&self, x:usize) -> &Vec<Square> {
    //     &self.grid[x]
    // }
    // pub fn get_col(&self, y:usize) -> &Vec<Square> {
    //     &self.grid.iter().enumerate().map(|(i,vec)| vec[y]).collect::<Vec<_>>()
    // }
    pub fn get_grid(self) -> Vec<Vec<i32>> {
        self.grid
    }
    /// hard-coded sample puzzle
    /// source: https://www.dailykillersudoku.com/pdfs/23745.solution.pdf
    /// this puzzle is known to be valid, so it'd be useful for an algorithm
    pub fn sample_puzzle() -> Self {
        let cage1 = Cage { index: 1, sum: 3 };
        let cage2 = Cage { index: 2, sum: 15 };
        let cage3 = Cage { index: 3, sum: 22 };
        let cage4 = Cage { index: 4, sum: 4 };
        let cage5 = Cage { index: 5, sum: 16 };
        let cage6 = Cage { index: 6, sum: 15 };
        let cage7 = Cage { index: 7, sum: 25 };
        let cage8 = Cage { index: 8, sum: 17 };
        let cage9 = Cage { index: 9, sum: 9 };
        let cage10 = Cage { index: 10, sum: 8 };
        let cage11 = Cage { index: 11, sum: 20 };
        let cage12 = Cage { index: 12, sum: 6 };
        let cage13 = Cage { index: 13, sum: 14 };
        let cage14 = Cage { index: 14, sum: 17 };
        let cage15 = Cage { index: 15, sum: 17 };
        let cage16 = Cage { index: 16, sum: 13 };
        let cage17 = Cage { index: 17, sum: 20 };
        let cage18 = Cage { index: 18, sum: 12 };
        let cage19 = Cage { index: 19, sum: 27 };
        let cage20 = Cage { index: 20, sum: 6 };
        let cage21 = Cage { index: 21, sum: 20 };
        let cage22 = Cage { index: 22, sum: 6 };
        let cage23 = Cage { index: 23, sum: 10 };
        let cage24 = Cage { index: 24, sum: 14 };
        let cage25 = Cage { index: 25, sum: 8 };
        let cage26 = Cage { index: 26, sum: 16 };
        let cage27 = Cage { index: 27, sum: 15 };
        let cage28 = Cage { index: 28, sum: 13 };
        let cage29 = Cage { index: 29, sum: 17 };
        Self {
            grid: vec![vec![0; 9]; 9],
            solved_grid: vec![
                vec![2, 1, 5, 6, 4, 7, 3, 9, 8],
                vec![3, 6, 8, 9, 5, 2, 1, 7, 4],
                vec![7, 9, 4, 3, 8, 1, 6, 5, 2],
                vec![5, 8, 6, 2, 7, 4, 9, 3, 1],
                vec![1, 4, 2, 5, 9, 3, 8, 6, 7],
                vec![9, 7, 3, 8, 1, 6, 4, 2, 5],
                vec![8, 2, 1, 7, 3, 9, 5, 4, 6],
                vec![6, 5, 9, 4, 2, 8, 7, 1, 3],
                vec![4, 3, 7, 1, 6, 5, 2, 8, 9],
            ],
            cage_grid: vec![
                vec![
                    cage1, cage1, cage2, cage2, cage2, cage3, cage4, cage5, cage6,
                ],
                vec![
                    cage7, cage7, cage8, cage8, cage3, cage3, cage4, cage5, cage6,
                ],
                vec![
                    cage7, cage7, cage9, cage9, cage3, cage10, cage11, cage11, cage6,
                ],
                vec![
                    cage12, cage13, cage13, cage9, cage14, cage10, cage11, cage15, cage6,
                ],
                vec![
                    cage12, cage16, cage16, cage17, cage14, cage10, cage15, cage15, cage18,
                ],
                vec![
                    cage19, cage16, cage20, cage17, cage14, cage21, cage22, cage22, cage18,
                ],
                vec![
                    cage19, cage20, cage20, cage17, cage23, cage21, cage21, cage24, cage24,
                ],
                vec![
                    cage19, cage25, cage26, cage23, cage23, cage27, cage27, cage24, cage24,
                ],
                vec![
                    cage19, cage25, cage26, cage23, cage28, cage28, cage28, cage29, cage29,
                ],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::board::Board;

    #[test]
    fn check_is_finished() {
        let mut board = Board::default();
        let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut vec3 = vec1.clone();
        vec3.rotate_right(3);
        let mut vec2 = vec3.clone();
        vec2.rotate_right(3);
        let mut vec9 = vec1.clone();
        vec9.rotate_right(1);
        let mut vec8 = vec9.clone();
        vec8.rotate_right(3);
        let mut vec7 = vec8.clone();
        vec7.rotate_right(3);

        let mut vec4 = vec7.clone();
        vec4.rotate_right(1);
        let mut vec6 = vec4.clone();
        vec6.rotate_right(3);
        let mut vec5 = vec6.clone();
        vec5.rotate_right(3);

        board.grid = vec![vec1, vec2, vec3, vec4, vec5, vec6, vec7, vec8, vec9];

        assert!(board.is_finished());
    }

    #[test]
    fn check_killer_sudoku() {
        let mut board = Board::sample_puzzle();
        assert!(board.is_possible(0, 0, 2));
        board.unchecked_set_num(0, 0, 2);

        assert!(board.is_possible(1, 0, 1));
        assert!(!board.is_possible(1, 0, 9));
        board.unchecked_set_num(1, 0, 1);
    }
}
