use nih_plug_iced::{
    alignment, event, keyboard, layout, mouse, renderer, text, touch, Background, Clipboard, Color,
    Element, Event, Font, Layout, Length, Point, Rectangle, Shell, Size, TextInput, Vector, Widget,
};
use nih_plug_iced::backend::Renderer;
use nih_plug_iced::renderer::Renderer as GraphicsRenderer;
use nih_plug::prelude::Param;
use nih_plug_iced::widgets::ParamMessage;

pub struct XyInput<'a, P: Param> {
    state: &'a mut State,

    param: &'a P,

    height: Length,
    width: Length,
}

#[derive(Default)]
pub struct State {
    x: f32,
    y: f32,
}

pub enum Message {
    Position((f32, f32)),
}

impl<'a, P: Param> XyInput<'a, P> {
    pub fn new(state: &'a mut State, param: &'a P) -> Self {
        Self {
            state,

            param,

            width: Length::Units(180),
            height: Length::Units(30),
        }
    }

    /// Sets the width of the [`XyInput`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`XyInput`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<'a, P: Param> Widget<ParamMessage, Renderer> for XyInput<'a, P> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
	let bounds = layout.bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_color: Color::from_rgb(1.0, 0.0, 0.0),
                border_width: 5.0,
                border_radius: 3.0,
            },
            Color::from_rgb(0.0, 1.0, 0.0),
        );

	// {
    //     let bounds = layout.bounds();
    //     // I'm sure there's some philosophical meaning behind this
    //     let bounds_without_borders = Rectangle {
    //         x: bounds.x + BORDER_WIDTH,
    //         y: bounds.y + BORDER_WIDTH,
    //         width: bounds.width - (BORDER_WIDTH * 2.0),
    //         height: bounds.height - (BORDER_WIDTH * 2.0),
    //     };
    //     let is_mouse_over = bounds.contains(cursor_position);

    //     // The bar itself, show a different background color when the value is being edited or when
    //     // the mouse is hovering over it to indicate that it's interactive
    //     let background_color =
    //         if is_mouse_over || self.state.drag_active || self.state.text_input_value.is_some() {
    //             Color::new(0.5, 0.5, 0.5, 0.1)
    //         } else {
    //             Color::TRANSPARENT
    //         };

    //     renderer.fill_quad(
    //         renderer::Quad {
    //             bounds,
    //             border_color: Color::BLACK,
    //             border_width: BORDER_WIDTH,
    //             border_radius: 0.0,
    //         },
    //         background_color,
    //     );

    //     // Only draw the text input widget when it gets focussed. Otherwise, overlay the label with
    //     // the slider.
    //     if let Some(current_value) = &self.state.text_input_value {
    //         self.with_text_input(
    //             layout,
    //             renderer,
    //             current_value,
    //             |text_input, layout, renderer| {
    //                 text_input.draw(renderer, layout, cursor_position, None)
    //             },
    //         )
    //     } else {
    //         // We'll visualize the difference between the current value and the default value if the
    //         // default value lies somewhere in the middle and the parameter is continuous. Otherwise
    //         // this appraoch looks a bit jarring.
    //         let current_value = self.param.modulated_normalized_value();
    //         let default_value = self.param.default_normalized_value();
    //         let fill_start_x = util::remap_rect_x_t(
    //             &bounds_without_borders,
    //             if self.param.step_count().is_none() && (0.45..=0.55).contains(&default_value) {
    //                 default_value
    //             } else {
    //                 0.0
    //             },
    //         );
    //         let fill_end_x = util::remap_rect_x_t(&bounds_without_borders, current_value);

    //         let fill_color = Color::from_rgb8(196, 196, 196);
    //         let fill_rect = Rectangle {
    //             x: fill_start_x.min(fill_end_x),
    //             width: (fill_end_x - fill_start_x).abs(),
    //             ..bounds_without_borders
    //         };
    //         renderer.fill_quad(
    //             renderer::Quad {
    //                 bounds: fill_rect,
    //                 border_color: Color::TRANSPARENT,
    //                 border_width: 0.0,
    //                 border_radius: 0.0,
    //             },
    //             fill_color,
    //         );

    //         // To make it more readable (and because it looks cool), the parts that overlap with the
    //         // fill rect will be rendered in white while the rest will be rendered in black.
    //         let display_value = self.param.to_string();
    //         let text_size = self.text_size.unwrap_or_else(|| renderer.default_size()) as f32;
    //         let text_bounds = Rectangle {
    //             x: bounds.center_x(),
    //             y: bounds.center_y(),
    //             ..bounds
    //         };
    //         renderer.fill_text(text::Text {
    //             content: &display_value,
    //             font: self.font,
    //             size: text_size,
    //             bounds: text_bounds,
    //             color: style.text_color,
    //             horizontal_alignment: alignment::Horizontal::Center,
    //             vertical_alignment: alignment::Vertical::Center,
    //         });

    //         // This will clip to the filled area
    //         renderer.with_layer(fill_rect, |renderer| {
    //             let filled_text_color = Color::from_rgb8(80, 80, 80);
    //             renderer.fill_text(text::Text {
    //                 content: &display_value,
    //                 font: self.font,
    //                 size: text_size,
    //                 bounds: text_bounds,
    //                 color: filled_text_color,
    //                 horizontal_alignment: alignment::Horizontal::Center,
    //                 vertical_alignment: alignment::Vertical::Center,
    //             });
    //         });
    //     }
    // }
    }

}

impl<'a, P: Param> XyInput<'a, P> {
    pub fn map<Message, F>(self, f: F) -> Element<'a, Message>
    where
        Message: 'static,
        F: Fn(ParamMessage) -> Message + 'static,
    {
        Element::from(self).map(f)
    }
}

impl<'a, P: Param> From<XyInput<'a, P>> for Element<'a, ParamMessage> {
    fn from(widget: XyInput<'a, P>) -> Self {
        Element::new(widget)
    }
}
