# K-Math
Math library in Rust by Kirdow. Made to practice math and understand algorithms by making them from scratch, but also to learn Rust by making various projects using it.

# Building
Requires a few dependencies on the OS. Notably the usually GCC tools and dependencies. Also requires m4 for GMP. You should be able to figure that out on your own.

To build, simply run `./build.sh`. This is a wrapper for `cargo build` as well as a copy of the target debug binary to `./kmath`.
*Always look at a script before running it.*

# Running
First read the previous section, then proceed here.

To run the program, run `./kmath`. This will ask you for a number and then proceed to calculate the square root of that number.

You can also do it inline by running `./kmath <number>` which in turn will also output only the desired number with no extra data.

You can also to both of these provide `--precision <integer>` which will determine how many decimals you get. The default for this value is 3000.

You can also run `./run.sh` which also invokes `./build.sh` prior to running the program. `./run.sh` also runs without an argument, so you'll get the prompt with the extra data in doing so.

### Example
`./kmath --precision 5 2` will output the square root of 2 with 5 decimal precision.<br>
`./kmath 2` will output the square root of 2 with 3000 decimal precision.

# License
This repository and its code is licensed under [MIT License](https://github.com/Kirdow/kmath-rs/blob/master/LICENSE).

