# Example: Non-nullability - JS

```js
function doSomething(s) {
    console.log(s);
    console.log(s.length);
}
```

```js
// hello, 5
doSomething("hello");

// `null`, Uncaught TypeError: s is null
doSomething(null);

// `undefined`, Uncaught TypeError: s is undefined
doSomething();
doSomething(undefined);
````

<!--

1. If you've programmed for more than 2 days, you will know that `null` means nothing.
2. And in Javascript, you can call `doSomething` with "null", which means "nothing".
3. or, you can call it with, actually nothing.
4. or, you can call it with `undefined`, which is also nothing.

-->