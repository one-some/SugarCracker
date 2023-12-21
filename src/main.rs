// Naive collision bruteforcer for Java String.hashCode() (and cloned implementations)
// First Rust project that compiles. Sry, don't know enough about how data oughta be structured to
// add command line args last minute

use std::time::SystemTime;

// Jemalloc speeds up the loop by ~30%
use tikv_jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;


const TARGET: i32 = 1395333309; // "some string"
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz ";

fn factorial(num: u128) -> u128 {
    return (1..=num).product();
}

fn permute(bit: &str, ttl: u16) {
    // Originally I thought that just passing the hash through the recursive function would be more
    // effecient, since we wouldn't have to lug around the string. Then I realized you need the
    // string because finding the string is literally the whole point of the program
	let mut hash: i32 = 0;

    // The actual hashing algorithm
	for c in bit.chars() {
		let chr = c as i32;
		hash = ((hash << 5).wrapping_sub(hash)) + chr;
	}

    // Cool, sound the victory alarm and continue
    if hash == TARGET {
        println!("{} == {}", bit, TARGET);
    }

    // Stop iterating!
    if ttl == 0 {
        return;
    }

    // Somehow we are wasting calls on duplicate hashes here (or maybe somewhere else)
    for c in ALPHABET.chars() {
        // May be a better way to do this.
        permute(&[bit, &c.to_string()].join(""), ttl - 1);
    }
}

fn main() {
    // You'll get a few collisions on 7, and a bajillion on 8. 12 is entirely unreasonable unless
    // you happen to be working at Oak Ridge
    for n_chars in 1..12 {
        // Very questionable-looking cast-splattering. It may at this point become apparent I have
        // no idea how Rust works
        let alpha_len = ALPHABET.chars().count() as u128;
        let perm_top = factorial(alpha_len) as u128;
        let perm_bottom = factorial(alpha_len-n_chars) as u128;
        let perms: u64 = (perm_top / perm_bottom) as u64;

        println!("{} chars, {} permutations", n_chars, perms);

        let mut handles = vec![];
        let start = SystemTime::now();

        for c in ALPHABET.chars() {
            // Spawns lotsa (26+) threads. This will probably be overhead unless you have a bunch
            // of cores. (Sorry!!)
            handles.push(
                std::thread::spawn(move || {
                    // Obviously optional, but since the bottleneck is mostly on malloc calls for
                    // string stuff this doesn't seem to have any penalty
                    permute(&["flexing-1337-cracking-skillz-", &c.to_string()].join(""), (n_chars-1) as u16);
                })
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = SystemTime::now().duration_since(start).unwrap();
        println!("nloop took {} seconds", (duration.as_millis() as f64) / 1000.0);
    }
}
