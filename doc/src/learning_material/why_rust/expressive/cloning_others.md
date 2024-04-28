# Cloning - Others

### C#

```cs
var serialized = JsonConvert.SerializeObject(source);
return JsonConvert.DeserializeObject<T>(serialized);
```


### JS

```js
JSON.parse(JSON.stringify(original));
```

### 💎 Ruby

```ruby
Marshal.load(Marshal.dump(@object))
```
