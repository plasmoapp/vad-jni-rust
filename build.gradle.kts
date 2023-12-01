plugins {
    java
    `maven-publish`
}

repositories {
    mavenCentral()
}

dependencies {
    testCompileOnly(libs.junit.api)
    testAnnotationProcessor(libs.junit.api)
    testRuntimeOnly(libs.junit.engine)
}

tasks {
    java {
        withSourcesJar()

        toolchain.languageVersion.set(JavaLanguageVersion.of(8))
    }

    test {
        useJUnitPlatform()
    }
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])
        }
    }

    repositories {
        maven("https://repo.plasmoverse.com/releases") {
            name = "PlasmoVerseReleases"

            credentials {
                username = System.getenv("MAVEN_USERNAME")
                password = System.getenv("MAVEN_PASSWORD")
            }
        }
    }
}
