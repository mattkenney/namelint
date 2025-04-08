---
title: Common Case Styles
h1: Common Uppercase/lowercase Naming Conventions

cases:
- name: Snake
  example: this_is_a_test
  description: All lower case with underscores
- name: Constant
  example: THIS_IS_A_TEST
  description: All uppercase with underscores
- name: Kebab
  example: this-is-a-test
  description: All lower case with hyphens
- name: Pascal
  example: ThisIsATest
  description: First letter of every word capitalized, spaces removed
- name: Camel
  example: thisIsATest
  description: First letter of words 2-end capitalized, spaces removed
- name: Title
  example: This Is A Test
  description: First letter of every word capitalized, spaces kept
- name: Sentence
  example: This is a test
  description: First letter of first word capitalized, spaces kept
- name: Upper
  example: THIS IS A TEST
  description: Every letter uppercase
- name: Lower
  example: this is a test
  description: Every letter lowercase

---

<table class="mt-2 table table-bordered table-striped">
	<thead>
		<tr>
			<th>Name</th>
			<th>Example</th>
			<th>Detail</th>
		</tr>
	</thead>
	<tbody>
	{% for case in page.cases %}
			<tr>
				<td>{{case.name}}</td>
				<td><code>{{case.example}}</code></td>
				<td>{{case.description | markdownify | replace: "<p>", "" | replace: "</p>", "" }}</td>
			</tr>
	{% endfor %}
	</tbody>
</table>



* [zobweyt/textcase](https://github.com/zobweyt/textcase) - Python
* [PascalCase regex discussion](https://stackoverflow.com/a/31388507) on StackOverflow
* [Camel case definition](https://google.github.io/styleguide/javaguide.html#s5.3-camel-case) from Google's Java style guide.

<pre>
{{page | jsonify}}
</pre>
