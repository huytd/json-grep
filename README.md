# JSON Grep

**json-grep** is an utility that help you search for anything inside a JSON data.

All searches are case sensitive for now.

Usage:

```
json-grep <filename> <keyword>
```

For example:

```
$ json-grep testdata/simple.json author

Searching testdata/simple.json for 'author':

► .author = [...]
► .commit.author = [...]
► .commit.message = "...nCo-authored-by: dependabot[bot..."
► .commit.verification.payload = "...b9b86369d1c1f16\nauthor dependabot[bot]..."
```