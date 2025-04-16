---
title: Namelint
h1: Namelint

links:
- text: Hacking The Web With Unicode (Vickie Li)
  url: https://vickieli.dev/hacking/hack-unicode/
  notes: Using Unicode to confuse, spoof and make backdoors (2020)

- text: '"Jia Tan"ing Go code (Martin Tournoij)'
  url: https://www.arp242.net/jia-tan-go.html
  notes: smuggling `_test.go` files into a regular build (2024)

- text: "Git is case-sensitive and your filesystem may not be (Scott Hanselman)"
  url: https://www.hanselman.com/blog/git-is-casesensitive-and-your-filesystem-may-not-be-weird-folder-merging-on-windows
  notes: Weird `git` folder merging on Windows (2019)

- text: "I want off Mr. Golang's Wild Ride"
  url: https://fasterthanli.me/articles/i-want-off-mr-golangs-wild-ride
  notes: How Go's file handling makes things "simpler" with a host of side effects (2020, updated 2022)

- text: Our solution for the hell that is filename encoding (beets blog)
  url: https://beets.io/blog/paths.html
  notes: How media library software deals with portable filenames in Python (and the [HN discussion](https://news.ycombinator.com/item?id=16991263)) (2016)

- text: APFS’s “Bag of Bytes” Filenames (Michael Tsai)
  url: https://mjtsai.com/blog/2017/03/24/apfss-bag-of-bytes-filenames/
  notes: Troubles when Apple switched away from a file system with normalized file names (2017)

---
Check file and directory names for security, compatibility, best practices & standards.

## Why?
{% for link in page.links %}
- [{{ link.text }}]({{ link.url }}){% if link.notes %} - {{ link.notes}}{% endif %}
{% endfor %}

