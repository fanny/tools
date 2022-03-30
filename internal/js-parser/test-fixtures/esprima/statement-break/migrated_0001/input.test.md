# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > statement-break > migrated_0001`

### `ast`

```javascript
JSRoot {
	body: [
		JSLabeledStatement {
			body: JSWhileStatement {
				body: JSBlockStatement {
					body: [
						JSBreakStatement {
							label: JSIdentifier {
								name: "done"
								loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:27-1:31 (done)
							}
							loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:21-1:31
						}
					]
					directives: []
					loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:19-1:33
				}
				test: JSBooleanLiteral {
					value: true
					loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:13-1:17
				}
				loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:6-1:33
			}
			label: JSIdentifier {
				name: "done"
				loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:0-1:4 (done)
			}
			loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:0-1:33
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/statement-break/migrated_0001/input.js>
	loc: SourceLocation esprima/statement-break/migrated_0001/input.js 1:0-2:0
}
```

### `diagnostics`

```

```