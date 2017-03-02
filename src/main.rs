extern crate resource_proof;
#[macro_use]
extern crate clap;
#[cfg(not(windows))]
extern crate termion;
extern crate rand;

use clap::{App, Arg};
use resource_proof::ResourceProof;
use std::time::Instant;
#[cfg(not(windows))]
use termion::{clear, color};

fn test_it(dif: u8, size: usize, nonce: [u8; 32]) {
    let create = Instant::now();
    let rp = ResourceProof::new(size, dif);
    let data = &mut rp.create_proof_data(&nonce);
    let proof = rp.create_proof(data);
    let create_time = create.elapsed().as_secs();
    let check = Instant::now();
    if !rp.validate_proof(&nonce, proof) {
        println!("FAILED TO CONFIRM PROOF - POSSIBLE VIOLATION");
    }

    if !rp.validate_data(&nonce, data) {
        println!("FAILED TO CONFIRM PROOF DATA - POSSIBLE VIOLATION");
    }

    if !rp.validate_all(&nonce, data, proof) {
        println!("FAILED TO CONFIRM PROOF & DATA - POSSIBLE VIOLATION");
    }

    println!("Difficulty: {:<8}Size: {:<8}Created in {} seconds.  Checked in {} seconds.  Number \
              of attempts: {:?}",
             dif,
             size,
             create_time,
             check.elapsed().as_secs(),
             proof);
}

#[cfg(not(windows))]
fn print_red(message: &str) {
    println!("{}", clear::All);
    println!("{}{}{}",
             color::Fg(color::Red),
             message,
             color::Fg(color::Reset));
}

#[cfg(windows)]
fn print_red(message: &str) {
    println!("{}", message);
}

fn main() {
    let matches = App::new("=============================\nSimple Resource Proof \
                            example\n=============================\n")
        .about("______________________________\nPlease set the size and difficulty to test")
        .author(crate_authors!())
        .version(crate_version!())
        .before_help("Resource proof testing framework")
        .after_help("_____________________________________________________________\nSeveral \
                     proofs may be chained, i.e. a large difficulty and small size or large size \
                     and small difficulty to check specifically CPU And BW separately")
        .arg(Arg::with_name("Difficulty")
            .short("d")
            .required(true)
            .long("difficulty")
            .help("Set difficulty, i.e. the number of leading zeros of the proof when hashed \
                   with SHA3")
            .takes_value(true))
        .arg(Arg::with_name("Size")
            .required(true)
            .short("s")
            .long("size")
            .help("Set size, i.e. the minimum size of the proof in bytes")
            .takes_value(true))
        .arg(Arg::with_name("Increase")
            .short("i")
            .long("increase")
            .help("Will run continuously, increasing difficulty with every invocation. Note \
                   this will likely not stop in your lifetime :-)"))
        .get_matches();

    print_red("Running analysis ....");

    let repeat = matches.is_present("Increase");

    let dif = value_t!(matches, "Difficulty", u8).unwrap_or(1);

    let size = value_t!(matches, "Size", usize).unwrap_or(10);

    let nonce = [rand::random::<u8>(); 32];

    if repeat {
        for i in dif.. {
            test_it(i, size, nonce);
        }
    } else {
        test_it(dif, size, nonce);
    }

}
