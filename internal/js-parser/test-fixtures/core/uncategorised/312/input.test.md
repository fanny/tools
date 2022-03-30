# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 312`

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
							loc: SourceLocation core/uncategorised/312/input.js 1:13-1:19
						}
						JSExpressionStatement {
							leadingComments: ["0"]
							expression: JSReferenceIdentifier {
								name: "x"
								loc: SourceLocation core/uncategorised/312/input.js 2:10-2:11 (x)
							}
							loc: SourceLocation core/uncategorised/312/input.js 2:10-2:12
						}
					]
					directives: []
					loc: SourceLocation core/uncategorised/312/input.js 1:11-2:14
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: []
					loc: SourceLocation core/uncategorised/312/input.js 1:9-1:11
				}
				loc: SourceLocation core/uncategorised/312/input.js 1:1-2:14
			}
			loc: SourceLocation core/uncategorised/312/input.js 1:0-2:15
		}
	]
	comments: [
		CommentBlock {
			id: "0"
			value: " Multiline\nComment "
			loc: SourceLocation core/uncategorised/312/input.js 1:19-2:10
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/312/input.js>
	loc: SourceLocation core/uncategorised/312/input.js 1:0-2:15
}
```

### `diagnostics`

```

```