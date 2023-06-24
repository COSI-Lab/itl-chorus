function play() {
    JZZ.synth.MIDIjs({ soundfontUrl: "/sf/", instrument: "acoustic_grand_piano" })
        .note(0, 'C5', 127, 500).wait(500)
        .note(0, 'E5', 127, 500).wait(500)
        .note(0, 'G5', 127, 500).wait(500)
        .note(0, 'C6', 127, 500);
}