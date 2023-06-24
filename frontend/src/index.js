function play() {
    JZZ.synth.MIDIjs({ soundfontUrl: "https://gleitz.github.io/midi-js-soundfonts/FluidR3_GM/", instrument: "acoustic_grand_piano" })
        .note(0, 'C5', 127, 500).wait(500)
        .note(0, 'E5', 127, 500).wait(500)
        .note(0, 'G5', 127, 500).wait(500)
        .note(0, 'C6', 127, 500);
}