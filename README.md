# DumpBuffer
DumpBuffer is a place where you can quickly dump some commands/thoughts in terminal

## About
DumpBuffer is a command line based application, whose sole purpose is to store records for future use.

DumpBuffer is fully written in Rust.

## Install
WIP

## Usage

### Add a record to the DumpBuffer

**Command**
```bash
$ dumpb add <key> <value>
```
> Some shell commands may contain operational symbols (e.g. ">", "!", etc.)
> which can influence correctness of program execution.
>
> Thus, it is better to wrap inserted values into single quotes:
>```bash
> $ dumpb add <key> '<value>'
> $ e.g
> $ dumpb add key123 'echo "hello, world" > hello_world.txt'
>```
**Example**
```bash
$ dumpb add some_fancy_key docker run --rm -it -p 8080:8080 best_app:latest

Successfully added new value with key "some_fancy_key"
<OR>
[ERROR]: Key "some_fancy_key" already exists
```

### Get record from DumpBuffer by key

**Command**
```bash
$ dumpb get <key>
```
**Example**
```bash
$ dumpb get some_fancy_key

docker run --rm -it -p 8080:8080 best_app:latest
<OR>
[ERROR]: Key "some_fancy_key" does not exist
```

### List records from DumpBuffer
You can add `--keys-only` to show only key part of records.

**Command**
```bash
$ dumpb list [--keys-only]
```
**Example**
```bash
$ dumpb list
[
{
  key: some_fancy_key,
  value: docker run --rm -it -p 8080:8080 best_app:latest
},
{
  key: key123,
  value: echo "hello, world" > hello_world.txt
}

$ dumpb list --keys-only
[
  key123,
  some_fancy_key
]
```

### Remove records from DumpBuffer
You can add `--all` to erase all records completely. Either `key` or `--all` has to be specified.

**Command**
```bash
$ dumpb rm <key> [--all]
```
**Example**
```bash
$ dumpb rm some_fancy_key
Removed record with key "some_fancy_key"
<OR>
[ERROR]: Key "some_fancy_key" does not exist

$ dumpb rm --all
All records were removed!
```