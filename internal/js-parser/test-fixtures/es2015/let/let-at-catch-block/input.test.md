# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > let > let-at-catch-block`

### `ast`

```javascript
JSRoot {
	body: [
		JSTryStatement {
			block: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation es2015/let/let-at-catch-block/input.js 1:4-1:6
			}
			handler: JSCatchClause {
				body: JSBlockStatement {
					body: [
						JSVariableDeclarationStatement {
							declaration: JSVariableDeclaration {
								kind: "let"
								declarations: [
									JSVariableDeclarator {
										id: JSBindingIdentifier {
											name: "let"
											loc: SourceLocation es2015/let/let-at-catch-block/input.js 2:6-2:9 (let)
										}
										loc: SourceLocation es2015/let/let-at-catch-block/input.js 2:6-2:9
									}
								]
								loc: SourceLocation es2015/let/let-at-catch-block/input.js 2:2-2:10
							}
							loc: SourceLocation es2015/let/let-at-catch-block/input.js 2:2-2:10
						}
					]
					directives: []
					loc: SourceLocation es2015/let/let-at-catch-block/input.js 1:19-3:1
				}
				param: JSBindingIdentifier {
					name: "err"
					loc: SourceLocation es2015/let/let-at-catch-block/input.js 1:14-1:17 (err)
				}
				loc: SourceLocation es2015/let/let-at-catch-block/input.js 1:7-3:1
			}
			loc: SourceLocation es2015/let/let-at-catch-block/input.js 1:0-3:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/let/let-at-catch-block/input.js>
	loc: SourceLocation es2015/let/let-at-catch-block/input.js 1:0-3:1
}
```

### `diagnostics`

```

```