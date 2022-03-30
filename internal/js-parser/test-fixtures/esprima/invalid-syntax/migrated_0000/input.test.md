# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0000`

### `ast`

```javascript
JSRoot {
	body: [
		JSBlockStatement {
			body: []
			directives: []
			loc: SourceLocation esprima/invalid-syntax/migrated_0000/input.js 1:0-1:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: [
					log {
						category: "info"
						text: [RAW_MARKUP {value: "We expected to find the closing character <emphasis>"}, "}", RAW_MARKUP {value: "</emphasis> here"}]
					}
					frame {
						location: SourceLocation esprima/invalid-syntax/migrated_0000/input.js 2:0-2:0
					}
				]
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unclosed <emphasis>"}, "block", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0000/input.js>
				end: Position 1:0
				start: Position 1:0
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0000/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0000/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0000/input.js:1 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unclosed block

    {
    ^

  ℹ We expected to find the closing character } here

    {


```