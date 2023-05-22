use crate::{models::Options, print};
use std::{process::Command, time::Instant};

pub fn run_loop(options: Options) {
    let mut restart_count = 0;

    loop {
        if restart_count != 0 && !options.quiet_restart {
            print::ln(format!("restart attempt #{}", restart_count));
        }

        let start_time = Instant::now();
        let child = Command::new(options.command_name.clone())
            .args(options.command_args.clone())
            .spawn();

        if child.is_err() {
            print::ln(format!("failed to start process: {}", child.err().unwrap()));
            break;
        }
        let mut child = child.unwrap();

        let ecode = child.wait().expect("failed to wait on child");
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        let will_restart = duration.as_millis() > options.min_uptime.as_millis();

        if !options.quiet_restart {
            print::ln(format!(
                "process exited after {:?} with code {}{}",
                duration,
                ecode.code().unwrap(),
                if will_restart {
                    format!(", restarting in {:?}...", options.delay)
                } else {
                    "".to_string()
                }
            ));
        }

        if !will_restart {
            print::ln(format!(
                "process exited too quickly (minimum is {:?}), not restarting",
                options.min_uptime
            ));
            break;
        }

        restart_count += 1;
        std::thread::sleep(options.delay);
    }
}
