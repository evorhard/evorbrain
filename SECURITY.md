# Security Policy

## Supported Versions

I release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

I take security bugs seriously. I appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### Where to Report

To report a security vulnerability, please use one of the following methods:

1. **Email**: security@evorbrain.dev (preferred)
2. **GitHub Security Advisory**: [Create a security advisory](https://github.com/yourusername/evorbrain/security/advisories/new)

**Please do not report security vulnerabilities through public GitHub issues.**

### What to Include

When reporting a vulnerability, please include:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Timeline

- **Initial Response**: Within 48 hours, I will acknowledge receipt of your vulnerability report
- **Status Update**: Within 7 days, I will provide an initial assessment and expected timeline
- **Resolution**: I aim to resolve critical vulnerabilities within 30 days

### Disclosure Policy

- I will confirm the vulnerability and determine its impact
- I will release a fix as soon as possible, depending on complexity
- I will credit you for the discovery (unless you prefer to remain anonymous)
- I will publish a security advisory once the fix is released

## Security Best Practices

### For Users

1. **Keep EvorBrain Updated**: Always use the latest version
2. **Secure Your Data Directory**: Ensure proper file permissions on your data folder
3. **Backup Regularly**: Maintain backups of your markdown files
4. **Use Disk Encryption**: Enable full-disk encryption on your device
5. **Be Cautious with Plugins**: Only install plugins from trusted sources

### For Developers

1. **Code Reviews**: All code must be reviewed before merging
2. **Dependency Updates**: Keep all dependencies up to date
3. **Input Validation**: Validate all user inputs
4. **Secure Communication**: Use HTTPS for any network features
5. **Least Privilege**: Follow the principle of least privilege

## Security Features

### Data Protection

- **Local Storage Only**: All data stored locally, no cloud sync by default
- **Encryption at Rest**: Support for encrypted storage volumes
- **No Analytics**: Zero telemetry or usage tracking
- **Secure IPC**: Tauri's secure inter-process communication

### Application Security

- **Context Isolation**: Renderer processes isolated from system access
- **CSP Headers**: Strict Content Security Policy
- **Sandboxing**: Application runs in a sandboxed environment
- **Auto-Updates**: Secure auto-update mechanism (opt-in)

### Plugin Security

- **Permission System**: Plugins must declare required permissions
- **Sandboxed Execution**: Plugins run in isolated contexts
- **Code Signing**: Optional plugin signature verification
- **Review Process**: Official plugins undergo security review

## Known Security Limitations

1. **Local Access**: Anyone with physical access to your computer can access your data
2. **No Built-in Encryption**: Files are stored as plain markdown (use OS-level encryption)
3. **Plugin Trust**: Installing plugins requires trusting the plugin author

## Security Checklist for Contributors

Before submitting a pull request:

- [ ] No hardcoded secrets or credentials
- [ ] All user inputs are validated and sanitized
- [ ] Dependencies are up to date
- [ ] No use of deprecated or unsafe functions
- [ ] Error messages don't expose sensitive information
- [ ] File operations use safe paths (no path traversal)
- [ ] SQL queries use parameterized statements
- [ ] External content is properly sandboxed

## Contact

For any security-related questions that don't require immediate attention, please contact:
- Email: security@evorbrain.dev
- PGP Key: [Download](https://evorbrain.dev/security.asc)

## Acknowledgments

I would like to thank the following individuals for responsibly disclosing vulnerabilities:

- *Your name could be here!*

---

Last updated: July 2025