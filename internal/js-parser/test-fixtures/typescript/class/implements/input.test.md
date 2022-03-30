# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > class > implements`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "C"
				loc: SourceLocation typescript/class/implements/input.ts 1:6-1:7 (C)
			}
			meta: JSClassHead {
				body: []
				implements: [
					TSExpressionWithTypeArguments {
						expression: TSQualifiedName {
							left: JSReferenceIdentifier {
								name: "X"
								loc: SourceLocation typescript/class/implements/input.ts 1:19-1:20 (X)
							}
							right: JSIdentifier {
								name: "Y"
								loc: SourceLocation typescript/class/implements/input.ts 1:21-1:22 (Y)
							}
							loc: SourceLocation typescript/class/implements/input.ts 1:19-1:22
						}
						typeParameters: TSTypeParameterInstantiation {
							params: [
								TSTypeReference {
									typeName: JSReferenceIdentifier {
										name: "T"
										loc: SourceLocation typescript/class/implements/input.ts 1:23-1:24 (T)
									}
									loc: SourceLocation typescript/class/implements/input.ts 1:23-1:24
								}
							]
							loc: SourceLocation typescript/class/implements/input.ts 1:22-1:25
						}
						loc: SourceLocation typescript/class/implements/input.ts 1:19-1:25
					}
				]
				loc: SourceLocation typescript/class/implements/input.ts 1:0-1:28
			}
			loc: SourceLocation typescript/class/implements/input.ts 1:0-1:28
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/class/implements/input.ts>
	loc: SourceLocation typescript/class/implements/input.ts 1:0-2:0
}
```

### `diagnostics`

```

```