# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 136`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation es2015/uncategorised/136/input.js 1:6-1:7 (A)
			}
			meta: JSClassHead {
				body: [
					JSClassMethod {
						kind: "method"
						key: JSComputedPropertyKey {
							value: JSReferenceIdentifier {
								name: "foo"
								loc: SourceLocation es2015/uncategorised/136/input.js 1:18-1:21 (foo)
							}
							loc: SourceLocation es2015/uncategorised/136/input.js 1:17-1:22
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation es2015/uncategorised/136/input.js 1:10-1:22
							start: Position 1:10
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/uncategorised/136/input.js 1:25-1:27
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/uncategorised/136/input.js 1:22-1:24
						}
						loc: SourceLocation es2015/uncategorised/136/input.js 1:10-1:27
					}
				]
				loc: SourceLocation es2015/uncategorised/136/input.js 1:0-1:29
			}
			loc: SourceLocation es2015/uncategorised/136/input.js 1:0-1:29
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/136/input.js>
	loc: SourceLocation es2015/uncategorised/136/input.js 1:0-1:29
}
```

### `diagnostics`

```

```