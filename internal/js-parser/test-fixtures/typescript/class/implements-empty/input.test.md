# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > class > implements-empty`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "Foo"
				loc: SourceLocation typescript/class/implements-empty/input.ts 1:6-1:9 (Foo)
			}
			meta: JSClassHead {
				body: []
				implements: []
				loc: SourceLocation typescript/class/implements-empty/input.ts 1:0-2:1
			}
			loc: SourceLocation typescript/class/implements-empty/input.ts 1:0-2:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["implements", RAW_MARKUP {value: " list cannot be empty"}]}
			location: {
				language: "js"
				path: UIDPath<typescript/class/implements-empty/input.ts>
				end: Position 1:21
				start: Position 1:21
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/class/implements-empty/input.ts>
	loc: SourceLocation typescript/class/implements-empty/input.ts 1:0-3:0
}
```

### `diagnostics`

```

 typescript/class/implements-empty/input.ts:1:21 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ implements list cannot be empty

  > 1 │ class Foo implements {
      │                      ^
    2 │ }


```