# backstage-template-creator

This is a lightweight cli tool to create templates for backstage.io from existing projects. Tested only on linux, not tested throughoutly. Use at own risk.

## Requirements

Rust, cargo.

## Run

```
cat mappingsfile | cargo run -- -i input -o output
```

## Build

```
cargo build --release
```

## Usage

```sh
cat file-with-mappings | backstage-template-creator -i ~/projects/existing-project -o ~/projects/project-template
```

The command is silent per default. The exit code will tell you if it worked. If you need more infomation, set the `RUST_LOG` env variable to info or trace.

### Mappings

```
AWESOME_PROJECT=>${{values.package | capitalize}}
awesome_project=>${{values.package}}
project_name=>${{values.project}}
```

Maps the existing fields into the specified templates variables. The first element is replaced with the second if found in the name of the folder/file or in the content of an UTF-8 encoded file.

An example can be found in [demo/example](demo/example)