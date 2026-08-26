import fs from 'node:fs';

const file = process.argv[2] ?? 'openapi/nodecontroll-v1.json';
const document = JSON.parse(fs.readFileSync(file, 'utf8'));
const requiredPaths = ['/healthz', '/readyz', '/api/v1/system/version'];
const errors = [];
const canonicalRecoveryCodePattern = '^[0-9a-f]{4}(?:-[0-9a-f]{4}){7}$';
const maxSafeInteger = Number.MAX_SAFE_INTEGER;

if (!String(document.openapi).startsWith('3.1')) {
  errors.push(`expected OpenAPI 3.1, got ${document.openapi}`);
}

for (const path of requiredPaths) {
  if (!document.paths?.[path]) errors.push(`missing path ${path}`);
}

const operationIds = [];
for (const pathItem of Object.values(document.paths ?? {})) {
  for (const operation of Object.values(pathItem ?? {})) {
    if (!operation || typeof operation !== 'object' || !('responses' in operation)) continue;
    if (!operation.operationId) errors.push('operation without operationId');
    else operationIds.push(operation.operationId);
  }
}

const duplicates = operationIds.filter((id, index) => operationIds.indexOf(id) !== index);
if (duplicates.length) errors.push(`duplicate operationIds: ${[...new Set(duplicates)].join(', ')}`);

for (const schemaName of ['BootstrapCreated', 'RecoveryCodesCreatedData']) {
  const schema = document.components?.schemas?.[schemaName]?.properties?.one_time_recovery_codes;
  if (!schema) {
    errors.push(`missing ${schemaName}.one_time_recovery_codes schema`);
    continue;
  }
  if (schema.type !== 'array' || schema.minItems !== 8 || schema.maxItems !== 8) {
    errors.push(`${schemaName}.one_time_recovery_codes must be an exact eight-item array`);
  }
  if (
    schema.items?.type !== 'string' ||
    schema.items?.minLength !== 39 ||
    schema.items?.maxLength !== 39 ||
    schema.items?.pattern !== canonicalRecoveryCodePattern
  ) {
    errors.push(`${schemaName}.one_time_recovery_codes items must use the canonical wire format`);
  }
}

const expectedRecoveryMetadataBounds = {
  created_at_ms: [0, maxSafeInteger],
  remaining_count: [0, 8],
  set_version: [1, maxSafeInteger],
  total_count: [8, 8],
};
const recoverySummaryProperties = document.components?.schemas?.RecoveryCodeSummaryData?.properties;
for (const [property, [minimum, maximum]] of Object.entries(expectedRecoveryMetadataBounds)) {
  const schema = recoverySummaryProperties?.[property];
  if (
    !schema ||
    schema.minimum !== minimum ||
    schema.maximum !== maximum
  ) {
    errors.push(`RecoveryCodeSummaryData.${property} has incomplete numeric bounds`);
  }
}

const recoveryCreatedProperties = document.components?.schemas?.RecoveryCodesCreatedData?.properties;
for (const [property, minimum] of [
  ['created_at_ms', 0],
  ['set_version', 1],
]) {
  const schema = recoveryCreatedProperties?.[property];
  if (
    !schema ||
    schema.minimum !== minimum ||
    schema.maximum !== maxSafeInteger
  ) {
    errors.push(`RecoveryCodesCreatedData.${property} has incomplete numeric bounds`);
  }
}

if (errors.length) {
  for (const error of errors) console.error(error);
  process.exit(1);
}

console.log(JSON.stringify({
  openapi: document.openapi,
  paths: Object.keys(document.paths).length,
  operations: operationIds.length,
  operation_ids: operationIds.sort(),
}, null, 2));
