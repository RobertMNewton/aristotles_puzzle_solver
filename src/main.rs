use std::collections::HashSet;
use itertools::Itertools;
use std::time::Instant;

fn generate_valid_rows(length: usize, target: i32, values: Vec<i32>) -> Vec<Vec<i32>> {
    values
        .iter()
        .copied()
        .permutations(length)
        .filter(|combo| combo.iter().sum::<i32>() == target)
        .collect()
}

fn solve(
    grid: &mut Vec<Option<i32>>,
    solns: &mut Vec<Vec<Option<i32>>>,
    indices: &[Vec<usize>],
    used: &mut HashSet<i32>,
    i: usize,
    candidates: &[Vec<i32>],
) -> bool {
    if i == indices.len() {
        return true;
    }
    
    let filtered_candidates: Vec<&Vec<i32>> = candidates
        .iter()
        .filter(|candidate| {
            candidate.iter().zip(indices[i].iter()).all(|(&num, &j)| {
                (grid[j].is_none() && !used.contains(&num)) || 
                (grid[j].unwrap_or(-1) == num && used.contains(&num))
            })
        })
        .collect();
    
    for candidate in filtered_candidates {
        let mut placed_indices: Vec<usize> = Vec::new();
        
        for (&num, &j) in candidate.iter().zip(indices[i].iter()) {
            if grid[j].is_some() {
                continue;
            }
            grid[j] = Some(num);
            used.insert(num);
            placed_indices.push(j);
        }
        
        // Important: pass the correct arguments
        if solve(grid, solns, indices, used, i+1, candidates) {
            solns.push(grid.clone());
        }
        
        // Backtrack
        for j in placed_indices {
            used.remove(&grid[j].unwrap());
            grid[j] = None;
        }
    }
    
    false
}

fn main() {
    let start = Instant::now();

    let mut grid: Vec<Option<i32>> = vec![None; 19];
    let mut used: HashSet<i32> = HashSet::new();
    
    let _3_long_perms = generate_valid_rows(3, 38, (1..=19).collect());
    let outer_ring: Vec<Vec<usize>> = vec![
        vec![0, 1, 2], vec![2, 6, 11],
        vec![11, 15, 18], vec![18, 17, 16],
        vec![16, 12, 7], vec![7, 3, 0]
    ];
    
    let mut outer_ring_solns: Vec<Vec<Option<i32>>> = Vec::new();
    solve(&mut grid, &mut outer_ring_solns, &outer_ring, &mut used, 0, &_3_long_perms);
    
    println!("Found {} outer rings", outer_ring_solns.len());
    
    let inner_ring = vec![
        vec![3, 4, 5, 6], vec![12, 13, 14, 15],
        vec![3, 8, 13, 17], vec![1, 5, 10, 15],
        vec![1, 4, 8, 12], vec![6, 10, 14, 17],
    ];
    
    let _4_long_perms = generate_valid_rows(4, 38, (1..=19).collect());
    let mut inner_ring_solns: Vec<Vec<Option<i32>>> = Vec::new();
    
    for outer_ring_soln in outer_ring_solns {
        let mut used: HashSet<i32> = outer_ring_soln.iter()
            .filter_map(|&num| num)
            .collect();
        
        let mut inner_grid = outer_ring_soln.clone();
        
        solve(&mut inner_grid, &mut inner_ring_solns, &inner_ring, &mut used, 0, &_4_long_perms);
    }
    
    println!("Found {} inner ring solutions", inner_ring_solns.len());

    let centre_indices = vec![
        vec![0, 4, 9, 14, 18], vec![2, 5, 9, 13, 16],
        vec![7, 8, 9, 10, 11],
    ];

    let _5_long_perms = generate_valid_rows(5, 38, (1..=19).collect());
    let mut solutions: Vec<Vec<Option<i32>>> = Vec::new();

    for inner_ring_soln in inner_ring_solns {
        let mut used: HashSet<i32> = inner_ring_soln.iter()
            .filter_map(|&num| num)
            .collect();
        
        let mut grid = inner_ring_soln.clone();
        
        solve(&mut grid, &mut solutions, &centre_indices, &mut used, 0, &_5_long_perms);
    }

    println!("Found {} solutions to Aristotle's puzzle", solutions.len());

    for grid in solutions.iter() {
        // Hexagonal row widths: 3, 4, 5, 4, 3
        let row_widths = [3, 4, 5, 4, 3];
        let mut current_row = 0;
        let mut current_row_index = 0;
    
        while current_row < row_widths.len() {
            let row_width = row_widths[current_row];
            let max_row_width = *row_widths.iter().max().unwrap();
            let padding = max_row_width - row_width;
    
            // Top of hexagons
            if current_row_index == 0 {
                print!("{:width$}", "", width = padding * 4);
            }
            for _ in 0..row_width {
                print!("  ___   ");
            }
            println!();
    
            // Middle part with values
            if current_row_index == 0 {
                print!("{:width$}", "", width = padding * 4);
            }
            for i in 0..row_width {
                let index = row_widths[..current_row].iter().sum::<usize>() + i;
                let value = grid.get(index).unwrap_or(&None).unwrap_or(-1);
                print!(" /{:^3}\\  ", value);
            }
            println!();
    
            // Bottom of hexagons
            if current_row_index == 0 {
                print!("{:width$}", "", width = padding * 4);
            }
            for _ in 0..row_width {
                print!(" \\___/  ");
            }
            println!();
    
            current_row += 1;
            current_row_index = 0;
        }
    
        // Extra newline between grids
        println!();
        println!();
    }
    
    

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}