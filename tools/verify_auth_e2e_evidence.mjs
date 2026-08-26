import assert from "node:assert/strict";
import { Buffer } from "node:buffer";
import { createHash } from "node:crypto";
import { lstat, readFile, readdir, realpath } from "node:fs/promises";
import { basename, isAbsolute, join, relative, resolve, sep } from "node:path";
import process from "node:process";

const argumentsList = process.argv.slice(2);
if (argumentsList.length !== 13) {
  throw new Error(
    "usage: verify_auth_e2e_evidence.mjs <evidence> <run-root> <behavior-marker> " +
      "<scan-marker> <run-id> <source-revision> <browser-image-digest> " +
      "<playwright-version> <node-version> <browser-version> <browser-executable-sha256> " +
      "<gate-script-sha256> <tls-certificate-sha256>",
  );
}

const [
  evidenceInput,
  runRootInput,
  behaviorMarkerInput,
  scanMarkerInput,
  expectedRunId,
  expectedSourceRevision,
  expectedBrowserImageDigest,
  expectedPlaywrightVersion,
  expectedNodeVersion,
  expectedBrowserVersion,
  expectedBrowserExecutableSha256,
  expectedGateScriptSha256,
  expectedTlsCertificateSha256,
] = argumentsList;

const isWithin = (root, candidate) => {
  const pathFromRoot = relative(root, candidate);
  return (
    pathFromRoot === "" ||
    (pathFromRoot !== ".." &&
      !pathFromRoot.startsWith(`..${sep}`) &&
      !isAbsolute(pathFromRoot))
  );
};

const regularPrivateFile = async (input, label) => {
  const metadata = await lstat(input);
  assert.ok(
    metadata.isFile() && !metadata.isSymbolicLink(),
    `${label} must be a regular file`,
  );
  assert.equal(
    metadata.mode & 0o077,
    0,
    `${label} must not grant group/other access`,
  );
  return metadata;
};

const runRootMetadata = await lstat(runRootInput);
assert.ok(
  runRootMetadata.isDirectory() && !runRootMetadata.isSymbolicLink(),
  "run root must be a real directory",
);
const runRoot = await realpath(runRootInput);
assert.equal(
  runRootInput,
  "/evidence",
  "formal auth E2E run root must be /evidence",
);
assert.equal(
  runRoot,
  resolve(runRootInput),
  "run root must use its canonical path",
);
const expectedControlFiles = [
  [evidenceInput, ["browser/handshake/evidence.json", "browser evidence"]],
  [
    behaviorMarkerInput,
    ["browser/handshake/behavior-ready", "behavior-ready marker"],
  ],
  [scanMarkerInput, ["browser/handshake/scan-ready", "scan-ready marker"]],
];
const controlMetadata = new Map();
for (const [input, [relativePath, label]] of expectedControlFiles) {
  const expectedPath = resolve(runRoot, relativePath);
  assert.equal(input, expectedPath, `${label} must use its exact formal path`);
  assert.equal(
    resolve(input),
    expectedPath,
    `${label} path differs from the formal contract`,
  );
  const metadata = await regularPrivateFile(input, label);
  assert.equal(
    await realpath(input),
    expectedPath,
    `${label} path contains a symlink or alias`,
  );
  controlMetadata.set(label, metadata);
}
const markerContents = `${expectedRunId}\n`;
for (const [input, label] of [
  [behaviorMarkerInput, "behavior-ready marker"],
  [scanMarkerInput, "scan-ready marker"],
]) {
  assert.equal(
    await readFile(input, "utf8"),
    markerContents,
    `${label} belongs to another run`,
  );
}
assert.ok(
  controlMetadata.get("behavior-ready marker").mtimeMs <=
    controlMetadata.get("scan-ready marker").mtimeMs,
  "scan-ready marker predates behavior-ready",
);
assert.ok(
  controlMetadata.get("scan-ready marker").mtimeMs <=
    controlMetadata.get("browser evidence").mtimeMs,
  "browser evidence predates scan-ready",
);
const evidencePath = await realpath(evidenceInput);
assert.ok(
  isWithin(runRoot, evidencePath) && evidencePath !== runRoot,
  "evidence escaped run root",
);

for (const [relativePath, expectedEntries] of [
  [
    "browser",
    [
      "database",
      "database-dump",
      "gate-runtime.log",
      "handshake",
      "runtime-logs",
      "test-artifacts",
    ],
  ],
  ["browser/database-dump", ["control.sql"]],
  ["browser/handshake", ["behavior-ready", "evidence.json", "scan-ready"]],
  ["browser/runtime-logs", ["master-runtime.log"]],
  ["browser/test-artifacts", ["gate-attestation.json", "tls-certificate.pem"]],
]) {
  const entries = await readdir(join(runRoot, relativePath));
  entries.sort((left, right) => left.localeCompare(right, "en"));
  assert.deepEqual(
    entries,
    expectedEntries,
    `${relativePath} contains undeclared evidence`,
  );
}

const expectedTargets = new Map([
  ["build_artifacts", { kind: "directory", path: "compiled/bin" }],
  ["database", { kind: "file", path: "browser/database" }],
  [
    "database_dump",
    { kind: "file", path: "browser/database-dump/control.sql" },
  ],
  ["openapi", { kind: "file", path: "compiled/openapi/nodecontroll-v1.json" }],
  ["runtime_logs", { kind: "directory", path: "browser/runtime-logs" }],
  ["test_artifacts", { kind: "directory", path: "browser/test-artifacts" }],
  ["web_dist", { kind: "directory", path: "compiled/web" }],
]);

const requireAsciiPath = (value, label) => {
  assert.match(
    value,
    /^[\x20-\x7e]+$/u,
    `${label} must use printable ASCII for stable ordering`,
  );
};

const regularFiles = async (root) => {
  const metadata = await lstat(root);
  if (metadata.isSymbolicLink())
    throw new Error(`artifact target contains symlink: ${root}`);
  assert.equal(
    metadata.mode & 0o222,
    0,
    `artifact target remains writable: ${root}`,
  );
  if (metadata.isFile()) return [root];
  assert.ok(
    metadata.isDirectory(),
    `artifact target contains a special root: ${root}`,
  );
  const files = [];
  const entries = await readdir(root, { withFileTypes: true });
  entries.sort((left, right) => left.name.localeCompare(right.name, "en"));
  for (const entry of entries) {
    requireAsciiPath(entry.name, "artifact entry name");
    const child = join(root, entry.name);
    if (entry.isSymbolicLink())
      throw new Error(`artifact target contains symlink: ${child}`);
    if (entry.isDirectory()) files.push(...(await regularFiles(child)));
    else if (entry.isFile()) {
      const childMetadata = await lstat(child);
      assert.ok(
        childMetadata.isFile() && !childMetadata.isSymbolicLink(),
        `artifact target contains a special file: ${child}`,
      );
      assert.equal(
        childMetadata.mode & 0o222,
        0,
        `artifact file remains writable: ${child}`,
      );
      files.push(child);
    } else throw new Error(`artifact target contains a special file: ${child}`);
  }
  return files;
};

const calculateTarget = async (label, specification) => {
  const { kind, path: relativePath } = specification;
  requireAsciiPath(relativePath, `${label} relative path`);
  const requested = resolve(runRoot, relativePath);
  assert.ok(
    isWithin(runRoot, requested) && requested !== runRoot,
    `${label} escaped run root`,
  );
  const requestedMetadata = await lstat(requested);
  assert.ok(
    !requestedMetadata.isSymbolicLink(),
    `${label} root must not be a symlink`,
  );
  assert.equal(
    kind === "file"
      ? requestedMetadata.isFile()
      : requestedMetadata.isDirectory(),
    true,
    `${label} root kind differs from ${kind}`,
  );
  assert.equal(
    requestedMetadata.mode & 0o222,
    0,
    `${label} root remains writable`,
  );
  const canonical = await realpath(requested);
  assert.ok(
    isWithin(runRoot, canonical),
    `${label} canonical root escaped run root`,
  );
  assert.equal(
    canonical,
    requested,
    `${label} path contains a symlink or alias`,
  );
  const files = await regularFiles(canonical);
  assert.ok(files.length > 0, `${label} target must not be empty`);
  const treeHash = createHash("sha256");
  let bytes = 0;
  for (const file of files) {
    const body = await readFile(file);
    const relativeFile = relative(canonical, file) || basename(file);
    requireAsciiPath(
      relativeFile.replaceAll(sep, "/"),
      `${label} artifact path`,
    );
    bytes += body.byteLength;
    treeHash.update(Buffer.from(relativeFile.replaceAll(sep, "/"), "utf8"));
    treeHash.update(Buffer.from([0]));
    treeHash.update(Buffer.from(String(body.byteLength), "utf8"));
    treeHash.update(Buffer.from([0]));
    treeHash.update(body);
  }
  return {
    canonical,
    identity: `${requestedMetadata.dev}:${requestedMetadata.ino}`,
    reported: {
      bytes,
      files: files.length,
      label,
      path: relativePath,
      treeSha256: treeHash.digest("hex"),
    },
  };
};

const evidence = JSON.parse(await readFile(evidencePath, "utf8"));
assert.deepEqual(
  Object.keys(evidence).sort(),
  [
    "browserExecutable",
    "browserImageDigest",
    "browserVersion",
    "certificate",
    "cookieContract",
    "crossTabContract",
    "failedMutationContract",
    "gateScriptSha256",
    "loginSessionId",
    "logoutContract",
    "logoutFailureContract",
    "newCsrfStatus",
    "nodeVersion",
    "oldCredentialContract",
    "oldCredentialStatus",
    "oldCsrfStatus",
    "playwrightVersion",
    "postLogoutCredentialStatus",
    "recoveredCredentialStatus",
    "recoverySessionId",
    "rotatedCredentialStatus",
    "rotatedSessionId",
    "runId",
    "scanContract",
    "scanTargets",
    "scannedArtifactBytes",
    "scannedArtifactFiles",
    "sourceRevision",
    "storage",
    "test",
  ].sort(),
);
assert.deepEqual(Object.keys(evidence.browserExecutable ?? {}).sort(), [
  "bytes",
  "name",
  "sha256",
]);
assert.deepEqual(Object.keys(evidence.certificate ?? {}).sort(), [
  "currentlyValid",
  "derSha256",
  "ipSan",
  "keyPairMatched",
  "pemSha256",
  "validFrom",
  "validTo",
]);
assert.deepEqual(Object.keys(evidence.cookieContract ?? {}).sort(), [
  "csrfHttpOnly",
  "host",
  "loginMaxAgeSeconds",
  "rotatedMaxAgeSeconds",
  "sameSite",
  "secure",
  "sessionHttpOnly",
]);
assert.equal(evidence.test, "nodecontroll-auth-c1-https-v3");
assert.equal(evidence.runId, expectedRunId);
assert.equal(evidence.sourceRevision, expectedSourceRevision);
assert.equal(evidence.browserImageDigest, expectedBrowserImageDigest);
assert.equal(evidence.playwrightVersion, expectedPlaywrightVersion);
assert.equal(evidence.nodeVersion, expectedNodeVersion);
assert.equal(evidence.browserVersion, expectedBrowserVersion);
assert.equal(
  evidence.browserExecutable?.sha256,
  expectedBrowserExecutableSha256,
);
assert.equal(evidence.browserExecutable?.name, "chrome");
assert.ok(Number.isSafeInteger(evidence.browserExecutable?.bytes));
assert.ok(evidence.browserExecutable.bytes > 0);
assert.equal(evidence.gateScriptSha256, expectedGateScriptSha256);
assert.equal(evidence.certificate?.pemSha256, expectedTlsCertificateSha256);
assert.equal(evidence.certificate?.ipSan, "127.0.0.1");
assert.equal(evidence.certificate?.keyPairMatched, true);
assert.equal(evidence.certificate?.currentlyValid, true);
assert.match(evidence.certificate?.derSha256, /^[0-9a-f]{64}$/u);
assert.equal(typeof evidence.certificate?.validFrom, "string");
assert.equal(typeof evidence.certificate?.validTo, "string");

assert.equal(evidence.oldCredentialStatus, 401);
assert.equal(evidence.oldCsrfStatus, 403);
assert.equal(evidence.newCsrfStatus, 403);
assert.equal(evidence.rotatedCredentialStatus, 200);
assert.equal(evidence.recoveredCredentialStatus, 200);
assert.equal(evidence.postLogoutCredentialStatus, 401);
assert.equal(typeof evidence.loginSessionId, "string");
assert.ok(evidence.loginSessionId.length > 0);
assert.equal(typeof evidence.rotatedSessionId, "string");
assert.ok(evidence.rotatedSessionId.length > 0);
assert.equal(typeof evidence.recoverySessionId, "string");
assert.ok(evidence.recoverySessionId.length > 0);
assert.notEqual(evidence.loginSessionId, evidence.rotatedSessionId);
assert.notEqual(evidence.rotatedSessionId, evidence.recoverySessionId);
assert.notEqual(evidence.loginSessionId, evidence.recoverySessionId);

assert.equal(evidence.cookieContract?.host, "127.0.0.1");
assert.equal(evidence.cookieContract?.secure, true);
assert.equal(evidence.cookieContract?.sameSite, "Lax");
assert.equal(evidence.cookieContract?.sessionHttpOnly, true);
assert.equal(evidence.cookieContract?.csrfHttpOnly, false);
assert.ok(Number.isSafeInteger(evidence.cookieContract?.loginMaxAgeSeconds));
assert.ok(evidence.cookieContract.loginMaxAgeSeconds > 0);
assert.ok(Number.isSafeInteger(evidence.cookieContract?.rotatedMaxAgeSeconds));
assert.ok(evidence.cookieContract.rotatedMaxAgeSeconds > 0);
assert.ok(
  evidence.cookieContract.rotatedMaxAgeSeconds <=
    evidence.cookieContract.loginMaxAgeSeconds,
  "rotation increased browser Max-Age",
);
assert.deepEqual(evidence.storage, {
  cacheNames: 0,
  indexedDatabases: 0,
  journal: {
    disposition: "reconcile",
    fieldNames: [
      "baseSeq",
      "disposition",
      "epoch",
      "opId",
      "operation",
      "phase",
      "senderId",
      "seq",
      "v",
    ],
    key: "nodecontroll:credential-coordination:v1",
    operation: "logout",
    phase: "settled",
    strictNonSecretSchema: true,
    version: 1,
  },
  localStorageEntriesByPage: [1, 1],
  localStorageKeys: ["nodecontroll:credential-coordination:v1"],
  pagesChecked: 2,
  sessionStorageEntriesByPage: [0, 0],
});
assert.deepEqual(evidence.oldCredentialContract, {
  browserCookiesUnchanged: true,
  contextCleanedAfterProbe: true,
  setCookieFields: 0,
  status: 401,
});
assert.deepEqual(evidence.failedMutationContract, {
  authenticationSetCookieFields: 0,
  cookiesUnchanged: true,
  sessionProjectionUnchanged: true,
});
assert.deepEqual(evidence.logoutFailureContract, {
  authenticationSetCookieFields: 0,
  automaticAuthenticationReadsAfterReload: 0,
  automaticLoginRequestsAfterReload: 0,
  coordinationDisposition: "quarantine",
  coordinationPhase: "settled",
  cookiesUnchanged: true,
  protectedDomClosed: true,
  quarantinePersistedAcrossReload: true,
  recoveredByExplicitLogin: true,
  recoveryLoginStatus: 200,
  sessionProjectionUnchanged: true,
  status: 503,
});
assert.deepEqual(evidence.logoutContract, {
  coordinationJournalSettled: true,
  cookiesCleared: true,
  protectedDomClosed: true,
  status: 204,
});
assert.deepEqual(evidence.crossTabContract, {
  explicitLoginRecoveredBothPages: true,
  lateStaleInvalidationBoundToOldCursor: true,
  lateStaleInvalidationDelivered: true,
  lateStaleInvalidationIgnored: true,
  lateStaleInvalidationStructurallyValid: true,
  newerCredentialStatus: 200,
  newerProtectedDomRetainedAcrossPages: true,
  newerSessionPreserved: true,
  pages: 2,
  remoteMutationProtectedDomClosed: true,
});

assert.deepEqual(evidence.scanContract, [
  { kind: "directory", label: "build_artifacts", path: "compiled/bin" },
  { kind: "file", label: "database", path: "browser/database" },
  {
    kind: "file",
    label: "database_dump",
    path: "browser/database-dump/control.sql",
  },
  {
    kind: "file",
    label: "openapi",
    path: "compiled/openapi/nodecontroll-v1.json",
  },
  { kind: "directory", label: "runtime_logs", path: "browser/runtime-logs" },
  {
    kind: "directory",
    label: "test_artifacts",
    path: "browser/test-artifacts",
  },
  { kind: "directory", label: "web_dist", path: "compiled/web" },
]);

const gateAttestation = JSON.parse(
  await readFile(
    join(runRoot, "browser/test-artifacts/gate-attestation.json"),
    "utf8",
  ),
);
assert.deepEqual(Object.keys(gateAttestation).sort(), [
  "browser_image_digest",
  "run_id",
  "source_revision",
]);
assert.deepEqual(gateAttestation, {
  browser_image_digest: expectedBrowserImageDigest,
  run_id: expectedRunId,
  source_revision: expectedSourceRevision,
});
const frozenCertificate = await readFile(
  join(runRoot, "browser/test-artifacts/tls-certificate.pem"),
);
assert.equal(
  createHash("sha256").update(frozenCertificate).digest("hex"),
  expectedTlsCertificateSha256,
);
for (const forbiddenPath of [
  ...["-wal", "-shm", "-journal"].map(
    (suffix) => `${join(runRoot, "browser/database")}${suffix}`,
  ),
  join(runRoot, "browser/.database.temporary"),
  join(runRoot, "browser/.database.temporary-wal"),
  join(runRoot, "browser/.database.temporary-shm"),
  join(runRoot, "browser/.database.temporary-journal"),
  join(runRoot, "browser/database-dump/.control.sql.temporary"),
]) {
  await assert.rejects(
    lstat(forbiddenPath),
    (error) => error?.code === "ENOENT",
  );
}

assert.ok(
  Array.isArray(evidence.scanTargets),
  "evidence scanTargets must be an array",
);
assert.equal(evidence.scanTargets.length, expectedTargets.size);
const reported = new Map();
for (const target of evidence.scanTargets) {
  assert.ok(target && typeof target === "object" && !Array.isArray(target));
  assert.equal(typeof target.label, "string");
  assert.ok(
    !reported.has(target.label),
    `duplicate evidence target ${target.label}`,
  );
  assert.equal(
    target.path,
    expectedTargets.get(target.label)?.path,
    `unexpected path for ${target.label}`,
  );
  reported.set(target.label, target);
}
assert.deepEqual(
  [...reported.keys()].sort(),
  [...expectedTargets.keys()].sort(),
);

let totalBytes = 0;
let totalFiles = 0;
const canonicalTargets = [];
const targetIdentities = new Set();
for (const [label, specification] of expectedTargets) {
  const actual = await calculateTarget(label, specification);
  for (const previous of canonicalTargets) {
    assert.ok(
      !isWithin(previous, actual.canonical) &&
        !isWithin(actual.canonical, previous),
      `${label} overlaps another canonical target`,
    );
  }
  assert.ok(
    !targetIdentities.has(actual.identity),
    `${label} aliases another target root`,
  );
  canonicalTargets.push(actual.canonical);
  targetIdentities.add(actual.identity);
  assert.deepEqual(
    reported.get(label),
    actual.reported,
    `${label} tree evidence changed after browser scan`,
  );
  totalBytes += actual.reported.bytes;
  totalFiles += actual.reported.files;
}
assert.equal(evidence.scannedArtifactBytes, totalBytes);
assert.equal(evidence.scannedArtifactFiles, totalFiles);

process.stdout.write(
  `verified auth E2E evidence for ${expectedRunId}: ${totalFiles} files, ${totalBytes} bytes\n`,
);
