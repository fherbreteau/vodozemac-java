package io.github.fherbreteau.vodozemac.types;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.SignatureException;
import org.junit.jupiter.api.Test;

class TypesTest {

    @Test
    void testInvalidCurve25519PublicKeyLoading() {
        assertThatThrownBy(() -> Curve25519PublicKey.fromBase64("invalid"))
                .as("Loading an invalid Curve25519 public key throw KeyException")
                .isInstanceOf(KeyException.class)
                .hasMessageContaining("Failed to decode Curve25519 key from Base64");
    }

    @Test
    void testInvalidEd25519PublicKeyLoading() {
        assertThatThrownBy(() -> Ed25519PublicKey.fromBase64("invalid"))
                .as("Loading an invalid Ed25519 public key throw KeyException")
                .isInstanceOf(KeyException.class)
                .hasMessageContaining("Failed to decode Ed25519 key from Base64");
    }

    @Test
    void testInvalidEd25519SignatureLoading() {
        assertThatThrownBy(() -> Ed25519Signature.fromBase64("invalid"))
                .as("Loading an invalid Ed25519 signature throw SignatureException")
                .isInstanceOf(SignatureException.class)
                .hasMessageContaining("The signature couldn't be decoded");
    }

    @Test
    void testCurve25519PublicKeyEqualsHashCodeToString() {
        Curve25519PublicKey key = Curve25519PublicKey.fromBase64("WHKTK+K7GSjf83JuPfGV0KAZjxQU/3HKOb0DD1MaOm4");
        Curve25519PublicKey same = Curve25519PublicKey.fromBase64("WHKTK+K7GSjf83JuPfGV0KAZjxQU/3HKOb0DD1MaOm4");
        Curve25519PublicKey different = Curve25519PublicKey.fromBase64("dMjtgG+vJgd7P7zpZSDH/sJLcqu87gISGA7d/brNfmQ");

        assertThat(key).isEqualTo(key)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo("not Curve25519PublicKey")
                .isNotEqualTo(null)
                .hasToString("WHKTK+K7GSjf83JuPfGV0KAZjxQU/3HKOb0DD1MaOm4");
    }

    @Test
    void testEd25519PublicKeyEqualsHashCodeToString() {
        Ed25519PublicKey key = Ed25519PublicKey.fromBase64("NnTo+WL1n6ZjGN1EdHKtrYMRKAlrNUlxrZLtX0hDkbs");
        Ed25519PublicKey same = Ed25519PublicKey.fromBase64("NnTo+WL1n6ZjGN1EdHKtrYMRKAlrNUlxrZLtX0hDkbs");
        Ed25519PublicKey different = Ed25519PublicKey.fromBase64("Zv95Ka4ThW9hogCkG0MLTI+0+i9K6qS/S3tdfuWsMI4");

        assertThat(key).isEqualTo(key)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo("not Ed25519PublicKey")
                .isNotEqualTo(null)
                .hasToString("NnTo+WL1n6ZjGN1EdHKtrYMRKAlrNUlxrZLtX0hDkbs");
    }

    @Test
    void testEd25519SignatureEqualsHashCodeToString() {
        Ed25519Signature signature = Ed25519Signature.fromBase64("SucffO/oXYCEPa2lSLPiutmbbN+F3fKMd4Bps8ONOQJ/QjjwlpuXL/ag0kfa9vC0LeH0b+Y7/Qy+83jpExuUCQ");
        Ed25519Signature same = Ed25519Signature.fromBase64("SucffO/oXYCEPa2lSLPiutmbbN+F3fKMd4Bps8ONOQJ/QjjwlpuXL/ag0kfa9vC0LeH0b+Y7/Qy+83jpExuUCQ");
        Ed25519Signature different = Ed25519Signature.fromBase64("tmKC0y1NtWISC0OnUgGwBNqCGuyD3FmK+3dnA/143ijpI6ivPMU7AD+12fCwKszIbiPLcDz331eFjKvSzRsyAQ");

        assertThat(signature).isEqualTo(signature)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo("not Ed25519Signature")
                .isNotEqualTo(null)
                .hasToString("SucffO/oXYCEPa2lSLPiutmbbN+F3fKMd4Bps8ONOQJ/QjjwlpuXL/ag0kfa9vC0LeH0b+Y7/Qy+83jpExuUCQ");

    }

    @Test
    void testSignatureVerificationFailure() {
        Ed25519PublicKey key = Ed25519PublicKey.fromBase64("QNs9mlmds2G9ufOB1RC+u+gSa2OVdLeuDxJxHrYyKAQ");
        Ed25519Signature signature = Ed25519Signature.fromBase64("SucffO/oXYCEPa2lSLPiutmbbN+F3fKMd4Bps8ONOQJ/QjjwlpuXL/ag0kfa9vC0LeH0b+Y7/Qy+83jpExuUCQ");

        assertThat(key.verify("invalid", signature)).isFalse();

    }
}
