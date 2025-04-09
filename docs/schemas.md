---
title: JSON Schemas
---

These are the JSON schemas for Rules libraries and namelint configurations.

<div class="alert alert-info">
	Note: The <code>.yaml</code> and <code>.json</code> versions are identical: we generate the <code>.json</code> from the <code>.yaml</code> at build time.
</div>

{% for theFile in site.static_files -%}
{% if theFile.path contains '-schema.' -%}
* [{{theFile.name}}]({{theFile.name}})
{% endif -%}
{% endfor %}
