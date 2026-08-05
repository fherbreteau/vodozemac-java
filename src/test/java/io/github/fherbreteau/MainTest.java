package io.github.fherbreteau;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.ByteArrayOutputStream;
import java.io.PrintStream;

import static org.assertj.core.api.Assertions.assertThat;

class MainTest {

    private PrintStream originalOut;
    private ByteArrayOutputStream capturedOut;

    @BeforeEach
    void captureSystemOut() {
        originalOut = System.out;
        capturedOut = new ByteArrayOutputStream();
        System.setOut(new PrintStream(capturedOut));
    }

    @AfterEach
    void restoreSystemOut() {
        System.setOut(originalOut);
    }

    @Test
    void testMain() {
        Main.main();

        String output = capturedOut.toString();
        assertThat(output)
                .as("Main should print Curve25519, Ed25519 and Signature lines")
                .contains("Curve25519:")
                .contains("Ed25519:")
                .contains("Signature:");
    }
}
