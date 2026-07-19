# G.ART

![Version](https://img.shields.io/badge/version-0.1.0-green)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

## **G**it **A**dvanced **R**ebase **T**ool

A small CLI tool that provides some shortcuts to simplify interactive rebase.
The tool allows to describe rebase commands in commit message itself using special syntax.
Those commands will be automatically transformed into git rebase commands during rebase.

## Example
Example commit message that can be processed by `gart`:
```
\gart mv 1 & sq
```

Breakdown:
| Command | Description |
|---------|-------------|
| `\gart` | Commit message prefix that tells `gart` tool to process this line. |
| `mv 1`  | A command to move current commit up by 1 position in git commit history. |
| `&`     | Logical `and` operator to combine commands into a sequence. |
| `sq`    | A command to squash the current commit with previous one. |

## Build & Install
Building requires `cargo` to be available on your machine.
Build and install the tool set by running the script:
```bash
./script/install.sh
```

On success the following executables will be built and installed:
```
~/.local/bin/gart
~/.local/bin/gart_editor
~/.local/bin/gart_seq_editor
```

Might be convenient to add `~/.local/bin` to `PATH` to simplify usage.
Note that `gart_editor` and `gart_seq_editor` are also available in `~/.cargo/bin`.
`gart` is only installed into `~/.local/bin`.

## Targets

### 1. gart_editor

The executable is used by `gart` script as `GIT_EDITOR` during interactive rebase to edit commit messages. Is not meant to be used directly.

### 2. gart_seq_editor

The executable is used by `gart` script as `GIT_SEQUENCE_EDITOR` during interactive rebase to edit sequence files. Is not meant to be used directly.

### 3. gart

The main script that runs interactive rebase. The script uses `gart_editor` and `gart_seq_editor` as git editors to process commit messages and sequence files during interactive rebase.

```bash
$ gart --help
Usage: gart [OPTIONS] <COMMIT_HASH>

Options:
  -h, --help      Show help message
  -V, --version   Show version

Arguments:
  COMMIT_HASH     Commit hash to rebase to
```

## Syntax

General syntax for commit message to be processed by `gart`:

```
\gart <COMMAND> [& <COMMAND>]*
```

Supported list of commands:

| Command        | Description |
|----------------|-------------|
| `mv <POS:INT>` | Move commit by <POS> position in git commit history. Positive <POS> moves the current commit up (towards the root). Negative <POS> moves the commit down (towards the `HEAD`). Target position must be a valid position within rebase range.
| `sq`           | Squash commit with the previous one. The message with `\gart` command will be cleared. Equivalent to `fixup` git rebase command |

## Environment Variables

The following environment variables are available to customize `gart` behavior:

| Variable          | Default Value     | Description |
|-------------------|-------------------|-------------|
| `GART_EDITOR`     | `gart_editor`     | Executable to be used as `GIT_EDITOR`. If using default value ensure that the path to it is provided by `PATH`. Default installation dir for `gart_editor` is `~/.local/bin` |
| `GART_SEQ_EDITOR` | `gart_seq_editor` | Executable to be used as `GIT_SEQUENCE_EDITOR`. If using default value ensure that the path to it is provided by `PATH`. Default installation dir for `gart_seq_editor` is `~/.local/bin` |

## TODO

Some ideas for further improvement (TBD):

* Run `gart` util in default mode (without providing commit hash). Process fixed number of last commits.
* Split commit utils.
* Define command prefix based on env variable.
* Implement drop commits by hash.
