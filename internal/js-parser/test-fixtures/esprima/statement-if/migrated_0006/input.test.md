# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > statement-if > migrated_0006`

### `ast`

```javascript
JSRoot {
	body: [
		JSIfStatement {
			alternate: JSEmptyStatement {
				loc: SourceLocation esprima/statement-if/migrated_0006/input.js 1:22-1:23
			}
			consequent: JSExpressionStatement {
				expression: JSCallExpression {
					arguments: []
					callee: JSReferenceIdentifier {
						name: "that"
						loc: SourceLocation esprima/statement-if/migrated_0006/input.js 1:10-1:14 (that)
					}
					loc: SourceLocation esprima/statement-if/migrated_0006/input.js 1:10-1:16
				}
				loc: SourceLocation esprima/statement-if/migrated_0006/input.js 1:10-1:17
			}
			test: JSBooleanLiteral {
				value: true
				loc: SourceLocation esprima/statement-if/migrated_0006/input.js 1:4-1:8
			}
			loc: SourceLocation esprima/statement-if/migrated_0006/input.js 1:0-1:23
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/statement-if/migrated_0006/input.js>
	loc: SourceLocation esprima/statement-if/migrated_0006/input.js 1:0-2:0
}
```

### `diagnostics`

```

```