function prepare_synth(instruments) {
    return JZZ.synth.MIDIjs({ soundfontUrl: "/sf/", instruments: instruments });
}

function note_on(synth, channel, note, velocity) {
    synth.noteOn(channel, note, velocity);
}

function note_off(synth, channel, note, velocity) {
    synth.noteOff(channel, note, velocity);
}