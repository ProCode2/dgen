# Dgen

Generate and Maintain a collection of json blueprints that can generate your entire starter repos or any directory of that sort. One JSON file per repository.

```
Dgen 1.0
ProCode
Create your starter repositories from a single json blueprint.

USAGE:
    dgen-rs [FLAGS] [OPTIONS]

FLAGS:
    -b, --blueprint    Create json blueprint of the directory you are in.
    -h, --help         Prints help information
    -r, --repository   Create json blueprint of a GitHub repository.
    -V, --version      Prints version information

OPTIONS:
    -g, --generate <path to JSON blueprint>    Create the directory from the json blueprint.
```

# Install

For now, if you have rust installed you can play around with the project.

```bash
# clone the repo
$ git clone https://github.com/ProCode2/dgen.git

# get in the project diretory
$ cd dgen

# build the binary
$ cargo build --release

# check if its working
$ ./target/release/dgen-rs -V
```

The binary file should be created in `dgen/target/release` with the name `dgen-rs`.

# Generate a blueprint

```bash
# to create a JSON blueprint, go inside the directory you want to create a blueprint of and run
$ ./path/to/dgen-rs -b
```

if you have a repository in github and would like to generate a blueprint for that: While I will build this feature in the core library, here's a weird trick to do that for now:

```bash
$ git clone https://github.com/username/repo_name.git && cd repo_name && ~/path/to/dgen-rs -b && cd ../ && mv ./repo_name/repo_name.json . && rm -rf repo_name
```

[**Note**: Make sure to replace `username`, `repo_name` with valid values.]

# Generate a repository from a blueprint

```bash
# to generate the repository from the JSON blueprint
$ ./path/to/dgen-rs -g /path/to/json
```

# Creates a blueprint form a repository

```bash
# to generate a JSON blueprint from the repository
$ ./path/to/dgen-rs -r <Username or Organization>/<Repository Name>
# example: ./path/to/dgen-rs -r rodgeraraujo/Pokedex
```

# Known Issues

- Currently it only stores files with valid UTF-8 content. Which basically means it does not store image, audio, video content and binaries. So far, I am thinking of storing images as base64 strings because some starter repos might have images.
