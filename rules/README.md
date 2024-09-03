# Rules for file names

Each `.yaml` file has one or more rules.

A rule consists of:
- handle: kebab-case unique id
- name: Human-readable name
- description: longer markdown description/discussion including sources/background/references/etc.
- regex: the rule
- LATER rules: list of other rules to apply in addition to the regex.  Allows various combinations with `+` (and), '|' (or) `!` (not) and `()` (precedence)
- tests: a list of tests for the rule

A test contists of:
- target: string to test
- result: if the rule should pass/fail (boolean)
- notes: optional notes about the test

## Utilities

- [ ] `codegen.sh` - generates the `.rs` file with all the rules.
- [ ] `docgen.sh` - generates the documentation.

## Rules to make

- [x] utf8
- [x] leading/trailing whitespace: trim-start/trim-end
- [x] windows reserved names
- [ ] no chars that need shell escapes
- [ ] internal whitespace: none
- [ ] internal whitespace: only (single?) spaces
- [ ] dotfiles
- [ ] posix safe chars
- [ ] url safe/url component safe/etc
- [ ] punctuation: singledot,dashunderscore,most
- [ ] case: upper/lower/camel/snake/kebab/pascal/...
- [ ] extension case: upper/lower
- [ ] extensions: all/web/mime/common/[list]/...
- [ ] tests (and `--test-rules` to run the tests)
