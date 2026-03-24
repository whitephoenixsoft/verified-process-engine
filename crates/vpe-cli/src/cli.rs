use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vpe")]
#[command(about = "Verified Process Engine CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Validate {
        #[arg(long)]
        schema: String,
        #[arg(long)]
        law: String,
    },
    Compile {
        #[arg(long)]
        schema: String,
        #[arg(long)]
        law: String,
    },
    Manifest {
        #[arg(long)]
        schema: String,
        #[arg(long)]
        law: String,
        #[arg(long)]
        state: String,
    },
    Execute {
        #[arg(long)]
        schema: String,
        #[arg(long)]
        law: String,
        #[arg(long)]
        request: String,
    },
    Simulate {
        #[arg(long)]
        schema: String,
        #[arg(long)]
        law: String,
        #[arg(long)]
        input: String,
    },
    Lift {
        #[arg(long)]
        schema: String,
        #[arg(long)]
        law: String,
        #[arg(long)]
        input: String,
    },
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Validate { schema, law } => crate::commands::validate::run(&schema, &law)?,
        Command::Compile { schema, law } => crate::commands::compile::run(&schema, &law)?,
        Command::Manifest { schema, law, state } => {
            crate::commands::manifest::run(&schema, &law, &state)?
        }
        Command::Execute { .. } => println!(r#"{{\"success\":false,\"data\":null,\"warnings\":[],\"errors\":[{{\"code\":\"NOT_IMPLEMENTED\",\"message\":\"execute not implemented yet\"}}]}}"#),
        Command::Simulate { .. } => println!(r#"{{\"success\":false,\"data\":null,\"warnings\":[],\"errors\":[{{\"code\":\"NOT_IMPLEMENTED\",\"message\":\"simulate not implemented yet\"}}]}}"#),
        Command::Lift { .. } => println!(r#"{{\"success\":false,\"data\":null,\"warnings\":[],\"errors\":[{{\"code\":\"NOT_IMPLEMENTED\",\"message\":\"lift not implemented yet\"}}]}}"#),
    }

    Ok(())
}
