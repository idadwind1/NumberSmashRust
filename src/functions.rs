use colored::*;
use std::io::Write;

// fn permutation_combinations(things: Vec<usize>, count_in_one_group: usize) -> Vec<Vec<usize>> {
//     let mut result = Vec::new();

//     if count_in_one_group == 1 {
//         for &item in &things {
//             result.push(vec![item]);
//         }
//         return result;
//     }

//     for i in 0..things.len() - 1 {
//         for j in (i + 1)..things.len() {
//             result.push(vec![things[i], things[j]]);
//         }
//     }

//     if count_in_one_group != 2 {
//         let temp = result.clone();
//         result.clear();
//         for group in temp {
//             // Create a temporary list excluding the first two elements
//             let mut tmp2: Vec<usize> = things.iter().skip(2).cloned().collect();
//             tmp2.insert(0, 0); // Placeholder for -114514 equivalent if needed

//             let sub_combinations = permutation_combinations(tmp2, count_in_one_group - 1);
//             for sub in sub_combinations {
//                 if !sub.contains(&0) || sub[1] <= group[1] {
//                     continue;
//                 }
//                 let mut new_group = vec![group[0], group[1]];
//                 for &item in &sub {
//                     if item != 0 {
//                         new_group.push(item);
//                     }
//                 }
//                 result.push(new_group);
//             }
//         }
//     }

//     result
// }

// pub fn get_valid_smashes(values: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut result: Vec<Vec<i32>> = Vec::new();
//     for i in 1..values.len() {
//         let combinations = permutation_combinations((0..values.len()).collect(), i);
//         for combination in combinations {
//             let possible: Vec<i32> = combination.iter().map(|&index| values[index].clone()).collect();
//             if possible.iter().sum::<i32>() == 0 {
//                 result.push(possible);
//             }
//         }
//     }
//     result
// }

/// Finds all possible subsets of `numbers` that sum to zero, regardless of subset size.
///
/// # Arguments
///
/// * `numbers` - A slice of integers to search within.
///
/// # Returns
///
/// A vector of vectors, each containing a subset of `numbers` that sums to zero.
pub fn find_valid_smashes(numbers: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let len = numbers.len();

    // Iterate over all possible subset sizes from 1 to len
    for subset_size in 1..=len {
        backtrack(0, subset_size, &mut Vec::with_capacity(subset_size), numbers, &mut results);
    }

    results
}

/// Recursive helper function to generate combinations of a specific size and check their sum.
///
/// # Arguments
///
/// * `start` - The starting index for the current recursion level.
/// * `subset_size` - The desired size of the subset.
/// * `current` - The current subset being built.
/// * `numbers` - The slice of numbers to choose from.
/// * `results` - The accumulator for valid subsets.
fn backtrack(
    start: usize,
    subset_size: usize,
    current: &mut Vec<i32>,
    numbers: &[i32],
    results: &mut Vec<Vec<i32>>,
) {
    if current.len() == subset_size {
        if current.iter().sum::<i32>() == 0 {
            results.push(current.clone());
        }
        return;
    }

    for i in start..numbers.len() {
        current.push(numbers[i]);
        backtrack(i + 1, subset_size, current, numbers, results);
        current.pop();
    }
}

pub fn number_to_string(num: i32, selected: bool) -> String {
    if num == 0 {
        if selected {
            return "0".to_string().on_yellow().italic().bold().to_string();
        }
        return "0".to_string().yellow().to_string();
    }
    if num < 0 {
        if selected {
            return ("(".to_string() + &num.to_string() + ")").on_red().italic().bold().to_string();
        }
        return ("(".to_string() + &num.to_string().red().to_string() + ")").red().to_string();
    }
    if selected {
        return num.to_string().on_green().italic().bold().to_string();
    }
    return num.to_string().green().to_string();
}

pub fn clear_console() {
    // clear console using char 27
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    std::io::stdout().flush().unwrap();
}

pub fn calculate_space(num: i32) -> String {
    if num < 0 {
        return "   ".to_string();
    }
    return "".to_string();
}