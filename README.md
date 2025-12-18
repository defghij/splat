# Introduction

SPLAT is a backronym for Simple Parallel Launch And Track. The aim of `splat` is to be a simple utility to easily and intuitively spawn a large amount of processes in a controlled manner. The primary motivation of splat is filling nodes cores or a clusters nodes with jobs in some simply fashion. This is a pattern that has occurred numerous times during my work in HPC and figured it is about time to write some thing a bit more robust than bash.


# Usage

Splat is feed a `*.toml` file which determines the shape of the _Session_. As such, all of the "configuration" complexity is captured in the `*.toml` file. Invoking `splat` is as simple as:

```
╭─⦗0⦘─⦗defghij@numenor:~/splat)─⦗main⦘
╰─➤ splat -i session.toml
```

# Goals

The goals of this project are:
- Simplicity and repeatability of defining sets of processes
- Parallelism:
  - Provide random spawning of jobs from a set
  - Interleave jobs with a particular chunk value
  - Just launch job sequentially
  - Maximum Total Steps
  - Minimum active Steps
- Tracking:
  - [ ] Logging of current Session Status
  - [ ] Simple text based output
  - [ ] Advanced TUI
