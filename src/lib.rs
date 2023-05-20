//use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;

mod editor;
mod xy_input;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
//const PEAK_METER_DECAY_MS: f64 = 150.0;

/// This is mostly identical to the gain example, minus some fluff, and with a GUI.
pub struct Xygrid {
    params: Arc<XygridParams>,

    // /// Needed to normalize the peak meter's response based on the sample rate.
    // // peak_meter_decay_weight: f32,
    // /// The current data for the peak meter. This is stored as an [`Arc`] so we can share it between
    // /// the GUI and the audio processing parts. If you have more state to share, then it's a good
    // /// idea to put all of that in a struct behind a single `Arc`.
    // ///
    // /// This is stored as voltage gain.
    // //peak_meter: Arc<AtomicF32>,
}

#[derive(Params)]
struct XygridParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
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

            // peak_meter_decay_weight: 1.0,
            // peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
        }
    }
}

impl Default for XygridParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

	    x: FloatParam::new("X", 0.0, FloatRange::Linear { min: -1.0, max: 1.0 }),

	    y: FloatParam::new("Y", 0.0, FloatRange::Linear { min: -1.0, max: 1.0 }),

            // See the main gain example for more details
            // gain: FloatParam::new(
            //     "Gain",
            //     util::db_to_gain(0.0),
            //     FloatRange::Skewed {
            //         min: util::db_to_gain(-30.0),
            //         max: util::db_to_gain(30.0),
            //         factor: FloatRange::gain_skew_factor(-30.0, 30.0),
            //     },
            // )
            // .with_smoother(SmoothingStyle::Logarithmic(50.0))
            // .with_unit(" dB")
            // .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            // .with_string_to_value(formatters::s2v_f32_gain_to_db()),
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
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        // self.peak_meter_decay_weight = 0.25f64
        //     .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
        //     as f32;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            // let gain = self.params.gain.smoothed.next();
            // for sample in channel_samples {
            //     *sample *= gain;
            //     amplitude += *sample;
            // }

            // // To save resources, a plugin can (and probably should!) only perform expensive
            // // calculations that are only displayed on the GUI while the GUI is open
            // if self.params.editor_state.is_open() {
            //     amplitude = (amplitude / num_samples as f32).abs();
            //     let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
            //     let new_peak_meter = if amplitude > current_peak_meter {
            //         amplitude
            //     } else {
            //         current_peak_meter * self.peak_meter_decay_weight
            //             + amplitude * (1.0 - self.peak_meter_decay_weight)
            //     };

            //     self.peak_meter
            //         .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
            // }
        }

	while let Some(event) = context.next_event() {
	    //context.send_event(event);
	}

	context.send_event(NoteEvent::MidiCC {
            timing: 0,
            channel: 1,
            cc: 70,
            value: self.params.x.modulated_plain_value(),
        });

	context.send_event(NoteEvent::MidiCC {
            timing: 0,
            channel: 1,
            cc: 71,
            value: self.params.y.modulated_plain_value(),
        });

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

// impl Vst3Plugin for Xygrid {
//     const VST3_CLASS_ID: [u8; 16] = *b"XygridAaAAaAaAaA";
//     const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
//         &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
// }

nih_export_clap!(Xygrid);
// nih_export_vst3!(Xygrid);
