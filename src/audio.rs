use graphics::Context;
use openal::al;
use openal::alc;
use opengl_graphics::GlGraphics;
use sndfile::{SndFile, OpenMode};

use world;

const READ_BUFFER_SIZE: usize = 4096;

impl world::AudioSource {

    pub fn new() -> world::AudioSource {
        world::AudioSource {
            source: al::Source::gen(),
            buffer: al::Buffer::gen(),
        }
    }

    pub fn load_file(&mut self, path: &str) {
        load_buffer(path, &mut self.buffer);
    }

    pub fn play(&mut self) {
        self.source.set_buffer(self.buffer.clone());
        self.source.play();
    }

    pub fn play_buffer(&mut self, buffer: al::Buffer) {
        self.source.stop();
        self.source.set_buffer(buffer.clone());
        self.source.play();
    }
}

// TODO - figure out how to get the AudioSystem to own device/context and cleanup properly..
// use Drop trait?

pub struct AudioSystem
{
    device: alc::Device,
    context: alc::Context,
}

impl AudioSystem {
    pub fn new() -> AudioSystem {

        let device = alc::Device::open(None).expect("Could not open device");
        let context = device.create_context(&[]).expect("Could not create context");
        context.make_current();

        AudioSystem {
            device: device,
            context: context,
        }
    }
}

impl world::System for AudioSystem {
    fn update(&mut self, control_state: &world::ControlState, components: &mut world::Components, entities: &mut Vec<world::Entity>) { }
    fn render(&mut self, context: &Context, gl: &mut GlGraphics, components: &mut world::Components, entities: &mut Vec<world::Entity>) { }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        self.device.close().ok().expect("Unable to close device");
    }
}

pub fn load_buffer(path: &str, buffer: &mut al::Buffer) -> f32 {

  let mut sound_file = SndFile::new(path, OpenMode::Read).unwrap();

  let sndinfo = sound_file.get_sndinfo();
  let duration = sndinfo.frames as f32 / sndinfo.samplerate as f32;

  let mut samples: Vec<i16> = Vec::new();

  let mut read_buf: [i16; READ_BUFFER_SIZE] = [0; READ_BUFFER_SIZE];
  let mut samples_read = 0;

  loop {
      let samples_read = sound_file.read_i16(&mut read_buf[..], READ_BUFFER_SIZE as i64);
      if samples_read == 0 { break; }
      samples.push_all(read_buf.slice(0, samples_read as usize));
  }

  let format = if sndinfo.channels == 1 { al::Format::Mono16 } else { al::Format::Stereo16 };

  unsafe { buffer.buffer_data(format, &samples, sndinfo.samplerate as al::ALsizei) };

  duration

}
