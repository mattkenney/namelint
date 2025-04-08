---
title: "Mandatory Rules: UTF-8 and no nulls"
---

There are two rules which are mandatory and cannot be disabled.

1. Must be UTF-8
2. Must not have any nul (`0`) bytes

If it is encoding in something other than UTF-8, we don't really want to try converting encodings.  It should be switched to UTF-8.

If it isn't valid UTF-8 on a system that uses UTF-8, there then something is really wrong.

If it has a null byte, again, something is really wrong.
