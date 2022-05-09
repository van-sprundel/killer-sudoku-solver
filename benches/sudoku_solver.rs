extern crate core;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::Duration;

use killer_sudoku::data::board::{Board, Difficulty};

fn bench(c: &mut Criterion) {
    let mut board = Board::sample_puzzle();

    c.bench_function("brute force", |b| {
        b.iter(|| {
            if !simple_brute_force_function(&mut board, 0) {
                panic!("Solution not found")
            }
        })
    });
    c.bench_function("brute force + backtracking", |b| {
        b.iter(|| {
            if !brute_force_with_backtracking(&mut board) {
                panic!("Solution not found")
            }
        })
    });
}

/// Iters over all possibilities, unoptimized
fn simple_brute_force_function(mut board: &mut Board, i: usize) -> bool {
    if i >= 81 {
        return board.is_finished();
    }
    let (x, y) = (i % 9, i / 9);

    if board.get_value(x, y) != 0 {
        return simple_brute_force_function(board, i + 1);
    }

    // since the biggest cell size is 4 in this example, the range of sums is 3..30
    let mut range = 1..=9;

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
                return true;
            } else {
                board.set_num(x, y, 0);
            }
        }
    }

    false
}

criterion_group!(benches, bench);
criterion_main!(benches);
