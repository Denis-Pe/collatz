use std::thread;
use std::io::{self, Write};

const STEP: u128 = 5_000_000;
const PRINTING: u128 = 100_000_000_000; // to stop spamming numbers, this is very fast to print each few million

fn main() -> io::Result<()> {
  let threads = num_cpus::get();

  print!("From which number do we start? ");
  io::stdout().flush()?;
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  let mut subject: u128 = input.trim().parse().expect("Expected numerical input >= 0");

  loop {
    if subject % PRINTING == 0 {
      println!("Iteration -- {}", subject);
    }
    let mut threadvec = Vec::with_capacity(threads);

    for _thread in 0..threads {
      let initial = subject+1;

      let processing = move || {
        let mut my_subject = initial;
    
        while my_subject != 1 {
          if my_subject & 0x1 != 0 {
            my_subject >>= 2;
          } else {
            my_subject = 3*my_subject + 1;
          }
        }
      };

      threadvec.push(thread::spawn(processing));

      subject += STEP;
    }

    for thread in threadvec {
      thread.join().expect("PANIC");
    }
  }
}
