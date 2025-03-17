use colored::*;

#[allow(dead_code)]
pub enum CCSSError {
    ParseError {
        line: usize,
        column: usize,
        message: String,
        snippet: String,
    },
    FileError {
        path: String,
        message: String,
    },
    VariableError {
        name: String,
        message: String,
    },
    Generic(String),
}

pub fn display_error(error: &CCSSError) {
    match error {
        CCSSError::ParseError { line, column, message, snippet } => {
            let header = " ERROR ".black().on_red().bold();
            let location = format!("{}: {}:{}", "SYNTAX ERROR".red().bold(), line, column);

            println!("\n{} {}\n", header, location);
            let trimmed = message.split("expected").nth(1).unwrap_or(&message);
            println!("{}  Expected: {}\n", "→".red().bold(), trimmed.white().bold());

            // Display code snippet with highlighting
            let lines: Vec<&str> = snippet.lines().collect();
            println!("{}", "╭─────────────────────────────────────────────────────".bright_red());
            println!("{}", "│".bright_red());

            for (i, line_content) in lines.iter().enumerate() {
                let line_num = (line - 1 + i).to_string();
                println!("{} {: >3} │ {}", "│".bright_red(), line_num.white(), line_content);

                if i == 1 {
                    // Highlight the error position with an arrow
                    let mut pointer = " ".repeat(*column);
                    pointer.push('↑');
                    println!(
                        "{} {: >3} │ {}",
                        "│".bright_red(),
                        " ".bright_yellow(),
                        pointer.bright_red().bold()
                    );
                }
            }

            println!("{}", "│".bright_red());
            println!("{}", "╰─────────────────────────────────────────────────────".bright_red());
        }

        CCSSError::FileError { path, message } => {
            let header = " ERROR ".black().on_red().bold();
            println!("\n{} {}\n", header, "FILE ERROR".red().bold());
            println!("{}  {} {}", "→".red().bold(), "Path:".yellow(), path);
            println!("{}  {}\n", "→".red().bold(), message.white().bold());
        }

        CCSSError::VariableError { name, message } => {
            let header = " ERROR ".black().on_red().bold();
            println!("\n{} {}\n", header, "VARIABLE ERROR".red().bold());
            println!("{}  {} ${}", "→".red().bold(), "Variable:".yellow(), name);
            println!("{}  {}\n", "→".red().bold(), message.white().bold());
        }

        CCSSError::Generic(message) => {
            let header = " ERROR ".black().on_red().bold();
            println!("\n{} {}\n", header, message.red().bold());
        }
    }
}
