# Bizarre behaviour

Crate `a` has 2 features `ax` and `ay`. By default the crate prints:
```
a
```

when run. If `ax` is enabled it'll also print `ax`, so:
```
a
ax
```

and likewise with `ay`.

Both features are enabled by default.

Now running
```
cargo run --bin a --no-default-features --features ax
```

prints out
```
a
ax
ay
```

despite the `--no-default-features --features ax` flags.

on the other hand:
```
cargo run -p a --no-default-features --features ax
```

prints out
```
a
ax
```

as expected.

The issue seems to stem from the fact that there's an additional crate `b` in the workspace that depends on `a`.
