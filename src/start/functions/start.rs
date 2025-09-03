pub fn start() {
    println!("Welcome to my Portfolio CLI!");
    println!(
        "This application gives you a quick glance at my CV, experience, and contact information."
    );
    println!();
    println!("Available commands:");
    println!("  start                   - Show this introduction message");
    println!(
        "  cover-letter            - Here you can see the cover letter I have written for you"
    );
    println!("      --company-name/-c   - Provide your company name");
    println!("      --in-english/-i     - Display the cover letter in English");
    println!("  all                     - Display all information");
    println!("  personal-info           - Show personal information");
    println!("      --details/-d        - Show detailed personal info");
    println!("  contact-info            - Show contact information");
    println!("  experience-info         - Show work experience");
    println!("      --number/-n         - Specify the experience number");
    println!("      --details/-d        - Show detailed information about the experience");
    println!("  contact                 - Prepare contact details");
    println!("      --subject/-s        - Email subject");
    println!("      --email/-e          - Your email address");
    println!();
    println!("Use `--help` with any command to see more details.");
}
