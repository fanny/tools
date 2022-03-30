# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0223`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "hello"
				loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:9-1:14 (hello)
			}
			body: JSBlockStatement {
				body: [
					JSFunctionDeclaration {
						id: JSBindingIdentifier {
							name: "inner"
							loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:42-1:47 (inner)
						}
						body: JSBlockStatement {
							body: []
							directives: [
								JSDirective {
									value: "octal directive\\1"
									loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:52-1:72
								}
							]
							loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:50-1:74
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:47-1:49
						}
						loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:33-1:74
					}
				]
				directives: [
					JSDirective {
						value: "use strict"
						loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:19-1:32
					}
				]
				loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:17-1:76
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:14-1:16
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:0-1:76
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
				message: RAW_MARKUP {value: "Octal literal in strict mode"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0223/input.js>
				end: Position 1:69
				start: Position 1:69
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0223/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0223/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0223/input.js:1:69 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Octal literal in strict mode

    function hello() { "use strict"; function inner() { "octal directive\1"; } }
                                                                         ^


```