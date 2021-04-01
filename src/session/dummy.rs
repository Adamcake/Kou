use crate::{error::Error, session, source::{self, ChannelCount, SampleRate, Source}};

pub struct Device;
pub struct OutputStream;
pub struct Session;

impl Device {
    pub fn channel_count(&self) -> ChannelCount {
        source::consts::CH_STEREO
    }

    pub fn sample_rate(&self) -> SampleRate {
        source::consts::SR_48000
    }
}

impl Session {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }

    pub fn default_output_device(&self) -> Option<session::Device> {
        session_wrap!(Some(Device), Device(DeviceImpl), Dummy)
    }

    pub fn open_output_stream(
        &self,
        _device: session::Device,
        _source: impl Source + Send + 'static,
    ) -> Result<session::OutputStream, Error> {
        session_wrap!(Ok(OutputStream), OutputStream(OutputStreamImpl), Dummy)
    }
}
