plugins {
    `java-library`
    `maven-publish`
    signing
}

group = "dev.caretly"
version = "0.1.0"
description = "Client for the Caretly API: render ZPL labels to PDF or PNG."

repositories {
    mavenCentral()
}

java {
    withSourcesJar()
    withJavadocJar()
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.11.4"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    testImplementation("org.junit.jupiter:junit-jupiter-params")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

tasks.withType<JavaCompile>().configureEach {
    options.release.set(11)
    options.encoding = "UTF-8"
    options.compilerArgs.addAll(listOf("-Xlint:all", "-Werror"))
}

tasks.compileTestJava {
    // Tests use the JDK's built-in HTTP server, which is not part of the library's module.
    options.compilerArgs.remove("-Werror")
}

tasks.javadoc {
    (options as StandardJavadocDocletOptions).apply {
        encoding = "UTF-8"
        links("https://docs.oracle.com/en/java/javase/11/docs/api/")
        addBooleanOption("Xdoclint:all", true)
        addBooleanOption("Werror", true)
    }
}

tasks.jar {
    manifest {
        attributes(
            "Implementation-Title" to "caretly",
            "Implementation-Version" to project.version,
        )
    }
}

tasks.test {
    useJUnitPlatform()
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            artifactId = "caretly"
            from(components["java"])
            pom {
                name.set("Caretly")
                description.set(project.description)
                url.set("https://caretly.dev")
                licenses {
                    license {
                        name.set("MIT License")
                        url.set("https://opensource.org/licenses/MIT")
                        distribution.set("repo")
                    }
                }
                developers {
                    developer {
                        id.set("jaapieaapie1")
                        name.set("Jaap Elst")
                        email.set("jaapelst@gmail.com")
                    }
                }
                scm {
                    url.set("https://github.com/jaapieaapie1/zpl")
                    connection.set("scm:git:https://github.com/jaapieaapie1/zpl.git")
                    developerConnection.set("scm:git:ssh://git@github.com/jaapieaapie1/zpl.git")
                }
                issueManagement {
                    system.set("GitHub")
                    url.set("https://github.com/jaapieaapie1/zpl/issues")
                }
            }
        }
    }
    repositories {
        // A local Maven layout that is zipped into a Central Portal bundle (see README).
        maven {
            name = "staging"
            url = uri(layout.buildDirectory.dir("staging-deploy"))
        }
    }
}

signing {
    // Keys come from `signingKey` / `signingPassword` Gradle properties (for example the
    // ORG_GRADLE_PROJECT_signingKey environment variable) or from the gpg `signing.*` properties.
    val signingKey = findProperty("signingKey") as String?
    val signingPassword = findProperty("signingPassword") as String?
    if (signingKey != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
    }
    // Only publishing to a repository needs signatures; builds and publishToMavenLocal do not.
    setRequired({ gradle.taskGraph.allTasks.any { it is PublishToMavenRepository } })
    sign(publishing.publications["maven"])
}

val cleanStaging by tasks.registering(Delete::class) {
    delete(layout.buildDirectory.dir("staging-deploy"))
}

tasks.named("publishMavenPublicationToStagingRepository") {
    dependsOn(cleanStaging)
}

val centralBundle by tasks.registering(Zip::class) {
    group = "publishing"
    description = "Zips the signed staging repository into a bundle for the Central Portal."
    dependsOn("publishMavenPublicationToStagingRepository")
    from(layout.buildDirectory.dir("staging-deploy")) {
        exclude("**/maven-metadata*")
    }
    archiveFileName.set("caretly-${project.version}-bundle.zip")
    destinationDirectory.set(layout.buildDirectory.dir("central"))
}
