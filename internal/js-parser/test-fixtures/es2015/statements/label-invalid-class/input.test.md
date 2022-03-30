# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > statements > label-invalid-class`

### `ast`

```javascript
JSRoot {
	body: [
		JSLabeledStatement {
			body: JSClassDeclaration {
				id: JSBindingIdentifier {
					name: "X"
					loc: SourceLocation es2015/statements/label-invalid-class/input.js 1:11-1:12 (X)
				}
				meta: JSClassHead {
					body: []
					loc: SourceLocation es2015/statements/label-invalid-class/input.js 1:5-1:15
				}
				loc: SourceLocation es2015/statements/label-invalid-class/input.js 1:5-1:15
			}
			label: JSIdentifier {
				name: "foo"
				loc: SourceLocation es2015/statements/label-invalid-class/input.js 1:0-1:3 (foo)
			}
			loc: SourceLocation es2015/statements/label-invalid-class/input.js 1:0-1:15
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
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, "c", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2015/statements/label-invalid-class/input.js>
				end: Position 1:10
				start: Position 1:5
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/statements/label-invalid-class/input.js>
	loc: SourceLocation es2015/statements/label-invalid-class/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2015/statements/label-invalid-class/input.js:1:5 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected character c

    foo: class X {}
         ^^^^^


```