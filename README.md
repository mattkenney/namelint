# namelint [<img alt="Logo for namelint" src="docs/favicon.svg" height="96" align="right"/>](https://github.com/FileFormatInfo/namelint)

> [!NOTE]  
> This is "README-driven development".  There is no code yet, just an overview of what I think it should do.

Checks for invalid/deceptive/hacked file names.

## Background

## Installing

## Running

## Building

## License

Copyright &copy; 2024 by Andrew Marcuse under [CC BY-SA 4.0](LICENSE.txt)

Attribution requirement: a follow-able link to this repo on Github

## Credits

[![Git](https://www.vectorlogo.zone/logos/git-scm/git-scm-ar21.svg)](https://git-scm.com/ "Version control")
[![Github](https://www.vectorlogo.zone/logos/github/github-ar21.svg)](https://github.com/ "Code hosting")
[![Rust](https://www.vectorlogo.zone/logos/rust-lang/rust-lang-ar21.svg)](https://www.rust-lang.org/?utm_source=vectorlogozone&utm_medium=referrer "Programming language")

* [Just Another Hand](https://fonts.google.com/specimen/Just+Another+Hand) font from [Astigmatic](http://www.astigmatic.com/)

## Links

- https://dwheeler.com/essays/filenames-in-shell.html
- https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file
- https://learn.microsoft.com/en-us/windows/win32/fileio/filesystem-functionality-comparison
- https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_282
- https://fasterthanli.me/articles/i-want-off-mr-golangs-wild-ride

## Prior Art

- [ls-lint](https://ls-lint.org/) - Go, focused on file name conventions ([source](https://github.com/loeffel-io/ls-lint))
- [eslint-plugin-filenames](https://www.npmjs.com/package/eslint-plugin-filenames) - JavaScript, focused on file naming conventions, no longer maintained ([source](https://github.com/selaux/eslint-plugin-filenames))
- [batista/lint-filenames](https://github.com/batista/lint-filenames) - Go, regex rules
- https://richjenks.com/filename-regex/
- https://github.com/sindresorhus/filename-reserved-regex

## To Do

Also see the to-do list in [rules/README.md]

- [ ] check flags/attributes
- [ ] rules per extension
- [ ] duplicates with only lower/upper case differences
- [ ] action: delete/sanitize/report/etc.
- [ ] dryrun mode for action
- [ ] specify params as regex/bytes/utf8 codepoint numbers/ascii/utf8
- [ ] overrides: allow/deny list
- [ ] CI job
- [ ] output report format: text/json/???, 
- [ ] output encoding: how to show bad chars in report
- [ ] support for a `.namelint` file


