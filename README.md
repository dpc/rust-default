This crate implements standalone `default()` function that can be imported
with `use default::default;` and then in your code you can just:

```
let foo = default();
```

instead of the usual:


```
let foo = Default::default();
```


