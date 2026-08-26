import fs from "node:fs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:18080";
const publicOrigin = new URL(baseUrl).origin;
const setupTokenFile = process.env.NODECONTROLL_TEST_SETUP_TOKEN_FILE;
if (!setupTokenFile)
  throw new Error("NODECONTROLL_TEST_SETUP_TOKEN_FILE is required");
const setupToken = fs.readFileSync(setupTokenFile, "utf8").trim();
if (!/^[0-9a-f]{64}$/.test(setupToken))
  throw new Error("test setup token is malformed");

const observedResponses = [];
const browserCredentialSecrets = new Set();

async function record(label, pending) {
  const result = await pending;
  observedResponses.push([label, result]);
  return result;
}

async function readResponse(path, response) {
  let body;
  if (response.status === 204) {
    body = null;
  } else {
    try {
      body = await response.json();
    } catch {
      throw new Error(`${path}: response was not JSON`);
    }
  }
  const requestId = response.headers.get("x-request-id");
  if (!requestId)
    throw new Error(`${path}: missing x-request-id response header`);
  return { response, body, requestId };
}

async function get(path, headers = undefined) {
  const response = await fetch(new URL(path, baseUrl), { headers });
  const result = await readResponse(path, response);
  if (!response.ok) throw new Error(`${path}: HTTP ${response.status}`);
  return result;
}

async function post(path, payload, capability = undefined) {
  const headers = { "content-type": "application/json", origin: publicOrigin };
  if (capability !== undefined)
    headers["x-nodecontroll-setup-token"] = capability;
  const response = await fetch(new URL(path, baseUrl), {
    method: "POST",
    headers,
    body: JSON.stringify(payload),
  });
  return readResponse(path, response);
}

async function rawPost(path, { body, headers }) {
  const response = await fetch(new URL(path, baseUrl), {
    method: "POST",
    headers: { origin: publicOrigin, ...headers },
    body,
  });
  return readResponse(path, response);
}

async function browserRequest(method, path, browser, options = {}) {
  const { includeCsrf = method !== "GET", payload } = options;
  const headers = { cookie: browser.cookieHeader };
  if (method !== "GET") headers.origin = publicOrigin;
  if (includeCsrf) headers["x-nodecontroll-csrf"] = browser.csrfToken;
  if (payload !== undefined) headers["content-type"] = "application/json";
  const response = await fetch(new URL(path, baseUrl), {
    method,
    headers,
    ...(payload === undefined ? {} : { body: JSON.stringify(payload) }),
  });
  return readResponse(path, response);
}

function browserGet(path, browser) {
  return browserRequest("GET", path, browser, { includeCsrf: false });
}

function browserPost(path, payload, browser, options = undefined) {
  return browserRequest("POST", path, browser, { payload, ...options });
}

function browserDelete(path, browser) {
  return browserRequest("DELETE", path, browser);
}

function responseCookies(response) {
  if (typeof response.headers.getSetCookie === "function") {
    return response.headers.getSetCookie();
  }
  const combined = response.headers.get("set-cookie");
  return combined ? combined.split(/, (?=__Host-)/u) : [];
}

function cookieHeader(setCookies) {
  return setCookies.map((value) => value.split(";", 1)[0]).join("; ");
}

function cookieValue(setCookies, name) {
  const prefix = `${name}=`;
  const encoded = setCookies.find((value) => value.startsWith(prefix));
  return encoded?.slice(prefix.length).split(";", 1)[0];
}

function positiveCookieMaxAge(label, cookie) {
  const matches = [...cookie.matchAll(/(?:^|;\s*)Max-Age=([^;]*)/gu)];
  if (matches.length !== 1 || !/^\d+$/u.test(matches[0][1])) {
    throw new Error(
      `${label}: security cookie must contain one integer Max-Age`,
    );
  }
  const maxAge = Number(matches[0][1]);
  if (!Number.isSafeInteger(maxAge) || maxAge <= 0) {
    throw new Error(
      `${label}: issued security cookie Max-Age must be positive`,
    );
  }
  return maxAge;
}

function issuedBrowserCredential(label, result) {
  const setCookies = responseCookies(result.response);
  const sessionToken = cookieValue(setCookies, "__Host-nodecontroll_session");
  const csrfToken = cookieValue(setCookies, "__Host-nodecontroll_csrf");
  if (setCookies.length !== 2 || !sessionToken || !csrfToken) {
    throw new Error(
      `${label}: response did not issue both browser credentials`,
    );
  }
  const sessionCookie = setCookies.find((value) =>
    value.startsWith("__Host-nodecontroll_session="),
  );
  const csrfCookie = setCookies.find((value) =>
    value.startsWith("__Host-nodecontroll_csrf="),
  );
  if (
    !sessionCookie?.includes("; Path=/;") ||
    !sessionCookie.includes("; Secure;") ||
    !sessionCookie.includes("; HttpOnly;") ||
    !sessionCookie.endsWith("SameSite=Lax")
  ) {
    throw new Error(
      `${label}: session cookie attributes do not match the security contract`,
    );
  }
  if (
    !csrfCookie?.includes("; Path=/;") ||
    !csrfCookie.includes("; Secure;") ||
    csrfCookie.includes("HttpOnly") ||
    !csrfCookie.endsWith("SameSite=Lax")
  ) {
    throw new Error(
      `${label}: CSRF cookie attributes do not match the security contract`,
    );
  }
  if (setCookies.some((value) => value.includes("Domain="))) {
    throw new Error(`${label}: security cookies must remain host-only`);
  }
  const sessionMaxAge = positiveCookieMaxAge(label, sessionCookie);
  const csrfMaxAge = positiveCookieMaxAge(label, csrfCookie);
  const absoluteExpiresAtMs =
    result.body?.data?.session?.absolute_expires_at_ms;
  if (
    sessionMaxAge !== csrfMaxAge ||
    typeof absoluteExpiresAtMs !== "number" ||
    !Number.isSafeInteger(absoluteExpiresAtMs) ||
    sessionMaxAge * 1_000 > absoluteExpiresAtMs - Date.now() + 2_000
  ) {
    throw new Error(
      `${label}: security cookie lifetime exceeds the session projection`,
    );
  }
  browserCredentialSecrets.add(sessionToken);
  browserCredentialSecrets.add(csrfToken);
  return {
    cookieHeader: cookieHeader(setCookies),
    csrfToken,
    sessionToken,
  };
}

function expectCredentialRotation(label, before, after) {
  if (
    before.sessionToken === after.sessionToken ||
    before.csrfToken === after.csrfToken ||
    before.cookieHeader === after.cookieHeader
  ) {
    throw new Error(`${label}: browser credentials were not fully rotated`);
  }
}

function expectClearedBrowserCredentials(label, result) {
  const cleared = responseCookies(result.response);
  if (
    cleared.length !== 2 ||
    !cleared.every((value) => value.includes("Max-Age=0")) ||
    !cleared.some((value) =>
      value.startsWith("__Host-nodecontroll_session="),
    ) ||
    !cleared.some((value) => value.startsWith("__Host-nodecontroll_csrf="))
  ) {
    throw new Error(
      `${label}: response did not expire both browser credentials`,
    );
  }
}

function sessionId(label, result) {
  const id = result.body?.data?.session?.id;
  if (
    typeof id !== "string" ||
    !/^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(
      id,
    )
  ) {
    throw new Error(
      `${label}: authenticated response did not contain a UUIDv7 session ID`,
    );
  }
  return id;
}

function expectProblem(label, result, expectedStatus, expectedCode) {
  if (
    result.response.status !== expectedStatus ||
    result.body?.status !== expectedStatus ||
    result.body?.code !== expectedCode ||
    typeof result.body?.type !== "string" ||
    result.body.type.length === 0
  ) {
    throw new Error(`${label}: Problem contract mismatch`);
  }
  if (
    !result.response.headers
      .get("content-type")
      ?.startsWith("application/problem+json")
  ) {
    throw new Error(`${label}: response did not use Problem Details`);
  }
}

function expectSessionInvalid(label, result) {
  expectProblem(label, result, 401, "SESSION_INVALID");
  if (responseCookies(result.response).length !== 0) {
    throw new Error(
      `${label}: generic session-invalid Problem changed shared browser credentials`,
    );
  }
}

function expectUsableSession(label, result, expectedSessionId) {
  if (
    result.response.status !== 200 ||
    result.body?.data?.session?.id !== expectedSessionId ||
    result.body?.data?.actor?.username !== firstBootstrap.username
  ) {
    throw new Error(`${label}: expected server-side session was not usable`);
  }
}

function expectOnlyCurrentSession(label, result, expectedSessionId) {
  const sessions = result.body?.data?.sessions;
  if (
    result.response.status !== 200 ||
    !Array.isArray(sessions) ||
    sessions.length !== 1 ||
    sessions[0]?.id !== expectedSessionId ||
    sessions[0]?.is_current !== true
  ) {
    throw new Error(
      `${label}: session list did not contain only the replacement session`,
    );
  }
}

const firstBootstrap = {
  instance_name: "VPS smoke instance",
  username: "smoke_owner",
  password: "VPS smoke bootstrap passphrase",
};
const repeatedBootstrap = {
  instance_name: "Must not replace instance",
  username: "second_owner",
  password: "Another smoke bootstrap passphrase",
};
const rejectedContractPassword = "Rejected contract passphrase";
const wrongLoginPassword = "Incorrect smoke login passphrase";
const wrongReauthenticationPassword = wrongLoginPassword;
const replacementPassword = repeatedBootstrap.password;

const forgedRequestId = "client-controlled-request-id";
const health = await record(
  "healthz",
  get("/healthz", { "x-request-id": forgedRequestId }),
);
const ready = await record("readyz", get("/readyz"));
const bootstrapBefore = await record(
  "bootstrap-before",
  get("/api/v1/bootstrap"),
);
const malformedJson = await record(
  "malformed-json",
  rawPost("/api/v1/bootstrap", {
    headers: { "content-type": "application/json" },
    body: '{"instance_name":',
  }),
);
const wrongMediaType = await record(
  "wrong-media-type",
  rawPost("/api/v1/bootstrap", { headers: {}, body: "{}" }),
);
const unknownField = await record(
  "unknown-field",
  post("/api/v1/bootstrap", {
    instance_name: "Rejected contract instance",
    username: "rejected_owner",
    password: rejectedContractPassword,
    unexpected: true,
  }),
);
const oversizedBody = await record(
  "oversized-body",
  rawPost("/api/v1/bootstrap", {
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      instance_name: "Oversized contract instance",
      username: "oversized_owner",
      password: "x".repeat(17_000),
    }),
  }),
);
const invalidCapability = await record(
  "invalid-capability",
  post("/api/v1/bootstrap", firstBootstrap),
);
const bootstrapCreate = await record(
  "bootstrap-create",
  post("/api/v1/bootstrap", firstBootstrap, setupToken),
);
const bootstrapAfter = await record(
  "bootstrap-after",
  get("/api/v1/bootstrap"),
);
const bootstrapConflict = await record(
  "bootstrap-conflict",
  post("/api/v1/bootstrap", repeatedBootstrap, setupToken),
);
const missingOriginLogin = await record(
  "missing-origin-login",
  readResponse(
    "/api/v1/auth/login",
    await fetch(new URL("/api/v1/auth/login", baseUrl), {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        username: firstBootstrap.username,
        password: firstBootstrap.password,
      }),
    }),
  ),
);
const wrongLogin = await record(
  "wrong-login",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: wrongLoginPassword,
  }),
);

const loginA = await record(
  "login-a",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: firstBootstrap.password,
  }),
);
const browserAInitial = issuedBrowserCredential("login-a", loginA);
const sessionAInitialId = sessionId("login-a", loginA);
const loginB = await record(
  "login-b",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: firstBootstrap.password,
  }),
);
const browserB = issuedBrowserCredential("login-b", loginB);
const sessionBId = sessionId("login-b", loginB);
if (sessionAInitialId === sessionBId) {
  throw new Error("parallel logins reused one server-side session");
}
const meAInitial = await record(
  "me-a-initial",
  browserGet("/api/v1/me", browserAInitial),
);
const meBInitial = await record(
  "me-b-initial",
  browserGet("/api/v1/me", browserB),
);

const rejectedReauthentication = await record(
  "reauth-rejected",
  browserPost(
    "/api/v1/auth/reauth",
    { method: "password", password: wrongReauthenticationPassword },
    browserAInitial,
  ),
);
const meAAfterRejectedReauthentication = await record(
  "me-a-after-rejected-reauth",
  browserGet("/api/v1/me", browserAInitial),
);

const reauthentication = await record(
  "reauth-success",
  browserPost(
    "/api/v1/auth/reauth",
    { method: "password", password: firstBootstrap.password },
    browserAInitial,
  ),
);
const browserAReauthenticated = issuedBrowserCredential(
  "reauth-success",
  reauthentication,
);
expectCredentialRotation(
  "reauth-success",
  browserAInitial,
  browserAReauthenticated,
);
const sessionAReauthenticatedId = sessionId("reauth-success", reauthentication);
if (
  sessionAReauthenticatedId === sessionAInitialId ||
  sessionAReauthenticatedId === sessionBId
) {
  throw new Error(
    "reauthentication did not create a unique replacement session",
  );
}
const meAOldAfterReauthentication = await record(
  "me-a-old-after-reauth",
  browserGet("/api/v1/me", browserAInitial),
);
const meAReauthenticated = await record(
  "me-a-new-after-reauth",
  browserGet("/api/v1/me", browserAReauthenticated),
);
const meBAfterReauthentication = await record(
  "me-b-after-reauth",
  browserGet("/api/v1/me", browserB),
);
const sessionsBeforeSingleRevoke = await record(
  "sessions-before-single-revoke",
  browserGet("/api/v1/me/sessions", browserAReauthenticated),
);

const revokeSibling = await record(
  "revoke-sibling",
  browserDelete(
    `/api/v1/me/sessions/${encodeURIComponent(sessionBId)}`,
    browserAReauthenticated,
  ),
);
const revokeSiblingAgain = await record(
  "revoke-sibling-idempotent",
  browserDelete(
    `/api/v1/me/sessions/${encodeURIComponent(sessionBId)}`,
    browserAReauthenticated,
  ),
);
const meBAfterSingleRevoke = await record(
  "me-b-after-single-revoke",
  browserGet("/api/v1/me", browserB),
);
const meAAfterSingleRevoke = await record(
  "me-a-after-single-revoke",
  browserGet("/api/v1/me", browserAReauthenticated),
);
const sessionsAfterSingleRevoke = await record(
  "sessions-after-single-revoke",
  browserGet("/api/v1/me/sessions", browserAReauthenticated),
);

const loginC = await record(
  "login-c-before-password-change",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: firstBootstrap.password,
  }),
);
const browserC = issuedBrowserCredential(
  "login-c-before-password-change",
  loginC,
);
const sessionCId = sessionId("login-c-before-password-change", loginC);
if (
  sessionCId === sessionAInitialId ||
  sessionCId === sessionAReauthenticatedId ||
  sessionCId === sessionBId
) {
  throw new Error(
    "pre-password-change sibling did not receive a unique session",
  );
}
const meCBeforePasswordChange = await record(
  "me-c-before-password-change",
  browserGet("/api/v1/me", browserC),
);

const passwordChange = await record(
  "password-change",
  browserPost(
    "/api/v1/me/password",
    { new_password: replacementPassword },
    browserAReauthenticated,
  ),
);
const browserAfterPasswordChange = issuedBrowserCredential(
  "password-change",
  passwordChange,
);
expectCredentialRotation(
  "password-change",
  browserAReauthenticated,
  browserAfterPasswordChange,
);
const passwordReplacementSessionId = sessionId(
  "password-change",
  passwordChange,
);
if (
  [
    sessionAInitialId,
    sessionAReauthenticatedId,
    sessionBId,
    sessionCId,
  ].includes(passwordReplacementSessionId)
) {
  throw new Error("password change reused an old session ID");
}
const meAAfterPasswordChange = await record(
  "me-a-old-after-password-change",
  browserGet("/api/v1/me", browserAReauthenticated),
);
const meCAfterPasswordChange = await record(
  "me-c-after-password-change",
  browserGet("/api/v1/me", browserC),
);
const mePasswordReplacement = await record(
  "me-password-replacement",
  browserGet("/api/v1/me", browserAfterPasswordChange),
);
const sessionsAfterPasswordChange = await record(
  "sessions-after-password-change",
  browserGet("/api/v1/me/sessions", browserAfterPasswordChange),
);
const oldPasswordAfterChange = await record(
  "old-password-after-change",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: firstBootstrap.password,
  }),
);
const loginWithReplacementPassword = await record(
  "login-with-replacement-password",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: replacementPassword,
  }),
);
const browserReplacementSibling = issuedBrowserCredential(
  "login-with-replacement-password",
  loginWithReplacementPassword,
);
const replacementSiblingSessionId = sessionId(
  "login-with-replacement-password",
  loginWithReplacementPassword,
);
const sessionsBeforeKeepCurrent = await record(
  "sessions-before-keep-current",
  browserGet("/api/v1/me/sessions", browserAfterPasswordChange),
);

const recentAuthBeforeKeepCurrent =
  passwordChange.body?.data?.session?.recent_auth_expires_at_ms;
if (!Number.isSafeInteger(recentAuthBeforeKeepCurrent)) {
  throw new Error(
    "password replacement did not expose a safe recent-auth expiry",
  );
}
const logoutAllKeepCurrent = await record(
  "logout-all-keep-current",
  browserPost(
    "/api/v1/auth/logout-all",
    { keep_current: true },
    browserAfterPasswordChange,
  ),
);
const browserAfterKeepCurrent = issuedBrowserCredential(
  "logout-all-keep-current",
  logoutAllKeepCurrent,
);
expectCredentialRotation(
  "logout-all-keep-current",
  browserAfterPasswordChange,
  browserAfterKeepCurrent,
);
const keepCurrentReplacementSessionId = sessionId(
  "logout-all-keep-current",
  logoutAllKeepCurrent,
);
const meOldAfterKeepCurrent = await record(
  "me-old-after-keep-current",
  browserGet("/api/v1/me", browserAfterPasswordChange),
);
const meSiblingAfterKeepCurrent = await record(
  "me-sibling-after-keep-current",
  browserGet("/api/v1/me", browserReplacementSibling),
);
const meKeepCurrentReplacement = await record(
  "me-keep-current-replacement",
  browserGet("/api/v1/me", browserAfterKeepCurrent),
);
const sessionsAfterKeepCurrent = await record(
  "sessions-after-keep-current",
  browserGet("/api/v1/me/sessions", browserAfterKeepCurrent),
);

const loginBeforeLogoutEverywhere = await record(
  "login-before-logout-everywhere",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: replacementPassword,
  }),
);
const browserBeforeLogoutEverywhere = issuedBrowserCredential(
  "login-before-logout-everywhere",
  loginBeforeLogoutEverywhere,
);
const sessionBeforeLogoutEverywhereId = sessionId(
  "login-before-logout-everywhere",
  loginBeforeLogoutEverywhere,
);
if (sessionBeforeLogoutEverywhereId === keepCurrentReplacementSessionId) {
  throw new Error("logout-everywhere sibling reused the current session ID");
}
const meSiblingBeforeLogoutEverywhere = await record(
  "me-sibling-before-logout-everywhere",
  browserGet("/api/v1/me", browserBeforeLogoutEverywhere),
);
const logoutEverywhere = await record(
  "logout-everywhere",
  browserPost(
    "/api/v1/auth/logout-all",
    { keep_current: false },
    browserAfterKeepCurrent,
  ),
);
const meKeepCurrentAfterLogoutEverywhere = await record(
  "me-keep-current-after-logout-everywhere",
  browserGet("/api/v1/me", browserAfterKeepCurrent),
);
const meSiblingAfterLogoutEverywhere = await record(
  "me-sibling-after-logout-everywhere",
  browserGet("/api/v1/me", browserBeforeLogoutEverywhere),
);

const finalLogin = await record(
  "final-login",
  post("/api/v1/auth/login", {
    username: firstBootstrap.username,
    password: replacementPassword,
  }),
);
const finalBrowser = issuedBrowserCredential("final-login", finalLogin);
const finalSessionId = sessionId("final-login", finalLogin);
const rejectedLogout = await record(
  "rejected-current-logout",
  browserPost("/api/v1/auth/logout", undefined, finalBrowser, {
    includeCsrf: false,
  }),
);
const meAfterRejectedLogout = await record(
  "me-after-rejected-current-logout",
  browserGet("/api/v1/me", finalBrowser),
);
const logout = await record(
  "current-logout",
  browserPost("/api/v1/auth/logout", undefined, finalBrowser),
);
const meAfterLogout = await record(
  "me-after-current-logout",
  browserGet("/api/v1/me", finalBrowser),
);

const version = await record("version", get("/api/v1/system/version"));
const openapi = await record("openapi", get("/api-docs/openapi.json"));
const missing = await record(
  "not-found",
  readResponse(
    "/api/v1/does-not-exist",
    await fetch(new URL("/api/v1/does-not-exist", baseUrl)),
  ),
);

const requestIds = observedResponses.map(([, result]) => result.requestId);
if (health.requestId === forgedRequestId) {
  throw new Error("runtime trusted a client-selected x-request-id");
}
for (const requestId of requestIds) {
  if (
    !/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(
      requestId,
    )
  ) {
    throw new Error("runtime returned a non-v4 request ID");
  }
}
if (new Set(requestIds).size !== requestIds.length) {
  throw new Error("runtime responses reused an x-request-id");
}
for (const [label, result] of observedResponses) {
  if (
    result.body?.meta !== undefined &&
    result.body.meta?.request_id !== result.requestId
  ) {
    throw new Error(
      `${label}: envelope request ID did not match response header`,
    );
  }
  if (
    result.response.status >= 400 &&
    result.body?.request_id !== result.requestId
  ) {
    throw new Error(
      `${label}: Problem request ID did not match response header`,
    );
  }
  if (
    result.response.status >= 400 &&
    !result.response.headers
      .get("content-type")
      ?.startsWith("application/problem+json")
  ) {
    throw new Error(`${label}: error response did not use Problem Details`);
  }
}

const forbiddenStaticSecrets = new Set([
  firstBootstrap.password,
  repeatedBootstrap.password,
  rejectedContractPassword,
  wrongLoginPassword,
  wrongReauthenticationPassword,
  replacementPassword,
  setupToken,
  "x".repeat(128),
  "$argon2id$",
]);
for (const [label, result] of observedResponses) {
  const responseMaterial = `${JSON.stringify(result.body)}\n${JSON.stringify([
    ...result.response.headers,
  ])}`;
  for (const forbidden of forbiddenStaticSecrets) {
    if (responseMaterial.includes(forbidden)) {
      throw new Error(
        `${label}: response exposed password, setup token, or PHC material`,
      );
    }
  }
  const bodyMaterial = JSON.stringify(result.body);
  for (const credentialSecret of browserCredentialSecrets) {
    if (bodyMaterial.includes(credentialSecret)) {
      throw new Error(`${label}: response body exposed a browser credential`);
    }
  }
}

if (health.body.status !== "ok")
  throw new Error("unexpected liveness response");
if (ready.body.status !== "ready")
  throw new Error("unexpected readiness response");
if (ready.body.checks?.[0]?.name !== "database")
  throw new Error("database readiness missing");
if (
  !ready.body.checks?.some(
    (check) => check.name === "secret_store" && check.status === "ready",
  )
) {
  throw new Error("secret-store readiness missing");
}
if (bootstrapBefore.body.data?.initialized !== false) {
  throw new Error("unexpected initial bootstrap state");
}
if (bootstrapBefore.body.data?.setup_capability_required !== true) {
  throw new Error(
    "bootstrap projection did not require the deployment capability",
  );
}
for (const [label, result, expectedStatus, expectedCode] of [
  ["malformed JSON", malformedJson, 400, "BOOTSTRAP_JSON_INVALID"],
  ["wrong media type", wrongMediaType, 415, "UNSUPPORTED_MEDIA_TYPE"],
  ["unknown field", unknownField, 422, "BOOTSTRAP_JSON_SHAPE_INVALID"],
  ["oversized body", oversizedBody, 413, "PAYLOAD_TOO_LARGE"],
]) {
  expectProblem(label, result, expectedStatus, expectedCode);
}
expectProblem(
  "missing setup capability",
  invalidCapability,
  403,
  "SETUP_CAPABILITY_INVALID",
);
if (bootstrapCreate.response.status !== 201) {
  throw new Error("bootstrap create did not return 201");
}
if (
  !bootstrapCreate.body.data?.instance_id ||
  !bootstrapCreate.body.data?.owner_id
) {
  throw new Error("bootstrap create IDs missing");
}
if (bootstrapAfter.body.data?.initialized !== true) {
  throw new Error("bootstrap state did not persist");
}
if (bootstrapAfter.body.data?.setup_capability_required !== false) {
  throw new Error(
    "bootstrap projection still requested a consumed setup capability",
  );
}
if (bootstrapAfter.body.data?.login_methods?.join(",") !== "password") {
  throw new Error("bootstrap did not advertise the password login endpoint");
}
expectProblem(
  "repeat bootstrap",
  bootstrapConflict,
  409,
  "ALREADY_INITIALIZED",
);
expectProblem(
  "login without exact Origin",
  missingOriginLogin,
  403,
  "BROWSER_ORIGIN_INVALID",
);
expectProblem("wrong password", wrongLogin, 401, "INVALID_CREDENTIALS");

for (const [label, result] of [
  ["login-a", loginA],
  ["login-b", loginB],
]) {
  if (
    result.response.status !== 200 ||
    result.body.data?.actor?.username !== firstBootstrap.username
  ) {
    throw new Error(
      `${label}: password login did not return the authenticated actor`,
    );
  }
  if (
    result.body.data?.actor?.role !== "owner" ||
    !result.body.data?.actor?.capabilities?.includes("instance:manage")
  ) {
    throw new Error(`${label}: owner role/capability projection is incomplete`);
  }
}
expectUsableSession("initial session A", meAInitial, sessionAInitialId);
expectUsableSession("initial session B", meBInitial, sessionBId);

expectProblem(
  "wrong reauthentication proof",
  rejectedReauthentication,
  403,
  "REAUTHENTICATION_FAILED",
);
if (responseCookies(rejectedReauthentication.response).length !== 0) {
  throw new Error(
    "wrong reauthentication unexpectedly changed browser credentials",
  );
}
expectUsableSession(
  "session after wrong reauthentication",
  meAAfterRejectedReauthentication,
  sessionAInitialId,
);
if (reauthentication.response.status !== 200) {
  throw new Error("successful reauthentication did not return 200");
}
expectSessionInvalid(
  "old token after reauthentication",
  meAOldAfterReauthentication,
);
expectUsableSession(
  "replacement token after reauthentication",
  meAReauthenticated,
  sessionAReauthenticatedId,
);
expectUsableSession(
  "sibling after reauthentication",
  meBAfterReauthentication,
  sessionBId,
);
const sessionsBeforeRevoke = sessionsBeforeSingleRevoke.body?.data?.sessions;
if (
  !Array.isArray(sessionsBeforeRevoke) ||
  sessionsBeforeRevoke.length !== 2 ||
  !sessionsBeforeRevoke.some(
    (candidate) =>
      candidate.id === sessionAReauthenticatedId && candidate.is_current,
  ) ||
  !sessionsBeforeRevoke.some(
    (candidate) => candidate.id === sessionBId && !candidate.is_current,
  )
) {
  throw new Error(
    "parallel session list did not identify current and sibling sessions",
  );
}

if (revokeSibling.response.status !== 204 || revokeSibling.body !== null) {
  throw new Error("single-session revoke did not return an empty 204");
}
if (
  revokeSiblingAgain.response.status !== 204 ||
  revokeSiblingAgain.body !== null
) {
  throw new Error(
    "repeating a sibling-session revoke with a valid caller was not an empty 204",
  );
}
expectSessionInvalid("sibling after single revoke", meBAfterSingleRevoke);
expectUsableSession(
  "current session after sibling revoke",
  meAAfterSingleRevoke,
  sessionAReauthenticatedId,
);
expectOnlyCurrentSession(
  "session list after sibling revoke",
  sessionsAfterSingleRevoke,
  sessionAReauthenticatedId,
);
expectUsableSession(
  "sibling before password change",
  meCBeforePasswordChange,
  sessionCId,
);

if (
  passwordChange.response.status !== 200 ||
  !Number.isSafeInteger(passwordChange.body?.data?.revoked_sessions) ||
  passwordChange.body.data.revoked_sessions < 2 ||
  passwordChange.body.data?.actor?.force_password_change !== false
) {
  throw new Error(
    "password change did not revoke all old sessions and return a replacement",
  );
}
expectSessionInvalid(
  "old current after password change",
  meAAfterPasswordChange,
);
expectSessionInvalid(
  "old sibling after password change",
  meCAfterPasswordChange,
);
expectUsableSession(
  "password-change replacement",
  mePasswordReplacement,
  passwordReplacementSessionId,
);
expectOnlyCurrentSession(
  "session list after password change",
  sessionsAfterPasswordChange,
  passwordReplacementSessionId,
);
expectProblem(
  "old password after password change",
  oldPasswordAfterChange,
  401,
  "INVALID_CREDENTIALS",
);
if (loginWithReplacementPassword.response.status !== 200) {
  throw new Error("replacement password could not create a new session");
}
if (replacementSiblingSessionId === passwordReplacementSessionId) {
  throw new Error(
    "replacement-password login reused the password-change session",
  );
}
const sessionsBeforeKeep = sessionsBeforeKeepCurrent.body?.data?.sessions;
if (
  !Array.isArray(sessionsBeforeKeep) ||
  sessionsBeforeKeep.length !== 2 ||
  !sessionsBeforeKeep.some(
    (candidate) =>
      candidate.id === passwordReplacementSessionId && candidate.is_current,
  ) ||
  !sessionsBeforeKeep.some(
    (candidate) =>
      candidate.id === replacementSiblingSessionId && !candidate.is_current,
  )
) {
  throw new Error(
    "keep-current precondition did not contain current and sibling sessions",
  );
}

if (
  logoutAllKeepCurrent.response.status !== 200 ||
  !Number.isSafeInteger(logoutAllKeepCurrent.body?.data?.revoked_sessions) ||
  logoutAllKeepCurrent.body.data.revoked_sessions < 2
) {
  throw new Error("logout-all keep-current did not revoke all old sessions");
}
if (
  logoutAllKeepCurrent.body.data?.session?.recent_auth_expires_at_ms !==
  recentAuthBeforeKeepCurrent
) {
  throw new Error(
    "logout-all keep-current improperly elevated recent authentication",
  );
}
if (
  [passwordReplacementSessionId, replacementSiblingSessionId].includes(
    keepCurrentReplacementSessionId,
  )
) {
  throw new Error("logout-all keep-current reused an old session ID");
}
expectSessionInvalid("old current after keep-current", meOldAfterKeepCurrent);
expectSessionInvalid(
  "old sibling after keep-current",
  meSiblingAfterKeepCurrent,
);
expectUsableSession(
  "keep-current replacement",
  meKeepCurrentReplacement,
  keepCurrentReplacementSessionId,
);
expectOnlyCurrentSession(
  "session list after keep-current",
  sessionsAfterKeepCurrent,
  keepCurrentReplacementSessionId,
);
expectUsableSession(
  "sibling before logout-everywhere",
  meSiblingBeforeLogoutEverywhere,
  sessionBeforeLogoutEverywhereId,
);

if (
  logoutEverywhere.response.status !== 204 ||
  logoutEverywhere.body !== null
) {
  throw new Error("logout-everywhere did not return an empty 204");
}
expectClearedBrowserCredentials("logout-everywhere", logoutEverywhere);
expectSessionInvalid(
  "kept session after logout-everywhere",
  meKeepCurrentAfterLogoutEverywhere,
);
expectSessionInvalid(
  "sibling session after logout-everywhere",
  meSiblingAfterLogoutEverywhere,
);

expectProblem(
  "current logout without CSRF",
  rejectedLogout,
  403,
  "CSRF_INVALID",
);
if (responseCookies(rejectedLogout.response).length !== 0) {
  throw new Error(
    "CSRF-rejected logout unexpectedly changed browser credentials",
  );
}
expectUsableSession(
  "session after CSRF-rejected logout",
  meAfterRejectedLogout,
  finalSessionId,
);
if (logout.response.status !== 204 || logout.body !== null) {
  throw new Error("current logout did not return an empty 204");
}
expectClearedBrowserCredentials("current logout", logout);
expectSessionInvalid("current session after logout", meAfterLogout);

if (version.body.data?.product !== "NodeControll")
  throw new Error("unexpected product");
if (version.body.meta?.api_version !== "v1")
  throw new Error("unexpected API version");
if (!openapi.body.paths?.["/api/v1/system/version"]) {
  throw new Error("runtime OpenAPI path missing");
}
for (const path of [
  "/api/v1/auth/login",
  "/api/v1/me",
  "/api/v1/auth/logout",
]) {
  if (!openapi.body.paths?.[path])
    throw new Error(`runtime OpenAPI path missing: ${path}`);
}
for (const [path, method, operationId] of [
  ["/api/v1/auth/reauth", "post", "reauthenticate"],
  ["/api/v1/me/password", "post", "changeCurrentPassword"],
  ["/api/v1/me/sessions", "get", "listCurrentSessions"],
  ["/api/v1/auth/logout-all", "post", "logoutAll"],
  ["/api/v1/me/sessions/{session_id}", "delete", "revokeCurrentUserSession"],
]) {
  const operation = openapi.body.paths?.[path]?.[method];
  if (!operation || operation.operationId !== operationId) {
    throw new Error(
      `runtime OpenAPI operation missing: ${method.toUpperCase()} ${path}`,
    );
  }
}
for (const [method, status] of [
  ["get", "503"],
  ["post", "400"],
  ["post", "403"],
  ["post", "409"],
  ["post", "413"],
  ["post", "415"],
  ["post", "422"],
  ["post", "429"],
  ["post", "503"],
]) {
  const content =
    openapi.body.paths?.["/api/v1/bootstrap"]?.[method]?.responses?.[status]
      ?.content;
  if (!content?.["application/problem+json"]) {
    throw new Error(
      `runtime OpenAPI ${method.toUpperCase()} bootstrap ${status} media type mismatch`,
    );
  }
}
expectProblem("Problem Details fallback", missing, 404, "ROUTE_NOT_FOUND");

console.log(
  JSON.stringify(
    {
      liveness: health.body.status,
      readiness: ready.body.status,
      initialized_before: bootstrapBefore.body.data.initialized,
      initialized_after: bootstrapAfter.body.data.initialized,
      repeat_bootstrap: bootstrapConflict.body.code,
      parallel_sessions: true,
      rejected_reauthentication_preserved_session: true,
      reauthentication_rotated_session: true,
      reauthentication_preserved_sibling: true,
      single_session_revoke: true,
      password_change_replacement_only: true,
      old_password_rejected: oldPasswordAfterChange.body.code,
      keep_current_rotated_without_elevation: true,
      logout_everywhere_revoked_all: true,
      csrf_rejected_without_revocation: true,
      current_logout_revoked_session: meAfterLogout.body.code,
      product: version.body.data.product,
      version: version.body.data.version,
      api_version: version.body.meta.api_version,
      openapi_paths: Object.keys(openapi.body.paths).length,
      request_ids_unique: new Set(requestIds).size === requestIds.length,
    },
    null,
    2,
  ),
);
