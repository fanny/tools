# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > let > let-at-binding-list-fail-2`

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
											name: "let"
											loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:8-1:11 (let)
										}
										loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:8-1:11
									}
									value: JSBindingIdentifier {
										name: "let"
										loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:8-1:11 (let)
									}
									loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:8-1:11
								}
							]
							loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:6-1:13
						}
						init: JSObjectExpression {
							properties: []
							loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:16-1:18
						}
						loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:6-1:18
					}
				]
				loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:0-1:19
			}
			loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:0-1:19
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/let/let-at-binding-list-fail-2/input.js>
	loc: SourceLocation es2015/let/let-at-binding-list-fail-2/input.js 1:0-2:0
}
```

### `diagnostics`

```

```