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
3. Write your test functions inside the crate::test_cases! macro, to run them as test cases. Example - 
```rust
crate::test_cases! {
    fn box_allocation() {
        let heap_value_1 = Box::new(41);
        let heap_value_2 = Box::new(13);
        assert_eq!(*heap_value_1, 41);
        assert_eq!(*heap_value_2, 13);
    }

    fn vector_allocation() {
        let n = 1000;
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    }


    fn multiple_boxes_causing_reallocation() {
        for i in 0..HEAP_SIZE {
            let x = Box::new(i);
            assert_eq!(*x, i);
        }
    }

    fn reference_counting() {
        let rc = Rc::new(42);
        assert_eq!(Rc::strong_count(&rc), 1);
        let cloned_rc = rc.clone();
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(Rc::strong_count(&cloned_rc), 2);
        core::mem::drop(rc);
        assert_eq!(Rc::strong_count(&cloned_rc), 1);
    }
}
```

# Roadmap
Most of the roadmap follows the great [blog](https://os.phil-opp.com/) by [Philipp Oppermann](https://github.com/phil-opp). However, I sort of combined the first and second editions of the blog, since I couldn't get some things to work, or just wanted to build it from scratch.

  - [x] Basic kernel and booting
  - [x] Setup IDT and catch exceptions
  - [x] Setup logger (using the VGA buffer)
  - [x] Setup tests (using [linkme crate](https://crates.io/crates/linkme))
  - [x] Setup IDT and catch exceptions
  - [x] Handle hardware interrupts (using the [pic8259](https://crates.io/crates/pic8259) crate)
  - [x] Make a frame allocator to allocate physical frames
  - [x] Setup 4-level recursive paging
  - [x] Setup a heap allocator
  - [x] Create a linked list allocator for heap allocations
  - [x] Setup a VESA framebuffer
  - [x] Implement double buffering
  - [ ] Implement an asynchronous task executor
  - [ ] Create a process scheduler?
  - [ ] Implement some filesystem (FAT32 maybe?)
