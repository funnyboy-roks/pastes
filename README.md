# Pastes CLI

A cli tool to upload files to pastes.dev

## Usage

```sh
$ pastes my_text_file.txt # Upload a text file to pastes.dev
$ pastes my_image.png # Upload an image to bytebin.lucko.me
$ pastes my_unknown_file # Upload to bytebin
$ pastes my_unknown_file --pastes # Upload to pastebin (read the file as plaintext)
$ pastes my_unknown_file --bytebin # Upload to bytebin
$ pastes ... --type Application/json # Force application/json mime type
$ pastes ... --zip # Zip with gzip
```
