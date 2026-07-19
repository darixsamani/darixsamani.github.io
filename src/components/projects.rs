use dioxus::prelude::*;

const OXIGRAPH_WEB: Asset = asset!("assets/oxigraph_web.png");
const RMQTT_DASHBOARD : Asset = asset!("assets/rmqtt_dashboard.png");
const SIGNME: Asset = asset!("assets/signme.svg");
const PDF2IMAGES_WEB: Asset = asset!("assets/pdf2images_web.png");
const PDF2IMAGES_CLI: Asset = asset!("assets/pdf2image_cli.png");
const FASTPIA_CRONS_DASHBOARD: Asset = asset!("assets/fastapi_crons_dashboard.svg");
const SIMPARX: Asset = asset!("assets/simparx.svg");
const PDFDRIVE: Asset = asset!("assets/pdfdrive.svg");
const MONGODB_SCHOOL: Asset = asset!("assets/mogobd_school.svg");
const XML_RPOJECT: Asset = asset!("assets/xml_project.svg");

#[component]
pub fn Projects() -> Element {
    // Sample project data – replace with your own
    let projects = vec![
        Project {
            title: "Oxigraph Web".to_owned(),
            description: "A high-performance async REST API built with Rust and Salvo, exposing SPARQL query/update support, RDF import/export (Turtle, N-Triples, RDF/XML), named graph management, interactive dashboard, and OpenAPI/Swagger documentation on top of the Oxigraph RDF/SPARQL store for scalable semantic web applications.".to_owned(),
            image: OXIGRAPH_WEB,
            github: "https://github.com/darixsamani/oxigraph-web".to_owned(),
            skills: vec!["Rust".to_owned(), "HTML5".to_owned(), "CSS3".to_owned(), "JavaScript".to_owned(), "Docker".to_owned(), "Git".to_owned(), "React".to_owned()],
        },
        Project {
            title: "Pdf2image Web".to_owned(),
            description: "A web application that converts PDF files into images and lets users download the results as a ZIP archive.".to_owned(),
            image: PDF2IMAGES_WEB,
            github: "https://github.com/darixsamani/pdf2image-web".to_owned(),
            skills: vec!["Rust".to_owned() ,"Salvo".to_owned(), "React".to_owned(), "Typescript".to_owned(), "Ant Design".to_owned(), "Docker".to_owned(), "Git".to_owned() ],
        },
        Project {
            title: "Pdf2image CLI".to_owned(),
            description: "A fast and lightweight command-line tool to convert PDF pages into high-quality images. Perfect for automation, batch processing, and backend pipelines.".to_owned(),
            image: PDF2IMAGES_CLI,
            github: "https://github.com/darxisamani/pdf2images-cli".to_owned(),
            skills: vec!["Rust".to_owned(), "Git".to_owned(),],
        },

        Project {
            title: "FastAPI Crons Dashboard".to_owned(),
            description: "A real-time, standalone dashboard built for the fastapi_crons package - designed to monitor, visualize, and manage scheduled tasks with ease.".to_owned(),
            image: FASTPIA_CRONS_DASHBOARD,
            github: "https://github.com/darixsamani/fastapi-crons-dashboard".to_owned(),
            skills: vec!["React".to_owned(), "Typescript".to_owned(), "And Design".to_owned(), "FastAPI".to_owned(), "Python".to_owned(),  "Docker".to_owned(), "Git".to_owned()],
        },

        Project {
            title: "RMQTT Dashboard".to_owned(),
            description: "RMQTT Dashboard is a high-performance, real-time web-based monitoring and management dashboard for the RMQTT MQTT Broker, built with React and TypeScript to provide an intuitive interface for visualizing broker resources, client connections, topics, subscriptions, message statistics, and system metrics. Designed as a native RMQTT plugin, the dashboard is seamlessly integrated and bundled with the broker for production releases, while leveraging the Salvo Rust web framework as its embedded web container to deliver a fast, lightweight, and responsive user experience without requiring any external web server.".to_owned(),
            image: RMQTT_DASHBOARD,
            github: "https://github.com/darixsamani/rmqtt".to_owned(),
            skills: vec!["React".to_owned(), "Typescript".to_owned(), "And Design".to_owned(), "Salvo".to_owned(), "Rust".to_owned(),  "Docker".to_owned(), "Git".to_owned()],
        },

        Project {
            title: "SignMe".to_owned(),
            description: "SignMe provides an electronic, encrypted stamp of authentication
              for your most important documents. Faster than ink, safer than paper.".to_owned(),
            image: SIGNME,
            github: "https://github.com/darixsamani/signme".to_owned(),
            skills: vec!["React".to_owned(), "Typescript".to_owned(), "And Design".to_owned(), "FastAPI".to_owned(), "Python".to_owned(),"Cryptography".to_owned(),  "Docker".to_owned(), "Git".to_owned()],
        },

         Project {
            title: "Simparx".to_owned(),
            description: "This project develops a basic paragraph recognition method using morphological operations, which works well for standard layouts but struggles with complex image structures where it becomes unstable. To overcome these limitations, the team plans to implement masking techniques and integrate artificial intelligence—specifically deep learning—to significantly improve recognition accuracy and robustness in challenging, real-world document images.".to_owned(),
            image: SIMPARX,
            github: "https://github.com/darixsamani/simparx".to_owned(),
            skills: vec!["OpenCV".to_owned(), "Scikit-image".to_owned(), "Image Processing".to_owned(),"Git".to_owned()],
        },

        Project {
            title: "Pdfdrive".to_owned(),
            description: "A web scraper that gathers information from the pdfdrive.com website and saves it in a file (JSONL) as well as in
a MongoDB database.".to_owned(),
            image: PDFDRIVE,
            github: "https://github.com/darixsamani/pdfdrive".to_owned(),
            skills: vec!["Scrapy".to_owned(), "BeautifulSoup".to_owned(), "GitHub Action".to_owned(),"Docker".to_owned(), "MongoDB".to_owned(), "Redis".to_owned(), "Git".to_owned()],
        },

         Project {
            title: "MongoDB School".to_owned(),
            description: "This school project aimed to develop a series of complex MongoDB queries to meet the student management needs
in a university environment.".to_owned(),
            image: MONGODB_SCHOOL,
            github: "https://github.com/darixsamani/mongodb-school".to_owned(),
            skills: vec!["MongoDB".to_owned(), "Shell/bash".to_owned(), "JavaScript".to_owned(),"Git".to_owned()],
        },

        Project {
            title: "XML Project".to_owned(),
            description: "This school project aims to generate PDF receipts for university fee payments for students from an XML file using
XSL-FO and XSLT.".to_owned(),
            image: XML_RPOJECT,
            github: "https://github.com/darixsamani/tpe-xml".to_owned(),
            skills: vec!["XML".to_owned(), "XSLT".to_owned(), "XSL-FO".to_owned(),"Shell/bash".to_owned(), "Git".to_owned()],
        },
        
    ];

    rsx! {
        div {
            id: "projects",
            class: "flex flex-col items-center justify-center gap-8 px-4 py-12 w-full max-w-6xl mx-auto",

            // Section header
            div { class: "text-center mb-4",
                h2 { class: "text-3xl sm:text-4xl font-bold mb-2", "Selected Projects" }
                p { class: "text-base sm:text-lg text-gray-400 max-w-2xl mx-auto",
                    "A showcase of my work and contributions across various domains"
                }
            }

            // Projects grid
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 w-full",
                for project in projects {
                    ProjectCard { project }
                }
            }
        }
    }
}

// Reusable Project Card component
#[component]
fn ProjectCard(project: Project) -> Element {
    rsx! {
        div { class: "bg-gray-800/50 backdrop-blur-sm rounded-xl overflow-hidden border border-gray-700/50 hover:border-blue-500/50 transition-all duration-300 flex flex-col",

            // Image
            img {
                src: project.image,
                alt: &project.title,
                class: "w-full h-48 object-cover",
            }

            // Content
            div { class: "p-5 flex flex-col flex-1",

                // Title
                h3 { class: "text-xl font-semibold mb-2 text-white", "{project.title}" }

                // Description
                p { class: "text-sm text-gray-400 mb-4 flex-1", "{project.description}" }

                // Skills badges
                div { class: "flex flex-wrap gap-2 mb-4",
                    for skill in project.skills {
                        span { class: "px-3 py-1 bg-blue-500/10 text-blue-300 rounded-full text-xs border border-blue-500/20",
                            "{skill}"
                        }
                    }
                }

                // GitHub link
                a {
                    href: project.github,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-flex items-center gap-2 text-sm text-gray-400 hover:text-white transition-colors duration-200 mt-auto",
                    svg { class: "w-5 h-5 fill-current", view_box: "0 0 24 24",
                        path { d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.468-2.38 1.235-3.22-.123-.3-.535-1.52.117-3.16 0 0 1.008-.322 3.3 1.23.96-.267 1.98-.399 3-.399s2.04.132 3 .399c2.292-1.552 3.3-1.23 3.3-1.23.653 1.64.24 2.86.118 3.16.768.84 1.233 1.91 1.233 3.22 0 4.61-2.804 5.62-5.476 5.92.43.37.824 1.102.824 2.22 0 1.602-.015 2.894-.015 3.287 0 .322.216.694.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" }
                    }
                    "View on GitHub"
                }
            }
        }
    }
}

// Simple struct to hold project data – place this outside the component
#[derive(Clone, PartialEq)]
struct Project {
    title: String,
    description: String,
    image: Asset,
    github: String,
    skills: Vec<String>,
}