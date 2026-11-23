---
title: Const color constructors and palettes
authors: []
pull_requests: []
---

Most `bevy_color` constructors and channel setters were already `const`. The remaining pieces now
are too: `Alpha::with_alpha`/`Alpha::alpha`, `Hue::with_hue`/`Hue::rotate_hue`, and `Mix::mix`
have `const` inherent counterparts on every color type, `Color::with_alpha`/`Color::alpha` are
`const`, and hex parsing is available as `Srgba::from_hex`/`Srgba::from_hex_or_panic` plus the
`Color::from_hex`/`Color::from_hex_or_panic` wrappers. A palette can therefore be built, adjusted,
mixed, and parsed entirely at compile time.

```rust
use bevy_color::{Color, Oklcha, Srgba};

struct Chrome;

impl Chrome {
    const PANEL: Oklcha = Oklcha::lch(0.06, 0.0, 260.0);
    const TEXT: Color = Color::Oklcha(Self::PANEL.with_lightness(0.92));
    const DIM: Color = Color::Oklcha(Self::PANEL.with_lightness(0.72).with_alpha(0.6));
    const ACCENT: Oklcha = Oklcha::lch(0.83, 0.12, 225.0);
    const WARNING: Oklcha = Self::ACCENT.with_hue(90.0);
    const HOVER: Oklcha = Self::ACCENT.rotate_hue(15.0);
    const BETWEEN: Oklcha = Self::ACCENT.mix(&Self::WARNING, 0.5);
    const OCEAN: Srgba = Srgba::from_hex_or_panic("#2d4a7a");
}
```

Trait methods themselves stay non-`const` (const trait impls are unstable), so call the inherent
counterparts from a `const` context. Space conversions (`Color::to_srgba`, the `From` impls) also
remain runtime-only because they need `powf`, `sqrt`/`cbrt`, or `atan2`/`sin`/`cos`: convert once
at startup, or author palettes in the space the consumer needs.
