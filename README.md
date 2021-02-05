# truck-tutorial-code

This is a sample code for [truck-tutorial](https://ricos.gitlab.io/truck-tutorial/) or [truck-tutorial-ja](https://ricos.gitlab.io/truck-tutorial-ja/).

## Naming rule of executable code

Each excutable code corresponds each section. For example, one can write code equivalent to `section2_1.rs` by reading Section 2.1. `section2_1.rs` is excuted by the following command:

```bash
cargo run --bin section2_1
```

## system requirements

The prerequisites for using truck are as follows:

- Rust development environment must be in place.
- Must run Vulkan, Metal, or DirectX12, the backend of wgpu.

Currently, `cmake` is required to test. Honever, it is not necessary as long as you are using it as a pure package. The details for each OS are described below.

### Windows

#### Procedure for Windows

1. Make sure your Windows 10 is up to date.
2. Install Visual Studio C++ developer tools. If already installed, make sure it is up to date.
3. Install Rust development environment by [installer](https://www.rust-lang.org/tools/install). Choose MSVC for the backend. If already installed, update the environment by `rustup update`.

#### Remarks for Windows

- The Rust environment is assumed to have MSVC as its backend. We have not tested it with other backends.
- If your Windows is up to date, Vulkan and DirectX12 will work. Don't worry!
- We have already confirmed that codes does not work on WSL because Vulkan does not work on WSL.
- I think we cannot build on Windows 8, but I have not checked yet.

### MacOS

#### Procedure for MacOS

1. Make sure your MacOS is up to date.
2. Install Rust development environment by `curl https://sh.rustup.rs -sSf | sh`. If already installed, update the environment by `rustup update`.

#### Remarks for MacOS

- Metal runs as standard on MacOS.

### Linux

This is not supported on Linux, as it has not been tested in a native environment. [This site](https://vulkan.lunarg.com/doc/sdk/1.2.162.1/linux/getting_started_ubuntu.html) may be a good reference for downloading Vulkan, but running this procedure on [the official Rust Docker container](https://hub.docker.com/_/rust) did not work. The CI for truck is done by building a Rust environment in a ready-made Docker container [nvidia/vulkan](https://hub.docker.com/r/nvidia/vulkan).
