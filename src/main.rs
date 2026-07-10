use std::ops::Div;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const IMAGES: Asset = asset!("/assets/IMG_9109-1024x683.jpg");
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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // Hero {}
        About {}
        Skills {}
        Projects {}

    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            id: "aboutme",
            class: "flex flex-col md:flex-row gap-8 md:gap-16 lg:gap-40 items-center justify-center min-h-screen px-4 py-8",

            // Left column: Text + Contact buttons
            div {
                class: "flex flex-col gap-4 max-w-full md:max-w-md md:min-w-[600px] lg:min-w-[700px] text-center md:text-left",

                h1 { class: "text-3xl sm:text-4xl font-bold mb-4 text-center", "About Me" }

                p { class: "text-base sm:text-lg text-gray-400 mb-8",
                    "Passionate Software Engineer, Data Scientist, and Machine Learning Researcher with expertise in Python, JavaScript/TypeScript, Rust, Dart, SQL, NoSQL (MongoDB, Neo4j and Redis), alongside hands-on experience with frameworks such as FastAPI, React.js, Flutter, Dioxus, Salvo, Nest.Js, and AI/ML stack. Proven ability to deliver optimized solutions, troubleshoot complex issues, and contribute to Agile teams. Strong understanding of version control with Git, DevOps practices, CI/CD pipelines, and cloud technologies."
                }

                // --- CONTACT COMPONENT (placed at the end) ---
                div { class: "flex flex-wrap gap-4 mt-2",

                    // LinkedIn
                    a {
                        href: "https://linkedin.com/in/darixsamani",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "inline-flex items-center gap-2 px-4 py-2 bg-[#0A66C2] text-white rounded-lg hover:bg-[#0A66C2]/80 transition-colors duration-200 text-sm sm:text-base",
                        // Inline SVG for LinkedIn (simplified)
                        svg {
                            class: "w-5 h-5 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                        }
                        "LinkedIn"
                    }

                    // GitHub
                    a {
                        href: "https://github.com/darixsamani",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "inline-flex items-center gap-2 px-4 py-2 bg-[#181717] text-white rounded-lg hover:bg-[#181717]/80 transition-colors duration-200 text-sm sm:text-base",
                        // GitHub SVG
                        svg {
                            class: "w-5 h-5 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.468-2.38 1.235-3.22-.123-.3-.535-1.52.117-3.16 0 0 1.008-.322 3.3 1.23.96-.267 1.98-.399 3-.399s2.04.132 3 .399c2.292-1.552 3.3-1.23 3.3-1.23.653 1.64.24 2.86.118 3.16.768.84 1.233 1.91 1.233 3.22 0 4.61-2.804 5.62-5.476 5.92.43.37.824 1.102.824 2.22 0 1.602-.015 2.894-.015 3.287 0 .322.216.694.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" }
                        }
                        "GitHub"
                    }

                    // Google Scholar
                    a {
                        href: "https://scholar.google.com/citations?user=XCUqjFYAAAAJ&hl=fr",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "inline-flex items-center gap-2 px-4 py-2 bg-[#4285F4] text-white rounded-lg hover:bg-[#4285F4]/80 transition-colors duration-200 text-sm sm:text-base",
                        // Google Scholar SVG (simplified)
                        svg {
                            class: "w-5 h-5 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M12 24a1 1 0 0 1-.707-.293l-2.5-2.5a1 1 0 0 1 1.414-1.414L12 21.586l1.793-1.793a1 1 0 0 1 1.414 1.414l-2.5 2.5A1 1 0 0 1 12 24zM5 18a1 1 0 0 1-.707-1.707l3.5-3.5a1 1 0 0 1 1.414 0l1.793 1.793L15.293 9.5a1 1 0 0 1 1.414 0l3.5 3.5a1 1 0 0 1-1.414 1.414L17 11.914l-5.793 5.793a1 1 0 0 1-1.414 0L8 15.914l-2.293 2.293A1 1 0 0 1 5 18z" }
                        }
                        "Google Scholar"
                    }

                    // ORCID
                    a {
                        href: "https://orcid.org/0009-0001-0794-7120",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "inline-flex items-center gap-2 px-4 py-2 bg-[#A6CE39] text-white rounded-lg hover:bg-[#A6CE39]/80 transition-colors duration-200 text-sm sm:text-base",
                        // ORCID SVG (simplified)
                        svg {
                            class: "w-5 h-5 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm0-13c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3zm-3 8h6v2H9v-2z" }
                        }
                        "ORCID"
                    }

                    // ResearchGate (NEW)
                    a {
                        href: "https://www.researchgate.net/profile/Darix-Samani-Siewe",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "inline-flex items-center gap-2 px-4 py-2 bg-[#00CCBB] text-white rounded-lg hover:bg-[#00CCBB]/80 transition-colors duration-200 text-sm sm:text-base",
                        // ResearchGate simplified icon (a hexagon with "R")
                        svg {
                            class: "w-5 h-5 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 16h4v-2h-4v2zm0-4h4v-2h-4v2zm0-4h4V8h-4v2zm-2-2h2V8H8v2zm0 4h2v-2H8v2zm0 4h2v-2H8v2z" }
                        }
                        "ResearchGate"
                    }

                    // Download CV
                    a {
                        href: "https://darixsamani.github.io/Darix_SAMANI_SIEWE_CV.pdf",
                        download: true,
                        class: "inline-flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors duration-200 text-sm sm:text-base",
                        // Download icon (simple SVG)
                        svg {
                            class: "w-5 h-5 fill-current",
                            view_box: "0 0 24 24",
                            path { d: "M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z" }
                        }
                        "Download CV"
                    }
                }
                        // --- End of Contact ---
            }

            // Right column: Image
            img {
                src: IMAGES,
                class: "w-full max-w-xs sm:max-w-sm md:max-w-md rounded-lg shadow-lg",
            }
        }
    }
}


#[component]
pub fn Skills() -> Element {
    rsx! {
        div {
            id: "skills",
            class: "flex flex-col items-center justify-center gap-8 px-4 py-12 w-full max-w-6xl mx-auto",

            // Section header
            div { class: "text-center mb-4",
                h2 { class: "text-3xl sm:text-4xl font-bold mb-2", "Skills & Technologies" }
                p { class: "text-base sm:text-lg text-gray-400 max-w-2xl mx-auto",
                    "Technologies and tools I work with across the full stack"
                }
            }

            // Skills grid
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 w-full",

                // Category 1: Programming Languages
                div { class: "bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50 hover:border-blue-500/50 transition-all duration-300",
                    div { class: "flex items-center gap-3 mb-4",
                        span { class: "text-2xl", "💻" }
                        h3 { class: "text-lg font-semibold text-blue-400", "Programming Languages" }
                    }
                    div { class: "flex flex-wrap gap-2",
                        // Using badges for each skill
                        span { class: "px-3 py-1 bg-blue-500/10 text-blue-300 rounded-full text-sm border border-blue-500/20",
                            "Python"
                        }
                        span { class: "px-3 py-1 bg-yellow-500/10 text-yellow-300 rounded-full text-sm border border-yellow-500/20",
                            "JavaScript"
                        }
                        span { class: "px-3 py-1 bg-blue-600/10 text-blue-300 rounded-full text-sm border border-blue-600/20",
                            "TypeScript"
                        }
                        span { class: "px-3 py-1 bg-purple-500/10 text-purple-300 rounded-full text-sm border border-purple-500/20",
                            "Rust"
                        }
                        span { class: "px-3 py-1 bg-green-500/10 text-green-300 rounded-full text-sm border border-green-500/20",
                            "Dart"
                        }
                        span { class: "px-3 py-1 bg-gray-500/10 text-gray-300 rounded-full text-sm border border-gray-500/20",
                            "SQL"
                        }
                        span { class: "px-3 py-1 bg-red-500/10 text-red-300 rounded-full text-sm border border-red-500/20",
                            "Shel/Bash"
                        }
                    }
                }

                // Category 2: Databases
                div { class: "bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50 hover:border-green-500/50 transition-all duration-300",
                    div { class: "flex items-center gap-3 mb-4",
                        span { class: "text-2xl", "🗄️" }
                        h3 { class: "text-lg font-semibold text-green-400", "Databases" }
                    }
                    div { class: "flex flex-wrap gap-2",
                        span { class: "px-3 py-1 bg-blue-500/10 text-blue-300 rounded-full text-sm border border-blue-500/20",
                            "PostgreSQL"
                        }
                        span { class: "px-3 py-1 bg-green-600/10 text-green-300 rounded-full text-sm border border-green-600/20",
                            "MongoDB"
                        }
                        span { class: "px-3 py-1 bg-blue-400/10 text-blue-300 rounded-full text-sm border border-blue-400/20",
                            "Neo4j"
                        }
                        span { class: "px-3 py-1 bg-red-500/10 text-red-300 rounded-full text-sm border border-red-500/20",
                            "Redis"
                        }
                        span { class: "px-3 py-1 bg-blue-300/10 text-blue-300 rounded-full text-sm border border-blue-300/20",
                            "SQLite"
                        }
                        span { class: "px-3 py-1 bg-purple-500/10 text-purple-300 rounded-full text-sm border border-purple-500/20",
                            "GraphDB"
                        }
                        span { class: "px-3 py-1 bg-yellow-600/10 text-yellow-300 rounded-full text-sm border border-yellow-600/20",
                            "Cassandra"
                        }
                    }
                }

                // Category 3: Message Brokers
                div { class: "bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50 hover:border-purple-500/50 transition-all duration-300",
                    div { class: "flex items-center gap-3 mb-4",
                        span { class: "text-2xl", "📨" }
                        h3 { class: "text-lg font-semibold text-purple-400", "Message Brokers" }
                    }
                    div { class: "flex flex-wrap gap-2",
                        span { class: "px-3 py-1 bg-orange-500/10 text-orange-300 rounded-full text-sm border border-orange-500/20",
                            "RabbitMQ"
                        }
                        span { class: "px-3 py-1 bg-red-500/10 text-red-300 rounded-full text-sm border border-red-500/20",
                            "Apache Kafka"
                        }
                    }
                }

                // Category 4: Frameworks & Libraries
                div { class: "bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50 hover:border-yellow-500/50 transition-all duration-300",
                    div { class: "flex items-center gap-3 mb-4",
                        span { class: "text-2xl", "⚛️" }
                        h3 { class: "text-lg font-semibold text-yellow-400", "Frameworks & Libraries" }
                    }
                    div { class: "flex flex-wrap gap-2",
                        span { class: "px-3 py-1 bg-teal-500/10 text-teal-300 rounded-full text-sm border border-teal-500/20",
                            "FastAPI"
                        }
                        span { class: "px-3 py-1 bg-blue-400/10 text-blue-300 rounded-full text-sm border border-blue-400/20",
                            "React.js"
                        }
                        span { class: "px-3 py-1 bg-blue-500/10 text-blue-300 rounded-full text-sm border border-blue-500/20",
                            "Flutter"
                        }
                        span { class: "px-3 py-1 bg-purple-500/10 text-purple-300 rounded-full text-sm border border-purple-500/20",
                            "Dioxus"
                        }
                        span { class: "px-3 py-1 bg-orange-500/10 text-orange-300 rounded-full text-sm border border-orange-500/20",
                            "Salvo"
                        }
                        span { class: "px-3 py-1 bg-red-500/10 text-red-300 rounded-full text-sm border border-red-500/20",
                            "Nest.js"
                        }
                    }
                }

                // Category 5: Data Science & ML
                div { class: "bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50 hover:border-pink-500/50 transition-all duration-300",
                    div { class: "flex items-center gap-3 mb-4",
                        span { class: "text-2xl", "🤖" }
                        h3 { class: "text-lg font-semibold text-pink-400", "Data Science & ML" }
                    }
                    div { class: "flex flex-wrap gap-2",
                        span { class: "px-3 py-1 bg-blue-500/10 text-blue-300 rounded-full text-sm border border-blue-500/20",
                            "NumPy"
                        }
                        span { class: "px-3 py-1 bg-green-500/10 text-green-300 rounded-full text-sm border border-green-500/20",
                            "Pandas"
                        }
                        span { class: "px-3 py-1 bg-purple-500/10 text-purple-300 rounded-full text-sm border border-purple-500/20",
                            "Matplotlib"
                        }
                        span { class: "px-3 py-1 bg-blue-400/10 text-blue-300 rounded-full text-sm border border-blue-400/20",
                            "Scikit-Learn"
                        }
                        span { class: "px-3 py-1 bg-green-400/10 text-green-300 rounded-full text-sm border border-green-400/20",
                            "Scikit-Image"
                        }
                        span { class: "px-3 py-1 bg-yellow-500/10 text-yellow-300 rounded-full text-sm border border-yellow-500/20",
                            "Seaborn"
                        }
                        span { class: "px-3 py-1 bg-orange-500/10 text-orange-300 rounded-full text-sm border border-orange-500/20",
                            "Apache Spark"
                        }
                        span { class: "px-3 py-1 bg-red-500/10 text-red-300 rounded-full text-sm border border-red-500/20",
                            "PyTorch"
                        }
                    }
                }

                // Category 6: DevOps & Version Control
                div { class: "bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 border border-gray-700/50 hover:border-cyan-500/50 transition-all duration-300",
                    div { class: "flex items-center gap-3 mb-4",
                        span { class: "text-2xl", "🚀" }
                        h3 { class: "text-lg font-semibold text-cyan-400", "DevOps & Version Control" }
                    }
                    div { class: "flex flex-wrap gap-2",
                        span { class: "px-3 py-1 bg-blue-500/10 text-blue-300 rounded-full text-sm border border-blue-500/20",
                            "Docker"
                        }
                        span { class: "px-3 py-1 bg-blue-600/10 text-blue-300 rounded-full text-sm border border-blue-600/20",
                            "Kubernetes"
                        }
                        span { class: "px-3 py-1 bg-red-500/10 text-red-300 rounded-full text-sm border border-red-500/20",
                            "Jenkins"
                        }
                        span { class: "px-3 py-1 bg-orange-500/10 text-orange-300 rounded-full text-sm border border-orange-500/20",
                            "Git"
                        }
                        span { class: "px-3 py-1 bg-green-500/10 text-green-300 rounded-full text-sm border border-green-500/20",
                            "GitHub Actions"
                        }
                    }
                }
            }
        }
    }
}

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
        // Add more projects as needed
    ];

    rsx! {
        div {
            id: "projects",
            class: "flex flex-col items-center justify-center gap-8 px-4 py-12 w-full max-w-6xl mx-auto",

            // Section header
            div { class: "text-center mb-4",
                h2 { class: "text-3xl sm:text-4xl font-bold mb-2", "Projects" }
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

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
