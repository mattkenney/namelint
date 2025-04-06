---
title: Namelint
h1: Namelint

links:
- text: Hacking The Web With Unicode (Vickie Li)
  url: https://vickieli.dev/hacking/hack-unicode/

- text: APFS’s “Bag of Bytes” Filenames (Michael Tsai)
  url: https://mjtsai.com/blog/2017/03/24/apfss-bag-of-bytes-filenames/
  notes: Troubles when Apple switched away from a file system with normalized file names (2017)

- text: '"Jia Tan"ing Go code (Martin Tournoij)'
  url: https://www.arp242.net/jia-tan-go.html
  notes: smuggling `_test.go` files into a regular build

---
Check file names for security, compatibility, best practices & standards.

## Why?
{% for link in page.links %}
- [{{ link.text }}]({{ link.url }}){% if link.notes %} - {{ link.notes}}{% endif %}
{% endfor %}

