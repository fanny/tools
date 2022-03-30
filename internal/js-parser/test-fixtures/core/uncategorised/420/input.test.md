# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 420`

### `ast`

```javascript
JSRoot {
	body: [
		JSIfStatement {
			alternate: JSExpressionStatement {
				expression: JSReferenceIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation core/uncategorised/420/input.js 1:24-1:24
				}
				loc: SourceLocation core/uncategorised/420/input.js 1:24-1:24
			}
			consequent: JSExpressionStatement {
				expression: JSCallExpression {
					arguments: []
					callee: JSReferenceIdentifier {
						name: "doThis"
						loc: SourceLocation core/uncategorised/420/input.js 1:10-1:16 (doThis)
					}
					loc: SourceLocation core/uncategorised/420/input.js 1:10-1:18
				}
				loc: SourceLocation core/uncategorised/420/input.js 1:10-1:19
			}
			test: JSBooleanLiteral {
				value: false
				loc: SourceLocation core/uncategorised/420/input.js 1:3-1:8
			}
			loc: SourceLocation core/uncategorised/420/input.js 1:0-1:24
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: [RAW_MARKUP {value: "Unknown start to an "}, "statement expression"]}
			location: {
				language: "js"
				path: UIDPath<core/uncategorised/420/input.js>
				end: Position 1:24
				start: Position 1:24
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/420/input.js>
	loc: SourceLocation core/uncategorised/420/input.js 1:0-1:24
}
```

### `diagnostics`

```

 core/uncategorised/420/input.js:1:24 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unknown start to an statement expression

    if(false) doThis(); else
                            ^


```