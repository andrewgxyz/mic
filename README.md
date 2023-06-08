# Scat - Music Analytics

`scat` (mostly a play on the vocal style and the word stat) is a program that runs though your music folder and get general information on your collection.

## Installation

Some prerequisites, you need to install rust on your system and one external program that generates the collage image **ImageMagick**.

```bash
git clone https://github.com/andrewgxyz/scat.git
cd scat
cargo build --release && cp ./target/release/scat ~/.local/bin
```


## Usage 


Also some **warnings**:

- I only accept the `<artist>/<album>/01-<title>.<extention>` for what the program ends up finding
- Similarly the cover has to be named `cover.<extention>` and has to be placed with the respective album
- The first run will be the longest to generate the collage depending on collection size, the next runs will be much quicker since I cache everything afterwards
- This program will likely not work on Windows/MacOS, mostly been focusing on Linux development, might need to take some time on that

