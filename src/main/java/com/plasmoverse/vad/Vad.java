package com.plasmoverse.vad;

import java.io.IOException;

public final class Vad {

    /**
     * Creates a new VAD instance.
     *
     * @param sampleRate The sample rate for VAD instance. Valid values for the sample rate are 8000, 16000, 32000 and 48000.
     * @param mode The VAD mode.
     * @throws IOException If an error occurs while extracting the native library.
     * @throws UnsatisfiedLinkError If the native libraries fail to load.
     * @throws VadException If the VAD fail to initialize.
     * @return An instance of the VAD.
     */
    public static Vad create(int sampleRate, VadMode mode) throws IOException, VadException {
        VadLibrary.load();

        long pointer = createNative(sampleRate, mode);

        return new Vad(pointer);
    }

    private static native long createNative(int sampleRate, VadMode mode);


    private final long pointer;

    private Vad(long pointer) {
        this.pointer = pointer;
    }

    /**
     * Calculates a VAD decision for an audio frame.
     *
     * @param frame frame is an array of length signed 16-bit samples. Only frames with a length of 10, 20 or 30 ms are supported, so for example at 8 kHz, length must be either 80, 160 or 240.
     */
    public boolean isVoiceSegment(short[] frame) throws VadException {
        if (!isOpen()) throw new VadException("VAD is closed");

        return isVoiceSegmentNative(frame);
    }

    /**
     * Changes the VAD mode.
     *
     * @param mode The VAD mode to set.
     */
    public void setMode(VadMode mode) {
        if (!isOpen()) return;

        setModeNative(mode.ordinal());
    }

    /**
     * Changes the VAD sample rate.
     *
     * @param sampleRate The sample rate to set. Valid values for the sample rate are 8000, 16000, 32000 and 48000.
     */
    public void setSampleRate(int sampleRate) {
        if (!isOpen()) return;

        setSampleRateNative(sampleRate);
    }

    /**
     * Resets the VAD to its initial state.
     */
    public void reset() throws VadException {
        if (!isOpen()) return;

        resetNative();
    }

    /**
     * Closes the VAD, releasing any allocated resources.
     */
    public void close() {
        if (!isOpen()) return;

        closeNative();
    }

    /**
     * Checks if the VAD is currently open.
     *
     * @return {@code true} if the decoder is open, {@code false} otherwise.
     */
    public boolean isOpen() {
        return pointer > 0;
    }

    private native boolean isVoiceSegmentNative(short[] frame);

    private native void setModeNative(int mode);

    private native void setSampleRateNative(int sampleRate);

    private native void resetNative();

    private native void closeNative();
}
