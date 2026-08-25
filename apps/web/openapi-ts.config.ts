import { defineConfig } from '@hey-api/openapi-ts'

export default defineConfig({
  input: '../../openapi/nodecontroll-v1.json',
  output: {
    path: 'src/api/generated',
    clean: true,
    header: ['/* Generated from the Rust OpenAPI document. Do not edit. */'],
  },
})

