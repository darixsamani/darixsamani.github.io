use dioxus::prelude::*;
const IMAGES: Asset = asset!("/assets/IMG_9109-1024x683.jpg");

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
