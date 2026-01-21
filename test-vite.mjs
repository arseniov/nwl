const { createRequire } = require('module');
const require = createRequire(import.meta.url);

const NumberField = require('@base-ui/react/number-field');
console.log('NumberField (CommonJS):', typeof NumberField);
console.log('NumberField keys:', Object.keys(NumberField));
console.log('NumberField.Root:', typeof NumberField.Root);
