use rand::Rng;
use std::io::Write;
use colored::*;
mod functions;
use functions::*;

macro_rules! generate_number {
    () => {
        rand::thread_rng().gen_range(-9..=9)
    };
}

fn generate_numbers(count: i32) -> Vec<i32> {
    let mut nums = Vec::new();
    for _ in 0..count {
        let num = generate_number!();
        nums.push(num);
    }
    return nums;
}

fn main() {
    let num_count = 7;
    let mut nums = generate_numbers(num_count);

    clear_console();
 
    let mut selected_numbers: Vec<i32> = Vec::new();
    let mut selected_index: Vec<i32> = Vec::new();
    let mut selected_sum: i32 = 0;

    let mut info = "Type h for help".to_string();
    let mut lose = false;
    
    let mut score = 0;
    let mut help_used_time = 0;

    loop {
        clear_console();

        if lose {
            println!("{}", "You lose!".red().to_string());
            println!("{}", info.red().to_string());
            println!("{}", "Press r to reset".red().to_string());
        }
        else {
            if !info.is_empty(){
                println!("{}", info.blue().to_string());
                info = String::new();
            }
            let mut dump_index = -1;
            // Print the numbers
            println!("{}", nums.iter().map(|n| {
                dump_index += 1;
                number_to_string(*n, selected_index.contains(&dump_index))
            }).collect::<Vec<_>>().join(" + "));
            dump_index = -1;
            println!(
                "{}",
                nums.iter()
                .map(|n| {
                    dump_index += 1;
                    dump_index.to_string() + &calculate_space(*n)
                })
                .collect::<Vec<_>>()
                .join("   ")
            );
        }

        println!("Score: {}", score.to_string().magenta());
        println!("Help used time: {}", help_used_time.to_string().magenta());

        // Input command
        print!(": ");
        std::io::stdout().flush().unwrap();
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();

        // Split the command into arguments
        let args = command.trim().split(" ").collect::<Vec<_>>();

        // Check the command
        if args.is_empty() || command.trim().is_empty() {
            continue;
        }
        else if args[0] == "q" {
            return;
        }
        else if (args[0] == "i" || args[0] == "n") && !lose {
            // Submit selected numbers
            if args[0] == "i" {
                for arg in args.iter() {
                    if let Ok(index) = arg.parse::<usize>() {
                        if let Some(&num) = nums.get(index) {
                            if selected_index.contains(&(index as i32)) {
                                if let Some(pos) = selected_numbers.iter().position(|&x| x == num) {
                                    selected_numbers.remove(pos);
                                }
                                if let Some(pos) = selected_index.iter().position(|&x| x == index as i32) {
                                    selected_index.remove(pos);
                                }
                                selected_sum -= num;
                                continue;
                            }
                            selected_numbers.push(num);
                            selected_index.push(index as i32);
                            selected_sum += num;
                        }
                    }
                }
            }
            else if args[0] == "n" {
                // Select numbers by value
                for arg in args.iter() {
                    if let Ok(num) = arg.parse::<i32>() {
                        if let Some(index) = nums.iter().position(|&x| x == num) {
                            if selected_numbers.contains(&num) {
                                if let Some(pos) = selected_numbers.iter().position(|&x| x == num) {
                                    selected_numbers.remove(pos);
                                    selected_index.remove(pos);
                                }
                                selected_sum -= num;
                                continue;
                            }
                            selected_numbers.push(num);
                            selected_index.push(index as i32);
                            selected_sum += num;
                        }
                    }
                }
            }
            info = selected_numbers.len().to_string() + " numbers selected";
        }
        else if args[0] == "rs" && !lose {
            selected_numbers.clear();
            selected_index.clear();
            selected_sum = 0;
        }
        else if args[0] == "as" && !lose {
            // Select all numbers
            selected_numbers.clear();
            selected_index.clear();
            selected_sum = 0;
            for index in 0..nums.len() {
                selected_numbers.push(nums[index]);
                selected_index.push(index as i32);
                selected_sum += nums[index];
            }
        }
        else if args[0] == "s" && !lose {
            if selected_numbers.is_empty() {
                info = "No numbers selected".to_string();
                continue;
            }

            // Check if the sum of selected numbers is equal to 0
            if selected_sum == 0 {
                info = "Valid Smash".to_string();
                for index in selected_index.iter() {
                    nums[*index as usize] = generate_number!();
                }
                score += 1;
                
                selected_numbers.clear();
                selected_index.clear();
                selected_sum = 0;
            }
            else {
                info = format!("Invalid Smash, {} = {}",
                    selected_numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" + "),
                    selected_sum.to_string());
                lose = true;
            }
        }
        else if args[0] == "ss" && !lose {
            if selected_numbers.is_empty() {
                info = "No numbers selected".to_string();
                continue;
            }
            if selected_sum == 0 {
                info = "Sum: ".to_string() + &"0".green().to_string();
                continue;
            }
            info = "Sum: ".to_string() + &selected_sum.to_string().to_string();
        }
        // else if args[0] == "e" {
        //     // print args
        //     info = format!("{:?}", args);
        // }
        else if args[0] == "h" || args[0] == "?" {
            // Create a dictionary to store commands
            let commands = std::collections::HashMap::from([
                ("q", "Quit the game"),
                ("r", "Reset the game"),
                ("i", "Select numbers by index"),
                ("n", "Select numbers by value"),
                ("as", "Select all numbers"),
                ("rs", "Reset selection"),
                ("s", "Smash selected numbers"),
                ("c", "Check if a valid smash is possible"),
                ("ss", "Show sum of selected numbers"),
                ("sh", "Get smash help"),
                ("h/?", "Show command list")
            ]);

            info = "Commands:".to_string();
            // Command list
            for (command, description) in commands.iter() {
                info = info + &format!("\n{}: {}", command.to_string().bright_blue().bold(), description);
            }
        }
        else if args[0] == "r" {
            nums = generate_numbers(num_count);
            lose = false;
            score = 0;
            help_used_time = 0;
            info = "Resetted game".to_string();
        }
        else if args[0] == "sh" {
            //info = "Not implemented yet".to_string();
            let valid_smashes = find_valid_smashes(&nums);
            info = "Valid smashes:\n".to_string();
            // paint every number in light blue and then dump in multi line
            for smash in valid_smashes.iter() {
                info += &smash.iter().map(|n| number_to_string(*n, false)).collect::<Vec<_>>().join(" + ");
                info += "\n";
            }
            info += "---------------";
            help_used_time += 1;
        }
        else if args[0] == "c" {
            //info = "Check not implemented yet".to_string();
            let valid_smashes = find_valid_smashes(&nums);

            if valid_smashes.is_empty() {
                info = "No valid smashes found, refreshed".to_string();
                nums = generate_numbers(num_count);
                score += 1;
            }
            else {
                info = "valid smashes exist:".to_string();
                for smash in valid_smashes.iter() {
                    info = info + &format!("\n{}", smash.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" + "));
                }
                lose = true;
            }
        }
        else {
            info = format!("Unknown command {}, type ch for command help", args[0]);
        }
    }
}
