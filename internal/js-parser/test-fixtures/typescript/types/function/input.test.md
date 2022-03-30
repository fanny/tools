# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > function`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "let"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "f"
							meta: JSPatternMeta {
								typeAnnotation: TSFunctionType {
									meta: TSSignatureDeclarationMeta {
										parameters: [
											JSBindingIdentifier {
												name: "a"
												meta: JSPatternMeta {
													typeAnnotation: TSNumberKeywordTypeAnnotation {
														loc: SourceLocation typescript/types/function/input.ts 1:11-1:17
													}
													loc: SourceLocation typescript/types/function/input.ts 1:8-1:17
												}
												loc: SourceLocation typescript/types/function/input.ts 1:8-1:9 (a)
											}
											JSBindingIdentifier {
												name: "b"
												meta: JSPatternMeta {
													optional: true
													typeAnnotation: TSNumberKeywordTypeAnnotation {
														loc: SourceLocation typescript/types/function/input.ts 1:23-1:29
													}
													loc: SourceLocation typescript/types/function/input.ts 1:19-1:29
												}
												loc: SourceLocation typescript/types/function/input.ts 1:19-1:20 (b)
											}
										]
										rest: JSBindingIdentifier {
											name: "c"
											meta: JSPatternMeta {
												typeAnnotation: TSArrayType {
													elementType: TSNumberKeywordTypeAnnotation {
														loc: SourceLocation typescript/types/function/input.ts 1:37-1:43
													}
													loc: SourceLocation typescript/types/function/input.ts 1:37-1:45
												}
												loc: SourceLocation typescript/types/function/input.ts 1:34-1:45
											}
											loc: SourceLocation typescript/types/function/input.ts 1:34-1:35 (c)
										}
										loc: SourceLocation typescript/types/function/input.ts 1:7-1:54
									}
									typeAnnotation: TSVoidKeywordTypeAnnotation {
										loc: SourceLocation typescript/types/function/input.ts 1:50-1:54
									}
									loc: SourceLocation typescript/types/function/input.ts 1:7-1:54
								}
								loc: SourceLocation typescript/types/function/input.ts 1:4-1:54
							}
							loc: SourceLocation typescript/types/function/input.ts 1:4-1:54
						}
						loc: SourceLocation typescript/types/function/input.ts 1:4-1:54
					}
				]
				loc: SourceLocation typescript/types/function/input.ts 1:0-1:55
			}
			loc: SourceLocation typescript/types/function/input.ts 1:0-1:55
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/types/function/input.ts>
	loc: SourceLocation typescript/types/function/input.ts 1:0-2:0
}
```

### `diagnostics`

```

```