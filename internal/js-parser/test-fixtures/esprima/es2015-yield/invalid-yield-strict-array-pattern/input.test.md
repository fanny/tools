# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-yield > invalid-yield-strict-array-pattern`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentArrayPattern {
					elements: [
						JSAssignmentIdentifier {
							name: "yield"
							loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js 1:16-1:21 (yield)
						}
					]
					loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js 1:15-1:22
				}
				right: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js 1:25-1:26 (x)
				}
				loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js 1:15-1:26
			}
			loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js 1:14-1:27
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["yield", RAW_MARKUP {value: " is a reserved word"}]}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js>
				end: Position 1:21
				start: Position 1:16
			}
		}
	]
	directives: [
		JSDirective {
			value: "use strict"
			loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js 1:0-1:13
		}
	]
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js>
	loc: SourceLocation esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-yield/invalid-yield-strict-array-pattern/input.js:1:16 parse(js) ━━━━━━━━━━━━━━━━━━━

  ✖ yield is a reserved word

    "use strict"; ([yield] = x)
                    ^^^^^


```