This crate implements standalone `default()` function that can be imported
with `use default::default;` and then in your code you can just:

```
let foo = default();
```

instead of the usual:


```
let foo = Default::default();
```

## Verification Recommendation

To help with the maintaince, the ownership of this crate is potentially shared between multiple developers.
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)
to verify the trustworthiness of each of your dependencies, including this one.
