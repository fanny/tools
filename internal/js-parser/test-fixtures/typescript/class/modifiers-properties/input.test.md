# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > class > modifiers-properties`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			abstract: true
			id: JSBindingIdentifier {
				name: "C"
				loc: SourceLocation typescript/class/modifiers-properties/input.ts 1:15-1:16 (C)
			}
			meta: JSClassHead {
				body: [
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "r"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 2:13-2:14 (r)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 2:13-2:14
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: true
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 2:4-2:14
							start: Position 2:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 2:4-2:15
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "r2"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 3:13-3:15 (r2)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 3:13-3:15
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: true
							readonly: true
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 3:4-3:16
							start: Position 3:4
						}
						typeAnnotation: TSNumberKeywordTypeAnnotation {
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 3:18-3:24
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 3:4-3:25
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "a"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 4:13-4:14 (a)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 4:13-4:14
						}
						meta: JSClassPropertyMeta {
							abstract: true
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 4:4-4:14
							start: Position 4:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 4:4-4:15
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "s"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 5:11-5:12 (s)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 5:11-5:12
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 5:4-5:12
							start: Position 5:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 5:4-5:13
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "pu"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 7:11-7:13 (pu)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 7:11-7:13
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: "public"
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 7:4-7:13
							start: Position 7:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 7:4-7:14
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "po"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 8:14-8:16 (po)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 8:14-8:16
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: "protected"
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 8:4-8:16
							start: Position 8:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 8:4-8:17
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "pi"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 9:12-9:14 (pi)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 9:12-9:14
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: "private"
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 9:4-9:14
							start: Position 9:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 9:4-9:15
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "ra"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 11:22-11:24 (ra)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 11:22-11:24
						}
						meta: JSClassPropertyMeta {
							abstract: true
							optional: false
							readonly: true
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 11:4-11:24
							start: Position 11:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 11:4-11:25
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "ar"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 12:22-12:24 (ar)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 12:22-12:24
						}
						meta: JSClassPropertyMeta {
							abstract: true
							optional: false
							readonly: true
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 12:4-12:24
							start: Position 12:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 12:4-12:25
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "sr"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 13:20-13:22 (sr)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 13:20-13:22
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: true
							static: true
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 13:4-13:22
							start: Position 13:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 13:4-13:23
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "pur"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 15:20-15:23 (pur)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 15:20-15:23
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: "public"
							optional: false
							readonly: true
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 15:4-15:23
							start: Position 15:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 15:4-15:24
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "pua"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 16:20-16:23 (pua)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 16:20-16:23
						}
						meta: JSClassPropertyMeta {
							abstract: true
							accessibility: "public"
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 16:4-16:23
							start: Position 16:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 16:4-16:24
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "pus"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 17:18-17:21 (pus)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 17:18-17:21
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: "public"
							optional: false
							readonly: false
							static: true
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 17:4-17:21
							start: Position 17:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 17:4-17:22
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "pura"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 18:29-18:33 (pura)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 18:29-18:33
						}
						meta: JSClassPropertyMeta {
							abstract: true
							accessibility: "public"
							optional: false
							readonly: true
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 18:4-18:33
							start: Position 18:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 18:4-18:34
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "puar"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 19:29-19:33 (puar)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 19:29-19:33
						}
						meta: JSClassPropertyMeta {
							abstract: true
							accessibility: "public"
							optional: false
							readonly: true
							static: false
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 19:4-19:33
							start: Position 19:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 19:4-19:34
					}
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "pusr"
								loc: SourceLocation typescript/class/modifiers-properties/input.ts 20:27-20:31 (pusr)
							}
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 20:27-20:31
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: "public"
							optional: false
							readonly: true
							static: true
							loc: SourceLocation typescript/class/modifiers-properties/input.ts 20:4-20:31
							start: Position 20:4
						}
						loc: SourceLocation typescript/class/modifiers-properties/input.ts 20:4-20:32
					}
				]
				loc: SourceLocation typescript/class/modifiers-properties/input.ts 1:0-21:1
			}
			loc: SourceLocation typescript/class/modifiers-properties/input.ts 1:0-21:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/class/modifiers-properties/input.ts>
	loc: SourceLocation typescript/class/modifiers-properties/input.ts 1:0-22:0
}
```

### `diagnostics`

```

```