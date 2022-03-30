# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0136`

### `ast`

```javascript
JSRoot {
	body: [
		JSTryStatement {
			block: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:4-1:6
			}
			handler: JSCatchClause {
				body: JSBlockStatement {
					body: [
						JSExpressionStatement {
							expression: JSReferenceIdentifier {
								name: "x"
								loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:15-1:16 (x)
							}
							loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:15-1:16
						}
						JSExpressionStatement {
							expression: JSReferenceIdentifier {
								name: "INVALID_PLACEHOLDER"
								loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:16-1:17
							}
							loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:16-1:17
						}
						JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:18-1:20
						}
					]
					directives: []
					loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:15-1:20
				}
				param: JSBindingIdentifier {
					name: ""
					loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:14-1:15 ()
				}
				loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:7-1:20
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:0-1:20
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Expected an identifier"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0136/input.js>
				end: Position 1:14
				start: Position 1:14
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0136/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0136/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0136/input.js:1:14 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected an identifier

    try {} catch (-x) {}
                  ^


```