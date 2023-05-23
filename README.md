# Pastes CLI

A cli tool to upload files to pastes.dev or bytebin.lucko.me

The program will attempt to determine the file type based on the
extension, and thus, whether it should be uploaded to pastebin or 

## Usage

`pastes` can be run as a single command

```sh
$ pastes my_text_file.txt # Upload a text file to pastes.dev
$ pastes my_image.png # Upload an image to bytebin.lucko.me

$ pastes my_unknown_file # Upload to bytebin
$ pastes my_unknown_file --pastes # Upload to pastebin (read the file as plaintext)
$ pastes my_unknown_file --bytebin # Upload to bytebin

$ pastes ... -t application/json # Force application/json mime type
```

Or, it can be run in a pipeline

```sh
$ echo "hello" | pastes --json | jq '.url'
```

```sh
$ cat my_file.txt | pastes -t text/plain
# Is equivalent to
$ pastes my_file.txt -t text/plain
```

For full usage see `pastes --help`

## Platforms

While this project has been built with Linux in mind, it _should_
theoretically work on other platforms.

## Install

```sh
cargo install pastes
```

## Contributing

Please feel free to open an issue or pull request if you find a bug or
have a feature idea.  I'm always open to help on all projects!
