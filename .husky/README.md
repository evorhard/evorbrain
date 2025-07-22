# Git Hooks

This directory contains Git hooks managed by Husky to ensure code quality and consistent commit messages.

## Hooks

### pre-commit
Runs before each commit to:
- Run ESLint on staged JavaScript/TypeScript files
- Format code with Prettier
- Fix any auto-fixable linting issues

### commit-msg
Validates commit messages to ensure they follow the [Conventional Commits](https://www.conventionalcommits.org/) format:
```
<type>(<scope>): <subject>

<body>

<footer>
```

### prepare-commit-msg
Provides a helpful commit message template when creating new commits.

## Conventional Commit Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools
- **build**: Changes that affect the build system or external dependencies
- **ci**: Changes to CI configuration files and scripts
- **revert**: Reverts a previous commit

## Examples

```bash
feat: add user authentication system
fix(api): resolve database connection timeout
docs: update installation instructions
refactor(core): simplify event handling logic
```

## Bypassing Hooks

If you need to bypass hooks in an emergency (not recommended):
```bash
git commit --no-verify -m "emergency fix"
```

## Troubleshooting

If hooks are not running:
1. Ensure Husky is installed: `pnpm install`
2. Ensure hooks are executable: `chmod +x .husky/*`
3. Check that Git hooks path is set: `git config core.hooksPath`