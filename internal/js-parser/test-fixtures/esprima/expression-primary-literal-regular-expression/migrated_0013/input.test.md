# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > expression-primary-literal-regular-expression > migrated_0013`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "x"
							loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:4-1:5 (x)
						}
						init: JSMemberExpression {
							object: JSRegExpLiteral {
								global: true
								insensitive: false
								multiline: false
								noDotNewline: false
								sticky: false
								unicode: false
								expression: JSRegExpSubExpression {
									body: [
										JSRegExpCharacter {
											value: "4"
											loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:9-1:10
										}
										JSRegExpCharacter {
											value: "2"
											loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:10-1:11
										}
									]
									loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:9-1:11
								}
								loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:8-1:13
							}
							property: JSStaticMemberProperty {
								value: JSIdentifier {
									name: "test"
									loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:14-1:18 (test)
								}
								loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:14-1:18 (test)
							}
							loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:8-1:18
						}
						loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:4-1:18
					}
				]
				loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:0-1:18
			}
			loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:0-1:18
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: true
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/expression-primary-literal-regular-expression/migrated_0013/input.js>
	loc: SourceLocation esprima/expression-primary-literal-regular-expression/migrated_0013/input.js 1:0-1:18
}
```

### `diagnostics`

```

```