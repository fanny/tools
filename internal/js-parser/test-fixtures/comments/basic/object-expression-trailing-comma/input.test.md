# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `comments > basic > object-expression-trailing-comma`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "const"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingObjectPattern {
							properties: [
								JSBindingObjectPatternProperty {
									leadingComments: ["0"]
									trailingComments: ["1", "2"]
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "x"
											loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 3:2-3:3 (x)
										}
										loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 3:2-3:3
									}
									value: JSBindingIdentifier {
										name: "x"
										loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 3:2-3:3 (x)
									}
									loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 3:2-3:3
								}
							]
							trailingComments: ["3"]
							loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 1:6-6:1
						}
						init: JSObjectExpression {
							properties: []
							loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 6:15-6:17
						}
						loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 1:6-6:17
					}
				]
				loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 1:0-6:18
			}
			loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 1:0-6:18
		}
	]
	comments: [
		CommentBlock {
			id: "0"
			value: " One "
			loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 2:2-2:11
		}
		CommentBlock {
			id: "1"
			value: " Two "
			loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 4:2-4:11
		}
		CommentBlock {
			id: "2"
			value: " Three "
			loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 5:2-5:13
		}
		CommentBlock {
			id: "3"
			value: " Four "
			loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 6:2-6:12
		}
	]
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<comments/basic/object-expression-trailing-comma/input.js>
	loc: SourceLocation comments/basic/object-expression-trailing-comma/input.js 1:0-6:18
}
```

### `diagnostics`

```

```