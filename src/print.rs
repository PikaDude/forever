use crate::models::{HelpOption, OptionsArgs};

pub fn ln(message: String) {
    println!("[forever] {}", message);
}

pub fn error(message: String) {
    ln(format!("error: {}", message));
}

pub fn help() {
    let defaults = OptionsArgs::default();
    let options: Vec<HelpOption> = vec![
        HelpOption {
            command: "-h, --help".to_string(),
            description: "print this help menu".to_string(),
        },
        HelpOption {
            command: "-d, --delay <ms>".to_string(),
            description: format!("delay between restarts (default: {})", defaults.delay),
        },
        HelpOption {
            command: "-m, --min-uptime <ms>".to_string(),
            description: format!(
                "minimum uptime before restarting (default: {})",
                defaults.min_uptime
            ),
        },
        HelpOption {
            command: "-q, --quiet-restart".to_string(),
            description: "don't print anything when restarting".to_string(),
        },
    ];
    let longest_command = options
        .iter()
        .map(|option| option.command.len())
        .max()
        .unwrap()
        + 4;

    println!("forever v{}", env!("CARGO_PKG_VERSION"));
    println!("usage: forever [options] <command> [command args]");
    println!("options:");
    for option in options {
        println!(
            "  {:width$} {}",
            option.command,
            option.description,
            width = longest_command
        );
    }
}
