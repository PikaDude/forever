use crate::{
    models::{Options, OptionsArgs},
    print,
};

pub fn parse() -> Option<Options> {
    let mut options = OptionsArgs::default();

    let mut args = std::env::args();
    args.next(); // skip program name
    loop {
        let arg = args.next();
        if arg.is_none() {
            break;
        }
        let arg = arg.unwrap();

        if arg.starts_with('-') {
            // handle flags
            match arg.as_str() {
                "--help" | "-h" => {
                    print::help();
                    return None;
                }
                "--delay" | "-d" => {
                    let delay_arg = parse_u64("delay", args.next().unwrap());
                    delay_arg?;
                    options.delay = delay_arg.unwrap();
                }
                "--min-uptime" | "-m" => {
                    let min_uptime_arg = parse_u64("min-uptime", args.next().unwrap());
                    min_uptime_arg?;
                    options.min_uptime = min_uptime_arg.unwrap();
                }
                "--quiet-restart" | "-q" => {
                    options.quiet_restart = true;
                }
                _ => {
                    // unknown flag
                }
            }
        } else {
            // handle command name and args
            options.command_name = Some(arg);
            options.command_args = Some(args.collect::<Vec<String>>());
            break;
        }
    }

    if options.command_name.is_none() {
        print::help();
        return None;
    }

    Some(Options::new(options))
}

fn parse_u64(name: &str, value: String) -> Option<u64> {
    let parsed = value.parse::<u64>();
    if parsed.is_err() {
        print::error(format!(
            "option \"{}\" must be a valid number greater than or equal 0",
            name
        ));
        return None;
    }

    Some(parsed.unwrap())
}
