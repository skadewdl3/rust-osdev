# Rust OS Dev
 This repository documents my exploration in operating system development using (primarily) Rust.

# Install Required Tools
1. Setup [Docker](https://www.docker.com/) on your system.
2. Setup a package manager for your system (this will make it easier to install the required tools)
    - Windows: Install [Chocolatey](https://chocolatey.org/install).
    - MacOS: Install [Homebrew](https://brew.sh/).
    - Linux: No need to install anything, you can use your distributions package manager.
2. Install make for your OS. We'll be using this to automate our build system.
    - Windows: From the command line, run `choco install make`.
    - MacOS: From the terminal, run `brew install make`.
    - Linux: Use your distributions package manager. For eg., on Ubuntu run `apt-get install make`.
3. Install Qemu to test out the operating system.
    - Windows: From the command line, run `choco install qemu`.
    - MacOS: From the terminal, run `brew install qemu`.
    - Linux: Use your distributions package manager or follow the [install instructions](https://www.qemu.org/download/).

# Build Environment Setup
1. Clone this repository. `cd` into the directory.
2. Generate the build environment image by running `make env` (this might take a while).
3. This should install all required tools (GCC cross compiler, xorisso, GRUB tools, nasm, the nightly Rust toolchain etc.)

# Compiling and Running the OS
1. Firstly, run the build environment docker image: `make docker`.
2. Then, compile the kernel and create the iso using: `make iso`.
3. To test the OS, run `make run` on your system shell.

# Testing
1. The project does compiles for a bare metal target, hence it does not use the Rust standard library.
2. I couldn't get `cargo test` to work with no-std, so I've used a scrappy custom testing framework.
3. Annotate tests with `#[distributed_slice(crate::tests::TESTS)]` to add it to the list of tests.
4. This is very much of a work in progress. I intend to have a `#[test_case]` macro similar to standard Rust, to annotate test cases.
