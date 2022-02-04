# KaTeX WASM (KaTeX 0.15.2)

## Basic

Notice that this does not include css, so cdn still must be included.

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/katex.min.css">
```


```rust
use katex_wasmbind::KaTeXOptions;

fn main() {
    let d = KaTeXOptions::display_mode();
    let i = KaTeXOptions::inline_mode();
    assert_ne!(d.render("\\frac12"), i.render("\\frac12"));
}
```

## Todo list

- [ ] Remove serde dependencies
