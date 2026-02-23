use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scaffold a new framework project
    Scaffold {
        /// Name of the project
        #[arg(short, long)]
        name: String,

        /// Name of the framework (e.g. axum, actix-web)
        #[arg(short, long)]
        framework: String,
    },

    /// List available frameworks
    List,

    /// Add a dependency to the project
    Add {
        /// Name of the crate to add
        name: String,

        /// Version to use
        #[arg(short, long, default_value = "latest")]
        version: String,
    },
}

fn scaffold_project(name: &str, framework: &str) {
    println!("Creating new Cargo project: {}", name);

    // Run `cargo new <name>`
    let status = Command::new("cargo")
        .args(["new", name])
        .status()
        .expect("Failed to run cargo new");

    if !status.success() {
        eprintln!("Failed to create project '{}'", name);
        return;
    }

    // Build the dependency string e.g. axum or axum
    let dependency = framework.to_string();

    println!("Adding {} to {}", dependency, name);

    // Run `cargo add <framework>` inside the new project directory
    let mut add_cmd = Command::new("cargo");
    add_cmd.current_dir(name).arg("add").arg(framework);

    let status = add_cmd.status().expect("Failed to run cargo add");

    if !status.success() {
        eprintln!("Failed to add dependency '{}'", framework);
        return;
    }

    // Add a basic main.rs based on the framework
    let main_content = match framework {
        "axum" => {
            r#"use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello from Axum!" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
"#
        }
        "actix-web" => {
            r#"use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix-web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:3000");
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
"#
        }
        _ => {
            r#"fn main() {
    println!("Hello, world!");
}
"#
        }
    };

    let main_path = format!("{}/src/main.rs", name);
    fs::write(&main_path, main_content).expect("Failed to write main.rs");

    // Add tokio for async frameworks
    if framework == "axum" || framework == "actix-web" {
        Command::new("cargo")
            .current_dir(name)
            .args(["add", "tokio", "--features", "full"])
            .status()
            .expect("Failed to add tokio");
    }

    println!("\n✅ Project '{}' scaffolded successfully!", name);
    println!("👉 cd {} && cargo run", name);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scaffold { name, framework } => {
            scaffold_project(&name, &framework);
        }
        Commands::List => {
            println!("Available frameworks:");
            println!("  - axum");
            println!("  - actix-web");
        }
        Commands::Add { name, version } => {
            let status = if version == "latest" {
                Command::new("cargo")
                    .args(["add", &name])
                    .status()
                    .expect("Failed to run cargo add")
            } else {
                Command::new("cargo")
                    .args(["add", &format!("{}@{}", name, version)])
                    .status()
                    .expect("Failed to run cargo add")
            };

            if status.success() {
                println!("✅  Added {} successfully!", name);
            } else {
                eprintln!("❌ Failed to add {}", name);
            }
        }
    }
}
