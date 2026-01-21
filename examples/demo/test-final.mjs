// Verify named imports work correctly
import { Switch } from '@base-ui/react/switch';
import { NumberField } from '@base-ui/react/number-field';
import { Checkbox } from '@base-ui/react/checkbox';
import { Separator } from '@base-ui/react/separator';

console.log('Switch:', Switch);
console.log('Switch.Root:', Switch.Root);
console.log('Switch.Thumb:', Switch.Thumb);

console.log('NumberField:', NumberField);
console.log('NumberField.Root:', NumberField.Root);
console.log('NumberField.Input:', NumberField.Input);

console.log('Checkbox:', Checkbox);
console.log('Checkbox.Root:', Checkbox.Root);
console.log('Checkbox.Indicator:', Checkbox.Indicator);

console.log('Separator:', Separator);
console.log('Separator.Root:', Separator.Root);

console.log('\nAll imports working correctly!');
