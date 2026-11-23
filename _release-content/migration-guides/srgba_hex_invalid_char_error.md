---
title: "`Srgba::hex`: `HexColorError::Parse` removed"
pull_requests: []
---

`Srgba::hex` now shares its parser with the new `const fn Srgba::from_hex`, which cannot
construct a `core::num::ParseIntError`. Invalid characters are therefore reported as
`HexColorError::Char`, and `HexColorError::Parse` (along with its `From<ParseIntError>` impl) has
been removed.

Before:

```rust
assert!(matches!(Srgba::hex("yyy"), Err(HexColorError::Parse(_))));
```

After:

```rust
assert!(matches!(Srgba::hex("yyy"), Err(HexColorError::Char('y'))));
```

Exhaustive matches need their `HexColorError::Parse` arm deleted; non-exhaustive ones only need
the `Char('y')` case if they inspect the character.

The new parser is also stricter about inputs that `from_str_radix` used to tolerate. A leading `+`
was accepted as a sign, so `Srgba::hex("+fff")` previously parsed as
`Srgba::rgba_u8(0, 255, 255, 255)`; it now returns `HexColorError::Char('+')`.

Palette definitions that used `Srgba::hex` at runtime can switch to the new `const`
`Srgba::from_hex`, or to `Srgba::from_hex_or_panic`, which fails the build if the string is invalid:

```rust
const ACCENT: Srgba = Srgba::from_hex_or_panic("#2d4a7a");
```
