mod themes;

use anyhow::anyhow;
use anyhow::Result;
pub fn build_themes() -> Result<Vec<theme::Theme>> {
    todo!();
}

enum ThemeLicenseType {
    MIT,
    Apache2,
    Other(String),
}

enum ThemeAppearance {
    Light,
    Dark,
}
impl From<&str> for ThemeLicenseType {
    fn from(input: &str) -> Self {
        match input {
            "MIT" => Self::MIT,
            "Apache License 2.0" => Self::Apache2,
            _ => Self::Other(input.to_owned()),
        }
    }
}

struct ThemeMeta {
    /** The name of the theme */
    name: String,
    /** The theme's appearance. Either `light` or `dark`. */
    appearance: ThemeAppearance,
    /** The author of the theme
     *
     * Ideally formatted as `Full Name <email>`
     *
     * Example: `John Doe <john@doe.com>`
     */
    author: String,
    /** SPDX License string
     *
     * Example: `MIT`
     */
    license_type: ThemeLicenseType,
    license_url: String,
    license_file: String,
    theme_url: String,
}
#[cfg(test)]
mod tests {
    use colorgrad::Color;

    #[test]
    fn print_gradient() {
        let colors = [
            "#1E2025", "#23262E", "#292E38", "#2E323C", "#ACA8AE", "#CBC9CF", "#E1DDE4", "#F7F7F8",
        ]
        .into_iter()
        .map(Color::from_html)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
        let grad = colorgrad::CustomGradient::new()
            .colors(&colors)
            .domain(&[0., 0.15, 0.25, 0.35, 0.7, 0.8, 0.9, 1.])
            .mode(colorgrad::BlendMode::Oklab)
            .build()
            .unwrap();
        panic!(
            "{:?}",
            grad.colors(10)
                .iter()
                .map(colorgrad::Color::to_hex_string)
                .collect::<Vec<_>>()
        );
    }
}
