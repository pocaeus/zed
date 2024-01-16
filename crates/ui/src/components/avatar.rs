use crate::prelude::*;
use gpui::{img, Hsla, ImageSource, Img, IntoElement, Styled};

/// The shape of an [`Avatar`].
#[derive(Debug, Default, PartialEq, Clone)]
pub enum AvatarShape {
    /// The avatar is shown in a circle.
    #[default]
    Circle,
    /// The avatar is shown in a rectangle with rounded corners.
    RoundedRectangle,
}

/// An element that renders a user avatar with customizable appearance options.
///
/// # Examples
///
/// ```
/// use ui::{Avatar, AvatarShape};
///
/// Avatar::new("path/to/image.png")
///     .shape(AvatarShape::Circle)
///     .grayscale(true)
///     .border_color(gpui::red());
/// ```
#[derive(IntoElement)]
pub struct Avatar {
    image: Img,
    size: Option<Pixels>,
    border_color: Option<Hsla>,
    status_indicator: Option<bool>,
}

impl RenderOnce for Avatar {
    fn render(mut self, cx: &mut WindowContext) -> impl IntoElement {
        if self.image.style().corner_radii.top_left.is_none() {
            self = self.shape(AvatarShape::Circle);
        }

        let icon_size = self.size.unwrap_or_else(|| cx.rem_size());
        let border_width = if self.border_color.is_some() {
            px(3. * 2.)
        } else {
            px(0.)
        };
        let div_size = icon_size + border_width;

        div()
            .size(div_size)
            .map(|mut div| {
                div.style().corner_radii = self.image.style().corner_radii.clone();
                div
            })
            .when_some(self.border_color, |this, color| {
                this.border_3().border_color(color)
            })
            .child(
                self.image
                    .size(icon_size)
                    .bg(cx.theme().colors().ghost_element_background),
            )
            .children(self.status_indicator.map(|is_free| {
                // HACK: non-integer sizes result in oval indicators.
                let indicator_size = (icon_size * 0.4).round();

                div()
                    .absolute()
                    .z_index(1)
                    .bg(if is_free {
                        cx.theme().status().created
                    } else {
                        cx.theme().status().deleted
                    })
                    .size(indicator_size)
                    .rounded(indicator_size)
                    .bottom_0()
                    .right_0()
            }))
    }
}

impl Avatar {
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Avatar {
            image: img(src),
            status_indicator: None,
            border_color: None,
            size: None,
        }
    }

    /// Sets the shape of the avatar image.
    ///
    /// This method allows the shape of the avatar to be specified using a [`Shape`].
    /// It modifies the corner radius of the image to match the specified shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::{Avatar, AvatarShape};
    ///
    /// Avatar::new("path/to/image.png").shape(AvatarShape::Circle);
    /// ```
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.image = match shape {
            AvatarShape::Circle => self.image.rounded_full(),
            AvatarShape::RoundedRectangle => self.image.rounded_md(),
        };
        self
    }

    /// Applies a grayscale filter to the avatar image.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::{Avatar, AvatarShape};
    ///
    /// let avatar = Avatar::new("path/to/image.png").grayscale(true);
    /// ```
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.image = self.image.grayscale(grayscale);
        self
    }

    pub fn border_color(mut self, color: impl Into<Hsla>) -> Self {
        self.border_color = Some(color.into());
        self
    }

    /// A Some(true) indicates a green dot, and a Some(false) indicates a red dot.
    pub fn status_indicator(mut self, is_available: impl Into<Option<bool>>) -> Self {
        self.status_indicator = is_available.into();
        self
    }

    /// Size overrides the avatar size. By default they are 1rem.
    pub fn size(mut self, size: impl Into<Option<Pixels>>) -> Self {
        self.size = size.into();
        self
    }
}
