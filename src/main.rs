use getopts::Options;
use indicatif::{ProgressBar, ProgressStyle};
use parse_duration::parse;
use rustc_version::version;
use std::env;
use std::thread::sleep;
use std::time::Duration;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] SECONDS", program);
    println!("{}", opts.usage(&brief))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("v", "version", "Print the version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let show_version = matches.opt_present("v");
    if show_version {
        println!("{} {} (rustc {})", NAME, VERSION, version().unwrap());
        return;
    }

    if matches.free.is_empty() {
        print_usage(NAME, opts);
        return;
    }

    let duration = {
        let duration_string = matches.free[0].clone();
        let duration = parse(&duration_string).unwrap();
        duration
    };
    let millis = duration.as_millis() as u64;
    let progress_per_scale = {
        let u = millis / 100;
        if u < 1 {
            1
        } else {
            u
        }
    };

    let pb = ProgressBar::new(progress_per_scale);
    pb.set_style(ProgressStyle::with_template("{wide_bar} {percent}%").unwrap());
    for _ in 0..progress_per_scale {
        pb.inc(1);
        sleep(Duration::from_millis(100));
    }
    pb.finish();
}
