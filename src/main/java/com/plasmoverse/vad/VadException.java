package com.plasmoverse.vad;

/**
 * Exception indicates issues related to VAD.
 */
public class VadException extends Exception {

    public VadException() {
        super();
    }

    public VadException(String message) {
        super(message);
    }

    public VadException(String message, Throwable cause) {
        super(message, cause);
    }

    public VadException(Throwable cause) {
        super(cause);
    }
}
