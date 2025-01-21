// @ts-check

import { signin, register } from "./passkey.js";

/** @type {AbortController | null} */
let autofillAbortController = null;

async function setupPasskeyAutofill() {
  if (!(await PublicKeyCredential.isConditionalMediationAvailable())) {
    outputToLog("待ち受けようとしたが、Autofill に対応していない");
    return;
  }

  const abort = new AbortController();
  autofillAbortController = abort;

  try {
    await signin(true, abort.signal);
  } catch (err) {
    outputToLog(err.toString());
  }
}

/** @type {HTMLButtonElement} */
// @ts-expect-error
const signinPasskeyButton = document.getElementById("signin-passkey");
signinPasskeyButton.addEventListener("click", async () => {
  autofillAbortController?.abort();
  try {
    await signin(false, undefined);
  } catch (err) {
    outputToLog(err);
  } finally {
    setupPasskeyAutofill();
  }
});

/** @type {HTMLButtonElement} */
// @ts-expect-error
const registerPasskeyButton = document.getElementById("register-passkey");
registerPasskeyButton.addEventListener("click", async () => {
  autofillAbortController?.abort();
  try {
    await register();
  } catch (err) {
    outputToLog(err.toString());
  } finally {
    setupPasskeyAutofill();
  }
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
// @ts-expect-error
window.outputToLog = outputToLog;

async function checkPasskeyCapabilities() {
  const isCMA = await PublicKeyCredential.isConditionalMediationAvailable();
  const isUVPAA =
    await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();

  outputToLog(`isCMA: ${isCMA ? "true" : "false"}`);
  outputToLog(`isUVPAA: ${isUVPAA ? "true" : "false"}`);
}

checkPasskeyCapabilities();

setupPasskeyAutofill();
