plugins {
    kotlin("jvm") version "1.3.72"
}

group = "com.igor"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("org.kodein.di:kodein-di:7.1.0")
}
