# `harness.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/compiler/lint/rules/harness.test.ts --update-snapshots` to update.

## `react/useKey`

### `0`

```

 lint/react/useKey/reject/1/file.tsx:1:11 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in array.

    const a = [<div />, <div />]
               ^^^^^^^

  ℹ Keys help React identify which items have changed, are added, or are removed.

 lint/react/useKey/reject/1/file.tsx:1:20 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in array.

    const a = [<div />, <div />]
                        ^^^^^^^

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `0: formatted`

```tsx
const a = [<div />, <div />];

```

### `1`

```

 lint/react/useKey/reject/2/file.tsx:1:26 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    const a = [1, 2].map(x => <div>{x}</div>);
                              ^^^^^^^^^^^^^^

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `1: formatted`

```tsx
const a = [1, 2].map((x) =>
	<div>
		{x}
	</div>
);

```

### `2`

```

 lint/react/useKey/reject/3/file.tsx:1:24 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    const a = foo?.map(x => <div>{x}</div>);
                            ^^^^^^^^^^^^^^

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `2: formatted`

```tsx
const a = foo?.map((x) =>
	<div>
		{x}
	</div>
);

```

### `3`

```

 lint/react/useKey/reject/4/file.tsx:1:34 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    React.Children.map(children, x => <div>{x}</div>);
                                      ^^^^^^^^^^^^^^

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `3: formatted`

```tsx
React.Children.map(
	children,
	(x) =>
		<div>
			{x}
		</div>
	,
);

```

### `4`

```

 lint/react/useKey/reject/5/file.tsx:1:28 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    Children.map(children, x => <div>{x}</div>);
                                ^^^^^^^^^^^^^^

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `4: formatted`

```tsx
Children.map(
	children,
	(x) =>
		<div>
			{x}
		</div>
	,
);

```

### `5`

```

 lint/react/useKey/reject/6/file.tsx:2:8 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    1 │ const a = [1, 2].map(x => {
  > 2 │   return <div>{x}</div>;
      │          ^^^^^^^^^^^^^^
    3 │ });

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `5: formatted`

```tsx
const a = [1, 2].map((x) => {
	return <div>
		{x}
	</div>;
});

```

### `6`

```

 lint/react/useKey/reject/7/file.tsx:2:8 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    1 │ React.Children.map(children, x => {
  > 2 │   return <div>{x}</div>;
      │          ^^^^^^^^^^^^^^
    3 │ });

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `6: formatted`

```tsx
React.Children.map(
	children,
	(x) => {
		return <div>
			{x}
		</div>;
	},
);

```

### `7`

```

 lint/react/useKey/reject/8/file.tsx:2:8 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    1 │ Children.map(children, x => {
  > 2 │   return <div>{x}</div>;
      │          ^^^^^^^^^^^^^^
    3 │ });

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `7: formatted`

```tsx
Children.map(
	children,
	(x) => {
		return <div>
			{x}
		</div>;
	},
);

```

### `8`

```

 lint/react/useKey/reject/9/file.tsx:2:8 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    1 │ const a = [1, 2].map(function(x) {
  > 2 │   return <div>{x}</div>;
      │          ^^^^^^^^^^^^^^
    3 │ });

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `8: formatted`

```tsx
const a = [1, 2].map(function(x) {
	return <div>
		{x}
	</div>;
});

```

### `9`

```

 lint/react/useKey/reject/10/file.tsx:2:8 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    1 │ React.Children.map(children, function(x) {
  > 2 │   return <div>{x}</div>;
      │          ^^^^^^^^^^^^^^
    3 │ });

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `9: formatted`

```tsx
React.Children.map(
	children,
	function(x) {
		return <div>
			{x}
		</div>;
	},
);

```

### `10`

```

 lint/react/useKey/reject/11/file.tsx:2:8 lint/react/useKey ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide a key prop with a unique value for each element in iterator.

    1 │ Children.map(children, function(x) {
  > 2 │   return <div>{x}</div>;
      │          ^^^^^^^^^^^^^^
    3 │ });

  ℹ Keys help React identify which items have changed, are added, or are removed.


```

### `10: formatted`

```tsx
Children.map(
	children,
	function(x) {
		return <div>
			{x}
		</div>;
	},
);

```

### `11`

```

```

### `11: formatted`

```tsx
const a = [<div key="a" />, <div key={"b"} />];

```

### `12`

```

```

### `12: formatted`

```tsx
const a = [1, 2].map((x) =>
	<div key={x}>
		{x}
	</div>
);

```

### `13`

```

```

### `13: formatted`

```tsx
React.Children.map(
	children,
	(x) =>
		<div key={x}>
			{x}
		</div>
	,
);

```

### `14`

```

```

### `14: formatted`

```tsx
Children.map(
	children,
	(x) =>
		<div key={x}>
			{x}
		</div>
	,
);

```

### `15`

```

```

### `15: formatted`

```tsx
const a = [1, 2].map((x) => {
	return <div key={x}>
		{x}
	</div>;
});

```

### `16`

```

```

### `16: formatted`

```tsx
React.Children.map(
	children,
	(x) => {
		return <div key={x}>
			{x}
		</div>;
	},
);

```

### `17`

```

```

### `17: formatted`

```tsx
Children.map(
	children,
	(x) => {
		return <div key={x}>
			{x}
		</div>;
	},
);

```

### `18`

```

```

### `18: formatted`

```tsx
const a = [1, 2].map(function(x) {
	return <div key={x}>
		{x}
	</div>;
});

```

### `19`

```

```

### `19: formatted`

```tsx
React.Children.map(
	children,
	function(x) {
		return <div key={x}>
			{x}
		</div>;
	},
);

```

### `20`

```

```

### `20: formatted`

```tsx
Children.map(
	children,
	function(x) {
		return <div key={x}>
			{x}
		</div>;
	},
);

```