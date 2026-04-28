use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const FRAMES: [&str; 4] = [
    r"
        /\_/\
       ( o   o )
        >  ^  <
       (       )
      ( (     ) )
       `~~~~~~~`
",
    r"
        /\_/\
       ( -   - )
        >  ^  <
       (       )
      ( (     ) )
       `~~~~~~~`
",
    r"
        /\_/\
       ( o   o )
        >  w  <
       (       )
      ( (     ) )
       `~~~~~~~`
",
    r"
        /\_/\
       ( ^   ^ )
        >  w  <
       (       )
      ( (     ) )
       `~~~~~~~`
",
];

fn main() {
    print!("\x1b[2J");
    io::stdout().flush().ok();

    let mut frame: usize = 0;
    loop {
        print!("\x1b[H");
        println!("   ~ Kitty ~\n");
        print!("{}", FRAMES[frame % FRAMES.len()]);
        println!("\n   (Ctrl+C to exit)   ");
        io::stdout().flush().ok();

        thread::sleep(Duration::from_millis(350));
        frame = frame.wrapping_add(1);
    }
}
