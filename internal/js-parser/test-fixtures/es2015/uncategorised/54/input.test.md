# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 54`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "x"
					loc: SourceLocation es2015/uncategorised/54/input.js 1:0-1:1 (x)
				}
				right: JSObjectExpression {
					properties: [
						JSObjectMethod {
							kind: "method"
							key: JSStaticPropertyKey {
								value: JSStringLiteral {
									value: "method"
									loc: SourceLocation es2015/uncategorised/54/input.js 1:6-1:14
								}
								loc: SourceLocation es2015/uncategorised/54/input.js 1:6-1:14
							}
							body: JSBlockStatement {
								body: []
								directives: []
								loc: SourceLocation es2015/uncategorised/54/input.js 1:17-1:20
							}
							head: JSFunctionHead {
								async: false
								generator: false
								hasHoistedVars: false
								params: []
								loc: SourceLocation es2015/uncategorised/54/input.js 1:14-1:16
							}
							loc: SourceLocation es2015/uncategorised/54/input.js 1:6-1:20
						}
					]
					loc: SourceLocation es2015/uncategorised/54/input.js 1:4-1:22
				}
				loc: SourceLocation es2015/uncategorised/54/input.js 1:0-1:22
			}
			loc: SourceLocation es2015/uncategorised/54/input.js 1:0-1:22
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/54/input.js>
	loc: SourceLocation es2015/uncategorised/54/input.js 1:0-1:22
}
```

### `diagnostics`

```

```