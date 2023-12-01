# vad-jni-rust
Simple JNI wrapper for the [webrtc-vad](https://github.com/kaegi/webrtc-vad) using [jni-rs](https://github.com/jni-rs/jni-rs).

### Adding dependency to the project
<img alt="version" src="https://img.shields.io/badge/dynamic/xml?label=%20&query=/metadata/versioning/versions/version[not(contains(text(),'%2B'))][last()]&url=https://repo.plasmoverse.com/releases/com/plasmoverse/vad-jni-rust/maven-metadata.xml">

```kotlin
repositories {
    maven("https://repo.plasmoverse.com/releases")
}

dependencies {
    implementation("com.plasmoverse:vad-jni-rust:$version")
}
```

### Usage
```java
// Creates a new VAD instance
Vad vad = Vad.create(48_000, VadMode.QUALITY);

// Checks if provided frame is voice
boolean isVoiceSegment = vad.isVoiceSegment(frame);

// Closes the VAD, releasing allocated resources
vad.close();
```
