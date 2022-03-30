# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > arrow-function > annotated`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSArrowFunctionExpression {
				body: JSReferenceIdentifier {
					name: "x"
					loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:23-1:24 (x)
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					params: [
						JSBindingIdentifier {
							name: "x"
							meta: JSPatternMeta {
								typeAnnotation: TSNumberKeywordTypeAnnotation {
									loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:4-1:10
								}
								loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:23-1:22
							}
							loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:23-1:22
						}
					]
					returnType: TSNumberKeywordTypeAnnotation {
						loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:13-1:19
					}
					loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:0-1:22
				}
				loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:0-1:24
			}
			loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:0-1:25
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/arrow-function/annotated/input.ts>
	loc: SourceLocation typescript/arrow-function/annotated/input.ts 1:0-2:0
}
```

### `diagnostics`

```

```