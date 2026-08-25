import fs from 'node:fs';

const file = process.argv[2] ?? 'openapi/nodecontroll-v1.json';
const document = JSON.parse(fs.readFileSync(file, 'utf8'));
const requiredPaths = ['/healthz', '/readyz', '/api/v1/system/version'];
const errors = [];

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
