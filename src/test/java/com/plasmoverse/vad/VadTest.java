package com.plasmoverse.vad;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;

public final class VadTest {

    @Test
    public void testFrame() throws Exception {
        short[] frame = new short[960];

        // Creates a new VAD instance
        Vad vad = Vad.create(48_000, VadMode.QUALITY);

        // Checks if provided frame is voice
        assertFalse(vad.isVoiceSegment(frame));

        vad.reset();

        // Closes the VAD, releasing allocated resources
        vad.close();
    }

    @Test
    public void testBadFrame() throws Exception {
        short[] frame = new short[444];
        Vad vad = Vad.create(48_000, VadMode.QUALITY);

        assertThrows(VadException.class, () -> vad.isVoiceSegment(frame));

        vad.close();
    }
}
