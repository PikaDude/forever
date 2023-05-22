use forever::{args, print, run_loop};

fn main() {
    let options = args::parse();
    if options.is_none() {
        return;
    }
    let options = options.unwrap();

    print::ln(format!(
        "starting \"{}\" with a minimum uptime of {:?} and a delay of {:?}",
        options.command_name, options.min_uptime, options.delay
    ));

    run_loop(options);
}
