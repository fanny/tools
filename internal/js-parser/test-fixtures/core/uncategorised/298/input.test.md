# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 298`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "hello"
							loc: SourceLocation core/uncategorised/298/input.js 1:4-1:9 (hello)
						}
						init: JSFunctionExpression {
							id: JSBindingIdentifier {
								name: "hi"
								loc: SourceLocation core/uncategorised/298/input.js 1:21-1:23 (hi)
							}
							body: JSBlockStatement {
								body: [
									JSExpressionStatement {
										expression: JSCallExpression {
											arguments: []
											callee: JSReferenceIdentifier {
												name: "sayHi"
												loc: SourceLocation core/uncategorised/298/input.js 1:28-1:33 (sayHi)
											}
											loc: SourceLocation core/uncategorised/298/input.js 1:28-1:35
										}
										loc: SourceLocation core/uncategorised/298/input.js 1:28-1:35
									}
								]
								directives: []
								loc: SourceLocation core/uncategorised/298/input.js 1:26-1:37
							}
							head: JSFunctionHead {
								async: false
								generator: false
								hasHoistedVars: false
								params: []
								loc: SourceLocation core/uncategorised/298/input.js 1:23-1:25
							}
							loc: SourceLocation core/uncategorised/298/input.js 1:12-1:37
						}
						loc: SourceLocation core/uncategorised/298/input.js 1:4-1:37
					}
				]
				loc: SourceLocation core/uncategorised/298/input.js 1:0-1:38
			}
			loc: SourceLocation core/uncategorised/298/input.js 1:0-1:38
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/298/input.js>
	loc: SourceLocation core/uncategorised/298/input.js 1:0-1:38
}
```

### `diagnostics`

```

```