use std::fs::read_to_string;

use super::cell::MatrixCell;

#[derive(Default, Debug, Clone)]
pub struct MapMatrix {
    pub height: usize,
    pub width: usize,
    pub matrix: Vec<Vec<MatrixCell>>
}

impl MapMatrix {
    pub fn new(height: usize, width: usize, matrix: Vec<Vec<MatrixCell>>) -> Self {
        MapMatrix {
            height,
            width,
            matrix,
        }
    }

    pub fn load_matrix_from_file(path: &str) -> MapMatrix {

        let mut matrix: Vec<Vec<MatrixCell>> = Vec::new();

        let lines: Vec<String> = read_to_string(path) 
            .unwrap()  // panic on possible file-reading errors
            .lines()  // split the string into an iterator of string slices
            .map(String::from)  // make each slice into a string
            .collect();

        let (height, lines) = lines.split_first().unwrap();
        let height: usize = height.parse().unwrap();

        let (width, lines) = lines.split_first().unwrap();
        let width: usize = width.parse().unwrap();

        for i in 0..height {
            let mut line: Vec<MatrixCell> = Vec::new();
            for j in 0..width {
                let char = lines[i].chars().collect::<Vec<char>>()[j];
                if char == 'W' {
                    line.push(MatrixCell::wall());
                    continue;
                }
                if char == 'w' {
                    line.push(MatrixCell::pathway());
                    continue;
                }
                if char == 'S' {
                    line.push(MatrixCell::pacman());
                    continue;
                }
                if char == 'H' {
                    line.push(MatrixCell::ghost());
                    continue;
                }
                if char == 'P' {
                    line.push(MatrixCell::point());
                    continue;
                }
                if char == 'B' {
                    line.push(MatrixCell::bonus());
                    continue;
                }

                panic!()
            }

            matrix.push(line);
        }

        MapMatrix {
            height,
            width,
            matrix
        }
    }
}