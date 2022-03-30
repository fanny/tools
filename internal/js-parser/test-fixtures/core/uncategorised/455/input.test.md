# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 455`

### `ast`

```javascript
JSRoot {
	body: [
		JSBreakStatement {
			loc: SourceLocation core/uncategorised/455/input.js 1:0-1:5
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "No loop label found"}
			}
			location: {
				language: "js"
				path: UIDPath<core/uncategorised/455/input.js>
				end: Position 1:0
				start: Position 1:0
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/455/input.js>
	loc: SourceLocation core/uncategorised/455/input.js 1:0-1:5
}
```

### `diagnostics`

```

 core/uncategorised/455/input.js:1 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ No loop label found

    break
    ^


```