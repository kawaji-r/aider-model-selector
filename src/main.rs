use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // aider --list-models を実行してモデルリストを取得
    let output = Command::new("aider")
        .arg("--list-models")
        .arg("openrouter/")
        .stdout(Stdio::piped())
        .output()?;

    if !output.status.success() {
        eprintln!("Failed to execute 'aider --list-models'");
        eprintln!("Error: {}", output.status);
        if !output.stderr.is_empty() {
            eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        std::process::exit(1);
    }

    let models_str = String::from_utf8(output.stdout)?;
    let models: Vec<&str> = models_str
        .lines()
        .map(|line| line.trim_start_matches("- "))
        .collect();

    if models.is_empty() {
        eprintln!("No models available");
        std::process::exit(1);
    }

    // ユーザーにモデルを選択させる
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a model")
        .items(&models)
        .interact()?;

    let selected_model = models[selection];

    // 選択したモデルでaiderを起動
    Command::new("aider")
        .arg("--model")
        .arg(selected_model)
        .spawn()?;

    Ok(())
}
