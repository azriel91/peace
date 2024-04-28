# Multiple Values - Java

<div style="display: flex; justify-content: center; gap: 20px;">
<div style="flex-basis: 50%; flex: 1 0;">

```java
public static String fetch() {
    return "name";
}

public static void main(
    String[] args
) {
    String name = fetch();



    System.out.println(name);
}
```

</div>
<div style="flex-basis: 50%; flex: 1 0;">

```java
public static NameAndVal fetch() {
    return new NameAndVal("name", 123);
}

public static void main(
    String[] args
) {
    var nameAndVal = fetch();
    var name = nameAndVal.name;
    var val = nameAndVal.val;

    System.out.println(String.format(
        "%s %b", name, val
    ));
}

static class NameAndVal {
    final String name;
    final boolean val;

    public NameAndVal(
            final String name,
            final boolean val) {
        this.name = name;
        this.val = val;
    }
}
```

</div>
</div>
