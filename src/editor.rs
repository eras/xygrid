use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;
use std::time::Duration;
use baseview::WindowScalePolicy;

use crate::XygridParams;


// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(1500, 1000)
}

pub(crate) fn create(
    params: Arc<XygridParams>,
    // peak_meter: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<XygridEditor>(editor_state
				     , (params//, ()
				     ))
}

struct XygridEditor {
    params: Arc<XygridParams>,
    context: Arc<dyn GuiContext>,

    // peak_meter: Arc<AtomicF32>,

    // gain_slider_state: nih_widgets::param_slider::State,
    // peak_meter_state: nih_widgets::peak_meter::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for XygridEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<XygridParams>//, Arc<AtomicF32>
    );

    fn new(
        (params// , peak_meter
	): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = XygridEditor {
            params,
            context,

            // peak_meter,

            // gain_slider_state: Default::default(),
            // peak_meter_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn scale_policy(&self) -> WindowScalePolicy {
	WindowScalePolicy::ScaleFactor(2.0)
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("Xygrid GUI")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(
                Text::new("Xygrid")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            // .push(
            //     nih_widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
            //         .map(Message::ParamUpdate),
            // )
            .push(Space::with_height(10.into()))
            // .push(
            //     nih_widgets::PeakMeter::new(
            //         &mut self.peak_meter_state,
            //         util::gain_to_db(self.peak_meter.load(std::sync::atomic::Ordering::Relaxed)),
            //     )
            //     .hold_time(Duration::from_millis(600)),
            // )
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}
