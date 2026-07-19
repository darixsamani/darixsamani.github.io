use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct WorkExperience {
    company: String,
    logo: String,
    title: String,
    location: String,   // Added: city/country or "Remote"
    start_date: String,
    end_date: String,
    achievements: Vec<String>,
    skills: Vec<String>,
}

#[component]
pub fn WorkExperiences() -> Element {
    // Sample data – update with your own
    let experiences = vec![
        WorkExperience {
            company: "Shipd".to_string(),
            logo: "https://ui-avatars.com/api/?name=sd&background=0D8ABC&color=fff&size=64".to_string(),
            title: "Software Engineer".to_string(),
            location: "Remote, Douala, Cameroon".to_string(),
            start_date: "Jul. 2024".to_string(),
            end_date: "Now".to_string(),
            achievements: vec![
                "Identified and proposed a GitHub issue on a 500+ star open-source repository, then authored an integration test to reproduce and validate the fix, streamlining the resolution process for maintainers, and also wrote the Dockerfile.".to_owned(),
                
            ],
            skills: vec![
                "Rust".to_owned(), "Oxigraph".to_owned(), "Open source Development".to_owned(), "GitHub".to_owned(), "Docker".to_owned(), "Integration Testing".to_owned(),
            ],
        },

        WorkExperience {
            company: "Hoozon Sarl".to_string(),
            logo: "https://ui-avatars.com/api/?name=HS&background=0D8ABC&color=fff&size=64".to_string(),
            title: "Full Stack Engineer".to_string(),
            location: "Douala, Cameroon".to_string(),
            start_date: "Fev. 2024".to_string(),
            end_date: "Sep. 2024".to_string(),
            achievements: vec![
                "Engineered middleware in Flask (Python) to automate payment event processing between WordPress (via webhooks) and Node.js, streamlining client workflows.".to_owned(),
                "Oversaw Docker-based deployments to both development and production, ensuring robust, scalable, and maintainable environments.".to_owned(),
                "Translate business needs into functional and technical specifications for multi-development software projects.".to_owned(),
                "Developed mobile features using Flutter and BLoC, delivering seamless cross-platform user experiences.".to_owned(),
                "Implemented CI/CD pipelines, accelerating release cycles, and improving deployment reliability.".to_owned(),
                "Contribute to developing the frontend for multi-application using React, Typescript, and TailwindCSS.".to_owned(),
                "Support a team of five developers daily during their tasks.".to_owned(),
            ],
            skills: vec![
                "Flask".to_owned(), "FastAPI".to_owned(), "TypeScript".to_owned(),
                "SQLAlchemy".to_owned(), "Redis".to_owned(), "RabbitMQ".to_owned(),
                "Celery".to_owned(), "Docker".to_owned(), "Github".to_owned(),
                "WordPress".to_owned(), "Yup (form Validation)".to_owned(),
                "Tailwind CSS".to_owned(), "Nginx".to_owned(), "Docker Compose".to_owned(),
                "JavaScript".to_owned(), "Python3".to_owned(), "webhooks".to_owned(),
                "Vite".to_owned(), "TLS/SSL".to_owned(), "Shell/bash".to_owned(),
                "Flutter".to_owned(),
            ],
        },
        WorkExperience {
            company: "Data Touch Analytics".to_string(),
            logo: "https://ui-avatars.com/api/?name=DTA&background=8A2BE2&color=fff&size=64".to_string(),
            title: "Sofware Developer".to_string(),
            location: "Douala, Cameroon".to_string(),
            start_date: "Dec. 2023".to_string(),
            end_date: "Sep. 2024".to_string(),
            achievements: vec![
                "Developed end-to-end application for hospital blood donation management, automating data collection and
distribution workflows.".to_owned(),
                "Orchestrated Celery, Redis, and RabbitMQ integration to automate donor notifications(hemovigilance messages to
donors after 24 hours, 48 hours (after their samples), and on the date of their next eligible donation ), improving
engagement and compliance rates.".to_owned(),
                "Built RESTful APIs in Python, enabling secure, scalable data exchange between hospital systems.".to_owned(),
                "Create a monthly PDF report generator using Celery Beat.".to_owned(),
                "Implement a system to send birthday messages to donors.".to_owned(),
                "Oversee the deployment of applications to development and production environments.".to_owned(),
                "Leveraged Git/GitHub workflows to enforce code quality and streamline team collaboration.".to_owned(),
                "Utilized React and TypeScript for responsive frontend interfaces, enhancing user experience for medical staff.".to_owned(),
            ],
            skills: vec!["FastAPI".to_owned(), "React(TS)".to_owned(), "TypeScript".to_owned(), "PostgreSQL".to_owned(), "SQLAlchemy".to_owned(), "Redis".to_owned(), "RabbitMQ".to_owned(), "Celery".to_owned(), "Flower".to_owned(), "Docker".to_owned(), "GitHub".to_owned(), "REST".to_owned(), "Yup (form Validation)".to_owned(), "Tailwind CSS".to_owned(), "Nginx".to_owned(), 
                        "Docker Compose".to_owned(), "JavaScript".to_owned(), "Python3".to_owned(), "Celery-beat".to_owned(), "Vite".to_owned(), "TLS/SSL".to_owned(), "Shell/bash".to_owned(),
],
        },
        WorkExperience {
            company: "Scale AI".to_string(),
            logo: "https://ui-avatars.com/api/?name=SA&background=E87A24&color=fff&size=64".to_string(),
            title: "Artificial Intelligence Trainer".to_string(),
            location: "Remote / Douala, Cameroon".to_string(),
            start_date: "Jun. 2024".to_string(),
            end_date: "Sep. 2024".to_string(),
            achievements: vec![
                "Training AI Systems and Large Language Models (LLMs).".to_owned(),
                "Enhancing chatbots to be more useful for people worldwide.".to_owned(),
                "Have an impact in making LLMs safer, accurate, and reliable.".to_owned(),
                "Help improve the AI chatbot for their customers, and these tasks will directly affect the quality of the chatbot
experience environments.".to_owned()
            ],
            skills: vec![
                "Prompt Engineering".to_owned(), "Prompt Writing".to_owned(), "Creative Ideas and Thinkings".to_owned(),
                "Prompt Evaluation".to_owned(),
            ],
        },
        WorkExperience {
            company: "Astor Digitech".to_string(),
            logo: "https://ui-avatars.com/api/?name=AD&background=E87A24&color=fff&size=64".to_string(),
            title: "Graduate Internship Engineer".to_string(),
            location: "Douala, Cameroon".to_string(),
            start_date: "Feb. 2022".to_string(),
            end_date: "Jun. 2022".to_string(),
            achievements: vec![
                "Designed and deployed mobile RF drive-testing app, collecting and processing field data for network optimization.".to_owned(),
                "Developed RESTful APIs with FastAPI and Docker, enabling scalable, containerized data ingestion pipelines.".to_owned(),
                "Implemented MongoDB and SQLite for efficient, reliable data storage and retrievals.".to_owned(),
            ],
            skills: vec![
                "FastAPI".to_owned(), "MongoDB".to_owned(), "REST".to_owned(),
                "Docker".to_owned(),"Heroku".to_owned(),"Flutter".to_owned(),"Reactive programming(Dart)".to_owned(),"SQLite".to_owned(),
            ],
        },
        WorkExperience {
            company: "Astor Protect".to_string(),
            logo: "https://ui-avatars.com/api/?name=AP&background=E87A24&color=fff&size=64".to_string(),
            title: "Information Technology Trainee".to_string(),
            location: "Douala, Cameroon".to_string(),
            start_date: "Jul. 2021 ".to_string(),
            end_date: "Aug. 2021".to_string(),
            achievements: vec![
                "Conducted research and deployed an IaaS (DevStack) on a virtual machine".to_owned(),
                "Analyze multiple development projects.".to_owned(),
                "Monitor and manage project progress.".to_owned(),
                "Research and deploy the open-source identity and access management solution (Keycloak) on a Kubernetes cluster.".to_owned(),
            ],
            skills: vec![
                "KeyCloack".to_owned(), "DevStack (OpenStack)".to_owned(), "Kubernetes".to_owned(),
                "VirtualBox".to_owned(),"UML".to_owned(),"Project Management and Analysis".to_owned(),
            ],
        },
        WorkExperience {
            company: "Flysoft Engineering".to_string(),
            logo: "https://ui-avatars.com/api/?name=FE&background=E87A24&color=fff&size=64".to_string(),
            title: "Software Development Engineer Intern".to_string(),
            location: "Yaoundé, Cameroon".to_string(),
            start_date: "Jul. 2020".to_string(),
            end_date: "Aug. 2020".to_string(),
            achievements: vec![
                "Develop websites using HTML5, CSS3, and JavaScript.".to_owned(),
                "Analyze various development projects.".to_owned(),
                "Model a project using UML.".to_owned(),
                "Learn reactive programming in Dart, along with Bloc state management.".to_owned(),
            ],
            skills: vec![
                "UML".to_owned(), "HTML5".to_owned(), "CSS3".to_owned(),
                "Dart".to_owned(),"JavaScript".to_owned(),"Reactive programming (Dart)".to_owned(),"Flutter".to_owned(),
            ],
        },
    ];

    rsx! {
        div {
            id: "experiences",
            class: "flex flex-col items-center justify-center gap-8 px-4 py-12 w-full max-w-4xl mx-auto",

            div { class: "text-center mb-2",
                h2 { class: "text-3xl sm:text-4xl font-bold mb-2", "Work Experiences" }
                p { class: "text-base sm:text-lg text-gray-400 max-w-2xl mx-auto",
                    "My professional journey – roles, achievements, and technologies I've worked with."
                }
            }

            div { class: "relative flex flex-col gap-8 w-full",
                div { class: "absolute left-4 sm:left-6 top-4 bottom-4 w-0.5 bg-gray-600/50" }

                for (index , exp) in experiences.iter().enumerate() {
                    WorkExperienceCard {
                        key: "{index}",
                        experience: exp.clone(),
                        is_last: index == experiences.len() - 1,
                    }
                }
            }
        }
    }
}

#[component]
fn WorkExperienceCard(experience: WorkExperience, is_last: bool) -> Element {
    rsx! {
        div { class: "relative flex flex-col sm:flex-row gap-4 sm:gap-6 pl-12 sm:pl-16",

            // Timeline dot
            div { class: "absolute left-0 top-1 w-8 h-8 sm:w-10 sm:h-10 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 border-4 border-gray-900 flex items-center justify-center shadow-lg shadow-blue-500/20",
                img {
                    src: experience.logo,
                    alt: &experience.company,
                    class: "w-5 h-5 sm:w-6 sm:h-6 rounded-full object-cover",
                }
            }

            // Card
            div { class: "flex-1 bg-gray-800/50 backdrop-blur-sm rounded-xl p-5 sm:p-6 border border-gray-700/50 hover:border-blue-500/50 transition-all duration-300",

                // Row: Company, Title, and Date
                div { class: "flex flex-col sm:flex-row sm:items-center justify-between gap-1 sm:gap-4 mb-1",
                    div {
                        h3 { class: "text-lg sm:text-xl font-bold text-white", "{experience.company}" }
                        p { class: "text-sm sm:text-base text-blue-400 font-medium",
                            "{experience.title}"
                        }
                    }

                    div { class: "flex flex-col",
                        div { class: "text-xs sm:text-sm text-gray-400 whitespace-nowrap",
                            span { "{experience.start_date}" }
                            span { " — " }
                            span { "{experience.end_date}" }
                        }
                        // Location – placed after the date row, below the title and date
                        div { class: "flex items-center gap-1.5 text-gray-400 text-xs sm:text-sm mb-3",
                            span { "📍" }
                            span { "{experience.location}" }
                        }
                    }
                }

                // Achievements list
                ul { class: "list-disc list-inside space-y-1 mb-4 text-sm sm:text-base text-gray-300",
                    for achievement in experience.achievements {
                        li { class: "leading-relaxed", "{achievement}" }
                    }
                }

                // Skills badges
                div { class: "flex flex-wrap gap-2",
                    for skill in experience.skills {
                        span { class: "px-3 py-1 bg-blue-500/10 text-blue-300 rounded-full text-xs border border-blue-500/20",
                            "{skill}"
                        }
                    }
                }
            }
        }
    }
}