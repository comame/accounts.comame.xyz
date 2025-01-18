// @ts-check

import { decodeBase64URIToUint8Array } from "./conv.js";

/** @type {AbortController | null} */
let autofillAbortController = null;

/**
 * @typedef {CredentialCreationOptions & { publicKey: { challenge_base64: string, user: { id_base64: string }, excludeCredentials: { id_base64: string }[] }}} createOptions
 */

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
  /** @type {createOptions} */
  const opt = await fetch("/demo/passkey/register-options", {
    method: "POST",
    credentials: "include",
  }).then((res) => res.json());

  opt.publicKey.challenge = decodeBase64URIToUint8Array(
    opt.publicKey.challenge_base64
  );
  opt.publicKey.user.id = decodeBase64URIToUint8Array(
    opt.publicKey.user.id_base64
  );
  if (opt.publicKey.excludeCredentials) {
    for (let i = 0; i < opt.publicKey.excludeCredentials.length; i += 1) {
      opt.publicKey.excludeCredentials[i].id = decodeBase64URIToUint8Array(
        opt.publicKey.excludeCredentials[i].id_base64
      );
    }
  }

  return opt;
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

  outputToLog(JSON.stringify(res, null, 2));
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

  outputToLog(JSON.stringify(res, null, 2));
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

  outputToLog(`キーペアが作成された`);
  outputToLog(JSON.stringify(res, null, 2));

  await fetch("/demo/passkey/register", {
    method: "POST",
    body: JSON.stringify(res),
    credentials: "include",
  });

  setupPasskeyAutofill();
});

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
