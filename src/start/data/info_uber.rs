use crate::start::structs::cover_letter::{Company, CoverLetter, Paragraf, Person};

impl CoverLetter {
    pub fn info_uber() -> Self {
        CoverLetter::new(
            "Software Engineer - Backend - Catalog Platform".to_string(),
            Company::new(
                "Uber".to_string(),
                "Unknown".to_string(),
                "unkown".to_string(),
                "unkown".to_string(),
            ),
            Person::new(
                "unknown".to_string(),
                "unknown".to_string(),
                "unknown".to_string(),
                "unknown".to_string(),
            ),
            vec![
                Paragraf::new(
                    "Dear Uber Eats Engineering Team,".to_string(),
                    "Kære Uber Eats Engineering Team".to_string(),
                ),
                Paragraf::new("I am excited to apply for the Software Engineer position on the Denmark Eats Engineering team. Uber’s mission to reimagine how the world moves resonates deeply with me, and the opportunity to help build and scale the core catalog and inventory platform for Uber Eats is both inspiring and motivating.".to_string(), 
                    "Jeg vil gerne søge stillingen som Software Engineer i jeres Denmark Eats Engineering team. Ubers mission om at gentænke, hvordan verden bevæger sig, tiltaler mig meget, og muligheden for at være med til at bygge og skalere katalog- og inventory-platformen for Uber Eats finder jeg både inspirerende og motiverende.".to_string()),
                Paragraf::new("I recently graduated as a Datamatiker from UCL, specializing in C#/.NET, ASP.NET Core/MVC, React, and MS SQL. In my thesis project, I developed a modular backend architecture using Vertical Slice Architecture and the REPR pattern, paired with a React frontend in TypeScript and real-time communication through SignalR. This project gave me hands-on experience in designing distributed, modular systems with a strong focus on scalability, maintainability, and reliability.".to_string(), 
                    "Jeg er nyuddannet datamatiker fra UCL med speciale i C#/.NET, ASP.NET Core/MVC, React og MS SQL. I mit afsluttende projekt udviklede jeg en modulær backend-arkitektur baseret på Vertical Slice Architecture og REPR-mønsteret, kombineret med en React-frontend i TypeScript og realtidskommunikation via SignalR. Projektet gav mig praktisk erfaring med at designe distribuerede, modulære systemer med fokus på skalerbarhed, vedligeholdelse og driftssikkerhed.".to_string()),
                Paragraf::new("During my internship at BEMIIT ApS, I worked as a full-stack developer, delivering production-ready features in an agile environment. I gained valuable experience in collaborating across teams, maintaining code quality through peer reviews, and ensuring smooth deployments in production. These experiences taught me the importance of clean, testable code and the discipline required to contribute to systems that are relied upon every day.".to_string(), 
                    "Under mit praktikophold hos BEMIIT ApS arbejdede jeg som full-stack udvikler, hvor jeg leverede produktionsklare funktioner i et agilt team. Her fik jeg erfaring med tæt samarbejde på tværs af roller, kodekvalitet gennem peer reviews og sikre deployment-processer. Erfaringerne har lært mig vigtigheden af at skrive testbar, ren kode og samtidig kunne bidrage til systemer, som er afhængige i daglig drift.".to_string()),
                Paragraf::new("Beyond my technical background, I have worked as a mentor and tutor, which strengthened my ability to communicate complex concepts clearly and foster collaboration in a team setting. I believe this will allow me to contribute positively not only to Uber Eats’ technical goals but also to its engineering culture.".to_string(), 
                    "Derudover har jeg erfaring som mentor og lektiehjælper, hvilket har styrket mine evner til at formidle komplekse emner klart og skabe et samarbejdende og trygt miljø. Det ser jeg som en vigtig kompetence til at bidrage positivt – både fagligt og menneskeligt – til et team som jeres.".to_string()),
                Paragraf::new("What excites me most about this role is the challenge of contributing to a large-scale, distributed system that directly impacts millions of users worldwide. I am eager to bring my passion for building reliable, high-quality software, while also learning and growing within a team that values scalability, collaboration, and long-term impact.".to_string(), 
                    "Det, der tiltaler mig mest ved stillingen, er muligheden for at arbejde med et distribueret system i stor skala, som direkte påvirker millioner af brugere globalt. Jeg brænder for at udvikle driftssikre og skalerbare løsninger og ser frem til at kunne lære og vokse i et team, hvor kvalitet og samarbejde er i centrum.".to_string()),
                Paragraf::new("I look forward to the opportunity to discuss how I can contribute to the success and growth of the Uber Eats platform.".to_string(), 
                    "Jeg ser frem til muligheden for at uddybe, hvordan jeg kan bidrage til Uber Eats’ fortsatte succes.".to_string()),
            ],
        )
    }
}
