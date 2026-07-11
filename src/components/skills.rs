use dioxus::prelude::*;


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