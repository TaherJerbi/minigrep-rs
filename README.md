# minigrep-rs 🦀

minigrep-rs is a command line application written in Rust that allows users to search for a specific pattern within a file or set of files. This application was created based on the Chapter 12 tutorial in the Rust Book.

## Usage

To run minigrep-rs, navigate to the project directory in your terminal and enter the following command:

```shell
cargo run -- search_term file_name [OPTIONS]
```

**search_term**: The term you want to search for

**file_name**: The name of the file you want to search in.

To search for a term in multiple files, you can use shell globbing to specify a pattern of files to search in:

```
cargo run -- search_term file_pattern [OPTIONS]
```

**search_term**: The term you want to search for

**file_pattern**: A pattern of files to search in

⚠️ Note: shell globbing is not supported on Windows.

## Options

- --no-case: Case insensitive search

## Environment Variable

You can also set the IGNORE_CASE environment variable to `anything` to achieve the same result as passing the --no-case option.

## Examples

To search for the term "hello" in the file "example.txt":

```
cargo run -- hello example.txt --no-case
```

To search for the term "world" in all .txt files in the current directory:

```
IGNORE_CASE=1 cargo run -- world *.txt
```
