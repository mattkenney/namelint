---
title: Rust filesan library
tags: ["prior-art", "rust"]
---

[filesan](https://github.com/BonnyAD9/filesan) by Jakub Antonín Štigler is a rust library that sanitizes strings so they can safely be used as file names.

This is useful, but doesn't overlap with namelint: it is for rust programs that are creating files, and only prevents file names that are absolutely forbidden on the platform where it is running.  It does not prevent security hacks or cross-platform issues.
