# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > class > get-generic`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			declare: true
			id: JSBindingIdentifier {
				name: "C"
				loc: SourceLocation typescript/class/get-generic/input.ts 1:14-1:15 (C)
			}
			meta: JSClassHead {
				body: [
					TSDeclareMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "get"
								loc: SourceLocation typescript/class/get-generic/input.ts 2:4-2:7 (get)
							}
							loc: SourceLocation typescript/class/get-generic/input.ts 2:4-2:7
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/get-generic/input.ts 2:4-2:7
							start: Position 2:4
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							returnType: TSVoidKeywordTypeAnnotation {
								loc: SourceLocation typescript/class/get-generic/input.ts 2:14-2:18
							}
							typeParameters: TSTypeParameterDeclaration {
								params: [
									TSTypeParameter {
										name: "T"
										loc: SourceLocation typescript/class/get-generic/input.ts 2:8-2:9
									}
								]
								loc: SourceLocation typescript/class/get-generic/input.ts 2:7-2:10
							}
							loc: SourceLocation typescript/class/get-generic/input.ts 2:10-2:18
						}
						loc: SourceLocation typescript/class/get-generic/input.ts 2:4-2:19
					}
				]
				loc: SourceLocation typescript/class/get-generic/input.ts 1:0-3:1
			}
			loc: SourceLocation typescript/class/get-generic/input.ts 1:0-3:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/class/get-generic/input.ts>
	loc: SourceLocation typescript/class/get-generic/input.ts 1:0-4:0
}
```

### `diagnostics`

```

```