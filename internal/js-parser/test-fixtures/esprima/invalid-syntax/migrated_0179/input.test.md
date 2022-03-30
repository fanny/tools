# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0179`

### `ast`

```javascript
JSRoot {
	body: [
		JSLabeledStatement {
			body: JSWhileStatement {
				body: JSBlockStatement {
					body: [
						JSExpressionStatement {
							expression: JSFunctionExpression {
								body: JSBlockStatement {
									body: [
										JSContinueStatement {
											label: JSIdentifier {
												name: "x"
												loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:42-1:43 (x)
											}
											loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:33-1:44
										}
									]
									directives: []
									loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:31-1:46
								}
								head: JSFunctionHead {
									async: false
									generator: false
									hasHoistedVars: false
									params: []
									loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:28-1:30
								}
								loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:19-1:46
							}
							loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:18-1:48
						}
					]
					directives: []
					loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:16-1:50
				}
				test: JSBooleanLiteral {
					value: true
					loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:10-1:14
				}
				loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:3-1:50
			}
			label: JSIdentifier {
				name: "x"
				loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:0-1:1 (x)
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:0-1:50
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unknown label <emphasis>"}, "x", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0179/input.js>
				end: Position 1:33
				start: Position 1:33
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0179/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0179/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0179/input.js:1:33 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unknown label x

    x: while (true) { (function () { continue x; }); }
                                     ^


```