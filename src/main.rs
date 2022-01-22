use std::thread;
use std::io;

const STEP: u128 = 5_000_000;

fn main() -> io::Result<()> {
  let threads = num_cpus::get();

  print!("From which number do we start? ");
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  let mut subject: u128 = input.trim().parse().expect("Expected numerical input");

  loop {
    println!("Iteration -- {}", subject);
    let mut threadvec = Vec::with_capacity(threads);

    for _thread in 0..threads {
      let initial = subject+1;
      let boundary = subject+STEP;

      let processing = move || {
        let mut my_subject = initial;
        
        while my_subject <= boundary {
          let mut copy = my_subject;
    
          while copy != 1 {
            if copy & 0x1 != 0 {
              copy >>= 2;
            } else {
              copy = 3*copy + 1;
            }
          }
    
          my_subject += 1;
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
