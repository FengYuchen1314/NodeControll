import fs from 'node:fs';

const baseUrl = process.argv[2] ?? 'http://127.0.0.1:18080';
const setupTokenFile = process.env.NODECONTROLL_TEST_SETUP_TOKEN_FILE;
if (!setupTokenFile) throw new Error('NODECONTROLL_TEST_SETUP_TOKEN_FILE is required');
const setupToken = fs.readFileSync(setupTokenFile, 'utf8').trim();
if (!/^[0-9a-f]{64}$/.test(setupToken)) throw new Error('test setup token is malformed');

async function readResponse(path, response) {
  let body;
  try {
    body = await response.json();
  } catch {
    throw new Error(`${path}: response was not JSON`);
  }
  const requestId = response.headers.get('x-request-id');
  if (!requestId) throw new Error(`${path}: missing x-request-id response header`);
  return { response, body, requestId };
}

async function get(path, headers = undefined) {
  const response = await fetch(new URL(path, baseUrl), { headers });
  const result = await readResponse(path, response);
  if (!response.ok) throw new Error(`${path}: HTTP ${response.status}`);
  return result;
}

async function post(path, payload, capability = undefined) {
  const headers = { 'content-type': 'application/json' };
  if (capability !== undefined) headers['x-nodecontroll-setup-token'] = capability;
  const response = await fetch(new URL(path, baseUrl), {
    method: 'POST',
    headers,
    body: JSON.stringify(payload),
  });
  return readResponse(path, response);
}

async function rawPost(path, { body, headers }) {
  const response = await fetch(new URL(path, baseUrl), { method: 'POST', headers, body });
  return readResponse(path, response);
}

const firstBootstrap = {
  instance_name: 'VPS smoke instance',
  username: 'smoke_owner',
  password: 'VPS smoke bootstrap passphrase',
};
const repeatedBootstrap = {
  instance_name: 'Must not replace instance',
  username: 'second_owner',
  password: 'Another smoke bootstrap passphrase',
};
const rejectedContractPassword = 'Rejected contract passphrase';

const forgedRequestId = 'client-controlled-request-id';
const health = await get('/healthz', { 'x-request-id': forgedRequestId });
const ready = await get('/readyz');
const bootstrapBefore = await get('/api/v1/bootstrap');
const malformedJson = await rawPost('/api/v1/bootstrap', {
  headers: { 'content-type': 'application/json' },
  body: '{"instance_name":',
});
const wrongMediaType = await rawPost('/api/v1/bootstrap', { headers: {}, body: '{}' });
const unknownField = await post('/api/v1/bootstrap', {
  instance_name: 'Rejected contract instance',
  username: 'rejected_owner',
  password: rejectedContractPassword,
  unexpected: true,
});
const oversizedBody = await rawPost('/api/v1/bootstrap', {
  headers: { 'content-type': 'application/json' },
  body: JSON.stringify({
    instance_name: 'Oversized contract instance',
    username: 'oversized_owner',
    password: 'x'.repeat(17_000),
  }),
});
const invalidCapability = await post('/api/v1/bootstrap', firstBootstrap);
const bootstrapCreate = await post('/api/v1/bootstrap', firstBootstrap, setupToken);
const bootstrapAfter = await get('/api/v1/bootstrap');
const bootstrapConflict = await post('/api/v1/bootstrap', repeatedBootstrap, setupToken);
const version = await get('/api/v1/system/version');
const openapi = await get('/api-docs/openapi.json');

const missing = await readResponse(
  '/api/v1/does-not-exist',
  await fetch(new URL('/api/v1/does-not-exist', baseUrl)),
);

const observedResponses = [
  ['healthz', health],
  ['readyz', ready],
  ['bootstrap-before', bootstrapBefore],
  ['malformed-json', malformedJson],
  ['wrong-media-type', wrongMediaType],
  ['unknown-field', unknownField],
  ['oversized-body', oversizedBody],
  ['invalid-capability', invalidCapability],
  ['bootstrap-create', bootstrapCreate],
  ['bootstrap-after', bootstrapAfter],
  ['bootstrap-conflict', bootstrapConflict],
  ['version', version],
  ['openapi', openapi],
  ['not-found', missing],
];
const requestIds = observedResponses.map(([, result]) => result.requestId);
if (health.requestId === forgedRequestId) {
  throw new Error('runtime trusted a client-selected x-request-id');
}
for (const requestId of requestIds) {
  if (!/^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(requestId)) {
    throw new Error(`runtime returned a non-v4 request ID: ${requestId}`);
  }
}
if (new Set(requestIds).size !== requestIds.length) {
  throw new Error('runtime responses reused an x-request-id');
}
for (const [label, result] of observedResponses) {
  const responseMaterial = `${JSON.stringify(result.body)}\n${JSON.stringify([...result.response.headers])}`;
  for (const forbidden of [
    firstBootstrap.password,
    repeatedBootstrap.password,
    rejectedContractPassword,
    setupToken,
    'x'.repeat(128),
    '$argon2id$',
  ]) {
    if (responseMaterial.includes(forbidden)) {
      throw new Error(`${label}: response exposed password or PHC material`);
    }
  }
}
for (const [label, result] of [
  ['bootstrap-before', bootstrapBefore],
  ['bootstrap-create', bootstrapCreate],
  ['bootstrap-after', bootstrapAfter],
  ['version', version],
]) {
  if (result.body.meta?.request_id !== result.requestId) {
    throw new Error(`${label}: envelope request ID did not match response header`);
  }
}
for (const [label, result] of [
  ['bootstrap-conflict', bootstrapConflict],
  ['malformed-json', malformedJson],
  ['wrong-media-type', wrongMediaType],
  ['unknown-field', unknownField],
  ['oversized-body', oversizedBody],
  ['invalid-capability', invalidCapability],
  ['not-found', missing],
]) {
  if (result.body.request_id !== result.requestId) {
    throw new Error(`${label}: Problem request ID did not match response header`);
  }
}

if (health.body.status !== 'ok') throw new Error('unexpected liveness response');
if (ready.body.status !== 'ready') throw new Error('unexpected readiness response');
if (ready.body.checks?.[0]?.name !== 'database') throw new Error('database readiness missing');
if (!ready.body.checks?.some((check) => check.name === 'secret_store' && check.status === 'ready')) {
  throw new Error('secret-store readiness missing');
}
if (bootstrapBefore.body.data?.initialized !== false) throw new Error('unexpected initial bootstrap state');
if (bootstrapBefore.body.data?.setup_capability_required !== true) {
  throw new Error('bootstrap projection did not require the deployment capability');
}
for (const [label, result, expectedStatus, expectedCode] of [
  ['malformed JSON', malformedJson, 400, 'BOOTSTRAP_JSON_INVALID'],
  ['wrong media type', wrongMediaType, 415, 'UNSUPPORTED_MEDIA_TYPE'],
  ['unknown field', unknownField, 422, 'BOOTSTRAP_JSON_SHAPE_INVALID'],
  ['oversized body', oversizedBody, 413, 'PAYLOAD_TOO_LARGE'],
]) {
  if (result.response.status !== expectedStatus || result.body.code !== expectedCode) {
    throw new Error(`${label}: extractor rejection contract mismatch`);
  }
  if (!result.response.headers.get('content-type')?.startsWith('application/problem+json')) {
    throw new Error(`${label}: extractor rejection did not return Problem Details`);
  }
}
if (invalidCapability.response.status !== 403 || invalidCapability.body.code !== 'SETUP_CAPABILITY_INVALID') {
  throw new Error('missing setup capability was not rejected');
}
if (!invalidCapability.response.headers.get('content-type')?.startsWith('application/problem+json')) {
  throw new Error('setup capability rejection did not return Problem Details');
}
if (bootstrapCreate.response.status !== 201) throw new Error('bootstrap create did not return 201');
if (!bootstrapCreate.body.data?.instance_id || !bootstrapCreate.body.data?.owner_id) {
  throw new Error('bootstrap create IDs missing');
}
if (bootstrapAfter.body.data?.initialized !== true) throw new Error('bootstrap state did not persist');
if (bootstrapAfter.body.data?.setup_capability_required !== false) {
  throw new Error('bootstrap projection still requested a consumed setup capability');
}
if (bootstrapAfter.body.data?.login_methods?.length !== 0) {
  throw new Error('bootstrap advertised a login method before its endpoint exists');
}
if (bootstrapConflict.response.status !== 409 || bootstrapConflict.body.code !== 'ALREADY_INITIALIZED') {
  throw new Error('repeat bootstrap was not rejected');
}
if (!bootstrapConflict.response.headers.get('content-type')?.startsWith('application/problem+json')) {
  throw new Error('repeat bootstrap did not return Problem Details');
}
if (version.body.data?.product !== 'NodeControll') throw new Error('unexpected product');
if (version.body.meta?.api_version !== 'v1') throw new Error('unexpected API version');
if (!openapi.body.paths?.['/api/v1/system/version']) throw new Error('runtime OpenAPI path missing');
for (const [method, status] of [
  ['get', '503'],
  ['post', '400'],
  ['post', '403'],
  ['post', '409'],
  ['post', '413'],
  ['post', '415'],
  ['post', '422'],
  ['post', '429'],
  ['post', '503'],
]) {
  const content = openapi.body.paths?.['/api/v1/bootstrap']?.[method]?.responses?.[status]?.content;
  if (!content?.['application/problem+json']) {
    throw new Error(`runtime OpenAPI ${method.toUpperCase()} bootstrap ${status} media type mismatch`);
  }
}
if (missing.response.status !== 404 || missing.body.code !== 'ROUTE_NOT_FOUND') {
  throw new Error('Problem Details fallback mismatch');
}
if (!missing.response.headers.get('content-type')?.startsWith('application/problem+json')) {
  throw new Error('Problem Details content type missing');
}

console.log(JSON.stringify({
  liveness: health.body.status,
  readiness: ready.body.status,
  initialized_before: bootstrapBefore.body.data.initialized,
  initialized_after: bootstrapAfter.body.data.initialized,
  repeat_bootstrap: bootstrapConflict.body.code,
  product: version.body.data.product,
  version: version.body.data.version,
  api_version: version.body.meta.api_version,
  openapi_paths: Object.keys(openapi.body.paths).length,
  request_ids_unique: new Set(requestIds).size === requestIds.length,
}, null, 2));
