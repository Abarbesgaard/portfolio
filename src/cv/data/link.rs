use crate::cv::structs::cv_data::Link;

impl Link {
    pub fn new(
        id: u32,
        name: String,
        reference: String,
        short_description: String,
        long_description: String,
    ) -> Link {
        Link {
            id,
            name,
            reference,
            short_description,
            long_description,
        }
    }

    pub fn info() -> Vec<Link> {
        let links = vec![Link::new(
            1,
            "GitHub".to_string(),
            "https://github.com/Abarbesgaard".to_string(),
            "GitHub profile focused on software development with C#, Rust, React, and SignalR. Projects range from the real-time teaching platform CodeCraft to open-source tools like UseLess.".to_string(),
            "My GitHub profile reflects my journey from pedagogy to software development, where I combine a passion for learning with strong technical skills. I work with technologies such as C#, Rust, ASP.NET Core, React, PostgreSQL, Docker, and SignalR, and I’m passionate about building secure and scalable solutions. Among my projects, you’ll find CodeCraft, an online real-time code editor for education, and UseLess, an open-source tool for job application tracking. I’m eager to improve my skills in automated testing, code reviews, and application security – and in my free time, I’m a big fan of board games and roguelike games.".to_string(),
        ),
            Link::new(
                1,
                "Strackly".to_string(),
                "https://strackly.com".to_string(),
                "A job-tracking platform to organize applications, contacts, stages, and follow-ups in one place.".to_string(),
                "Strackly helps you manage your job search from lead to offer. Track roles, companies, contacts, and pipeline stages; log notes, tasks, and interview outcomes; set reminders for follow-ups; and view simple analytics on your progress. Designed for clarity and momentum so you always know what to do next.".to_string(),
            ),
            Link::new(
                1,
                "Zooml".to_string(),
                "https://zooml.dev".to_string(),
                "A zoomable UML-style modeling tool: pan in from system maps to class/sequence details, focusing relationships and linking specs to tests.".to_string(),
                "ZoomL Dev is a zoomable UML modeling app for software architecture and design. Start with high-level context diagrams, then zoom into components, classes, and sequence flows to inspect specific relationships. Models support focus mode (single relation/dependency exploration), annotations, and test-linked specifications so diagrams stay executable and verifiable. Built around a Rust Web API with planned real-time collaboration, it’s designed for clear communication from big-picture views to implementation details.".to_string(),
            )

        ];
        links
    }
}
