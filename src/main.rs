use num_cpus;
use std::thread;

const STEP: u128 = 5_000_000;

fn main() {
  let threads = num_cpus::get();
  let mut subject = 0u128;

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
