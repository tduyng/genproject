import importPlugin from 'eslint-plugin-import'
import globals from 'globals'
import tsParser from '@typescript-eslint/parser'
import eslintJs from '@eslint/js'
import eslintTs from 'typescript-eslint'

const ignores = ['build/*']
const tsFiles = ['src/**/*.ts']

const commonJsRules = {
    'no-duplicate-imports': 'error',
    'no-unneeded-ternary': 'error',
    'prefer-object-spread': 'error',

    'no-unused-vars': [
        'error',
        {
            ignoreRestSiblings: true,
            args: 'none',
        },
    ],
}

const languageOptions = {
    globals: {
        ...globals.node,
    },
    ecmaVersion: 2023,
    sourceType: 'module',
}

const esmConfig = {
    files: ['src/**/*.{js,mjs}'],
    ignores,
    languageOptions,
    rules: commonJsRules,
}

const typescriptConfig = {
    files: tsFiles,
    ignores,
    plugins: {
        import: importPlugin,
        'import/parsers': tsParser,
    },
    languageOptions: {
        ...languageOptions,
        parser: tsParser,
        parserOptions: {
            project: './tsconfig.json',
        },
    },
    settings: {
        'import/internal-regex': '^\\.\\.?/',
        'import/parsers': {
            '@typescript-eslint/parser': ['.ts'],
        },
    },
    rules: {
        'import/export': 'error',
        'import/no-duplicates': 'warn',
        ...importPlugin.configs.typescript.rules,
        'no-console': 'warn',
        'no-duplicate-imports': 'error',
        'no-return-await': 'error',
        'no-unneeded-ternary': 'error',
        'no-unused-vars': ['off'],
        'prefer-object-spread': 'error',
        'require-await': 'off',
        '@typescript-eslint/no-unsafe-argument': 'warn',
        '@typescript-eslint/explicit-function-return-type': 'error',
        '@typescript-eslint/no-unnecessary-condition': 'error',
        '@typescript-eslint/naming-convention': [
            'warn',
            {
                selector: 'default',
                format: ['camelCase'],
                leadingUnderscore: 'forbid',
                trailingUnderscore: 'forbid',
            },
            {
                selector: 'variable',
                format: ['camelCase', 'UPPER_CASE', 'PascalCase'],
            },
            {
                selector: 'property',
                format: ['camelCase', 'PascalCase'],
            },
            {
                selector: 'parameter',
                format: ['camelCase'],
                leadingUnderscore: 'allow',
            },
            {
                selector: 'typeLike',
                format: ['PascalCase'],
            },
            {
                selector: 'enumMember',
                format: ['PascalCase'],
            },
        ],

        '@typescript-eslint/no-unsafe-assignment': 'off',
        '@typescript-eslint/no-unsafe-call': 'warn',
        '@typescript-eslint/no-unsafe-member-access': 'off',
        '@typescript-eslint/no-unsafe-return': 'warn',
        '@typescript-eslint/no-unused-vars': ['error', { ignoreRestSiblings: true, args: 'none' }],
        '@typescript-eslint/no-use-before-define': 'off',
        '@typescript-eslint/prefer-optional-chain': 'error',
        '@typescript-eslint/restrict-template-expressions': 'warn',

        'import/order': [
            'error',
            {
                groups: [
                    'builtin',
                    'external',
                    ['internal', 'parent', 'sibling', 'index'],
                    'unknown',
                ],

                'newlines-between': 'always',

                alphabetize: {
                    order: 'asc',
                    caseInsensitive: true,
                },
            },
        ],

        '@typescript-eslint/no-misused-promises': [
            'error',
            {
                checksVoidReturn: false,
            },
        ],

        '@typescript-eslint/prefer-nullish-coalescing': 'off',
        '@typescript-eslint/non-nullable-type-assertion-style': 'off',
        '@typescript-eslint/no-base-to-string': 'off',
        '@typescript-eslint/no-invalid-void-type': 'warn',
        '@typescript-eslint/consistent-type-definitions': 'off',
        '@typescript-eslint/ban-ts-comment': 'warn',
        '@typescript-eslint/no-unused-expressions': 'off',
    },
}

const typescriptRecommendedConfigs = [
    ...eslintTs.configs.recommended.map((config) => ({
        ...config,
        files: tsFiles,
        ignores,
    })),
    ...eslintTs.configs.stylistic.map((config) => ({
        ...config,
        files: tsFiles,
        ignores,
    })),
]

export default [
    eslintJs.configs.recommended,
    esmConfig,
    ...typescriptRecommendedConfigs,
    typescriptConfig,
]
