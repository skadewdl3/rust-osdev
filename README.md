# Rust OS Dev
 This repository documents my exploration in operating system development using (primarily) Rust.

# Build Environment Setup
1. Setup [Docker](https://www.docker.com/) on your system.
2. Clone this repository.
3. Build the build environment image using `docker build buildenv -t myos-buildenv` (this might take a while).
4. This should install all required tools (gcc cross compiler, xorisso, grub tools, make, nasm, etc.)
5. Now create a container from this image using
    - Windows: `docker run --rm -it -v %cd%:/root/env myos-buildenv`
    - Linux/MacOS: `docker run --rm -it -v $(pwd):/root/env myos-buildenv`
6. This should be enough to compile the kernel and build the ISO file.
7. Install Qemu to test out the operating system.
    - Windows: Install [chocolatey](https://chocolatey.org/) and run `choco install qemu`.
    - Linux: Use your distributions package manager or follow the [install instructions](https://www.qemu.org/download/).
    - MacOS: Install [homebrew](https://brew.sh/) and run `brew install qemu`.

# Compiling and Running the OS
1. Run the container you created from the Docker image (if you haven't already). Run the following commands in the container's shell:
    - Compile the kernel: `make`.
    - Create the ISO image: `make iso`.
2. To test the OS, run `qemu-system-x86_64 -cdrom build/os-x86_64` on your system shell.
    - If you'd like to not type the above monstrocity out, install make on your system.
    - Then you can run `make run` form your system shell.
