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
- [ ] `check_schema.sh` - makes sure all rules obey the schema.
- [ ] `docgen.sh` - generates the documentation.
- [ ] `test.sh` - runs all tests for all rules.

## Rules to make

- [ ] whitespace: none/spaces/oneline/any/[list]/...
- [ ] leading/trailing whitespace: trim-both/trim-start/trim-end
- [ ] punctuation: singledot,dashunderscore,most
- [ ] wildcards
- [ ] utf8
- [ ] windows protected file
- [ ] dotfiles
- [ ] posix safe chars
- [ ] url safe/url component safe/etc
- [ ] case: upper/lower/camel/snake/kebab/pascal/...
- [ ] extensions: all/web/mime/common/uppercase/lowercase/[list]/...
