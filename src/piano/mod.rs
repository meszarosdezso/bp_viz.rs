use csv::{Reader, StringRecord};
use rodio::source::SineWave;
use rodio::{dynamic_mixer, OutputStream, Sink, Source};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Note {
    pub name: String,
    pub freq: f32,
}

impl From<StringRecord> for Note {
    fn from(rec: StringRecord) -> Self {
        Self {
            name: String::from(rec.get(0).unwrap()),
            freq: rec.get(1).unwrap().parse().expect("Failed to parse note"),
        }
    }
}

impl Default for Note {
    fn default() -> Self {
        Self {
            name: String::from("A4"),
            freq: 440.,
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} ({}Hz)", self.name, self.freq))
    }
}

#[derive(Debug, Default)]
pub struct Piano {
    pub keys: HashMap<String, Note>,
}

impl Piano {
    pub fn new() -> Self {
        let mut raw = Reader::from_path("./data/notes.csv").expect("Couldn't read notes.csv");
        let mut keys = HashMap::new();

        while let Some(Ok(record)) = raw.records().next() {
            keys.insert(record.get(0).unwrap().to_string(), Note::from(record));
        }

        Self { keys }
    }

    pub fn press_keys(&self, keys: impl IntoIterator<Item = &'static str>, duration: Duration) {
        let (ctrl, mixer) = dynamic_mixer::mixer::<f32>(2, 44100);

        for note in keys.into_iter() {
            if let Some(note) = self.keys.get(note) {
                let source = SineWave::new(note.freq)
                    .take_duration(duration)
                    .amplify(0.24);

                ctrl.add(source);
            }
        }

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(mixer);
        sink.sleep_until_end();
        sink.detach();
    }
}
