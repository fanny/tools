# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 56`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "x"
					loc: SourceLocation es2015/uncategorised/56/input.js 1:0-1:1 (x)
				}
				right: JSObjectExpression {
					properties: [
						JSObjectMethod {
							kind: "method"
							key: JSStaticPropertyKey {
								value: JSIdentifier {
									name: "set"
									loc: SourceLocation es2015/uncategorised/56/input.js 1:6-1:9 (set)
								}
								loc: SourceLocation es2015/uncategorised/56/input.js 1:6-1:9
							}
							body: JSBlockStatement {
								body: []
								directives: []
								loc: SourceLocation es2015/uncategorised/56/input.js 1:12-1:15
							}
							head: JSFunctionHead {
								async: false
								generator: false
								hasHoistedVars: false
								params: []
								loc: SourceLocation es2015/uncategorised/56/input.js 1:9-1:11
							}
							loc: SourceLocation es2015/uncategorised/56/input.js 1:6-1:15
						}
					]
					loc: SourceLocation es2015/uncategorised/56/input.js 1:4-1:17
				}
				loc: SourceLocation es2015/uncategorised/56/input.js 1:0-1:17
			}
			loc: SourceLocation es2015/uncategorised/56/input.js 1:0-1:17
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/56/input.js>
	loc: SourceLocation es2015/uncategorised/56/input.js 1:0-1:17
}
```

### `diagnostics`

```

```