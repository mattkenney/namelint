---
title: JSON Schemas
---

These are the JSON schemas for Rules libraries and namelint configurations.

The `.yaml` and `.json` versions are identical: we generate the `.json` from the `.yaml`

{% for theFile in site.static_files -%}
{% if theFile.path contains '-schema.' -%}
* [{{theFile.name}}](theFile.path)
{% endif -%}
{% endfor %}
