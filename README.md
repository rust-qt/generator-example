# Clipper generator

This is an example of using [ritual](https://github.com/rust-qt/ritual) to generate Rust bindings for a C++ library. The [clipper](http://www.angusj.com/delphi/clipper.php) library is used as an example.

Note: this project doesn't attempt to create a fully featured bindings for `clipper`. This is only a basic example. 

## Setting up the environment

### Using docker (recommended)

To make sure the parsing results are consistent and reproducible, it's recommended to use a reproducible environment, such as provided by `docker`. This will also help your future contributors.

Ritual provides a [Dockerfile](https://github.com/rust-qt/ritual/blob/master/docker.builder.dockerfile) containing its dependencies. This image is published at [riateche/ritual_builder](https://hub.docker.com/r/riateche/ritual_builder). If the published version isn't suitable, you can build the image locally:
```
cd ritual
docker build . -f docker.builder.dockerfile -t riateche/ritual_builder
``` 

Use this image as a base and create your own image that installs and sets up the C++ library you want to work with. See [Dockerfile](Dockerfile) for an example.

Note that the image contains only the environment. No pre-built ritual artifacts are included. This allows you to edit the source code of your generator and re-run it without the slow process of rebuilding the docker image. You can use `cargo` to run the generator, just like you would normally do it on the host system.

When running the container, mount `/build` to a persistent directory on the host system. This directory will contain all temporary build files, so making it persistent will allow you to recreate the container without having to recompile everything from scratch.  

In addition to the build directory, you should also mount one or more directories containing the source code of your generator and the ritual workspace directory (see below) to make it available in the container. The paths to these directories can be arbitrary.

Here is an example of running a container:
```
cd ~/clipper/repo
# build your image
docker build . -t clipper_generator
# remove previously created container
docker ps -a -q --filter "name=clipper_generator" | grep -q . && \
    docker rm clipper_generator
# create an container and run a command in it
docker run \
    --mount type=bind,source=~/clipper/repo,destination=/repo \
    --mount type=bind,source=~/clipper/workspace,destination=/workspace \
    --mount type=bind,source=~/clipper/tmp,destination=/build \
    --name clipper_generator \
    --hostname clipper_generator
    -it \
    clipper_generator \
    your_command
```
Replace `your_command` with a cargo command you want to run or just specify `bash` to open a shell inside the container. 

### Without docker

In case you don't want or can't use `docker`, you can just install all required dependencies on your host system and run your generator natively with `cargo`, like any Rust project.

The dependencies are  `cmake` and `libclang`. Sometimes you may need to set some environment variables to make `libclang` work correctly, for example:
```
export LIBCLANG_PATH=/usr/lib/llvm-9.0/lib
export CLANG_SYSTEM_INCLUDE_PATH=/usr/lib/clang/9.0.0/include
```  
You can clone [ritual](https://github.com/rust-qt/ritual) and run `cargo test` to make sure it's all working.

You should also install the C++ library you want to work with. If the library is not available in the system directories, you should set up environment variables to make it available. In addition to variables recognized by standard tools (`LIB`, `PATH`, `LIBRARY_PATH`, `LD_LIBRARY_PATH`, `DYLD_FRAMEWORK_PATH`), there are also variables recognized by ritual (`RITUAL_LIBRARY_PATH`, `RITUAL_FRAMEWORK_PATH`, `RITUAL_INCLUDE_PATH`) that you may also need to set.

## Creating the generator

Create a new binary Rust crate and add `ritual` as a dependency. In the `main` function, you need to construct a `ritual::config::GlobalConfig` and pass it to `ritual::cli::run_from_args`. `GlobalConfig` allows you to process multiple libraries (or parts of a library as separate crates) in a single run, but in the [simple case](src/main.rs) it just contains a function that creates a `Config` for your library.

`Config` is the main way of configuring ritual. You can use it to set up every property the generator supports. It also allows you to set up hooks that run in particular points of the processing and allow you to filter or alter the results programmatically. See the documentation of `Config` and [this example](src/main.rs) for more information.

## Running the generator
 
Ritual needs a workspace to work. Workspace is a directory that contains ritual's databases, generated crates, logs, and temporary files. You should use the same workspace for consecutive runs of the same generator.  

You can run the generator like this:
```
cargo run -- ~/clipper/workspace -c clipper -o main
```
The `-o` flag specifies the operations to run. Ritual's processing consists of [multiple steps](https://github.com/rust-qt/ritual/blob/master/ritual/src/processor.rs). `main` is an alias for the main sequence of operations that produce a complete crate from scratch. You can also specify a specific operation (e.g. `rust_generator`) or resume the main sequence from a certain point with `'[rust_generator..)'`.

If everything is successful, the generated crate will appear in the `out` subdirectory of the workspace.

## Adding tests and custom Rust code

In addition to `Config`, you can also specify a crate template to be used. Crate template is a directory normally placed in the repository of your generator. You can supply a crate template with `Config::set_crate_template_path`. It may contain tests, examples, extra source code, and any other files. These files are merged into the files generated by ritual. It's recommended to have some tests in your crate template. At the end of the main sequence, ritual will run these tests. 
