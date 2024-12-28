// @ts-check

import { decodeBase64URIToUint8Array } from "./conv.js";

/** @type {AbortController | null} */
let autofillAbortController = null;

/**
 * @returns {Promise<CredentialRequestOptions>}
 */
async function createCredentailsGetOptions() {
  return {
    publicKey: {
      challenge: createChallenge(),
      rpId: getRelyingPartyID(),
    },
  };
}

/**
 * @returns {Promise<CredentialCreationOptions>}
 */
async function createCredentialsCreateOptions() {
  /** @type {CredentialCreationOptions} */
  const param = {
    publicKey: {
      challenge: createChallenge(),
      authenticatorSelection: {
        authenticatorAttachment: "platform",
        requireResidentKey: true,
        residentKey: "required",
        userVerification: "required",
      },
      pubKeyCredParams: [
        {
          type: "public-key",
          alg: -8, // Ed25519
        },
        {
          type: "public-key",
          alg: -7, // ES256
        },
        {
          type: "public-key",
          alg: -257, // RS256
        },
      ],
      rp: {
        id: getRelyingPartyID(),
        name: "demo.accounts.comame.xyz",
      },
      user: {
        id: createUserID(),
        name: "Test User",
        displayName: "Test User",
      },
    },
  };

  const existID = getSavedKeyID();
  if (param.publicKey && existID) {
    param.publicKey.excludeCredentials = [
      {
        type: "public-key",
        id: existID,
      },
    ];
  }

  return param;
}

/**
 * @returns {string}
 */
function getRelyingPartyID() {
  return location.hostname;
}

/**
 * @returns {Uint8Array}
 */
function createChallenge() {
  return new Uint8Array([0, 1, 2, 3, 4, 5]);
}

/**
 * @returns {Uint8Array}
 */
function createUserID() {
  return new Uint8Array([6, 7, 8, 9, 10]);
}

async function setupPasskeyAutofill() {
  if (!(await PublicKeyCredential.isConditionalMediationAvailable())) {
    outputToLog("待ち受けようとしたが、Autofill に対応していない");
    return;
  }

  outputToLog("Autofill を待ち受けている...");

  const abort = new AbortController();
  autofillAbortController = abort;

  const options = await createCredentailsGetOptions();
  options.mediation = "conditional";
  options.signal = abort.signal;

  let res = null;
  try {
    res = await navigator.credentials.get(options);
  } catch (err) {
    if (abort.signal.aborted) {
      outputToLog("Autofill の待ち受けがキャンセルされた");
      return;
    }

    if (err instanceof Error) {
      outputToLog(err.name + ": " + err.message);
      setupPasskeyAutofill();
      return;
    }
    outputToLog("error");
    setupPasskeyAutofill();
    return;
  }
  outputToLog("mediaiton:conditional の credentials.get が解決した");
  if (res === null) {
    outputToLog("キーペアがない");
    setupPasskeyAutofill();
    return;
  }
  if (!(res instanceof PublicKeyCredential)) {
    outputToLog("PublicKeyCredential ではない値が返された");
    setupPasskeyAutofill();
    return;
  }

  console.log(res);
  outputToLog("ログインできた TODO: ユーザーIDを取る");
  setupPasskeyAutofill();
}

/** @type {HTMLButtonElement} */
// @ts-expect-error
const signinPasskeyButton = document.getElementById("signin-passkey");
signinPasskeyButton.addEventListener("click", async () => {
  autofillAbortController?.abort();
  const params = await createCredentailsGetOptions();
  let res = null;
  try {
    res = await navigator.credentials.get(params);
  } catch (err) {
    if (err instanceof Error) {
      outputToLog(err.name + ": " + err.message);
      setupPasskeyAutofill();
      return;
    }
    outputToLog("error");
    setupPasskeyAutofill();
    return;
  }
  if (res === null) {
    outputToLog("キーペアがない");
    setupPasskeyAutofill();
    return;
  }
  if (!(res instanceof PublicKeyCredential)) {
    outputToLog("PublicKeyCredential ではない値が返された");
    setupPasskeyAutofill();
    return;
  }

  console.log(res);
  outputToLog("ログインできた TODO: ユーザーIDを取る");
  setupPasskeyAutofill();
});

/** @type {HTMLButtonElement} */
// @ts-expect-error
const registerPasskeyButton = document.getElementById("register-passkey");
registerPasskeyButton.addEventListener("click", async () => {
  autofillAbortController?.abort();
  const options = await createCredentialsCreateOptions();
  let res = null;
  try {
    res = await navigator.credentials.create(options);
  } catch (err) {
    if (err instanceof Error) {
      outputToLog(err.name + ": " + err.message);
      setupPasskeyAutofill();
      return;
    }
    outputToLog("error");
    setupPasskeyAutofill();
    return;
  }

  if (res === null) {
    outputToLog("値が空");
    setupPasskeyAutofill();
    return;
  }
  if (!(res instanceof PublicKeyCredential)) {
    outputToLog("PublicKeyCredential ではない値が返された");
    setupPasskeyAutofill();
    return;
  }

  const keyID = res.id;
  saveKey(keyID);

  console.log(res);
  outputToLog(`キーペアが作成された ${keyID}`);

  setupPasskeyAutofill();
});

/**
 * @param {string} id
 */
function saveKey(id) {
  localStorage.setItem("webauthnsamplekey", id);
}

/**
 * @returns {Uint8Array|null}
 */
function getSavedKeyID() {
  const saved = localStorage.getItem("webauthnsamplekey");
  if (!saved) {
    return null;
  }

  return decodeBase64URIToUint8Array(saved);
}

/**
 * @param {string} msg
 */
function outputToLog(msg) {
  const elem = document.getElementById("events");
  if (!elem) {
    return;
  }
  elem.textContent += msg + "\n\n";
}

async function checkPasskeyCapabilities() {
  const isCMA = await PublicKeyCredential.isConditionalMediationAvailable();
  const isUVPAA =
    await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();

  outputToLog(`isCMA: ${isCMA ? "true" : "false"}`);
  outputToLog(`isUVPAA: ${isUVPAA ? "true" : "false"}`);
}

checkPasskeyCapabilities();

setupPasskeyAutofill();
