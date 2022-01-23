use std::thread;
use std::io::{self, Write};

fn main() -> io::Result<()> {
  let threads = num_cpus::get();

  let input = read_input("From which number do we start? ")?;
  let mut subject: u128 = input.trim().parse().expect("Expected numerical input >= 0");

  let input = read_input("What is the desired step? ")?;
  let step:        u128 = input.trim().parse().expect("Expected numerical input >= 0");

  let input = read_input("I'll print the number each this number of iterations: ")?;
  let printing:    u128 = input.trim().parse().expect("Expected numerical input >= 0");
  let printing = printing*step;

  drop(input); // makes me happy to drop these bytes before entering the loop :)

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
          if my_subject & 0x1 == 0 {
            my_subject >>= 1;
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

fn read_input(msg: &str) -> io::Result<String> {
  let mut string = String::new();
  print!("{}", msg);
  io::stdout().flush()?;
  io::stdin().read_line(&mut string)?;
  Ok(string)
}