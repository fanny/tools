# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 301`

### `ast`

```javascript
JSRoot {
	body: [
		JSBlockStatement {
			body: [
				JSExpressionStatement {
					expression: JSReferenceIdentifier {
						name: "x"
						loc: SourceLocation core/uncategorised/301/input.js 1:2-1:3 (x)
					}
					loc: SourceLocation core/uncategorised/301/input.js 1:2-1:3
				}
				JSExpressionStatement {
					expression: JSUpdateExpression {
						operator: "--"
						prefix: true
						argument: JSReferenceIdentifier {
							name: "y"
							loc: SourceLocation core/uncategorised/301/input.js 2:2-2:3 (y)
						}
						loc: SourceLocation core/uncategorised/301/input.js 2:0-2:3
					}
					loc: SourceLocation core/uncategorised/301/input.js 2:0-2:3
				}
			]
			directives: []
			loc: SourceLocation core/uncategorised/301/input.js 1:0-2:5
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/301/input.js>
	loc: SourceLocation core/uncategorised/301/input.js 1:0-2:5
}
```

### `diagnostics`

```

```