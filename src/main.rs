use nih_plug::prelude::*;

use rust_clap_test::Gain;

fn main() {
    nih_export_standalone::<Gain>();
}
