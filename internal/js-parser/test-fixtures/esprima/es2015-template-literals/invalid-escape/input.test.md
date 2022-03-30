# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-template-literals > invalid-escape`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSTemplateLiteral {
				expressions: []
				quasis: [
					JSTemplateElement {
						cooked: "\\1"
						raw: "\\1"
						tail: true
						loc: SourceLocation esprima/es2015-template-literals/invalid-escape/input.js 1:1-1:3
					}
				]
				loc: SourceLocation esprima/es2015-template-literals/invalid-escape/input.js 1:0-1:4
			}
			loc: SourceLocation esprima/es2015-template-literals/invalid-escape/input.js 1:0-1:5
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
				message: RAW_MARKUP {value: "Invalid escape sequence in template"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-template-literals/invalid-escape/input.js>
				end: Position 1:2
				start: Position 1:2
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-template-literals/invalid-escape/input.js>
	loc: SourceLocation esprima/es2015-template-literals/invalid-escape/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-template-literals/invalid-escape/input.js:1:2 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Invalid escape sequence in template

    `\1`;
      ^


```