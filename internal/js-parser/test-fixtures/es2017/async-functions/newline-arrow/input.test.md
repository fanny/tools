# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > newline-arrow`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation es2017/async-functions/newline-arrow/input.js 2:5-2:6 (x)
				}
				head: JSFunctionHead {
					async: true
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "x"
							loc: SourceLocation es2017/async-functions/newline-arrow/input.js 2:0-2:1 (x)
						}
					]
					loc: SourceLocation es2017/async-functions/newline-arrow/input.js 1:0-2:4
				}
				loc: SourceLocation es2017/async-functions/newline-arrow/input.js 1:0-2:6
			}
			loc: SourceLocation es2017/async-functions/newline-arrow/input.js 1:0-2:6
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/newline-arrow/input.js>
	loc: SourceLocation es2017/async-functions/newline-arrow/input.js 1:0-3:0
}
```

### `diagnostics`

```

```