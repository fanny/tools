# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-spread-element > new-spread-default`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSNewExpression {
				arguments: [
					JSReferenceIdentifier {
						name: "g"
						loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:6-1:7 (g)
					}
					JSSpreadElement {
						argument: JSAssignmentExpression {
							operator: "="
							left: JSAssignmentIdentifier {
								name: "h"
								loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:12-1:13 (h)
							}
							right: JSReferenceIdentifier {
								name: "i"
								loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:16-1:17 (i)
							}
							loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:12-1:17
						}
						loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:9-1:17
					}
				]
				callee: JSReferenceIdentifier {
					name: "f"
					loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:4-1:5 (f)
				}
				loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:0-1:18
			}
			loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:0-1:19
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-spread-element/new-spread-default/input.js>
	loc: SourceLocation esprima/es2015-spread-element/new-spread-default/input.js 1:0-2:0
}
```

### `diagnostics`

```

```