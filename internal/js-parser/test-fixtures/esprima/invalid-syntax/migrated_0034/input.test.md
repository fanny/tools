# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0034`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "x*"
				loc: SourceLocation esprima/invalid-syntax/migrated_0034/input.js 1:0-1:7 (x*)
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0034/input.js 1:0-1:7
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
				message: RAW_MARKUP {value: "Invalid Unicode escape"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0034/input.js>
				end: Position 1:7
				start: Position 1:7
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0034/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0034/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0034/input.js:1:7 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Invalid Unicode escape

    x\u002a
           ^


```