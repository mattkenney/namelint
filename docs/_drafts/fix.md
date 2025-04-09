---
title: Fixing file names
h1: Fixing file names that are found
---

I think this should be a separate program (`fnfix`, `namefix`, etc) but still part of this project.

Several levels of fixing:

- basic: just nulls and UTF-8
- portable: anything that causes cross-platform issues
- ascii: convert every character to ASCII
- posix: convert everything to POSIX-safe
- shellsafe?

Code:

- each rule should have an optional `fixlevel` and `replace`
- program takes a directory name and fixes all files in it
- optional `recursion` flag to go into subdirectories
- optional `dryrun` flag to show what it would do
