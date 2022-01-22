use std::thread;
use std::io::{self, Write};

fn main() -> io::Result<()> {
  let threads = num_cpus::get();

  let mut input = String::new();
  read_input("From which number do we start? ", &mut input)?;
  let mut subject: u128 = input.trim().parse().expect("Expected numerical input >= 0");

  let mut input = String::new(); // Parsing throws a ParseIntError { kind: InvalidDigit } if I try to use the same string for everything
  read_input("What is the desired step? ", &mut input)?;
  let step: u128        = input.trim().parse().expect("Expected numerical input >= 0");

  let mut input = String::new();
  read_input("I'll print the number each this number of iterations: ", &mut input)?;
  let printing: u128    = input.trim().parse().expect("Expected numerical input >= 0");
  let printing = printing*step;

  loop {
    if subject % printing == 0 {
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

      subject += step;
    }

    for thread in threadvec {
      thread.join().expect("PANIC");
    }
  }
}

fn read_input(msg: &str, string: &mut String) -> io::Result<()> {
  print!("{}", msg);
  io::stdout().flush()?;
  io::stdin().read_line(string)?;
  Ok(())
}