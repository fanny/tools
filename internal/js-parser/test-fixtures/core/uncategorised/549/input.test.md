# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 549`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "const"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingObjectPattern {
							properties: [
								JSBindingObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "arguments"
											loc: SourceLocation core/uncategorised/549/input.js 1:8-1:17 (arguments)
										}
										loc: SourceLocation core/uncategorised/549/input.js 1:8-1:17
									}
									value: JSBindingIdentifier {
										name: "arguments"
										loc: SourceLocation core/uncategorised/549/input.js 1:8-1:17 (arguments)
									}
									loc: SourceLocation core/uncategorised/549/input.js 1:8-1:17
								}
							]
							loc: SourceLocation core/uncategorised/549/input.js 1:6-1:19
						}
						init: JSCallExpression {
							arguments: []
							callee: JSReferenceIdentifier {
								name: "foo"
								loc: SourceLocation core/uncategorised/549/input.js 1:22-1:25 (foo)
							}
							loc: SourceLocation core/uncategorised/549/input.js 1:22-1:27
						}
						loc: SourceLocation core/uncategorised/549/input.js 1:6-1:27
					}
				]
				loc: SourceLocation core/uncategorised/549/input.js 1:0-1:28
			}
			loc: SourceLocation core/uncategorised/549/input.js 1:0-1:28
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/549/input.js>
	loc: SourceLocation core/uncategorised/549/input.js 1:0-2:0
}
```

### `diagnostics`

```

```