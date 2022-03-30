# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > meta-properties > new-target`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "foo"
				loc: SourceLocation es2015/meta-properties/new-target/input.js 1:9-1:12 (foo)
			}
			body: JSBlockStatement {
				body: [
					JSExpressionStatement {
						expression: JSMetaProperty {
							meta: JSIdentifier {
								name: "new"
								loc: SourceLocation es2015/meta-properties/new-target/input.js 1:17-1:20 (new)
							}
							property: JSIdentifier {
								name: "target"
								loc: SourceLocation es2015/meta-properties/new-target/input.js 1:21-1:27 (target)
							}
							loc: SourceLocation es2015/meta-properties/new-target/input.js 1:17-1:27
						}
						loc: SourceLocation es2015/meta-properties/new-target/input.js 1:17-1:27
					}
				]
				directives: []
				loc: SourceLocation es2015/meta-properties/new-target/input.js 1:15-1:29
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2015/meta-properties/new-target/input.js 1:12-1:14
			}
			loc: SourceLocation es2015/meta-properties/new-target/input.js 1:0-1:29
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/meta-properties/new-target/input.js>
	loc: SourceLocation es2015/meta-properties/new-target/input.js 1:0-2:0
}
```

### `diagnostics`

```

```