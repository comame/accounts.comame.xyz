// @ts-check

import { decodeBase64URL, publicKeyCredentialToJSON } from "./util.js";

/**
 * @typedef {CredentialRequestOptions & { publicKey: { challenge_base64: string } }} getOptions
 */

/**
 * @typedef {CredentialCreationOptions & { publicKey: { challenge_base64: string, user: { id_base64: string }, excludeCredentials: { id_base64: string }[] }}} createOptions
 */

/**
 * @returns {Promise<getOptions>}
 */
export async function createCredentialsGetOptions() {
  /** @type {getOptions} */
  const opt = await fetch("/demo/passkey/signin-options", {
    method: "POST",
    credentials: "include",
  }).then((res) => res.json());

  opt.publicKey.challenge = decodeBase64URL(opt.publicKey.challenge_base64);

  return opt;
}

/**
 * @returns {Promise<CredentialCreationOptions>}
 */
export async function createCredentialsCreateOptions() {
  /** @type {createOptions} */
  const opt = await fetch("/demo/passkey/register-options", {
    method: "POST",
    credentials: "include",
  }).then((res) => res.json());

  opt.publicKey.challenge = decodeBase64URL(opt.publicKey.challenge_base64);
  opt.publicKey.user.id = decodeBase64URL(opt.publicKey.user.id_base64);
  if (opt.publicKey.excludeCredentials) {
    for (let i = 0; i < opt.publicKey.excludeCredentials.length; i += 1) {
      opt.publicKey.excludeCredentials[i].id = decodeBase64URL(
        opt.publicKey.excludeCredentials[i].id_base64
      );
    }
  }

  return opt;
}

export async function register() {
  const options = await createCredentialsCreateOptions();

  const res = await navigator.credentials.create(options);
  if (res === null) {
    throw new Error("レスポンスが空");
  }
  if (!(res instanceof PublicKeyCredential)) {
    throw new Error("レスポンスがPublicKeyCredentialではない");
  }

  // @ts-expect-error
  outputToLog("登録");
  // @ts-expect-error
  outputToLog(JSON.stringify(publicKeyCredentialToJSON(res), null, 2));

  await fetch("/demo/passkey/register", {
    method: "POST",
    body: JSON.stringify(publicKeyCredentialToJSON(res)),
    credentials: "include",
  });
}

/**
 * @param {boolean} autofill
 * @param {AbortSignal|undefined} signal
 */
export async function signin(autofill, signal) {
  const options = await createCredentialsGetOptions();
  if (autofill) {
    options.mediation = "conditional";
  }
  if (signal) {
    options.signal = signal;
  }

  let res = null;
  try {
    res = await navigator.credentials.get(options);
  } catch (err) {
    if (signal?.aborted ?? false) {
      // Autofill の待ち受けがキャンセル
      return;
    }
    throw err;
  }

  if (res === null) {
    throw new Error("レスポンスが空");
  }
  if (!(res instanceof PublicKeyCredential)) {
    throw new Error("レスポンスがPublicKeyCredentialではない");
  }

  // @ts-expect-error
  outputToLog("ログイン");
  // @ts-expect-error
  outputToLog(JSON.stringify(publicKeyCredentialToJSON(res), null, 2));

  await fetch("/demo/passkey/verify", {
    method: "POST",
    credentials: "include",
    body: JSON.stringify(publicKeyCredentialToJSON(res)),
  });
}
