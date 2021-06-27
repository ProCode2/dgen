# Dgen

Generate and Maintain a collection of json blueprints that can generate your entire starter repos or any directory of that sort. One JSON file per repository.

```
Dgen 1.0
ProCode
Create your starter repositories from a single json blueprint.

USAGE:
    dgen [FLAGS]

FLAGS:
    -b, --blueprint    Create json blueprint of the directory you are in.
    -g, --generate     Create the directory from the json blueprint.
    -h, --help         Prints help information
    -V, --version      Prints version information
```

# Install

For now, if you have rust installed you can play around with the project.

```bash
# clone the repo
git clone https://github.com/ProCode2/dgen.git

# get in the project diretory
cd dgen

# build the binary
cargo build --release

# check if its working
./target/release/dgen-rs -V
```

The binary file should be created in `dgen/target/release` with the name `dgen-rs`.

# Generate a blueprint

```bash
# to create a JSON blueprint, go inside the directory you want to create a blueprint of
./path/to/dgen-rs -b
```

# Generate a repository from a blueprint

```bash
# to generate the repository from the JSON blueprint
./path/to/dgen-rs -g /path/to/json
```

# Known Issues

- Currently it only stores files with valid UTF-8 content. Which basically means it does not store image, audio, video content and binaries. So far, I am thinking of storing images as base64 strings because some starter repos might have images.
