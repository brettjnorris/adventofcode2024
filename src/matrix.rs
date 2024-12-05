
pub struct Matrix {
    pub data: Vec<Vec<String>>
}

impl Matrix {
    pub fn from_string(input: &str) -> Self {
        let data: Vec<Vec<String>> = input.lines().map(|line| {
            let parsed_line = line.to_owned().chars().map(|c| {
                 c.to_string()
            }).collect::<Vec<String>>();
            parsed_line
        }).collect();

        Self {
            data
        }
    }

    fn generate_row_permutations(row_count: usize, col_count: usize) -> Vec<Vec<(usize, usize)>> {
        let mut indices = Vec::new();

        // Rows
        for row in 0..row_count {
            let row_indices: Vec<(usize, usize)> = (0..col_count).map(|col| (row, col)).collect();
            indices.push(row_indices);
        }

        // Columns
        for col in 0..col_count {
            let col_indices: Vec<(usize, usize)> = (0..row_count).map(|row| (row, col)).collect();
            indices.push(col_indices);
        }

        // Main Diagonals (Top-Left to Bottom-Right)
        for start_col in 0..col_count {
            let mut diagonal = Vec::new();
            let mut row = 0;
            let mut col = start_col;
            while row < row_count && col < col_count {
                diagonal.push((row, col));
                row += 1;
                col += 1;
            }
            indices.push(diagonal);
        }

        for start_row in 1..row_count {
            let mut diagonal = Vec::new();
            let mut row = start_row;
            let mut col = 0;
            while row < row_count && col < col_count {
                diagonal.push((row, col));
                row += 1;
                col += 1;
            }
            indices.push(diagonal);
        }

        // Anti-Diagonals (Top-Right to Bottom-Left)
        for start_col in (0..col_count).rev() {
            let mut anti_diagonal = Vec::new();
            let mut row = 0;
            let mut col = start_col;
            while row < row_count && col < col_count {
                anti_diagonal.push((row, col));
                row += 1;
                if col == 0 {
                    break;
                }
                col -= 1;
            }
            indices.push(anti_diagonal);
        }

        for start_row in 1..row_count {
            let mut anti_diagonal = Vec::new();
            let mut row = start_row;
            let mut col = col_count - 1;
            while row < row_count && col < col_count {
                anti_diagonal.push((row, col));
                row += 1;
                if col == 0 {
                    break;
                }
                col -= 1;
            }
            indices.push(anti_diagonal);
        }


        indices
    }

    pub fn get_row_strings(&self) -> Vec<String> {
        let mut row_strings: Vec<String> = Vec::new();
        for permutation in Matrix::generate_row_permutations(self.data.len(), self.data[0].len()) {
            let mut string_chars: Vec<String> = Vec::new();
            for (x, y) in permutation.iter() {
                string_chars.push(self.get_character_at_coordinates(*x, *y));
            }
            row_strings.push(string_chars.join(""));
        }

        row_strings
    }



    fn get_character_at_coordinates(&self, x: usize, y: usize) -> String {
        self.data[x][y].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_string() {
        let matrix = Matrix::from_string("HELLO\nWORLD");
        assert_eq!(matrix.data, vec![
            vec!["H", "E", "L", "L", "O"],
            vec!["W", "O", "R", "L", "D"]
        ]);
    }

    #[test]
    fn test_generate_row_permutations() {
        let permutations: Vec<Vec<(usize, usize)>> = Matrix::generate_row_permutations(3, 3);
        assert_eq!(permutations, vec![vec![(0, 1)]])
    }

    #[test]
    fn test_get_row_strings() {
        let matrix: Matrix = Matrix::from_string("0123\n4567\n8900");
        assert_eq!(matrix.get_row_strings(), vec!["012", "345", "678", "036", "147", "258", "6", "37", "048", "15", "2", "246", "57", "8"]);
    }
}