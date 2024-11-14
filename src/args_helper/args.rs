use clap::Parser;

/// Simple program to change connstring value in secret.json
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ProgramArgs {
    /// The directory of the secret.json files (contains directories with guid as names)
    #[arg(short, long)]
    secret_dir: String,

    /// The key for connstring value in your connstring.json
    #[arg(short, long)]
    connstring_key: String,
}

impl ProgramArgs {
  pub fn new() -> Self {
    return ProgramArgs::parse();
  }

  pub fn secret_dir(&self) -> &str {
    &self.secret_dir
  }

  pub fn connstring_key(&self) -> &str {
    &self.connstring_key
  }
}