
// Question 1 for ca test
use std::io;

fn main() {
    for _ in 0..50{ // loop 50 times for 50 students
    let mut input = String::new();

    // Student Voting Eligibility Check
    println!("Enter the Student's Name:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let student_name = input.trim().to_string();

    input.clear();
    println!("Enter the Student's CGPA:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let student_cgpa: f32 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Is the Student a Class Rep? (yes/no):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let is_class_rep = input.trim().to_lowercase() == "yes";

    input.clear();
    println!("Enter the Student's level (e.g., 200):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let student_level: u32 = input.trim().parse().expect("Please enter a valid number");

    // Additional fields for student information
    input.clear();
    println!("Enter the Student's Email:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let student_email = input.trim().to_string();

    input.clear();
    println!("Enter the Student's Department:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let student_department = input.trim().to_string();

    input.clear();
    println!("Enter the Student's State of Origin:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let student_state = input.trim().to_string();

    // Check eligibility
    if is_class_rep && student_level != 100 && student_cgpa > 4.0 {
        println!(
            "You can vote, {} (Email: {}, Department: {}, State: {})",
            student_name, student_email, student_department, student_state
        );
    } else {
        println!("Sorry, you are not eligible to vote");
    }

     // Pause to keep the terminal window open after typing the last input
     println!("Press Enter to exit...");
     input.clear();
     io::stdin().read_line(&mut input).expect("Failed to read line");
     
        }
        
     }

// Question 2 for ca test
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
    


