# Reliquary
Medical Device Trust Anchor and evidence notary - DC 34 Human Badge hack

# RELIQUARY

Hardware backed evidence signing for medical device labs, CTFs, firmware testing and product security work.

This started with the DEF CON 34 badge and its removable Baochip security module.

The idea is pretty simple.

A laptop or lab system hashes something. Firmware, DICOM file, SBOM, test report, challenge token, whatever. RELIQUARY shows what is about to be signed, makes the operator physically approve it, then returns a signed evidence record.

No silent signing. No dumping private keys onto a laptop. No pretending a SHA256 in a text file is a chain of custody.

This is early. Like very early.

We dont have the final DC34 badge schematics, pinout, firmware process or recovery procedure yet. Anything hardware specific is considered provisional until we have the actual badge in hand and the official docs are released.

## What its for

RELIQUARY is meant to be a portable trust anchor for security work.

Some of the planned uses:

- Sign firmware assessment results
- Verify medical device firmware packages
- Create signed DICOM evidence records
- Sign CTF completion tokens
- Protect organizer side challenge keys
- Sign SBOM and release manifests
- Record hardware inspection results
- Create evidence receipts during lab testing
- Verify that an artifact has not changed since testing
- Keep signing keys off the host doing the testing

This is not meant to replace a production HSM.

It is also not a crypto wallet, password dumping toy or magic compliance box.

## Basic flow

```text
Artifact
   |
   v
Host calculates SHA256
   |
   v
Host sends hash and metadata to RELIQUARY
   |
   v
RELIQUARY displays what is being signed
   |
   v
Operator approves with a physical button
   |
   v
RELIQUARY signs the evidence manifest
   |
   v
Host stores artifact + manifest + signature
```

The badge should never need the full artifact for the normal signing flow.

It gets the digest and enough metadata for the human to understand what they are approving.

## First version

The first working version only needs to do four things.

1. Accept a SHA256 digest and metadata over USB
2. Display the requested operation and hash
3. Require physical confirmation
4. Return a signed evidence manifest

Thats it.

QR enrollment, DICOM parsing, CAN adapters, fancy UI stuff and hardware bus support can come later. The first version needs to work before it gets interesting.

## Example evidence record

```json
{
  "schema": "reliquary-evidence-v1",
  "artifact_type": "firmware",
  "artifact_name": "pump-controller-4.2.1.bin",
  "artifact_sha256": "4f5c9cf4f2d3887d90e75989fbd9c2e50c2897efbd6ce6ce9b9e1c58a39ca901",
  "device_class": "infusion-pump-simulator",
  "device_identifier": "LAB-PUMP-007",
  "assessment": "authorized-lab-test",
  "timestamp": "2026-08-08T18:42:31Z",
  "operator_key_id": "reliquary-bhv-01",
  "nonce": "83b1dcff31c7d87c",
  "signature_algorithm": "ed25519",
  "signature": "BASE64_SIGNATURE_HERE"
}
```

JSON is easy to read but the device side format will probably be CBOR.

The schema will be versioned so old evidence records dont become useless every time something changes.

## Example CLI

The CLI does not exist yet. This is the direction.

```bash
reliquary sign \
  --artifact pump-controller-4.2.1.bin \
  --type firmware \
  --device-class infusion-pump-simulator \
  --device-id LAB-PUMP-007 \
  --assessment authorized-lab-test \
  --output pump-controller-4.2.1.evidence.cbor
```

Verify it later:

```bash
reliquary verify \
  --artifact pump-controller-4.2.1.bin \
  --evidence pump-controller-4.2.1.evidence.cbor \
  --public-key reliquary-bhv-01.pub
```

Expected output:

```text
Artifact hash: VALID
Manifest signature: VALID
Signer: reliquary-bhv-01
Artifact type: firmware
Device: LAB-PUMP-007
Assessment: authorized-lab-test
```

## DICOM evidence mode

RELIQUARY will not store patient studies.

The host system will parse the DICOM object, select the allowed metadata and send a privacy limited evidence request to the badge.

A DICOM evidence record may contain:

- File SHA256
- Pixel data SHA256
- SOP Class UID
- Hashed SOP Instance UID
- Modality
- Transfer Syntax UID
- Parser validation result
- Test case identifier
- Timestamp
- Operator key identifier
- Signature

Patient names should not be placed into the signed record.

Patient IDs should be omitted or hashed unless there is a real reason to include them.

Example:

```json
{
  "schema": "reliquary-dicom-v1",
  "artifact_type": "dicom",
  "file_sha256": "7b2173f0...",
  "pixel_data_sha256": "32995f0a...",
  "sop_class_uid": "1.2.840.10008.5.1.4.1.1.2",
  "sop_instance_uid_sha256": "723b47a1...",
  "modality": "CT",
  "transfer_syntax_uid": "1.2.840.10008.1.2.1",
  "validation": "passed",
  "test_case": "SPICY-DICOM-014",
  "operator_key_id": "reliquary-bhv-01",
  "signature": "BASE64_SIGNATURE_HERE"
}
```

## Firmware verification mode

Another planned mode is firmware release verification.

The badge will store or enroll trusted public keys. The host sends a firmware digest and release manifest. RELIQUARY verifies the release signature and shows the result on the badge.

```text
VERIFIED

Signer:
BHV Test Manufacturer CA

Product:
Simulated Infusion Controller

Version:
4.2.1

SHA256:
4f5c...a901
```

Possible checks:

- Release signature
- Firmware digest
- Product identifier
- Version
- Rollback counter
- Expiration policy
- Allowed signer
- Lab or production classification

Production manufacturer keys should not be copied onto random conference hardware.

This project is for controlled labs, research and test environments unless a proper production security review says otherwise.

## CTF mode

RELIQUARY can also act as a physical trust anchor for CTF infrastructure.

Possible uses:

- Sign challenge completion tokens
- Issue short lived credentials
- Verify offline QR challenge tokens
- Protect organizer side secrets
- Bridge isolated challenge networks
- Generate signed receipts without exposing the flag
- Validate that a hardware challenge was completed on a real device

Example challenge token:

```json
{
  "schema": "reliquary-ctf-v1",
  "event": "BHV",
  "challenge": "medical-firmware-verification",
  "team": "team-042",
  "result": "completed",
  "nonce": "a13e99b2d091",
  "issued_at": "2026-08-08T21:14:00Z",
  "expires_at": "2026-08-08T22:14:00Z",
  "signer": "reliquary-organizer-01",
  "signature": "BASE64_SIGNATURE_HERE"
}
```

The badge should sign a completion claim.

It should not hand the player the private key or store every challenge flag in one easy to lose device.

## Security rules

Some rules for this project from day one.

### Every signature requires intent

The device must show enough information for the operator to understand what is being signed.

A request cannot just say:

```text
Approve?
```

It should say something closer to:

```text
SIGN FIRMWARE EVIDENCE

Device:
LAB-PUMP-007

SHA256:
4f5c9cf4...a901

Assessment:
authorized-lab-test

CONFIRM / DENY
```

### No automatic signing

No background service should be able to silently use the key.

No auto approve mode.

No hidden developer shortcut that becomes the normal workflow later.

### Keys stay on the device

Private signing keys should be generated on the device when possible.

They should not be exportable through the normal interface.

Public keys can be exported and published.

### Untrusted host

The connected laptop is considered untrusted.

It can lie about metadata.

It can send malformed messages.

It can replay requests.

It can disconnect halfway through an operation.

It can try to confuse the operator with long names, Unicode tricks or truncated hashes.

The badge UI needs to be designed with that in mind.

### No patient data by default

Do not send full patient records to the badge.

Do not store DICOM studies on it.

Do not place patient names into evidence records.

Keep the signed data limited to what is needed for verification.

### Signed does not mean safe

RELIQUARY proves that a key signed a specific claim.

It does not prove the firmware is secure.

It does not prove a medical device is safe.

It does not prove the operator performed a good assessment.

It gives us integrity, identity and a better audit trail. Thats all.

## Threat model

Things we care about:

- Host malware submitting fake evidence
- Operator approving the wrong artifact
- USB protocol parsing bugs
- Replay attacks
- Signature request confusion
- Key extraction
- Malicious firmware updates
- Rollback to vulnerable firmware
- Unicode and display truncation attacks
- Counterfeit evidence records
- Stolen or cloned badges
- Unauthorized key enrollment
- Bad random number generation
- Debug interfaces left enabled
- A developer build being mistaken for a trusted build
- Device loss
- Supply chain tampering

Things the first version probably will not solve:

- A fully compromised badge firmware
- Physical attacks from a well funded lab
- Evil maid attacks with unlimited access
- Compromised manufacturing infrastructure
- Operators approving things without reading the screen
- Time accuracy without a trusted time source

The threat model will live in `threat-model/` and should be updated as the hardware becomes better understood.

## Planned repo layout

```text
reliquary/
├── README.md
├── LICENSE
├── firmware/
│   ├── xous-app/
│   ├── signing-service/
│   ├── transport/
│   └── ui/
├── host/
│   ├── reliquary-cli/
│   ├── usb-transport/
│   ├── manifest/
│   └── verification/
├── schemas/
│   ├── evidence-v1.cddl
│   ├── evidence-v1.schema.json
│   ├── dicom-v1.cddl
│   └── ctf-v1.cddl
├── threat-model/
│   ├── assets.md
│   ├── trust-boundaries.md
│   ├── abuse-cases.md
│   └── assumptions.md
├── hardware/
│   ├── notes/
│   ├── photos/
│   ├── pinout/
│   └── adapters/
├── docs/
│   ├── acquisition-checklist.md
│   ├── recovery.md
│   ├── key-management.md
│   └── protocol.md
└── test-data/
    ├── firmware/
    ├── dicom/
    └── manifests/
```

## Project phases

### Phase 0: Dont brick the badge

Before flashing anything:

- Photograph all hardware
- Record chip markings
- Record USB descriptors
- Capture normal boot output
- Record firmware versions
- Find the official recovery process
- Find the official factory image
- Understand key lifecycle behavior
- Understand developer mode behavior
- Confirm whether flashing changes one way security state
- Confirm how application updates are installed
- Preserve the original firmware

Nothing gets erased until we know how to put it back.

### Phase 1: Host side prototype

Build the evidence format and CLI without needing the real badge.

The first prototype can use a temporary software key so we can finish:

- Manifest schema
- Canonical serialization
- Hashing
- Signature verification
- CLI behavior
- Error handling
- Test vectors

The software key is only for development.

It is not the final security model.

### Phase 2: Badge signing service

Move key generation and signing onto the badge.

Requirements:

- Device generated Ed25519 key
- Public key export
- USB request parser
- Physical confirmation
- Clear on screen metadata
- Signed CBOR response
- Request nonce
- Replay protection
- Error reporting

### Phase 3: Medical device profiles

Add profiles for:

- Firmware
- DICOM
- SBOM
- Vulnerability assessment result
- Device configuration baseline
- Hardware inspection
- CTF completion token

### Phase 4: Hardware adapters

Only after the official electrical details are known.

Possible adapters:

- UART
- SPI
- I2C
- CAN
- SWD read only acquisition
- JTAG identification
- QR based offline transfer

External interfaces need isolation and level shifting.

Do not wire a mystery badge pin directly into a medical device board because the connector looked close enough.

## Current status

```text
Project name: RELIQUARY
Status: Planning
Hardware: Waiting on DEF CON 34 badge
Host CLI: Not started
Firmware app: Not started
Evidence schema: Drafting
Threat model: Drafting
DICOM profile: Planned
CTF profile: Planned
Hardware adapters: Blocked on official pinout
```

## Hardware assumptions

Current assumptions based on the early badge announcement:

- The badge uses a removable Baochip based security module
- It has a display
- It has physical buttons
- It supports security token style use cases
- It is intended to be inspectable and hackable
- It may support custom Xous applications
- It may expose USB functionality
- It may contain protected key storage and cryptographic hardware

These are assumptions until verified against the actual DC34 hardware and documentation.

Do not build permanent hardware around teaser images.

## Development principles

Keep it inspectable.

Keep the protocol small.

Keep the parser boring.

Make signed data deterministic.

Use test vectors.

Fail closed.

Show the operator what matters.

Do not hide dangerous behavior behind a settings menu.

Do not collect patient data because it might be useful later.

Do not call something secure just because the key is inside a chip.

Document anything that can permanently change the badge.

## Contributing

Pull requests are welcome once there is something here to pull against.

Useful areas:

- Rust
- Xous
- USB protocol design
- CBOR and CDDL
- Ed25519
- Hardware security
- Medical device security
- DICOM
- Firmware signing
- Threat modeling
- UI security
- Hardware teardown
- Reproducible builds
- Fuzzing

Security issues should not be opened as public GitHub issues until a disclosure process is added.

For now document the issue privately and include:

- Affected commit
- Hardware version
- Firmware version
- Reproduction steps
- Expected behavior
- Actual behavior
- Impact
- Suggested fix if known

## License

License is not picked yet.

The plan is to use an open source license for the code and a documentation friendly license for schemas and research material.

That decision needs to be made before outside contributions are accepted.

## Final goal

The goal is a tool we actually keep using after DEF CON.

Not another badge demo that works once on a hotel table.

RELIQUARY should be able to sit in a medical device lab, sign a firmware assessment, verify the result six months later and tell us exactly which key approved it.

Small enough to understand.

Useful enough to keep.

Hard enough to trust.
