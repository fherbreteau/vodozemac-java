package io.github.fherbreteau.vodozemac.examples;

import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.types.Ed25519KeyPair;
import io.github.fherbreteau.vodozemac.types.Ed25519PublicKey;
import io.github.fherbreteau.vodozemac.types.Ed25519Signature;
import mjson.Json;

public final class SampleCrossSigning {

    private static final String USER_ID = "@alice:domain.com";
    private static final String DEVICE_ID = "JLAFKJWSCS";

    private static final String SIGNATURES = "signatures";
    private static final String ED25519_KEYID = "ed25519:%s";
    private static final String CURVE25519_KEYID = "curve25519:%s";

    private SampleCrossSigning() {
    }

    @SuppressWarnings("java:S106")
    public static void main(String[] args) {
        try (Ed25519KeyPair masterKeyPair = new Ed25519KeyPair();
            Ed25519KeyPair userSigningKeyPair = new Ed25519KeyPair();
            Ed25519KeyPair selfSigningKeyPair = new Ed25519KeyPair();
            Account account = new Account()) {

            Json masterKey = buildMatrixKey(masterKeyPair.publicKey(), "master");
            System.out.println("MSK pickle: " + masterKeyPair.pickle());
            Json userSigningKey = buildMatrixKey(userSigningKeyPair.publicKey(), "user_signing");
            System.out.println("USK pickle: " + userSigningKeyPair.pickle());
            Json selfSigningKey = buildMatrixKey(selfSigningKeyPair.publicKey(), "self_signing");
            System.out.println("SSK pickle: " + selfSigningKeyPair.pickle());

            System.out.println("Device pickle: " + account.pickle());

            String keyId = buildKeyId(masterKeyPair.publicKey());

            Ed25519Signature selfSigningKeySignature = masterKeyPair.sign(selfSigningKey.toString());
            Json signedSelfSigningKey = addSignature(selfSigningKey, keyId, selfSigningKeySignature);
            Ed25519Signature userSigningKeySignature = masterKeyPair.sign(userSigningKey.toString());
            Json signedUserSigningKey = addSignature(userSigningKey, keyId, userSigningKeySignature);

            Json device = buildMatrixDevice(account);

            String sSKKeyId = buildKeyId(selfSigningKeyPair.publicKey());
            Ed25519Signature accountSSKSignature = selfSigningKeyPair.sign(device.toString());

            String deviceKeyId = String.format(ED25519_KEYID, DEVICE_ID);
            Ed25519Signature accountDeviceSignature = account.sign(device.toString());
            Json signedByAccount = addSignature(device, deviceKeyId, accountDeviceSignature);
            Json signedDevice = addSignature(signedByAccount, sSKKeyId, accountSSKSignature);

            Ed25519Signature masterKeySignature = account.sign(masterKey.toString());
            Json signedMasterKey = addSignature(masterKey, deviceKeyId, masterKeySignature);

            Json crossSigningKeys = Json.object()
                .set("master_key", masterKey)
                .set("self_signing_key", signedSelfSigningKey)
                .set("user_signing_key", signedUserSigningKey);

            String toUpload = crossSigningKeys.toString();
            System.out.println("Uploaded Cross-Signing Keys: " + toUpload);

            Json signatures = Json.object()
                .set(USER_ID, Json.object()
                    .set(DEVICE_ID, signedDevice)
                    .set(masterKeyPair.publicKey().toBase64(), signedMasterKey));

            String signaturesToUpload = signatures.toString();
            System.out.println("Uploaded Signature: " + signaturesToUpload);
        }
    }

    private static Json buildMatrixKey(Ed25519PublicKey publicKey, String usage) {
        String publicKeyBase64 = publicKey.toBase64();
        String keyId = buildKeyId(publicKey);
        return Json.object()
            .set("keys", Json.object().set(keyId, publicKeyBase64))
            .set("usage", Json.array().add(usage))
            .set("user_id", USER_ID);
    }

    private static Json buildMatrixDevice(Account account) {
        String identityKey = account.curve25519Key().toBase64();
        String fingerprintKey = account.ed25519Key().toBase64();
        String identityKeyId = String.format(CURVE25519_KEYID, DEVICE_ID);
        String fingerprintKeyId = String.format(ED25519_KEYID, DEVICE_ID);

        return  Json.object()
            .set("algorithms", Json.array()
                .add("m.olm.v1.curve25519-aes-sha2")
                .add("m.megolm.v1.aes-sha2"))
            .set("device_id", DEVICE_ID)
            .set("keys", Json.object()
                .set(identityKeyId, identityKey)
                .set(fingerprintKeyId, fingerprintKey))
            .set("user_id", USER_ID);
    }

    private static Json addSignature(Json unsigned, String signatureId, Ed25519Signature signature) {
        Json signatures;
        if (unsigned.has(SIGNATURES)) {
            signatures = unsigned.at(SIGNATURES).at(USER_ID);
        } else {
            signatures = Json.object();
            Json container = Json.object()
                .set(USER_ID, signatures);
            unsigned.set(SIGNATURES, container);
        }
        signatures.set(signatureId, signature.toBase64());

        return unsigned;
    }

    private static String buildKeyId(Ed25519PublicKey publicKey) {
        return String.format(ED25519_KEYID, publicKey.toBase64());
    }
}
