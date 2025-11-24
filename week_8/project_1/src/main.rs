// Define a structure for an APS level
struct ApsLevel {
    level_range: &'static str,
    min_exp: u8, // Minimum years of experience
    max_exp: u8, // Maximum years of experience
    office_administrator: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

// Function to check the staff's APS level
fn check_aps_level(
    profession: &str,
    title: &str,
    years_of_experience: u8,
    levels_data: &Vec<ApsLevel>,
) -> String {
    // Iterate through the predefined levels
    for level in levels_data {
        let title_matches = match profession {
            "Office Administrator" => title == level.office_administrator,
            "Academic" => title == level.academic,
            "Lawyer" => title == level.lawyer,
            "Teacher" => title == level.teacher,
            _ => false, // Profession not found
        };

        // Check if the title matches AND the experience is within the range
        if title_matches {
            // Check for valid experience range
            if years_of_experience >= level.min_exp && years_of_experience <= level.max_exp {
                return format!(
                    "The staff holds position {}.",
                    level.level_range
                );
            } else {
                // Title matches, but experience is wrong for this level
                return format!(
                    "Error: The title '{}' is correct for level {} but requires {}-{} years of experience. Staff has {} years.",
                    title, level.level_range, level.min_exp, level.max_exp, years_of_experience
                );
            }
        }
    }

    // If the loop finishes without a match
    format!("Error: Could not find a matching level for profession '{}' and title '{}'.", profession, title)
}

fn main() {
    // 2. Create a Vector representing the APS table
    // NOTE: Experience ranges are estimated/derived from the prompt's example for APS 5-8 (5-8 years).
    // You would need to define the correct experience ranges for ALL levels.
    let aps_levels: Vec<ApsLevel> = vec![
        ApsLevel {
            level_range: "APS 1-2",
            min_exp: 0,
            max_exp: 1,
            office_administrator: "Intern",
            academic: "—", // Using an em dash for the 'Academic' column blank
            lawyer: "Paralegal",
            teacher: "Placement",
        },
        ApsLevel {
            level_range: "APS 3-5",
            min_exp: 2,
            max_exp: 4,
            office_administrator: "Administrator",
            academic: "Research Assistant",
            lawyer: "Junior Associate",
            teacher: "Classroom Teacher",
        },
        ApsLevel {
            level_range: "APS 5-8",
            min_exp: 5, // Derived from prompt's example
            max_exp: 8, // Derived from prompt's example
            office_administrator: "Senior Administrator",
            academic: "PhD Candidate",
            lawyer: "Associate",
            teacher: "Snr Teacher",
        },
        // ... (Add the rest of the levels: EL1 8-10, EL2 10-13, SES)
    ];

    // 3. Test Cases
    
    // ✅ Valid Case: Associate Lawyer with 7 years experience
    let result1 = check_aps_level("Lawyer", "Associate", 7, &aps_levels);
    println!("Associate Lawyer (7 yrs): {}", result1); 
    // Expected Output: The staff holds position APS 5-8.

    // ❌ Invalid Experience Case: Associate Lawyer with 3 years experience
    let result2 = check_aps_level("Lawyer", "Associate", 3, &aps_levels);
    println!("Associate Lawyer (3 yrs): {}", result2);
    // Expected Output: Error: The title 'Associate' is correct for level APS 5-8 but requires 5-8 years of experience. Staff has 3 years.
    
    // ✅ Valid Case: Intern with 0 years experience
    let result3 = check_aps_level("Office Administrator", "Intern", 0, &aps_levels);
    println!("Intern (0 yrs): {}", result3);
    
    // ❌ Invalid Title Case
    let result4 = check_aps_level("Lawyer", "Senior Associate 1-2", 10, &aps_levels);
    println!("Senior Associate 1-2 (10 yrs): {}", result4);
}
