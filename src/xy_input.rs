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

    x_param: &'a P,
    y_param: &'a P,

    height: Length,
    width: Length,
}

#[derive(Default)]
pub struct State {
    drag_active: bool,
}

impl<'a, P: Param> XyInput<'a, P> {
    pub fn new(state: &'a mut State, x_param: &'a P, y_param: &'a P) -> Self {
        Self {
            state,

            x_param,
            y_param,

            width: Length::Units(100),
            height: Length::Units(100),
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

impl<'a, P: Param> XyInput<'a, P> {
    fn set_normalized_value(&self,
			    shell: &mut Shell<'_, ParamMessage>,
			    normalized_x_value: f32,
			    normalized_y_value: f32) {
        let plain_x_value = self.x_param.preview_plain(normalized_x_value);
        let plain_y_value = self.y_param.preview_plain(normalized_y_value);
        let current_plain_x_value = self.x_param.modulated_plain_value();
        let current_plain_y_value = self.y_param.modulated_plain_value();
        if plain_x_value != current_plain_x_value || plain_y_value != current_plain_y_value {
	    shell.publish(ParamMessage::SetParameterNormalized(
                self.x_param.as_ptr(),
                normalized_x_value,
	    ));
	    shell.publish(ParamMessage::SetParameterNormalized(
                self.y_param.as_ptr(),
                normalized_y_value,
	    ));
        }
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

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, ParamMessage>,
    ) -> event::Status {
	let bounds = layout.bounds();
	let x = (cursor_position.x - bounds.x) / bounds.width;
	let y = (cursor_position.y - bounds.y) / bounds.height;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
		| Event::Touch(touch::Event::FingerPressed { .. }) if bounds.contains(cursor_position) => {
		    shell.publish(ParamMessage::BeginSetParameter(self.x_param.as_ptr()));
		    shell.publish(ParamMessage::BeginSetParameter(self.y_param.as_ptr()));
		    self.set_normalized_value(shell, x, y);
		    self.state.drag_active = true;
		    event::Status::Captured
		}
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
		| Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) if self.state.drag_active => {
		    shell.publish(ParamMessage::EndSetParameter(self.x_param.as_ptr()));
		    shell.publish(ParamMessage::EndSetParameter(self.y_param.as_ptr()));

                    self.state.drag_active = false;

                    event::Status::Captured
		}
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if self.state.drag_active {
		    self.set_normalized_value(shell, x, y);
                    event::Status::Captured
		} else {
                    event::Status::Ignored
		}
	    }
	    _ => event::Status::Ignored
	}
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
                border_color: Color::WHITE,
                border_width: 5.0,
                border_radius: 10.0,
            },
            Color::BLACK,
        );

	let x_in_bounds = bounds.x + self.x_param.unmodulated_normalized_value() * bounds.width;
	let y_in_bounds = bounds.y + self.y_param.unmodulated_normalized_value() * bounds.height;

	let xy_bounds = Rectangle::new(Point::new(x_in_bounds, y_in_bounds),
				       Size { width: 10.0, height: 10.0 });

        renderer.fill_quad(
            renderer::Quad {
                bounds: xy_bounds,
                border_color: Color::WHITE,
                border_width: 5.0,
                border_radius: 5.0,
            },
            Color::from_rgb(0.8, 0.8, 0.8),
        );

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
