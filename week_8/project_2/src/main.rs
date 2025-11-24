// The required Rust compound data type
struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    // 1. Create the list of developers
    let candidates: Vec<Developer> = vec![
        Developer {
            name: "Aisha Mohammed".to_string(),
            years_of_experience: 6,
        },
        Developer {
            name: "Chukwudi Nze".to_string(),
            years_of_experience: 10,
        },
        Developer {
            name: "Tunde Oladipo".to_string(),
            years_of_experience: 4,
        },
        Developer {
            name: "Femi Adebayo".to_string(),
            years_of_experience: 12,
        },
        Developer {
            name: "Hauwa Idris".to_string(),
            years_of_experience: 8,
        },
    ];

    // Initialize tracking variables
    let mut highest_exp: u32 = 0;
    // We use Option<&Developer> because we might start with an empty list
    let mut most_experienced_developer: Option<&Developer> = None;

    // 2. Find the developer with the maximum experience
    for dev in &candidates {
        if dev.years_of_experience > highest_exp {
            highest_exp = dev.years_of_experience;
            most_experienced_developer = Some(dev);
        }
    }

    // 3. Output the result
    match most_experienced_developer {
        Some(dev) => {
            println!("--- EY Nigeria Developer Scouting Results ---");
            println!("The candidate with the highest years of programming experience is:");
            println!("Name: {}", dev.name);
            println!("Experience: {} years", dev.years_of_experience);
        }
        None => {
            println!("Error: No candidates were found in the list.");
        }
    }
}