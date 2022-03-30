# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > invalid-escape-async-obj-method`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSObjectExpression {
				properties: [
					JSObjectProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "async"
								loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:3-1:13 (async)
							}
							loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:3-1:13
						}
						value: JSReferenceIdentifier {
							name: "async"
							loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:3-1:13 (async)
						}
						loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:3-1:13
					}
				]
				loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:1-1:13
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:0-1:13
		}
		JSExpressionStatement {
			expression: JSCallExpression {
				arguments: []
				callee: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:14-1:15 (x)
				}
				loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:14-1:17
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:14-1:17
		}
		JSBlockStatement {
			body: [
				JSExpressionStatement {
					expression: JSReferenceIdentifier {
						name: "await"
						loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:20-1:25 (await)
					}
					loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:20-1:25
				}
				JSExpressionStatement {
					expression: JSReferenceIdentifier {
						name: "x"
						loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:26-1:27 (x)
					}
					loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:26-1:27
				}
			]
			directives: []
			loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:18-1:29
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:30-1:31
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:30-1:31
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "INVALID_PLACEHOLDER"
				loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:31-1:32
			}
			loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:31-1:32
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: [log {category: "info", text: [RAW_MARKUP {value: "Expected character <emphasis>"}, ",", RAW_MARKUP {value: "</emphasis>"}]}]
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, "x", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2017/async-functions/invalid-escape-async-obj-method/input.js>
				end: Position 1:15
				start: Position 1:14
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/invalid-escape-async-obj-method/input.js>
	loc: SourceLocation es2017/async-functions/invalid-escape-async-obj-method/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2017/async-functions/invalid-escape-async-obj-method/input.js:1:14 parse(js) ━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected character x

    ({ \u0061sync x() { await x } })
                  ^

  ℹ Expected character ,


```