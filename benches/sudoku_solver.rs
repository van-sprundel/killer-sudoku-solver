extern crate core;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::Duration;

use killer_sudoku::data::board::{Board, Difficulty};
use killer_sudoku::data::cage::Cage;
use killer_sudoku::data::cage_table::CageTable;

fn bench(c: &mut Criterion) {
    let board = Board::sample_puzzle();
    let mut cage_table = CageTable::from_file("./assets/cage_table.txt");

    let mut group = c.benchmark_group("solver");
    group.bench_function("brute force", |b| {
        b.iter(|| {
            if !simple_brute_force_function(&mut board.clone(), 0) {
                panic!("Solution not found")
            }
        })
    });
    group.bench_function("brute force + backtracking", |b| {
        b.iter(|| {
            if !brute_force_with_backtracking(&mut board.clone()) {
                panic!("Solution not found")
            }
        })
    });

    group.bench_function("brute force + backtracking + cage table", |b| {
        b.iter(|| {
            if !brute_force_with_backtracking_and_cage_table(&mut board.clone(), &mut cage_table) {
                panic!("Solution not found")
            }
        })
    });
    group.finish();
}

/// Iters over all possibilities, unoptimized
fn simple_brute_force_function(board: &mut Board, i: usize) -> bool {
    if i >= 81 {
        // board.print();
        return board.is_finished();
    }
    let (x, y) = (i % 9, i / 9);

    if board.get_value(x, y) != 0 {
        return simple_brute_force_function(board, i + 1);
    }


    for num in 1..=9 {
        if board.is_possible(x, y, num) {
            board.unchecked_set_num(x, y, num);
            if simple_brute_force_function(board, i + 1) {
                return true;
            }
        }
        board.unchecked_set_num(x, y, 0); // remove wrong input
    }
    return false;
}

/// brute force with backtracking
/// finds the first value that's zero, then tries every possibility
// time O(9^81)
fn brute_force_with_backtracking(board: &mut Board) -> bool {
    let (mut x, mut y) = (0, 0);
    let mut is_empty = true;

    for j in 0..9 {
        for k in 0..9 {
            if board.get_value(j, k) == 0 {
                x = j;
                y = k;

                is_empty = true;
                break;
            }
        }
        if !is_empty {
            break;
        }
    }
    if is_empty {
        return true;
    }
    for num in 1..=9 {
        if board.is_possible(x, y, num as i32) {
            board.unchecked_set_num(x, y, num as i32);

            if brute_force_with_backtracking(board) {
                // board.print();
                return true;
            } else {
                board.set_num(x, y, 0);
            }
        }
    }

    false
}

fn brute_force_with_backtracking_and_cage_table(board: &mut Board, cage_table: &mut CageTable) -> bool {
    let (mut x, mut y) = (0, 0);
    let mut is_empty = true;

    for j in 0..9 {
        for k in 0..9 {
            if board.get_value(j, k) == 0 {
                x = j;
                y = k;

                is_empty = true;
                break;
            }
        }
        if !is_empty {
            break;
        }
    }

    if is_empty {
        return true;
    }

    let size = board.get_cage_size(x, y) as i32;
    let sum = board.get_sum(x, y);
    let possibilities = cage_table.find(size, sum).unwrap();
    for combination in possibilities.combinations {
        for num in combination {
            if board.is_possible(x, y, num as i32) {
                board.unchecked_set_num(x, y, num as i32);

                if brute_force_with_backtracking_and_cage_table(board, cage_table) {
                    // board.print();
                    return true;
                } else {
                    board.set_num(x, y, 0);
                }
            }
        }
    }

    false
}

criterion_group!(benches, bench);
criterion_main!(benches);
