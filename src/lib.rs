use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
pub struct Numple {
    pub numbers: [[u32; 9]; 9],
}

impl Numple {
    pub fn new(filename: &str) -> Result<Numple, std::io::Error> {
        let mut data: [[u32; 9]; 9] = [[0; 9]; 9];
        let f = File::open(filename)?;

        let buffer = BufReader::new(f);
        for (i, buf) in buffer.lines().enumerate() {
            let xline: Vec<u32> = buf
                .expect("Failed to read line")
                .trim()
                .split_whitespace()
                .map(|n| n.to_string().parse::<u32>().expect("Failed to parse"))
                .collect();

            for (j, num) in xline.into_iter().enumerate() {
                data[i][j] = num;
            }
        }
        Ok(Numple { numbers: data })
    }

    fn check_number(&self, i: usize, j: usize, number: u32) -> bool {
        for x in 0..9 {
            if number == self.numbers[i][x] {
                return false;
            }
        }

        for y in 0..9 {
            if number == self.numbers[y][j] {
                return false;
            }
        }

        let block_i: usize = (i / 3) * 3;
        let block_j: usize = (j / 3) * 3;

        for x in 0..3 {
            for y in 0..3 {
                if number == self.numbers[block_i + y][block_j + x] {
                    return false;
                }
            }
        }
        true
    }

    pub fn put_number(&mut self, i: usize, j: usize) -> bool {
        if i > 8 {
            return true;
        } else if self.numbers[i][j] != 0 {
            if j == 8 {
                if self.put_number(i + 1, 0) {
                    return true;
                }
            } else if self.put_number(i, j + 1) {
                return true;
            }
        } else {
            for num in 1..=9 {
                if self.check_number(i, j, num) {
                    self.numbers[i][j] = num;
                    if j == 8 {
                        if self.put_number(i + 1, 0) {
                            return true;
                        }
                    } else if self.put_number(i, j + 1) {
                        return true;
                    }
                }
                self.numbers[i][j] = 0;
            }
            return false;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let problem = Numple::new("test_problem.txt").unwrap();
        let check: [[u32; 9]; 9] = [
            [0, 0, 5, 3, 0, 0, 0, 0, 0],
            [8, 0, 0, 0, 0, 0, 0, 2, 0],
            [0, 7, 0, 0, 1, 0, 5, 0, 0],
            [4, 0, 0, 0, 0, 5, 3, 0, 0],
            [0, 1, 0, 0, 7, 0, 0, 0, 6],
            [0, 0, 3, 2, 0, 0, 0, 8, 0],
            [0, 6, 0, 5, 0, 0, 0, 0, 9],
            [0, 0, 4, 0, 0, 0, 0, 3, 0],
            [0, 0, 0, 0, 0, 9, 7, 0, 0],
        ];

        assert_eq!(check, problem.numbers);
    }

    #[test]
    fn it_not_works() {
        let f = Numple::new("wrong_file.txt");
        if let Err(e) = f {
            eprintln!("{}", &e);
            assert_eq!(
                "No such file or directory (os error 2)".to_string(),
                e.to_string()
            );
        }
    }
}
