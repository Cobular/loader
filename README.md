# MD Include

A simple tool to insert other files into markdown (or any) files.

Simply place the string `{{ embed_file(path="<FILE_NAME>") }}` into your file, then call `md-include` on it:

```bash
$ ls
file.md

$ cat file.md


$ md-include .
<file-contents>

<included file from FILE_NAME>

<other-file-contents>
```

## Installing

`cargo install md-include`

## Usage

```bash 
A tool to include any file in markdown files

Usage: md-include [OPTIONS] <ROOT_DIR> [CUSTOM_PANDOC_FLAGS]...

Arguments:
  <ROOT_DIR>                The root directory to recursively search for markdown files
  [CUSTOM_PANDOC_FLAGS]...  Custom flags to pass to pandoc

Options:
  -o, --output-pdf  Output to PDF instead of stdout
  -h, --help        Print help
  -V, --version     Print version
```