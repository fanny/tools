# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > import > import-default-and-named-id-type`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "typescript/import/import-default-and-named-id-type/input.ts"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array ["ts"]
	loc: Object {
		filename: "typescript/import/import-default-and-named-id-type/input.ts"
		end: Object {
			column: 0
			line: 2
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	body: Array [
		JSImportDeclaration {
			importKind: "type"
			namespaceSpecifier: undefined
			loc: Object {
				filename: "typescript/import/import-default-and-named-id-type/input.ts"
				end: Object {
					column: 32
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			source: JSStringLiteral {
				value: "foo"
				loc: Object {
					filename: "typescript/import/import-default-and-named-id-type/input.ts"
					end: Object {
						column: 31
						line: 1
					}
					start: Object {
						column: 26
						line: 1
					}
				}
			}
			defaultSpecifier: JSImportDefaultSpecifier {
				loc: Object {
					filename: "typescript/import/import-default-and-named-id-type/input.ts"
					end: Object {
						column: 11
						line: 1
					}
					start: Object {
						column: 0
						line: 1
					}
				}
				local: JSImportSpecifierLocal {
					name: JSBindingIdentifier {
						name: "type"
						loc: Object {
							filename: "typescript/import/import-default-and-named-id-type/input.ts"
							identifierName: "type"
							end: Object {
								column: 11
								line: 1
							}
							start: Object {
								column: 7
								line: 1
							}
						}
					}
					importKind: "type"
					loc: Object {
						filename: "typescript/import/import-default-and-named-id-type/input.ts"
						end: Object {
							column: 11
							line: 1
						}
						start: Object {
							column: 7
							line: 1
						}
					}
				}
			}
			namedSpecifiers: Array [
				JSImportSpecifier {
					loc: Object {
						filename: "typescript/import/import-default-and-named-id-type/input.ts"
						end: Object {
							column: 18
							line: 1
						}
						start: Object {
							column: 15
							line: 1
						}
					}
					imported: JSIdentifier {
						name: "bar"
						loc: Object {
							filename: "typescript/import/import-default-and-named-id-type/input.ts"
							identifierName: "bar"
							end: Object {
								column: 18
								line: 1
							}
							start: Object {
								column: 15
								line: 1
							}
						}
					}
					local: JSImportSpecifierLocal {
						name: JSBindingIdentifier {
							name: "bar"
							loc: Object {
								filename: "typescript/import/import-default-and-named-id-type/input.ts"
								identifierName: "bar"
								end: Object {
									column: 18
									line: 1
								}
								start: Object {
									column: 15
									line: 1
								}
							}
						}
						importKind: undefined
						loc: Object {
							filename: "typescript/import/import-default-and-named-id-type/input.ts"
							end: Object {
								column: 18
								line: 1
							}
							start: Object {
								column: 15
								line: 1
							}
						}
					}
				}
			]
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```