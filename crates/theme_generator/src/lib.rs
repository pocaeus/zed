mod themes;

use anyhow::Result;
pub fn build_themes() -> Result<Vec<theme::Theme>> {
    todo!();
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
            .build()
            .unwrap();
        panic!("{:?}", grad.colors(10));
    }
}
