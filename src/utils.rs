use simplelog::*;
use std::fmt::{Display, Write};
use std::fs::{self, OpenOptions};
use std::sync::mpsc::Receiver;
use std::time::Instant;
use std::usize;

pub struct Log {
    pub start: Instant,
    pub solver_design: String,
    pub puzzle_complexity: String,
    pub solution: String,
}

pub fn time_track(rx: Receiver<Log>) {
    while let Ok(log) = rx.recv() {
        println!("Logger received a msg");
        let elapsed = log.start.elapsed();
        log::info!(
            "Puzzle: {}, Solver: {}, Time taken: {:.2?}, Solution:\n{}",
            log.puzzle_complexity,
            log.solver_design,
            elapsed,
            log.solution
        );
    }
}

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    _ = fs::remove_file("solver.log");
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("solver.log")?;

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        log_file,
    )])?;

    Ok(())
}

#[derive(Clone)]
pub struct Matrix<T: Default + Display, const N: usize> {
    matrix: [[T; N]; N],
}

impl<T: Default + Display, const N: usize> Matrix<T, N> {
    pub fn row_neighbors_iter(&self, row: usize) -> impl Iterator<Item = &T> {
        assert!(row < N);
        self.matrix[row].iter()
    }

    pub fn clm_neighbors_iter(&self, clm: usize) -> impl Iterator<Item = &T> {
        assert!(clm < N);
        self.matrix.iter().map(move |row| &row[clm])
    }

    pub fn box_neighbors_iter(&self, row: usize, clm: usize) -> impl Iterator<Item = &T> {
        assert!(row < N);
        assert!(clm < N);
        let box_size = N / 3;
        let upper_row: usize = row / box_size * box_size;
        let leftmost_clm = clm / box_size * box_size;

        (0..box_size).flat_map(move |r| {
            (0..box_size).map(move |c| &self.matrix[upper_row + r][leftmost_clm + c])
        })
    }

    pub fn get(&self, row: usize, clm: usize) -> &T {
        assert!(row < N);
        assert!(clm < N);
        &self.matrix[row][clm]
    }

    pub fn get_mut(&mut self, row: usize, clm: usize) -> &mut T {
        assert!(row < N);
        assert!(clm < N);
        &mut self.matrix[row][clm]
    }

    pub fn new() -> Matrix<T, N> {
        Matrix {
            matrix: std::array::from_fn(|_| std::array::from_fn(|_| T::default())),
        }
    }
}

impl<T: Default + Display, const N: usize> ToString for Matrix<T, N> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        for i in 0..N {
            for j in 0..N {
                let _ = output.write_str(format!("{} ", self.matrix[i][j]).as_str());
            }
            output.push('\n');
        }
        output
    }
}
