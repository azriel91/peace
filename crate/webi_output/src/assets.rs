//! Assets to include with the shipped binary, but we can't get it bundled
//! automatically with either `trunk` or `cargo-leptos`. So we include the
//! bytes.

/// Styles shipped with `peace_webi_output`.
pub const PEACE_FAVICON_ICO: &[u8] = include_bytes!("assets/favicon.ico");

/// Provides CSS `@font-face` definitions for the following font families:
///
/// * liberationmono
/// * liberationmono-bold
/// * liberationmono-italic
/// * liberationmono-bold-italic
pub const FONTS_LIBERATION_MONO_CSS_FONT_FACES: &[u8] = include_bytes!("assets/fonts/fonts.css");

/// The Liberation Mono Regular font bytes.
pub const FONTS_LIBERATION_MONO_REGULAR: &[u8] =
    include_bytes!("assets/fonts/liberationmono/LiberationMono-Regular-webfont.woff");

/// The Liberation Mono Bold font bytes.
pub const FONTS_LIBERATION_MONO_BOLD: &[u8] =
    include_bytes!("assets/fonts/liberationmono/LiberationMono-Bold-webfont.woff");

/// The Liberation Mono Italic font bytes.
pub const FONTS_LIBERATION_MONO_ITALIC: &[u8] =
    include_bytes!("assets/fonts/liberationmono/LiberationMono-Italic-webfont.woff");

/// The Liberation Mono Bold Italic font bytes.
pub const FONTS_LIBERATION_MONO_BOLD_ITALIC: &[u8] =
    include_bytes!("assets/fonts/liberationmono/LiberationMono-BoldItalic-webfont.woff");

/// List of assets -- path and content.
pub const ASSETS: &[(&str, &[u8])] = &[
    ("webi/favicon.ico", PEACE_FAVICON_ICO),
    ("webi/fonts/fonts.css", FONTS_LIBERATION_MONO_CSS_FONT_FACES),
    (
        "webi/fonts/liberationmono/LiberationMono-Regular-webfont.woff",
        FONTS_LIBERATION_MONO_REGULAR,
    ),
    (
        "webi/fonts/liberationmono/LiberationMono-Bold-webfont.woff",
        FONTS_LIBERATION_MONO_BOLD,
    ),
    (
        "webi/fonts/liberationmono/LiberationMono-Italic-webfont.woff",
        FONTS_LIBERATION_MONO_ITALIC,
    ),
    (
        "webi/fonts/liberationmono/LiberationMono-BoldItalic-webfont.woff",
        FONTS_LIBERATION_MONO_BOLD_ITALIC,
    ),
];
