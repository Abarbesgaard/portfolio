use crate::cv::structs::{education::Education, skill::Skill};

impl Education {
    pub fn new(id: u32, name: String, description: String, skills: Vec<Skill>) -> Self {
        Education {
            id,
            name,
            description,
            skills,
        }
    }

    pub fn info() -> Vec<Education> {
        vec![
            Education::new(
                1,
                "Computer Science Graduate (Datamatiker)".to_string(),
                "UCL Erhvervsakademi og Professionshøjskole - Online computer science education offering the same content as traditional datamatiker program. 2½ year program divided into 5 semesters covering Programming, System Development, Technology, and Business. Grades: 7,10,12,12,12".to_string(),
                vec![
                    Skill::new(1, "C#".to_string(), "Programming language".to_string()),
                    Skill::new(2, "SQL".to_string(), "Database query language".to_string()),
                    Skill::new(3, "ASP.NET".to_string(), "Web framework".to_string()),
                    Skill::new(4, "API".to_string(), "Application Programming Interface".to_string()),
                    Skill::new(5, "HTML".to_string(), "Markup language".to_string()),
                    Skill::new(6, "CSS".to_string(), "Styling language".to_string()),
                    Skill::new(7, "Scrum".to_string(), "Agile methodology".to_string()),
                    Skill::new(8, "UML".to_string(), "Unified Modeling Language".to_string()),
                    Skill::new(9, "Git".to_string(), "Version control system".to_string()),
                    Skill::new(10, "GitHub".to_string(), "Code hosting platform".to_string()),
                    Skill::new(11, "WPF".to_string(), "Windows Presentation Foundation".to_string()),
                    Skill::new(12, "Blazor".to_string(), "Web framework".to_string()),
                    Skill::new(13, "Relational Databases".to_string(), "Database management".to_string()),
                    Skill::new(14, "Creative Problem Solving".to_string(), "Problem-solving methodology".to_string()),
                ],
            ),
            Education::new(
                2,
                "Behavioral Design Webinar".to_string(),
                "Prosa - Gained in-depth understanding of how behavioral psychology plays a crucial role in daily decision-making processes. Learned behavioral design concepts and applications in various societal areas.".to_string(),
                vec![
                    Skill::new(15, "Behavioral Design".to_string(), "Understanding and changing behavior through design".to_string()),
                ],
            ),
            Education::new(
                3,
                "Selenium Testing Course".to_string(),
                "Prosa - Introduction to Selenium providing skills needed to automate testing. Covered principles behind maintainable and robust automation using Selenium WebDriver.".to_string(),
                vec![
                    Skill::new(16, "Selenium".to_string(), "Web automation testing framework".to_string()),
                    Skill::new(17, "Test Automation".to_string(), "Automated testing principles".to_string()),
                ],
            ),
            Education::new(
                4,
                "GitHub Actions Course".to_string(),
                "Prosa - Gained in-depth knowledge of CI (Continuous Integration) and implementation using GitHub Actions Pipelines. Learned to maintain healthy master branch workflow.".to_string(),
                vec![
                    Skill::new(18, "GitHub Actions".to_string(), "CI/CD platform".to_string()),
                    Skill::new(19, "Continuous Integration".to_string(), "Development practice".to_string()),
                    Skill::new(20, "Continuous Delivery".to_string(), "Software delivery practice".to_string()),
                ],
            ),
            Education::new(
                5,
                "Penetration Testing I - Introduction and Basic Methods".to_string(),
                "Prosa - Introduction to penetration testing, rules and ethics, information gathering, port scanning with nmap, service scanning, exploits, buffer overflows, brute forcing, and demonstrations of tools like Nmap, Metasploit, and Armitage.".to_string(),
                vec![
                    Skill::new(21, "Penetration Testing".to_string(), "Security testing methodology".to_string()),
                    Skill::new(22, "Nmap".to_string(), "Network scanning tool".to_string()),
                    Skill::new(23, "Metasploit".to_string(), "Penetration testing framework".to_string()),
                    Skill::new(24, "Network Security".to_string(), "Network security principles".to_string()),
                ],
            ),
            Education::new(
                6,
                "Penetration Testing II - Web-Based Attacks".to_string(),
                "Prosa - Covered OWASP Top 10, web systems, HTTP protocols, servers and security, command and SQL injection, proxy tools (Tamper Data, Burp Suite), and web scanners (Nikto, w3af, Skipfish).".to_string(),
                vec![
                    Skill::new(25, "OWASP Top 10".to_string(), "Web security vulnerabilities".to_string()),
                    Skill::new(26, "SQL Injection".to_string(), "Web security attack".to_string()),
                    Skill::new(27, "Burp Suite".to_string(), "Web security testing tool".to_string()),
                    Skill::new(28, "Web Security".to_string(), "Web application security".to_string()),
                ],
            ),
            Education::new(
                7,
                "Design Patterns Course".to_string(),
                "Prosa - Modern, updated, and .NET-specialized overview of classic Design Patterns in contemporary forms using C# and .NET. Covered Creational, Structural, and Behavioral Patterns.".to_string(),
                vec![
                    Skill::new(29, "Design Patterns".to_string(), "Software design principles".to_string()),
                    Skill::new(30, "Software Architecture".to_string(), "System design principles".to_string()),
                ],
            ),
            Education::new(
                8,
                "Bachelor in Pedagogy, Health, Body and Movement".to_string(),
                "VIAUC Jydsk - Bachelor's degree in pedagogy focusing on health, body, and movement education.".to_string(),
                vec![
                    Skill::new(31, "Pedagogy".to_string(), "Educational theory and practice".to_string()),
                    Skill::new(32, "Health Education".to_string(), "Health and wellness teaching".to_string()),
                ],
            ),
            Education::new(
                9,
                "STX - Music Line".to_string(),
                "Ringkøbing Gymnasium - High school education with focus on music.".to_string(),
                vec![
                    Skill::new(33, "Music".to_string(), "Musical education and performance".to_string()),
                ],
            ),
            Education::new(
                10,
                "Efterskole - Music".to_string(),
                "Venø Efterskole - Continuation school with music focus.".to_string(),
                vec![
                    Skill::new(34, "Music Performance".to_string(), "Live music and performance skills".to_string()),
                ],
            ),
        ]
    }
}
