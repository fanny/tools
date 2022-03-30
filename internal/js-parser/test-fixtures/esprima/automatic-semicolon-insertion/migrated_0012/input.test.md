# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > automatic-semicolon-insertion > migrated_0012`

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
							loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 1:13-1:19
						}
						JSExpressionStatement {
							leadingComments: ["0"]
							expression: JSReferenceIdentifier {
								name: "x"
								loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 2:10-2:11 (x)
							}
							loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 2:10-2:12
						}
					]
					directives: []
					loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 1:11-2:14
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: []
					loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 1:9-1:11
				}
				loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 1:1-2:14
			}
			loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 1:0-2:15
		}
	]
	comments: [
		CommentBlock {
			id: "0"
			value: " Multiline\nComment "
			loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 1:19-2:10
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/automatic-semicolon-insertion/migrated_0012/input.js>
	loc: SourceLocation esprima/automatic-semicolon-insertion/migrated_0012/input.js 1:0-3:0
}
```

### `diagnostics`

```

```