use std::io;

fn main() {
    for _ in 0..100{ // loop 100 times for 100 staffs
    let mut input = String::new();

        // Staff Publication Incentive Calculation (2nd Part of test)
        input.clear();
        println!("\nEnter the Staff Member's name:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let staff_name = input.trim().to_string();
    
        input.clear();
        println!("Enter the Number of Papers published by the Staff Member:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let papers_published: u32 = input.trim().parse().expect("Please enter a valid number");
    
        // Calculate incentive
        let incentive = if papers_published >= 10 {
            1_000_000
        } else if papers_published >= 6 {
            800_000
        } else if papers_published >= 3 {
            500_000
        } else {
            100_000
        };
    
        println!("{}'s incentive is: ₦{}", staff_name, incentive);
    
         // Pause to keep the terminal window open after typing the last input
         println!("Press Enter to exit...");
         input.clear();
         io::stdin().read_line(&mut input).expect("Failed to read line");
    }

    }