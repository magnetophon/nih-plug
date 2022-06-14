use clap::{Parser, ValueEnum};

/// Configuration for a standalone plugin that would normally be provided by the DAW.
#[derive(Debug, Clone, Parser)]
#[clap(about = None, long_about = None)]
pub struct WrapperConfig {
    /// The audio and MIDI backend to use.
    ///
    /// The 'auto' option will try all backends in order, and falls back to the dummy backend with
    /// no audio input or output if the other backends are not available.
    #[clap(value_parser, short = 'b', long, default_value = "auto")]
    pub backend: BackendType,

    /// The number of input channels.
    #[clap(value_parser, short = 'i', long, default_value = "2")]
    pub input_channels: u32,
    /// The number of output channels.
    #[clap(value_parser, short = 'o', long, default_value = "2")]
    pub output_channels: u32,
    /// The audio backend's sample rate.
    ///
    /// This setting is ignored when using the JACK backend.
    #[clap(value_parser, short = 'r', long, default_value = "44100")]
    pub sample_rate: f32,
    /// The audio backend's period size.
    ///
    /// This setting is ignored when using the JACK backend.
    #[clap(value_parser, short = 'p', long, default_value = "512")]
    pub period_size: u32,

    /// The editor's DPI scaling factor.
    ///
    /// This option is ignored on macOS.
    //
    // Currently baseview has no way to report this to us, so we'll expose it as a command line
    // option instead.
    #[clap(value_parser, long, default_value = "1.0")]
    pub dpi_scale: f32,

    /// The transport's tempo.
    #[clap(value_parser, long, default_value = "120")]
    pub tempo: f32,
    /// The time signature's numerator.
    #[clap(value_parser, long, default_value = "4")]
    pub timesig_num: u32,
    /// The time signature's denominator.
    #[clap(value_parser, long, default_value = "4")]
    pub timesig_denom: u32,
}

/// Determines which audio and MIDI backend should be used.
#[derive(Debug, Clone, ValueEnum)]
pub enum BackendType {
    /// Automatically pick the backend depending on what's available.
    ///
    /// This defaults to JACK if JACK is available, and falls back to the dummy backend if not.
    Auto,
    /// Use JACK for audio and MIDI.
    Jack,
    /// Does not playback or receive any audio or MIDI.
    Dummmy,
}
