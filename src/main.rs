use sha1::Digest;
use std::{
     env, error::Error, fs::File, 
     io::{BufRead, BufReader,},
     time::Instant,
    };

const SHA1_HEX_STRING_LENGTH: usize = 40;


fn main() -> Result<(), Box<dyn Error>> {


    //read command line arguments   
    let args: Vec<String> = env::args().collect();

    if args.len() !=3 {
        print!("Usage: ");
        print!("sha1_cracker: <wordlist.txt> <sha1 hash>\n");
        return Ok(());
        }

    let hash = args[2].trim();
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wl = File::open(&args[1])?;
    let reader = BufReader::new(&wl);
    
    let start: Instant = Instant::now();
    let mut counter  = 0;
    for line in reader.lines() {
        let line = line?;
        let password = line.trim();
        counter += 1;
        if hash == 
            &hex::encode(sha1::Sha1::digest(password.as_bytes())){
            println!("\n----- PASSWORD FOUND -----\n");
            print!("\t{}", &password);
            print!("\n\n------------------------\n");
            // Calculate and print the elapsed time in milliseconds
            let duration = start.elapsed().as_millis();
            //println!("Total duration: {} milliseconds", duration);

            let duration_seconds = duration as f64 / 1000.0;
            println!("\nTotal duration: {} seconds", duration_seconds);

            let tpp = duration_seconds / counter as f64;
            println!("Time per password: {} seconds", tpp);
            return Ok(())
            }   
    }
    print!("\nPASS NOT FOUND :(\n ");


    Ok(())
    
}
