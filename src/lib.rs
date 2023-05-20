//use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;

mod editor;
mod xy_input;

pub struct Xygrid {
    params: Arc<XygridParams>,

    prev_sent_x_value: Option<f32>,
    prev_sent_y_value: Option<f32>,
}

#[derive(Params)]
struct XygridParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    #[id = "x"]
    pub x: FloatParam,

    #[id = "y"]
    pub y: FloatParam,
}

impl Default for Xygrid {
    fn default() -> Self {
        Self {
            params: Arc::new(XygridParams::default()),

	    prev_sent_x_value: None,
	    prev_sent_y_value: None,
        }
    }
}

impl Default for XygridParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

	    x: FloatParam::new("X", 0.0, FloatRange::Linear { min: -1.0, max: 1.0 }),

	    y: FloatParam::new("Y", 0.0, FloatRange::Linear { min: -1.0, max: 1.0 }),
        }
    }
}

impl Plugin for Xygrid {
    const NAME: &'static str = "Xygrid GUI (iced)";
    const VENDOR: &'static str = "Moist Plugins GmbH";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        // AudioIOLayout {
        //     main_input_channels: NonZeroU32::new(2),
        //     main_output_channels: NonZeroU32::new(2),
        //     ..AudioIOLayout::const_default()
        // },
        // AudioIOLayout {
        //     main_input_channels: NonZeroU32::new(1),
        //     main_output_channels: NonZeroU32::new(1),
        //     ..AudioIOLayout::const_default()
        // },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            // self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
	while let Some(event) = context.next_event() {
	    context.send_event(event);
	}

	let value = self.params.x.modulated_normalized_value();
	if Some(value) != self.prev_sent_x_value {
	    context.send_event(NoteEvent::MidiCC {
		timing: 0,
		channel: 0,
		cc: 70,
		value
            });
	    self.prev_sent_x_value = Some(value);
	}

	let value = self.params.y.modulated_normalized_value();
	if Some(value) != self.prev_sent_y_value {
	    context.send_event(NoteEvent::MidiCC {
		timing: 0,
		channel: 0,
		cc: 71,
		value,
            });
	    self.prev_sent_y_value = Some(value);
	}

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Xygrid {
    const CLAP_ID: &'static str = "fi.variaattori.plugin.xygrid";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("An XY grid input plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

nih_export_clap!(Xygrid);
