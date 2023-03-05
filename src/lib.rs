// Dillon Anderson
// January 2023

use std::error::Error;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Filled,
    Crossed,
}

pub struct Puzzle {
    pub array: Vec<Cell>,
    pub row_clues: Vec<Vec<usize>>,
    pub column_clues: Vec<Vec<usize>>,
}

impl Puzzle {
    pub fn new() -> Self {
        Puzzle {
            array: vec![],
            row_clues: vec![],
            column_clues: vec![],
        }
    }

    // create a puzzle struct out of strings in this format:
    // https://github.com/mikix/nonogram-db/blob/master/FORMAT.md
    pub fn from_string(string: &str) -> Result<Self, Box<dyn Error>> {
        let mut strings_iter = string.split("\n");
        let mut row_clues = vec![];
        let mut column_clues = vec![];

        // skip to the rows
        loop {
            if let Some(line) = strings_iter.next() {
                if line == "rows" {
                    break;
                }
                continue;
            }
            return Err("Invalid string format".into());
        }
        // parse rows
        loop {
            if let Some(row) = strings_iter.next() {
                if row.len() < 1 {
                    println!("break");
                    break;
                }

                println!("[{}]", row);
                let mut clues = vec![];
                for clue in row.split(",") {
                    let c: usize = clue.parse()?;
                    clues.push(c);
                }
                row_clues.push(clues);
                continue;
            }
            break;
        }

        // skip to the rows
        loop {
            if let Some(line) = strings_iter.next() {
                if line == "columns" {
                    break;
                }
                continue;
            }
            return Err("Invalid string format".into());
        }
        // parse columns
        loop {
            if let Some(column) = strings_iter.next() {
                if column.len() < 1 {
                    println!("break");
                    break;
                }
                println!("[{}]", column);
                let mut clues = vec![];
                for clue in column.split(",") {
                    let c: usize = clue.parse()?;
                    clues.push(c);
                }
                column_clues.push(clues);
                continue;
            }
            break;
        }

        row_clues = row_clues.into_iter().rev().collect();

        Ok(Puzzle {
            array: vec![],
            row_clues,
            column_clues,
        })
    }

    fn recalculate_size(&mut self) {
        self.array = vec![Cell::Empty; self.row_clues.len() * self.column_clues.len()]
    }

    pub fn push_clues_row(&mut self, clues: Vec<usize>) {
        self.row_clues.push(clues);
        self.recalculate_size();
    }

    pub fn push_clues_column(&mut self, clues: Vec<usize>) {
        self.column_clues.push(clues);
        self.recalculate_size();
    }

    // unfinished solves the current puzzle, returns true if puzzle was completable by the solver
    fn solve(&mut self) -> bool {
        let height = self.row_clues.len();
        let width = self.column_clues.len();
        let mut change_made = true;

        // loop over all rows and columns until no changes can be made, then return verify()
        while change_made {
            // becomes true if at least any one cell is solved
            change_made = false;

            // for each row
            for y in 0..height {
                // get cells and clues for row y
                let cells: Vec<&mut Cell> = self
                    .array
                    .iter_mut()
                    .enumerate()
                    .filter(|(i, _)| y * width <= *i && *i < y * width + width)
                    .map(|(_, e)| e)
                    .collect();
                let clues = &self.row_clues[y];

                // The functions called here are defined below
                change_made = mathematical_approach(cells, clues);
            }
            // for each column
            for x in 0..width {
                // get vec of mutable Cell references for our column
                // TEST THIS
                let cells: Vec<&mut Cell> = self
                    .array
                    .iter_mut()
                    .enumerate()
                    .filter(|(i, _)| *i % width == x)
                    .map(|(_, e)| e)
                    .collect();
                let clues = &self.column_clues[x];

                // The functions called here are defined below
                change_made = mathematical_approach(cells, clues);
            }
        }

        // Solves given cells using given clues. true if at least one cell is changed, false otherwise
        fn mathematical_approach(mut cells: Vec<&mut Cell>, clues: &Vec<usize>) -> bool {
            let mut change_made = false;
            // Mathematical Approach
            // https://en.wikipedia.org/wiki/Nonogram#Mathematical_approach

            /* wish this worked instead! only checking empty rows for now.
            match cells[..] {
                [Cell::Crossed.., Cell::Empty.., Cell::Crossed..] => {}
            }
            */
            if cells.iter().all(|cell| cell == &&mut Cell::Empty) {
                // calculate minimum number of cells needed to fulfill clues
                let mut clues_space = clues.len() - 1;
                for clue in clues {
                    clues_space += *clue
                }
                let min_clue_size = cells.len() - clues_space;
                // any clues bigger than backfill_distance will have cells filled
                for (i, clue) in clues.iter().enumerate() {
                    if clue > &min_clue_size {
                        let cells_to_backfill = clue - &min_clue_size;
                        // find the start of the clue counting from the left
                        let mut clue_start = 0;
                        for j in 0..i {
                            clue_start += clues[j];
                            clue_start += 1;
                        }
                        //backfill from the right
                        for cell in clue_start + clue - cells_to_backfill..clue_start + clue {
                            *cells[cell] = Cell::Filled;
                            change_made = true;
                        }
                    }
                }
            }
            return change_made;
        }

        return self.verify();
    }

    pub fn get_pos(&self, x: usize, y: usize) -> usize {
        y * self.column_clues.len() + x
    }

    pub fn get_width(&self) -> usize {
        self.column_clues.len()
    }

    pub fn get_height(&self) -> usize {
        self.row_clues.len()
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.array[self.get_pos(x, y)]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.array[y * self.column_clues.len() + x] = cell;
    }

    pub fn get_longest_row_clue_len(&self) -> usize {
        let mut longest = 0;
        for i in &self.row_clues {
            if i.len() > longest {
                longest = i.len()
            }
        }
        longest
    }

    pub fn get_longest_column_clue_len(&self) -> usize {
        let mut longest = 0;
        for i in &self.column_clues {
            if i.len() > longest {
                longest = i.len()
            }
        }
        longest
    }

    // verifies one row or column of rules
    fn verify_clues(&self, clues: &Vec<usize>, cells: &[Cell]) -> bool {
        let mut built_clues: Vec<usize> = vec![];
        let mut mid_set = false;
        let mut current_clue: usize = 0;
        for v in cells {
            match v {
                Cell::Filled => {
                    if mid_set {
                        current_clue += 1;
                    } else {
                        mid_set = true;
                        current_clue = 1;
                    }
                }
                _ => {
                    if mid_set {
                        mid_set = false;
                        built_clues.push(current_clue)
                    } else {
                    }
                }
            }
        }
        if mid_set || current_clue == 0 {
            built_clues.push(current_clue);
        }

        /*println!(
            "cells:{:?}\nclues:{:?}\nbuilt:{:?}\n",
            cells, clues, built_clues
        );*/
        clues.iter().eq(built_clues.iter())
    }

    fn verify_rows(&self) -> bool {
        for (y, v) in self.row_clues.iter().enumerate() {
            if !self.verify_clues(
                v,
                &self.array[self.get_pos(0, y)..self.get_pos(self.column_clues.len(), y)],
            ) {
                return false;
            }
        }
        true
    }

    fn verify_columns(&self) -> bool {
        for (x, v) in self.column_clues.iter().enumerate() {
            let mut cells = vec![];
            for (y, _) in self.row_clues.iter().enumerate() {
                let cell = self.array[self.get_pos(x, y)].clone();
                cells.push(cell)
            }

            if !self.verify_clues(v, &cells[..]) {
                return false;
            }
        }
        true
    }

    // verifies if the puzzle is complete by comparing the array to the clues
    pub fn verify(&self) -> bool {
        self.verify_rows() && self.verify_columns()
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_to_print = String::new();

        // figure out width of string
        let width_of_longest_row_clue = {
            let mut largest = 0;
            for i in &self.row_clues {
                if i.len() > largest {
                    largest = i.len();
                }
            }
            largest * 3
        };

        let width_of_puzzle_rows = &self.column_clues.len() * 3;

        let string_width = width_of_puzzle_rows + width_of_longest_row_clue + 1; // +1 for the |'s column

        // figure out height of string
        let height_of_longest_column_clue = {
            let mut largest = 0;
            for i in &self.column_clues {
                if i.len() > largest {
                    largest = i.len();
                }
            }
            largest
        };

        let height_of_puzzle_columns = self.row_clues.len();

        let string_height = height_of_longest_column_clue + height_of_puzzle_columns + 1; // +1 for the -'s column

        // fill string with spaces
        for _ in 0..string_height {
            for _ in 0..string_width {
                string_to_print.push(' ');
            }
            string_to_print.push('\n');
        }

        // define a lambda function to replace string values with x,y coordinates
        let mut set_string_pos = |x: usize, y: usize, c: &str| {
            string_to_print.replace_range(
                y * (string_width + 1) + x..y * (string_width + 1) + x + 1,
                c,
            )
        };

        // draw column numbers
        for (i, clues) in self.column_clues.iter().enumerate() {
            for (j, clue) in clues.iter().rev().enumerate() {
                if *clue < 10 {
                    set_string_pos(
                        width_of_longest_row_clue + 1 + 2 + i * 3,
                        height_of_longest_column_clue - j - 1,
                        &clue.to_string(),
                    );
                } else {
                    set_string_pos(
                        width_of_longest_row_clue + 1 + 2 + i * 3,
                        height_of_longest_column_clue - j - 1,
                        &(clue / 10).to_string(),
                    );
                    set_string_pos(
                        width_of_longest_row_clue + 1 + 2 + i * 3 + 1,
                        height_of_longest_column_clue - j - 1,
                        &(clue % 10).to_string(),
                    );
                }
            }
        }

        // draw row numbers
        for (i, clues) in self.row_clues.iter().enumerate() {
            for (j, clue) in clues.iter().rev().enumerate() {
                if *clue < 10 {
                    set_string_pos(
                        width_of_longest_row_clue - (j * 3) - 2,
                        height_of_longest_column_clue + 1 + i,
                        &clue.to_string(),
                    );
                } else {
                    set_string_pos(
                        width_of_longest_row_clue - (j * 3) - 2,
                        height_of_longest_column_clue + 1 + i,
                        &(clue / 10).to_string(),
                    );
                    set_string_pos(
                        width_of_longest_row_clue - (j * 3) - 2 + 1,
                        height_of_longest_column_clue + 1 + i,
                        &(clue % 10).to_string(),
                    );
                }
            }
        }

        // draw -'s
        for i in 0..width_of_puzzle_rows {
            set_string_pos(
                i + width_of_longest_row_clue + 1,
                height_of_longest_column_clue,
                "-",
            )
        }

        // draw |'s
        for i in 0..height_of_puzzle_columns {
            set_string_pos(
                width_of_longest_row_clue,
                i + height_of_longest_column_clue + 1,
                "|",
            )
        }

        // draw cells
        let mut x = 0;
        let mut y = 0;
        for cell in &self.array {
            if x >= self.column_clues.len() {
                y += 1;
                x = 0;
            }
            let s: &str;

            match cell {
                Cell::Empty => s = ".",
                Cell::Filled => s = "0",
                Cell::Crossed => s = "/",
            }
            set_string_pos(
                width_of_longest_row_clue + x * 3 + 3,
                height_of_longest_column_clue + y + 1,
                s,
            );

            x += 1;
        }

        write!(f, "{}", string_to_print)
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            array: vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            row_clues: vec![
                vec![7],
                vec![1, 1, 1],
                vec![2, 1],
                vec![1, 1, 1],
                vec![1, 2, 1],
                vec![4, 2],
                vec![2, 1, 1],
                vec![1, 1, 1],
                vec![2, 2],
                vec![5],
            ],
            column_clues: vec![
                vec![1, 1],
                vec![4, 3],
                vec![2, 3, 1],
                vec![1, 1, 1],
                vec![1, 2, 1],
                vec![1, 1, 2],
                vec![2, 1, 1],
                vec![5, 1],
                vec![1, 1],
                vec![4],
            ],
        }
    }
}

/////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let mut p = Puzzle::new();
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![0]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![3]);

        p.push_clues_column(vec![1]);
        p.push_clues_column(vec![1, 2]);
        p.push_clues_column(vec![1]);
        p.push_clues_column(vec![1, 2]);
        p.push_clues_column(vec![1]);

        let mut s = String::from("junk\nrows\n3\n1,1\n0\n1,1\n1,1\n\ncolumns\n1\n1,2\n1\n1,2\n1\n");
        let mut pu = Puzzle::from_string(&s).unwrap();

        assert_eq!(p.row_clues, pu.row_clues);
        assert_eq!(p.column_clues, pu.column_clues);

        p = Puzzle::new();
        p.push_clues_row(vec![2]);
        p.push_clues_row(vec![1, 2]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![2]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![3]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![2, 1]);
        p.push_clues_row(vec![2]);
        p.push_clues_column(vec![2, 1]);
        p.push_clues_column(vec![2, 1, 3]);
        p.push_clues_column(vec![7]);
        p.push_clues_column(vec![1, 3]);
        p.push_clues_column(vec![2, 1]);

        s =String::from("catalogue \"webpbn.com #1\"\ntitle \"Demo Puzzle from Front Page\"\nby \"Jan Wolter\"\ncopyright \"© Copyright 2004 by Jan Wolter\"\nlicense CC-BY-3.0\nwidth 5\nheight 10\n\nrows\n2\n2,1\n1,1\n3\n1,1\n1,1\n2\n1,1\n1,2\n2\n\ncolumns\n2,1\n2,1,3\n7\n1,3\n2,1\n\ngoal \"01100011010010101110101001010000110010100101111000\"");
        pu = Puzzle::from_string(&s).unwrap();

        assert_eq!(p.row_clues, pu.row_clues);
        assert_eq!(p.column_clues, pu.column_clues);
    }

    //#[test]
    fn test_solver_tmp() {
        let mut p = Puzzle::new();
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![0]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![3]);
        p.push_clues_column(vec![1]);
        p.push_clues_column(vec![1, 2]);
        p.push_clues_column(vec![1]);
        p.push_clues_column(vec![1, 2]);
        p.push_clues_column(vec![1]);

        // [Empty, Empty, Filled, Filled, Filled, Filled, Empty, Empty, Empty, Empty, Empty, Empty, Filled, Empty, Empty]
        println!("{}", p);
        p.solve();
        println!("{}", p);
        assert!(true);
    }

    #[test]
    fn test_verify() {
        let p = Puzzle {
            array: vec![
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
            ],
            row_clues: vec![vec![1, 1], vec![1, 1], vec![0], vec![1, 1], vec![3]],
            column_clues: vec![vec![1], vec![2, 1], vec![1], vec![2, 1], vec![1]],
        };

        assert!(p.verify());
    }

    #[test]
    fn test_size() {
        let mut p = Puzzle::new();
        assert_eq!(p.array.len(), 0);

        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![0]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![3]);
        assert_eq!(p.array.len(), 0);
        p.push_clues_column(vec![1]);
        assert_eq!(p.array.len(), 5);
        p.push_clues_column(vec![1, 2]);
        assert_eq!(p.array.len(), 10);
        p.push_clues_column(vec![1]);
        assert_eq!(p.array.len(), 15);
        p.push_clues_column(vec![1, 2]);
        assert_eq!(p.array.len(), 20);
        p.push_clues_column(vec![1]);
        assert_eq!(p.array.len(), 25);
    }

    //#[test]
    fn test_solver() {
        let mut p = Puzzle::new();
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![0]);
        p.push_clues_row(vec![1, 1]);
        p.push_clues_row(vec![3]);
        p.push_clues_column(vec![1]);
        p.push_clues_column(vec![1, 2]);
        p.push_clues_column(vec![1]);
        p.push_clues_column(vec![1, 2]);
        p.push_clues_column(vec![1]);
        p.solve();

        let solution = Puzzle {
            array: vec![
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Filled,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled,
                Cell::Empty,
            ],
            row_clues: vec![vec![1, 1], vec![1, 1], vec![0], vec![1, 1], vec![3]],
            column_clues: vec![vec![1], vec![2, 1], vec![1], vec![2, 1], vec![1]],
        };

        for (i, v) in solution.array.iter().enumerate() {
            match v {
                Cell::Filled => assert_eq!(p.array[i], *v),
                _ => continue,
            }
        }
    }
}
