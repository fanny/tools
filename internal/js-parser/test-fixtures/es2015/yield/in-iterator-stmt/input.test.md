# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > yield > in-iterator-stmt`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "g"
				loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 1:10-1:11 (g)
			}
			body: JSBlockStatement {
				body: [
					JSForInStatement {
						body: JSEmptyStatement {
							loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 2:21-2:22
						}
						left: JSAssignmentIdentifier {
							name: "INVALID_PLACEHOLDER"
							loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 2:16-2:15
						}
						right: JSObjectExpression {
							properties: []
							loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 2:19-2:21
						}
						loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 2:2-2:22
					}
					JSEmptyStatement {
						loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 2:23-2:24
					}
					JSExpressionStatement {
						expression: JSReferenceIdentifier {
							name: "INVALID_PLACEHOLDER"
							loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 2:25-2:26
						}
						loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 2:25-2:28
					}
				]
				directives: []
				loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 1:14-3:1
			}
			head: JSFunctionHead {
				async: false
				generator: true
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 1:11-1:13
			}
			loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 1:0-3:1
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: [RAW_MARKUP {value: "Invalid left-hand side in "}, "for-in statement"]}
			location: {
				language: "js"
				path: UIDPath<es2015/yield/in-iterator-stmt/input.js>
				end: Position 2:15
				start: Position 2:7
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/yield/in-iterator-stmt/input.js>
	loc: SourceLocation es2015/yield/in-iterator-stmt/input.js 1:0-4:0
}
```

### `diagnostics`

```

 es2015/yield/in-iterator-stmt/input.js:2:7 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Invalid left-hand side in for-in statement

    1 │ function* g() {
  > 2 │   for (yield '' in {}; ; ) ;
      │        ^^^^^^^^
    3 │ }


```