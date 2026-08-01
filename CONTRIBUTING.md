\# Contributing to RELIQUARY



RELIQUARY is an early-stage medical-device trust-anchor and evidence-notary

project targeting the DEF CON 34 badge and its removable Baochip-based

security module.



The project is currently in planning and architecture development. Hardware

behavior, firmware lifecycle details, recovery procedures, pin assignments,

and protected-key capabilities remain provisional until they are verified

against the physical badge and official documentation.



\## Current contribution status



The final project license and contribution terms have not yet been selected.



Until those decisions are complete:



\- substantial external code contributions should not be submitted;

\- contributors may open design discussions and documentation proposals;

\- no contributor should assume that repository publication grants permission

&#x20; to reuse, modify, or redistribute the project;

\- hardware-specific claims must be clearly labeled as verified, observed, or

&#x20; provisional.



See `LICENSE-NOTICE.md` for the current licensing status.



\## Project principles



Contributions should preserve the following principles:



\- Keep the implementation inspectable.

\- Keep the protocol small.

\- Keep parsers simple and strictly bounded.

\- Use deterministic, versioned evidence formats.

\- Require clear operator intent for every signing action.

\- Treat the connected host as untrusted.

\- Keep private signing keys on the trusted device whenever hardware support

&#x20; permits it.

\- Fail closed when requests are malformed, ambiguous, or incomplete.

\- Do not collect patient data by default.

\- Do not claim that a valid signature proves that firmware or a medical device

&#x20; is safe.

\- Document anything that can permanently change the badge or its security

&#x20; state.



\## Areas of interest



Useful contribution areas include:



\- Rust

\- Xous

\- USB protocol design

\- CBOR

\- CDDL

\- Ed25519

\- hardware security

\- medical-device security

\- DICOM

\- firmware signing

\- threat modeling

\- user-interface security

\- hardware teardown

\- reproducible builds

\- fuzzing

\- test-vector development



\## Development workflow



Start from an updated `main` branch:



```text

git switch main

git pull --ff-only

Create a focused branch:

git switch -c feature/short-description

Use clear branch prefixes where practical:

feature/
fix/
docs/
schema/
firmware/
host/
threat-model/

Keep commits focused and use descriptive commit messages.

Examples:

Add initial evidence manifest schema
Document USB trust boundary
Add Ed25519 verification test vectors
Clarify operator confirmation requirements

Before proposing a change, review the working tree:

git status
git diff
git diff --check
Security-sensitive changes

Changes affecting any of the following require explicit security review:

private-key generation or storage;
signature authorization;
USB request parsing;
evidence canonicalization;
nonce generation;
replay protection;
firmware updates;
rollback behavior;
developer mode;
key enrollment;
display truncation;
Unicode handling;
recovery procedures;
debug interfaces.

Security-sensitive changes should include:

the affected trust boundary;
expected security behavior;
failure behavior;
relevant abuse cases;
tests or test vectors;
hardware and firmware assumptions.
Hardware claims

Do not present early announcements, teaser images, inferred pinouts, or
unverified behavior as established fact.

Hardware information should be classified as one of:

verified: confirmed from official documentation or repeatable testing;
observed: measured directly on available hardware;
provisional: inferred from incomplete information;
unknown: not yet determined.

Do not erase firmware, change permanent security state, enable debug access, or
load private keys until recovery behavior is understood.

Test data

Only fictional, synthetic, or properly authorized test data may be committed.

Do not commit:

patient records;
protected health information;
production DICOM studies;
private signing keys;
passwords;
access tokens;
production certificates;
manufacturer secrets;
confidential firmware;
uncontrolled vulnerability data.

DICOM test data should be synthetic or appropriately de-identified.

Pull-request expectations

A proposed change should explain:

what changed;
why the change is needed;
which component is affected;
whether the change affects the threat model;
how the change was tested;
which hardware assumptions it depends on;
whether documentation or schemas also need updates.

Changes to serialization or signing behavior should include deterministic test
vectors.

Security reports

Do not publish suspected vulnerabilities as ordinary public GitHub issues.

A private disclosure process will be documented before the project begins
accepting security reports from outside contributors.

Until then, preserve the technical details privately and record:

affected commit;
hardware version;
firmware version;
reproduction steps;
expected behavior;
actual behavior;
security impact;
suggested mitigation, when known.

