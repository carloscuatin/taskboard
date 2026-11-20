export default {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'scope-enum': [
      2,
      'always',
      ['api', 'web', 'domain', 'application', 'adapters', 'db', 'auth', 'ci', 'docs', 'deps'],
    ],
    'body-max-line-length': [2, 'always', 100],
  },
};
