use std::time::Duration;

use common::server_to_client::{Event, EventKind};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn prepare_synth(instruments: Vec<JsString>) -> JsValue;
    fn note_on(synth: &JsValue, channel: u8, note: u8, velocity: u8);
    fn note_off(synth: &JsValue, channel: u8, note: u8, velocity: u8);
}

pub struct MidiPlayer {
    synth: JsValue,
    events: Vec<Event>,
}

impl MidiPlayer {
    pub fn new(instruments: &[String]) -> Self {
        let instruments = instruments
            .iter()
            .cloned()
            .map(JsString::from)
            .collect::<Vec<_>>();

        Self {
            synth: prepare_synth(instruments),
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn play(&mut self) {
        for event in &self.events {
            std::thread::sleep(Duration::from_secs_f32(event.delta));

            match event.kind {
                EventKind::NoteOn {
                    channel,
                    key,
                    velocity,
                } => {
                    note_on(&self.synth, channel, key, velocity);
                }
                EventKind::NoteOff {
                    channel,
                    key,
                    velocity,
                } => {
                    note_off(&self.synth, channel, key, velocity);
                }
            }
        }
    }
}
