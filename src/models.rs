use std::time::Duration;

pub struct OptionsArgs {
    pub command_name: Option<String>,
    pub command_args: Option<Vec<String>>,
    pub delay: u64,
    pub min_uptime: u64,
    pub quiet_restart: bool,
}
impl Default for OptionsArgs {
    fn default() -> Self {
        Self {
            command_name: None,
            command_args: None,
            delay: 1000,
            min_uptime: 1000,
            quiet_restart: false,
        }
    }
}

pub struct Options {
    pub command_name: String,
    pub command_args: Vec<String>,
    pub delay: Duration,
    pub min_uptime: Duration,
    pub quiet_restart: bool,
}
impl Options {
    pub fn new(options_args: OptionsArgs) -> Self {
        Self {
            command_name: options_args.command_name.unwrap(),
            command_args: options_args.command_args.unwrap(),
            delay: Duration::from_millis(options_args.delay),
            min_uptime: Duration::from_millis(options_args.min_uptime),
            quiet_restart: options_args.quiet_restart,
        }
    }
}

pub struct HelpOption {
    pub command: String,
    pub description: String,
}
