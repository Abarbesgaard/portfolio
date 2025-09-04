use crate::cv::structs::{bullet_point::BulletPoint, personalinformation::PersonalInformation};

impl PersonalInformation {
    pub fn info() -> Self {
        PersonalInformation {
            id: 1,
            first_name: "Andreas".to_string(),
            last_name: "Barbesgaard".to_string(),
            title: "Software Developer".to_string(),
            age: 36,
            tag_line: "C# dev with rusty tendencies".to_string(),
            short_description: "Passionate Software Developer with a strong foundation in .NET (C#), React, and TypeScript — currently exploring Rust with a growing interest in systems programming and high-performance applications.".to_string(),
            long_description: "I am a passionate Software Developer with a strong foundation in .NET (C#), React, and TypeScript — and I’m currently diving into Rust to broaden my expertise in systems programming and high-performance solutions.

My path into software is shaped by a background in pedagogy and years of mentoring, giving me strong communication skills, empathy, and a people-first mindset. I strive to build software that is not only clean and maintainable, but also genuinely useful and intuitive for its users.

I thrive in teams where curiosity, feedback, and innovation drive the process — building technology that makes a real difference in people’s lives.".to_string(),
            bullet_points: vec![
                BulletPoint { number: 1, description: "Solid experience with .NET (C#)".to_string() },
                BulletPoint { number: 2, description: "experience with Rust, React, and TypeScript".to_string() },
                BulletPoint { number: 3, description: "Hands-on work with full-stack projects focused on usability and simplicity".to_string() },
                BulletPoint { number: 4, description: "Currently learning Rust and applying it to real-world projects".to_string() },
                BulletPoint { number: 5, description: "Strong problem-solving skills fueled by curiosity and continuous learning".to_string() },
                BulletPoint { number: 6, description: "Background in education and mentorship — strong collaboration and communication skills".to_string() },
                BulletPoint { number: 7, description: "Focus on real-time collaboration and interactive systems".to_string() },
                BulletPoint { number: 8, description: "A proactive mindset: learn quickly, take initiative, and embrace iteration".to_string() },
            ],
        }
    }
}
