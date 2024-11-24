export default {
    roots: ['<rootDir>'],
    collectCoverage: false,
    collectCoverageFrom: ['<rootDir>/src/**/*.ts', '!<rootDir>/src/**/definitions.ts'],
    coverageDirectory: 'coverage',
    coveragePathIgnorePatterns: ['<rootDir>/node_modules/', '<rootDir>/build/'],
    coverageProvider: 'v8',
    coverageReporters: ['lcov', 'text', 'text-summary', 'cobertura'],
    transform: {
        '^.+\\.(t|j)s$': [
            'ts-jest',
            {
                isolatedModules: true, //Disable type-checking
            },
        ],
    },
    testEnvironment: 'node',
    testMatch: ['<rootDir>/tests/unit/!(_)**/*.test.ts'],
    verbose: false,
};