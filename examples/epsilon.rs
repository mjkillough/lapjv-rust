extern crate pretty_assertions;
extern crate serde;
extern crate lapjv;

use std::fs::File;
use std::time::Instant;
use std::fmt;

use pretty_assertions::{assert_eq};
use serde::de::DeserializeOwned;

use lapjv::{LapJVCost, Matrix};

fn main() {
    // Remove .0 to compare swaps and cost of each swap:
    assert_eq!(test::<f64>(false).0, test::<f64>(true).0);
}

fn test<T>(epsilon: bool) -> (T, Vec<(usize, usize, T)>)
where
    T: LapJVCost + DeserializeOwned + std::iter::Sum + fmt::Display,
{
    let time = Instant::now();

    let mut matrix = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(File::open("matrix.csv").unwrap());
    for result in rdr.deserialize() {
        let row: Vec<T> = result.unwrap();
        matrix.extend(row);
    }

    println!("time to load matrix: {:?}", time.elapsed());

    let time = Instant::now();

    let size = (matrix.len() as f64).sqrt() as usize; // assume integer
    let matrix = Matrix::from_shape_vec((size, size), matrix).unwrap();

    println!("dimensions = {} x {}", size, size);
    println!("matrix calc: {:?}", time.elapsed());

    let (solution, _) = lapjv::lapjv(&matrix, epsilon).unwrap();

    println!("solution: {:?}", time.elapsed());

    println!("");
    println!("");

    let swaps = (0..size)
        .zip(solution)
        .map(|(row, col)| (row, col as usize));
    let costs: Vec<_> = swaps.map(|(row, col)| (row, col, matrix[(row, col)])).collect();
    let cost = costs.iter().map(|(_row, _col, cost)| *cost).sum();

    println!("cost = {}", cost);

    (cost, costs)
}