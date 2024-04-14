# Multiple Values - Java

```java
public static String fetchName() {
    return "name";
}

public static void main(String[] args) {
    String name = fetchName();

    System.out.println(name);
}
```


```java
public static NameAndCached fetchName() {
    return new NameAndCached("name", false);
}

public static void main(String[] args) {
    NameAndCached nameAndIsCached = fetchName();
    String name = nameAndIsCached.name;
    boolean cached = nameAndIsCached.cached;

    System.out.println(String.format("%s %b", name, cached));
}

static class NameAndCached {
    final String name;
    final boolean cached;

    public NameAndCached(final String name, final boolean cached) {
        this.name = name;
        this.cached = cached;
    }
}
```
