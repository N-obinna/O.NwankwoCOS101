use std::io;

fn main() {
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!(" Public Service APS Level Checker ");
    println!("Select Profession:");
    println!("1. Office Admin\n2. Academic\n3. Lawyer\n4. Teacher");

    let mut choice_str = String::new();
    io::stdin().read_line(&mut choice_str).expect("Failed to read choice");
    let choice: usize = choice_str.trim().parse().

    let mut role_input = String::new();
    println!("Enter your position/role title:");
    io::stdin().read_line(&mut role_input).expect("Failed to read role");
    let role = role_input.trim();


    let selected_vector = match choice {
        1 => office_admin,
        2 => academic,
        3 => lawyer,
        4 => teacher,
        _ => vec![],
    };

    let mut found = false;


    for i in 0..selected_vector.len() {
        if selected_vector[i].to_lowercase() == role.to_lowercase() {
            println!("\nStaff Position: {}", selected_vector[i]);
            println!("Validated Level: {}", levels[i]);
            found = true;
            break;
        }
    }

    if !found {
        println!("\nRole not found in the public service table.");
    }
}