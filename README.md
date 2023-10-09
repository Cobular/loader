# MD Include

A simple tool to insert other files into markdown (or any) files.

Simply place the string `{{ embed_file(path="<FILE_NAME>") }}` into your file, then call `md-include` on it:

```bash
$ ls
file.md

$ cat file.md
some stuff
more stuff

{{ embed_file(path="<FILE_NAME>") }}

last_stuff

$ md-include file.md
some stuff
more stuff

CONTENTS FROM THAT FILE

last_stuff
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

If you pass a glob instead of a single file, this tool will parse and embed all `embed_file`s, then concat them all seperated by `\n\n * * * \n\n` (a Markdown <hr>)

The sort order is based on depth of paths, with ties broken by alphabetical order.

You can also use `-o` and this will call pandoc to convert markdown to pdf.
