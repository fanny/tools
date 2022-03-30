# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2020 > dynamic-import > direct-calls-only`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "failsParse"
				loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 1:9-1:19 (failsParse)
			}
			body: JSBlockStatement {
				body: [
					JSReturnStatement {
						argument: JSCallExpression {
							arguments: []
							callee: JSMetaProperty {
								meta: JSIdentifier {
									name: "import"
									loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 2:9-2:15 (import)
								}
								property: JSIdentifier {
									name: "then"
									loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 2:16-2:20 (then)
								}
								loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 2:9-2:20
							}
							loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 2:9-2:22
						}
						loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 2:2-2:23
					}
				]
				directives: []
				loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 1:22-3:1
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 1:19-1:21
			}
			loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 1:0-3:1
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
				message: [RAW_MARKUP {value: "The only valid meta property for "}, "import", RAW_MARKUP {value: " is "}, "import", RAW_MARKUP {value: "."}, "meta"]
			}
			location: {
				language: "js"
				path: UIDPath<es2020/dynamic-import/direct-calls-only/input.js>
				end: Position 2:20
				start: Position 2:16
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2020/dynamic-import/direct-calls-only/input.js>
	loc: SourceLocation es2020/dynamic-import/direct-calls-only/input.js 1:0-4:0
}
```

### `diagnostics`

```

 es2020/dynamic-import/direct-calls-only/input.js:2:16 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ The only valid meta property for import is import.meta

    1 │ function failsParse() {
  > 2 │   return import.then();
      │                 ^^^^
    3 │ }


```