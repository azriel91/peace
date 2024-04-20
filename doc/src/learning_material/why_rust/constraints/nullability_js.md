# Nullability - JS

```js
function print(s) {
    console.log(s);
    console.log(s.length);
}

// incorrect usage, valid
print();          // `undefined`, Uncaught TypeError: s is undefined
print(undefined); // `undefined`, Uncaught TypeError: s is undefined
print(null);      // `null`,      Uncaught TypeError: s is null

// correct usage, valid
print("hello");  // hello, 5
````

<!--

1. If you've programmed for more than 2 days, you will know that `null` means nothing.
2. And in Javascript, you can call `print` with "null", which means "nothing".
3. or, you can call it with, actually nothing.
4. or, you can call it with `undefined`, which is also nothing.

-->
