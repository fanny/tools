# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 311`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSFunctionExpression {
				body: JSBlockStatement {
					body: [
						JSReturnStatement {
							trailingComments: ["0"]
							loc: SourceLocation core/uncategorised/311/input.js 1:13-1:19
						}
						JSExpressionStatement {
							leadingComments: ["0"]
							expression: JSReferenceIdentifier {
								name: "x"
								loc: SourceLocation core/uncategorised/311/input.js 2:0-2:1 (x)
							}
							loc: SourceLocation core/uncategorised/311/input.js 2:0-2:2
						}
					]
					directives: []
					loc: SourceLocation core/uncategorised/311/input.js 1:11-2:4
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: []
					loc: SourceLocation core/uncategorised/311/input.js 1:9-1:11
				}
				loc: SourceLocation core/uncategorised/311/input.js 1:1-2:4
			}
			loc: SourceLocation core/uncategorised/311/input.js 1:0-2:5
		}
	]
	comments: [
		CommentLine {
			id: "0"
			value: " Comment"
			loc: SourceLocation core/uncategorised/311/input.js 1:20-1:30
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/311/input.js>
	loc: SourceLocation core/uncategorised/311/input.js 1:0-2:5
}
```

### `diagnostics`

```

```