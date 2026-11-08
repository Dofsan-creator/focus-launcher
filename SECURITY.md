# Security

Focus Launcher stops services through an optional elevated Windows service (`focus-guardian`). Its attack surface is kept small on purpose:

- it accepts only a service name, checked against a list compiled into the binary;
- its named pipe is open only to SYSTEM, administrators and interactive users;
- it writes its journal under `ProgramData`, writable by administrators only;
- it never runs a program or reads a path sent by a client.

Please report vulnerabilities privately through the repository's security advisory feature, not in a public issue. Include the version, steps to reproduce and impact.
