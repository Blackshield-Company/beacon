# beacon

An interactive, local-first cybercrime reporting guide for victims.

Most victims accidentally destroy evidence in the first hours after an incident —
they delete the threatening messages, pay the ransom, block the harasser, or
reboot the infected machine. **beacon** flips the order: preserve evidence
correctly *first*, then report.

beacon asks what happened, walks you through a concrete, numbered
evidence-preservation checklist for your incident type, then interactively
builds a structured markdown report you can attach to an
[IC3](https://www.ic3.gov) complaint or hand to local police.

## Who it's for

- Victims of online fraud, identity theft, cyberstalking/harassment, phishing,
  ransomware, sextortion, or anyone who has discovered CSAM online and needs to
  report it safely and legally.
- Advocates, IT staff, family members, and counselors helping a victim through
  the first critical hours.
- Anyone who needs a clean, structured incident write-up for IC3, the FTC,
  NCMEC, or a local police non-emergency report.

## What it is NOT (honest disclaimer)

- **beacon is not legal advice.** It is general educational information. For
  advice about your situation, consult a licensed attorney.
- **beacon is not a law enforcement product** and is not affiliated with,
  endorsed by, or a substitute for the FBI, IC3, FTC, NCMEC, or any police
  agency. It generates a victim-prepared summary only.
- **beacon does not report anything for you.** It runs entirely offline —
  zero network access, zero telemetry. Reports are written to your local disk
  and it is your choice where to send them.
- beacon cannot recover stolen money or decrypt ransomware. Anyone who
  promises that for a fee is running a recovery scam.

## Install

```sh
cargo install --path .
```

Requires a recent stable Rust toolchain.

## Usage

```sh
# 1. Get the evidence-preservation checklist for your incident type
beacon start --type ransomware

# 2. Build the structured report (interactive stdin prompts)
beacon report --type ransomware

# 2b. ...or non-interactively, for scripting
beacon report --type online-fraud --non-interactive \
  --date "2026-09-01" \
  --description "Paid for goods never delivered" \
  --amount "USD 240" \
  --platform "marketplace.example" \
  --suspect "handle: seller-ghost-99" \
  --evidence "screenshots, .eml receipt, bank statement" \
  --output report.md

# 3. See official reporting channels and what each handles
beacon resources
```

### Incident types

| `--type` value    | Covers |
|-------------------|--------|
| `online-fraud`    | Scams, non-delivery, marketplace fraud |
| `identity-theft`  | Fraudulent accounts, account takeover |
| `cyberstalking`   | Harassment, threats, doxxing |
| `phishing`        | Credential-harvesting messages/sites |
| `ransomware`      | Encrypted files, ransom demands |
| `sextortion`      | Threats to release intimate images |
| `csam`            | Discovery of child sexual abuse material |

**CSAM is handled specially:** the checklist emphatically tells you **not** to
download, copy, screenshot, or forward the material — possessing or
distributing it is a federal crime even with good intentions — and routes you
directly to the NCMEC CyberTipline (<https://report.cybertip.org> /
1-800-843-5678). Report the *location*, never the files.

## How it works

```
beacon start  →  numbered, concrete checklist (do NOT delete messages,
                 screenshot with timestamps, export chat logs, save email
                 headers, note URLs/usernames, do NOT pay, do NOT confront…)
beacon report →  stdin prompts for dates, amounts lost, platforms, accounts,
                 suspect identifiers, evidence preserved
              →  writes beacon-report-<type>-<timestamp>.md locally
beacon resources → IC3, FTC/IdentityTheft.gov, NCMEC CyberTipline, local PD
                    non-emergency guidance, platform reporting
```

## Development

```sh
cargo build
cargo test
```

Dependencies: `clap` (derive), `chrono`, `anyhow`. That's it.

## License

Apache-2.0 — see [LICENSE](LICENSE). Copyright synth (synthalorian).

Made by synth with blackclaw
