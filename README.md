# QuantiiSans Fonts

**note**: Everything here is a work-in-progress, so the README may describe the
future.

The QuantiiSans fonts are a font family based upon [DejaVu fonts] v2.37.

QuanttiSans Fonts is comprised of two variable fonts; QuantiiSans and
QuantiiSans Mono.  Both fonts contain QuantiiSans Emoji - a fullwidth monospace
font (which has variable axes for text-presentation).

## Variable Font Axes

Variable font axes are used to customize the font's look.  `slnt` and `wght` are
available on QuantiiSans and QuantiiSans Mono, while `wdth` is only available on
QuantiiSans.  The axes customizations are designed to be noticeable, but not
extreme.

 - `slnt`: Slant / Obliqueness ("normal" 0, "oblique" 12 degrees)
 - `wght`: Weight / Boldness ("light" 0.8 weight, "book" 1.0 weight, "bold"
   1.2 weight)
 - `wdth`: Width ("condensed" 8/9, "regular" 1.0, "wide" 10/9)

## Getting Started

The included tools to aid in the development of this font requires the Rust
programming language, most recent stable release.  Get it with:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Xtasks

To get the most recent version of the unicode data (used to generate unicode
coverage report):

```shell
cargo xtask unicode
```

## License

For license information, see [`LICENSE`](./LICENSE).

Fonts are published in source form as SFD files (Spline Font Database from
[FontForge]) and in compiled form as OTF files (OpenType fonts).

Characters from Arev fonts
--------------------------
__Copyright (c) 2006 by Tavmjong Bah__

    U+01BA, U+01BF, U+01F7, U+021C-U+021D, U+0220, U+0222-U+0223,
    U+02B9, U+02BA, U+02BD, U+02C2-U+02C5, U+02d4-U+02D5,
    U+02D7, U+02EC-U+02EE, U+0346-U+034E, U+0360, U+0362,
    U+03E2-03EF, U+0460-0463, U+0466-U+0486, U+0488-U+0489, U+04A8-U+04A9,
    U+0500-U+050F, U+2055-205E, U+20B0, U+20B2-U+20B3, U+2102, U+210D, U+210F,
    U+2111, U+2113, U+2115, U+2118-U+211A, U+211C-U+211D, U+2124, U+2135,
    U+213C-U+2140, U+2295-U+2298, U+2308-U+230B, U+26A2-U+26B1, U+2701-U+2704,
    U+2706-U+2709, U+270C-U+274B, U+2758-U+275A, U+2761-U+2775, U+2780-U+2794,
    U+2798-U+27AF, U+27B1-U+27BE, U+FB05-U+FB06

<!-- $Id$ -->

[DejaVu fonts]: http://gnome.org/fonts/
[FontForge]: https://fontforge.github.io/
