// @ts-check

/**
 * @template T
 * @param {(type: T) => any} def
 * @param {unknown} from
 * @returns {T}
 */
function as(def, from) {
  // @ts-expect-error
  return from;
}

/**
 * @param {string} base64url
 * @returns {Uint8Array}
 */
export function decodeBase64URL(base64url) {
  const padding = "=".repeat((4 - (base64url.length % 4)) % 4);
  const base64 = (base64url + padding).replace(/\-/g, "+").replace(/_/g, "/");

  // Base64 デコード
  const rawData = atob(base64);

  // Uint8Array に変換
  const outputArray = new Uint8Array(rawData.length);
  for (let i = 0; i < rawData.length; ++i) {
    outputArray[i] = rawData.charCodeAt(i);
  }
  return outputArray;
}

/**
 * @param {ArrayBuffer} source
 * @returns {string}
 */
export function encodeToBase64URL(source) {
  const arr = new Uint8Array(source);
  const base64 = btoa(String.fromCharCode(...arr));
  return base64.replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

/**
 * Safari では toJSON が生えていないので Polyfill
 *
 * @param {PublicKeyCredential} cred
 * @returns {PublicKeyCredentialJSON}
 */
export function publicKeyCredentialToJSON(cred) {
  if ("toJSON" in cred) {
    return cred.toJSON();
  }

  cred = as(/** @param {PublicKeyCredential} _ */ (_) => _, cred);

  const j = {
    type: cred.type,
    id: cred.id,
    rawId: encodeToBase64URL(cred.rawId),
  };

  switch (true) {
    case cred.response instanceof AuthenticatorAttestationResponse:
      j["response"] = attestationToJSON(cred.response);
      break;
    case cred.response instanceof AuthenticatorAssertionResponse:
      j["response"] = assertionToJSON(cred.response);
      break;
    default:
      console.error(cred);
      throw new RangeError(
        "publicKeyCredential.responseの型が想定しないものになっている"
      );
  }

  return j;
}

/**
 * @param {AuthenticatorAttestationResponse} attestation
 * @returns {any}
 */
function attestationToJSON(attestation) {
  const j = {
    clientDataJSON: encodeToBase64URL(attestation.clientDataJSON),
    publicKeyAlgorithm: attestation.getPublicKeyAlgorithm(),
    transports: attestation.getTransports(),
  };

  const pubKey = attestation.getPublicKey();
  if (pubKey) {
    j["publicKey"] = encodeToBase64URL(pubKey);
  }

  return j;
}

/**
 * @param {AuthenticatorAssertionResponse} assertion
 * @returns {any}
 */
function assertionToJSON(assertion) {
  const j = {
    authenticatorData: encodeToBase64URL(assertion.authenticatorData),
    clientDataJSON: encodeToBase64URL(assertion.clientDataJSON),
    signature: encodeToBase64URL(assertion.signature),
  };

  if (assertion.userHandle) {
    j["userHandle"] = encodeToBase64URL(assertion.userHandle);
  }

  return j;
}
