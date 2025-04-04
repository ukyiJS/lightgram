import globals from 'globals';

import { eslintFormatConfig } from './config/eslint/eslint-format.config.mjs';
import { eslintJavascriptConfig } from './config/eslint/eslint-javascript.config.mjs';
import { eslintReactConfig } from './config/eslint/eslint-react.config.mjs';
import { eslintTypescriptConfig } from './config/eslint/eslint-typescript.config.mjs';

export default [
  { languageOptions: { globals: globals.browser } },
  ...eslintFormatConfig,
  ...eslintJavascriptConfig,
  ...eslintTypescriptConfig,
  ...eslintReactConfig,
  {
    ignores: [
      'node_modules',
      'dist',
    ],
  },
];
