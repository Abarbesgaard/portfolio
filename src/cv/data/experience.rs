use crate::cv::structs::{experience::Experience, link::Link, skill::Skill};

impl Experience {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        start_date: String,
        end_date: String,
        detailed_description: String,
        duration: String,
        skills: Vec<Skill>,
        link: Link,
        company_name: String,
    ) -> Experience {
        Experience {
            id,
            title,
            description,
            start_date,
            end_date,
            detailed_description,
            duration,
            skills,
            link,
            company_name,
        }
    }

    pub fn info() -> Vec<Experience> {
        vec![
            Experience::new(
                1,
                "Mentor".to_string(),
                "I am a mentor for six individuals in computer science (datamatiker) programs across Denmark.".to_string(),
                "01-03-2025".to_string(),
                "01-06-2025".to_string(),
                "As a mentor, I supported students in their professional and academic development through regular one-on-one sessions and ongoing guidance. 
My focus was on helping them strengthen technical skills, improve problem-solving strategies, and build confidence in real-world software development practices. 
In addition to academic support, I emphasized personal growth by encouraging goal setting, reflective learning, and career-oriented discussions. 
This role required adaptability, patience, and strong communication skills to meet the unique needs of each mentee, ensuring both their professional and personal progress.".to_string(),
                "4 months".to_string(),
                vec![
                    Skill::new(
                        11,
                        "Relationship building".to_string(),
                        "Building trust and strong connections".to_string(),
                    ),
                    Skill::new(
                        12,
                        "Coaching".to_string(),
                        "Guiding others toward growth".to_string(),
                    ),
                    Skill::new(
                        13,
                        "Teaching".to_string(),
                        "Explaining concepts in a clear way".to_string(),
                    ),
                ],
                Link::new(
                    1,
                    "Duos".to_string(),
                    "https://duos.dk/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "DUOS".to_string(),
            ),
            Experience::new(
                2,
                "Intern".to_string(),
                "Here, I had the role of fullstack developer, where I helped implement complete features from front end to back end into their system.".to_string(),
                "01-01-2025".to_string(),
                "01-03-2025".to_string(),
                "During my internship at BEMIIT ApS, I worked as a fullstack developer on real production systems. 
My responsibilities included implementing features across the entire stack, from designing user-facing functionality with jQuery to building robust backend services using Entity Framework. 
This role gave me hands-on experience in delivering complete features, understanding cross-team workflows, and adapting to a professional agile environment.".to_string(),
                "3 months".to_string(),
                vec![
                    Skill::new(
                        21,
                        "Entity Framework".to_string(),
                        "Practical use in data access and modeling".to_string(),
                    ),
                    Skill::new(
                        22,
                        "jQuery".to_string(),
                        "Enhancing UI with interactive components".to_string(),
                    ),
                    Skill::new(
                        23,
                        "Fullstack development".to_string(),
                        "Delivering features across frontend and backend".to_string(),
                    ),
                ],
                Link::new(
                    2,
                    "BEMIIT ApS".to_string(),
                    "https://bemiit.dk/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "BEMIIT ApS".to_string(),
            ),
            Experience::new(
                3,
                "Student Assistant".to_string(),
                "I help support new students in their development toward becoming better programmers.".to_string(),
                "01-03-2024".to_string(),
                "01-01-2025".to_string(),
                "As a student assistant at UCL, I supported incoming computer science students in building a solid programming foundation. 
My work involved answering technical questions, reviewing assignments, and offering guidance on study techniques. 
Beyond coding, I also focused on soft skills such as communication, independence, and collaboration — all key aspects of helping students succeed academically and personally.".to_string(),
                "11 months".to_string(),
                vec![
                    Skill::new(
                        31,
                        "Communication".to_string(),
                        "Explaining technical concepts clearly".to_string(),
                    ),
                    Skill::new(
                        32,
                        "Self-leadership".to_string(),
                        "Taking initiative and managing responsibilities".to_string(),
                    ),
                    Skill::new(
                        33,
                        "Student support".to_string(),
                        "Helping others progress in their studies".to_string(),
                    ),
                ],
                Link::new(
                    3,
                    "UCL Erhvervsakademi og Professionshøjskole".to_string(),
                    "https://ucl.dk/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "UCL Erhvervsakademi og Professionshøjskole".to_string(),
            ),
            Experience::new(
                4,
                "Tutor".to_string(),
                "In this position, I am responsible for ensuring the development of programming skills in an individual.".to_string(),
                "01-03-2024".to_string(),
                "01-06-2024".to_string(),
                "As a tutor, I worked one-on-one with a student to ensure steady progress in programming. 
The focus was on strengthening fundamental coding skills, problem-solving ability, and independent thinking. 
This role required patience, adaptability, and the ability to tailor explanations to the individual’s learning style, ensuring meaningful and lasting skill development.".to_string(),
                "4 months".to_string(),
                vec![
                    Skill::new(
                        41,
                        "Communication".to_string(),
                        "Adapting explanations to different learning styles".to_string(),
                    ),
                    Skill::new(
                        42,
                        "Entity Framework".to_string(),
                        "Supporting learning in backend technologies".to_string(),
                    ),
                    Skill::new(
                        43,
                        "Mentorship".to_string(),
                        "Guiding students through individual challenges".to_string(),
                    ),
                ],
                Link::new(
                    4,
                    "UCL Erhvervsakademi og Professionshøjskole".to_string(),
                    "https://ucl.dk/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "UCL Erhvervsakademi og Professionshøjskole".to_string(),
            ),
            Experience::new(
                5,
                "Boardgame Designer".to_string(),
                "Worked on board game development and Kickstarter campaigns at Luudos Studio.".to_string(),
                "01-01-2022".to_string(),
                "01-08-2022".to_string(),
                "During my time at Luudos Studio, I focused on board game design and development, 
including creating rulebooks and ensuring project deadlines were consistently met. 
I collaborated with the team to structure workflows and contributed to successful Kickstarter funding campaigns. 
In total, I designed and wrote nine rulebooks that were well-received by both colleagues and customers.".to_string(),
                "8 months".to_string(),
                vec![
                    Skill::new(51, "Game design".to_string(), "Creating board game mechanics and rules".to_string()),
                    Skill::new(52, "Project management".to_string(), "Organizing workflows and meeting deadlines".to_string()),
                    Skill::new(53, "Creative problem-solving".to_string(), "Designing engaging and balanced systems".to_string()),
                ],
                Link::new(
                    5,
                    "Luudos Studio".to_string(),
                    "https://www.luudos.dk/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "Luudos Studio".to_string(),

            ),
            Experience::new(
                6,
                "Design Developer".to_string(),
                "Created games from scratch, focusing on mechanics and immersive player experiences.".to_string(),
                "01-06-2020".to_string(),
                "01-08-2022".to_string(),
                "As a game developer at Luudos Studio, I was responsible for designing and developing games end-to-end. 
I identified essential mechanics while removing unnecessary complexity, ensuring cohesive and engaging game experiences. 
I worked to align gameplay elements with narrative consistency, building immersive game worlds that resonated with players.".to_string(),
                "2 years 3 months".to_string(),
                vec![
                    Skill::new(61, "Game development".to_string(), "Building complete games from scratch".to_string()),
                    Skill::new(62, "Creative storytelling".to_string(), "Ensuring cohesive narratives in gameplay".to_string()),
                    Skill::new(63, "Teamwork".to_string(), "Collaborating to deliver quality games".to_string()),
                ],
                Link::new(
                    6,
                    "Luudos Studio".to_string(),
                    "https://www.luudos.dk/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "Luudos Studio".to_string(),
            ),
            Experience::new(
                7,
                "Author & Illustrator".to_string(),
                "Created illustrations and authored material for the podcast 'Stemmernes Tårn'.".to_string(),
                "01-10-2018".to_string(),
                "01-06-2020".to_string(),
                "As author and illustrator, I contributed creative material to 'Stemmernes Tårn', including maps, illustrations, 
and narrative content. 
This role required strong communication and teamwork to align visuals and storytelling with the podcast’s themes and audience expectations.".to_string(),
                "1 year 9 months".to_string(),
                vec![
                    Skill::new(71, "Illustration".to_string(), "Creating detailed maps and artwork".to_string()),
                    Skill::new(72, "Storytelling".to_string(), "Developing creative and narrative content".to_string()),
                    Skill::new(73, "Collaboration".to_string(), "Working with a team to produce cohesive material".to_string()),
                ],
                Link::new(
                    7,
                    "Stemmernes Tårn".to_string(),
                    "https://christianfuhlendorff.dk/stemmernes-taarn/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "Stemmernes Tårn".to_string(),
            ),
            Experience::new(
                8,
                "Pædagog".to_string(),
                "Worked full-time at Børnehuset Bakkely with children and as union representative.".to_string(),
                "01-11-2017".to_string(),
                "01-08-2019".to_string(),
                "At Børnehuset Bakkely, I worked as a pædagog, supporting children’s learning and well-being. 
Alongside this, I served as union representative for over 100 colleagues, advocating for healthy working conditions, 
mental well-being, and work-life balance. 
This role strengthened my leadership, communication, and organizational skills.".to_string(),
                "1 year 10 months".to_string(),
                vec![
                    Skill::new(81, "Leadership".to_string(), "Representing and supporting colleagues".to_string()),
                    Skill::new(82, "Childcare".to_string(), "Creating meaningful daily learning for children".to_string()),
                    Skill::new(83, "Communication".to_string(), "Advocating for healthy workplace practices".to_string()),
                ],
                Link::new(
                    8,
                    "Børnehuset Bakkely".to_string(),
                    "https://ry-bakkely.inst.dk/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "Børnehuset Bakkely".to_string(),
            ),
            Experience::new(
                9,
                "Illustrator".to_string(),
                "Worked as freelance illustrator for Cackleberry Studio.".to_string(),
                "01-03-2018".to_string(),
                "01-07-2018".to_string(),
                "At Cackleberry Studio, I contributed illustrations for projects, using tools like Photoshop 
to bring visual concepts to life. 
This position sharpened my skills in visual communication and digital art creation.".to_string(),
                "5 months".to_string(),
                vec![
                    Skill::new(91, "Illustration".to_string(), "Creating digital artwork".to_string()),
                    Skill::new(92, "Photoshop".to_string(), "Designing and editing visual material".to_string()),
                    Skill::new(93, "Collaboration".to_string(), "Working with a creative team on projects".to_string()),
                ],
                Link::new(
                    9,
                    "Cackleberry Studio".to_string(),
                    "https://cackleberrystudio.com/".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                "Cackleberry Studio".to_string(),
            ),
        ]
    }
}
