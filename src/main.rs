use enigo::*;
use rand::prelude::*;
use std::{thread::sleep, time::Duration};
use structopt::StructOpt;

fn main() -> Result<(), String> {
    print_title();

    let mut enigo = Enigo::new();

    let opt = Opt::from_args();

    if opt.debug {
        println!("Options: {:#?}\n", opt);
    }

    let mut rng = rand::thread_rng();

    loop {
        if let Err(message) = move_mouse(&mut enigo, &opt, &mut rng) {
            return Err(format!("{}. Exiting gracefully.", message));
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "afk",
    about = "A command-line program to make your mouse wander. ;)"
)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Max distance in pixels for mouse to move. Don't make this too big.
    /// The mouse won't move at all if there is no room!
    #[structopt(long = "max-distance", default_value = "100")]
    max_distance: i32,

    /// Min distance in pixels for mouse to move
    #[structopt(long = "min-distance", default_value = "1")]
    min_distance: i32,

    /// Max delay time in seconds
    #[structopt(long = "max-delay", default_value = "30")]
    max_delay: u64,

    /// Min delay time in seconds
    #[structopt(long = "min-delay", default_value = "5")]
    min_delay: u64,
}

fn print_title() {
    println!(
        "\n
█████╗ ███████╗██╗  ██╗   ██████╗ ███████╗
██╔══██╗██╔════╝██║ ██╔╝   ██╔══██╗██╔════╝
███████║█████╗  █████╔╝    ██████╔╝███████╗
██╔══██║██╔══╝  ██╔═██╗    ██╔══██╗╚════██║
██║  ██║██║     ██║  ██╗██╗██║  ██║███████║
╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝╚═╝╚═╝  ╚═╝╚══════╝
                                        
Press CTRL-C to exit.\n
    "
    );
}

fn move_mouse(enigo: &mut Enigo, opt: &Opt, rng: &mut ThreadRng) -> Result<(), &'static str> {
    if opt.min_delay > opt.max_delay {
        return Err("min-delay is greater than max-delay");
    }

    let (x_delta, y_delta, distance) = get_coordinate_deltas(opt, rng)?;

    enigo.mouse_move_relative(x_delta, y_delta);

    if opt.verbose > 0 {
        println!("🐭 Mouse Delta (x, y): ({}, {})", x_delta, y_delta);
        println!("Traveled ~{} pixels.", distance);
    }

    let sleep_time = rng.gen_range(opt.min_delay..=opt.max_delay);

    println!("Will move again in {} seconds.", sleep_time);

    sleep(Duration::from_secs(sleep_time));

    if opt.verbose > 0 {
        println!();
    }

    Ok(())
}

fn get_coordinate_deltas(opt: &Opt, rng: &mut ThreadRng) -> Result<(i32, i32, i32), &'static str> {
    if opt.min_distance > opt.max_distance {
        return Err("min-distance is greater than max-distance");
    }

    let distance = rng.gen_range(opt.min_distance..=opt.max_distance);

    let x_absolute = rng.gen_range(0..=distance);

    let x_delta = if rng.gen() { x_absolute } else { -x_absolute };

    let y_absolute = ((distance.pow(2) - x_absolute.pow(2)) as f64).sqrt() as i32;

    let y_delta = if rng.gen() { y_absolute } else { -y_absolute };

    Ok((x_delta, y_delta, distance))
}
