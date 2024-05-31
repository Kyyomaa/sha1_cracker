# SHA1 Cracker

This is a simple SHA1 hash cracker written in Rust. It reads a wordlist from a file and tries to find a matching password for a given SHA1 hash.

## Getting Started

These instructions will help you set up and run the SHA1 cracker on your local machine.

### Prerequisites

- Rust (installation guide: [Rust Install](https://www.rust-lang.org/tools/install))

### Usage

1. Clone the repository:
   ```
    git clone <repository_url>
    cd <repository_folder>
    ```
2. Compile the code:
   ```
    cargo build --release
    
3. Run the cracker:
   ```
    ./target/release/sha1_cracker <wordlist.txt> <sha1 hash>
    
### Example

```
./target/release/sha1_cracker wordlist.txt 5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8
```

## NOTES 
- This code is taken from "Black Hat Rust" by Sylvain Kerkour, to which i have added and i will be adding some things. It's an exercise part of my journey of learning Rust for cybersecurity.
  
  message: If you use this software, please cite it using these metadata.
   - title: Black Hat Rust
   - abstract: Applied offensive security with the Rust programming language
   - authors:
     - name: "Sylvain Kerkour"
   - version: v2022.56

## TODO 
- Multithreading 
- Implement a more efficent search algorithm
- Make a better UI 
(some day i will :) ) 

much love


