use clap::{Parser, Subcommand};
use git2::{Repository, Signature};
use std::{fs, path::Path, process::Command};

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

        /// Additional dependencies to add (e.g. dotenvy)
        #[arg(short, long)]
        deps: Option<Vec<String>>,
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

fn get_main_content(framework: &str) -> &'static str {
    match framework {
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
    }
}

fn create_module_dir(project_name: &str, module_name: &str) {
    let module_dir = Path::new(project_name).join("src").join(module_name);
    fs::create_dir_all(&module_dir)
        .unwrap_or_else(|_| panic!("Failed to create {} directory", module_name));

    let mod_path = module_dir.join("mod.rs");
    fs::write(mod_path, "").unwrap_or_else(|_| panic!("Failed to create {}/mod.rs", module_name));
}

fn add_dependency(project_name: &str, dep: &str, features: Option<&str>) -> bool {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(project_name).arg("add").arg(dep);

    if let Some(feat) = features {
        cmd.args(["--features", feat]);
    }

    cmd.status().expect("Failed to run cargo add").success()
}

fn create_gitignore(project_name: &str) {
    let gitignore_content = r#"# Rust
/target/


# Environment
.env
.env.local
.env.*.local


"#;
    
    let gitignore_path = Path::new(project_name).join(".gitignore");
    fs::write(gitignore_path, gitignore_content)
        .unwrap_or_else(|_| panic!("Failed to create .gitignore file"));
}

fn init_git_repo(project_name: &str) -> Result<(), git2::Error> {
    let repo_path = Path::new(project_name);

    // Initialize a new repository
    let repo = Repository::init(repo_path)?;

    // Create a signature for the commit
    let sig = Signature::now("Rust Backend Scaffolder", "scaffolder@example.com")?;

    // Add all files to the index
    let mut index = repo.index()?;
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create the initial commit
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Since this is a new repository, there are no parent commits
    let parents: Vec<&git2::Commit> = vec![];

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit: Scaffolded project",
        &tree,
        &parents,
    )?;

    Ok(())
}

fn scaffold_project(name: &str, framework: &str, deps: Option<Vec<String>>) {
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

    // Add framework dependency
    println!("Adding {} to {}", framework, name);
    if !add_dependency(name, framework, None) {
        eprintln!("Failed to add framework dependency '{}'", framework);
        return;
    }

    // Add additional dependencies
    if let Some(deps) = deps {
        for dep in deps {
            if !add_dependency(name, &dep, None) {
                eprintln!("Failed to add dependency '{}'", dep);
                return;
            }
        }
    }

    // Write main.rs based on framework
    let main_content = get_main_content(framework);
    let main_path = format!("{}/src/main.rs", name);
    fs::write(&main_path, main_content).expect("Failed to write main.rs");

    // Add additional dependencies for async frameworks
    if matches!(framework, "axum" | "actix-web") {
        add_dependency(name, "serde", Some("derive"));
        add_dependency(name, "tokio", Some("full"));
    }

    // Create module directories
    let modules = vec!["services", "models", "handlers", "routes"];
    for module in modules {
        create_module_dir(name, module);
    }

    // Create .gitignore file
    println!("Creating .gitignore file");
    create_gitignore(name);

    // Initialize git repository
    println!("Initializing git repository");
    match init_git_repo(name) {
        Ok(_) => println!("Git repository initialized successfully"),
        Err(e) => eprintln!("Failed to initialize git repository: {}", e),
    }

    println!("\n✅ Project '{}' scaffolded successfully!", name);
    println!("👉 cd {} && cargo run", name);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scaffold {
            name,
            framework,
            deps,
        } => {
            scaffold_project(&name, &framework, deps);
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
