# heapvue
A heap visualizer (mainly for embeded uses).

- [Overview](#overview)
- [Installation](#installation)
- [How to use it](#how-to-use-it)
- [Format](#format)
- [License](#license)

## Overview
A memory viewer that show allocations occuring in a process or on embeded targets. The viewer is
very rudimentary and is based on the terminal output (stdout) of a process. The application was
design this way to be *very* generic and to work on microcontroller where a debugger is not
available. *heapvue* is used to find *free after free* and/or heap corruption.

To use this application, the process that will be analyzed needs to be modified to print
informations on every allocation (`malloc`) and on every free (`free`). Hooks should therefore be
added to those functions. This may not be possible in every scenario and this application may not
be useful in every scenario.

## Installation
You can build the application with
```
cargo build --release
```
or run it with
```
cargo run --release
```

## How to use it
TODO

## Format
3 different events can be tracked by *heapvue*.
### Allocation (`malloc`)
On every allocation, the tracked process should print a line with the following format:
```
m:{ptr},{size},{identifier}
```
The `ptr` is a hex value representing the address that was returned by `malloc`. The size is a hex
value with the size of the allocation. Finally the identifier can be anything useful to know what
the allocation refers to. For example, the name of the function calling `malloc` could be used.

### Free (`free`)
On every free, the tracked process should print a line with the following format:
```
f:{ptr},{identifier}
```
The `ptr` is a hex value representing the address that was freed by `free`. The identifier is not
currently used.

### Corruption
When a heap corruption is detected by the tracked process itself, the following should be printed:
```
c:{ptr}
```
This will either paint an alreay existing chunk purple or create a new one. It will also stop the
*heapvue* from updating the drawing.

## License
MIT - Enjoy!
