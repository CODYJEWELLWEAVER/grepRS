# GrepRS

[![Rust](https://github.com/CODYJEWELLWEAVER/grepRS/actions/workflows/rust.yml/badge.svg)](https://github.com/CODYJEWELLWEAVER/grepRS/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/CODYJEWELLWEAVER/grepRS/graph/badge.svg?token=2ZH77PI5IH)](https://codecov.io/github/CODYJEWELLWEAVER/grepRS)
[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://opensource.org/licenses/)

## DESCRIPTION

An implementation of _grep_ in Rust. This project was inspired by the mini project in "The Rust Book".
I decided I wanted to expand on the idea and try to fully write _grep_ using Rust as a way to
increase my knowledge of both. All credit for [_grep_](https://www.gnu.org/software/grep/manual/grep.html)
goes to Ken Thompson & AT&T Bell Laboratories. This project is licensed under GPLv3 which you can find
[here](https://www.gnu.org/licenses/gpl-3.0.en.html#license-text).

I have made it a point of emphasis to keep the usage of grepRS identical to the usage of _grep_ so far.
My hope is to allow for grepRS to be used without having to learn anything new.

As of right now only unix is fully supported and this project may not be fully functional on macOS or Windows. However,
I fully plan to support both in the future.

## BASIC USAGE

### **Basic**

```text
greprs [options...] pattern [sources...]
```

A source is a file or stream such as stdin.

There are no restrictions on where options must be given in the command.

```text
greprs --ignore-case pattern source # valid
greprs pattern source --ignore-case # also valid
```

If only one non-option argument is given it is interpreted as a pattern
and stdin will be used as the source for content to search in.

## CONTACT

Found a bug? Hit me up here:

* Email: <cody.weaver@colorado.edu>
* GitHub: [CODYJEWELLWEAVER](https://github.com/CODYJEWELLWEAVER)
