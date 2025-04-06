---
title: Handling Homoglyphs
---

Homeglyphs are different Unicode characters that look the same (or even identical).  They can be a source of security holes and should generally not be allowed.


- [codebox/homoglyph] - JavaScript
- `check_similar` in [notox](https://github.com/Its-Just-Nans/notox/blob/main/src/lib.rs) - Rust
- [Wikipedia](https://en.wikipedia.org/wiki/Homoglyph)
- Unicode Consortium's [Tech Report #36](https://www.unicode.org/reports/tr36/) - focused on domain names, but generally applicable.
- [shadawck/homoglyph](https://github.com/shadawck/homoglyph/blob/main/homoglyph-core/src/confusable.rs) - Rust library designed to generate homoglyph string variations, but has a useful list.
- [confusable_homoglyphs](https://pypi.org/project/confusable-homoglyphs/) - Python
