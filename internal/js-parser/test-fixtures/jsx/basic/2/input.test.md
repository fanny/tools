# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `jsx > basic > 2`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSXElement {
				attributes: [
					JSXAttribute {
						name: JSXNamespacedName {
							name: JSXIdentifier {
								name: "v"
								loc: SourceLocation jsx/basic/2/input.jsx 1:7-1:8
							}
							namespace: JSXIdentifier {
								name: "n"
								loc: SourceLocation jsx/basic/2/input.jsx 1:5-1:6
							}
							loc: SourceLocation jsx/basic/2/input.jsx 1:5-1:8
						}
						loc: SourceLocation jsx/basic/2/input.jsx 1:5-1:8
					}
				]
				children: []
				selfClosing: true
				name: JSXNamespacedName {
					name: JSXIdentifier {
						name: "a"
						loc: SourceLocation jsx/basic/2/input.jsx 1:3-1:4
					}
					namespace: JSXIdentifier {
						name: "n"
						loc: SourceLocation jsx/basic/2/input.jsx 1:1-1:2
					}
					loc: SourceLocation jsx/basic/2/input.jsx 1:1-1:4
				}
				loc: SourceLocation jsx/basic/2/input.jsx 1:0-1:11
			}
			loc: SourceLocation jsx/basic/2/input.jsx 1:0-1:11
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: ["jsx"]
	path: UIDPath<jsx/basic/2/input.jsx>
	loc: SourceLocation jsx/basic/2/input.jsx 1:0-1:11
}
```

### `diagnostics`

```

```