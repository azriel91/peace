# Equality - Java

<!-- 2 coke cans. Does this equal that. -->

```java
# class Data {
#     public int value;
#     public Data(int value) { this.value = value; }
# }
#
String ab = "ab";
String c = "c";
# boolean[] equality = new boolean[] {

// Which of the following are true / false?
"abc" == "abc",
"abc" == "ab" + "c",
"abc" == ab + c,
123 == 123,
new Data(123) == new Data(123)
#
# };
#
# for (boolean equal: equality) {
#     System.out.println(equal);
# }
```

<https://dev.java/playground/>


<details><summary>C#</summary>

<https://dotnetfiddle.net/#>

```cs
~ using System;
~
~ public class Program
~ {
class Data {
    public int value { get; set; }
    public Data(int value) { this.value = value; }
}
~     public static void Main()
~     {

String ab = "ab";
String c = "c";
bool[] equality = new bool[] {
    "abc" == "abc",
    "abc" == "ab" + "c",
    "abc" == ab + c,
    123 == 123,
    new Data(123) == new Data(123)
};

foreach (bool equal in equality) {
    Console.WriteLine(equal);
}
~
~     }
~ }
```

</details>
