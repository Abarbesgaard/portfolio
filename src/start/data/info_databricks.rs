use crate::start::structs::cover_letter::{Company, CoverLetter, Person,Paragraf};

impl CoverLetter {
 pub   fn info_databricks() -> Self {

        CoverLetter::new(
            "Software Engineer".to_string(),
            Company::new(
                "Databricks".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ),
            Person::new(
                "first name".to_string(),
                "last name".to_string(),
                "phone".to_string(),
                "email.com".to_string(),
            ),
            vec![   Paragraf::new("Dear Databricks Hiring Team".to_string(),"
Kære Databricks Hiring Team
".to_string()),

Paragraf::new("I am excited to apply for the Software Engineer position at Databricks. I am motivated by tackling complex problems where architecture, code quality, and collaboration go hand in hand – qualities I recognize as core to Databricks’ engineering culture.".to_string(),
                    "Jeg blev begejstret, da jeg så stillingen som Software Engineer, og jeg vil hermed søge rollen med stor interesse. Jeg motiveres af at arbejde med komplekse problemområder, hvor arkitektur, kodekvalitet og samarbejde går hånd i hånd – og jeg fornemmer, at netop dette er kernen i Databricks’ udviklingskultur.".to_string()),

Paragraf::new("I recently graduated as a Datamatiker from UCL, specializing in C#/.NET, ASP.NET Core/MVC, React, and MS SQL. For my thesis, I developed a modular backend using Vertical Slice Architecture and the REPR pattern, paired with a React frontend in TypeScript and real-time functionality via SignalR. This project gave me hands-on experience across the full software development lifecycle – from design and implementation to testing and deployment – and strengthened my ability to build scalable, maintainable solutions.".to_string(),

    "Jeg er nyuddannet datamatiker fra UCL med erfaring i C#/.NET, ASP.NET Core/MVC, React og MS SQL. Til min hovedopgave udviklede jeg en modulær backend baseret på Vertical Slice Architecture og REPR-mønsteret, med en React-frontend i TypeScript og realtidsfunktionalitet via SignalR. Dette projekt gav mig solid erfaring med hele softwareudviklingscyklussen – fra design og udvikling til test og drift – samt med at bygge løsninger, der er både skalerbare og vedligeholdelsesvenlige.".to_string()),

Paragraf::new("During my internship at BEMIIT ApS, I worked as a full-stack developer delivering production-ready features in an agile team. This experience taught me to collaborate closely with colleagues, integrate feedback effectively, and maintain high code quality, while adapting to existing systems and processes.".to_string(), "Under mit praktikforløb hos BEMIIT ApS arbejdede jeg som full-stack udvikler, hvor jeg leverede produktionsklare funktioner i et agilt team. Her fik jeg erfaring med at samarbejde tæt med kolleger, håndtere feedback og sikre høj kodekvalitet, samtidig med at jeg lærte at tilpasse mig eksisterende systemer og processer.".to_string()),

Paragraf::new("Beyond my technical skills, I have experience as a mentor and tutor, which enhanced my ability to explain complex concepts, foster team trust, and communicate effectively across different audiences – competencies I see as essential for contributing in a collaborative environment like Databricks.".to_string(),"Ud over min tekniske erfaring har jeg arbejdet som mentor og lektiehjælper, hvilket har styrket mine evner til at formidle komplekse emner, samarbejde effektivt og skabe tryghed i teams – kompetencer, som jeg ser som afgørende for at bidrage positivt i et miljø som Databricks, hvor både teamwork og dyb faglig indsigt vægtes højt.".to_string()),

                    Paragraf::new("I thrive when I can follow a project from initial ideas to a finished solution and enjoy taking responsibility for both quality and progress. The opportunity to contribute to Databricks’ platform, working on solutions that handle large-scale data and complex challenges, excites me as a chance to grow professionally and make a meaningful impact.".to_string(),
                    "Jeg trives med at følge et projekt hele vejen – fra de første idéer til den færdige løsning – og jeg nyder at tage ansvar for både kvalitet og fremdrift. At kunne bidrage til Databricks’ platform og arbejde med løsninger, der håndterer store datamængder og komplekse problemstillinger, ser jeg som en spændende mulighed, hvor jeg både kan udvikle mig fagligt og gøre en forskel.".to_string()),

Paragraf::new("I look forward to the opportunity to discuss how I can contribute to Databricks’ journey – both technically and as part of your team.".to_string(), "Jeg ser frem til muligheden for at uddybe, hvordan jeg kan bidrage til Databricks’ rejse – både teknisk og menneskeligt.".to_string()),
     ] 
        )
    
    }
}
