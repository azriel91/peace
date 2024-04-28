# Nullability - C#

```cs
static void print(String s) {
    Console.WriteLine(s);
    Console.WriteLine(s.Length);
}

~ public static void Main() {
print();        // incorrect usage, invalid
print(null);    // incorrect usage, valid
print("hello"); // correct usage,   valid
~ }
```
