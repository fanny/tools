# `harness.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/compiler/lint/rules/harness.test.ts --update-snapshots` to update.

## `js/useWhile`

### `0`

```

 lint/js/useWhile/reject/1/file.ts:1 lint/js/useWhile  FIXABLE  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Use while loops instead of for loops.

  > 1 │ for (; x.running;) {
      │ ^^^^^^^^^^^^^^^^^^^^
  > 2 │   x.step();
  > 3 │ }
      │ ^

  ℹ Safe fix

    1   │ - for·(;·x.running;·)·{
      1 │ + while·(x.running)·{
    2 2 │   → x.step();
    3 3 │   }


```

### `0: formatted`

```ts
while (x.running) {
	x.step();
}

```

### `1`

```

 lint/js/useWhile/reject/2/file.ts:1 lint/js/useWhile  FIXABLE  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Use while loops instead of for loops.

  > 1 │ for (;;) {
      │ ^^^^^^^^^^
  > 2 │   doSomething();
  > 3 │ }
      │ ^

  ℹ Safe fix

    1   │ - for·(;;)·{
      1 │ + while·(true)·{
    2 2 │   → doSomething();
    3 3 │   }


```

### `1: formatted`

```ts
while (true) {
	doSomething();
}

```