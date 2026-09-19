//! Structured report generation: interactive prompts, non-interactive flags,
//! and markdown rendering suitable for attaching to an IC3 complaint.

use crate::incident::{checklist, IncidentType};
use anyhow::{Context, Result};
use chrono::Local;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

/// Everything beacon knows about one incident, as entered by the victim.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReportFields {
    /// When the incident occurred / began (free text, e.g. "2026-09-10 to 2026-09-12").
    pub incident_dates: String,
    /// Free-text narrative of what happened.
    pub description: String,
    /// Total money lost, if any (free text, e.g. "USD 1,250" or "none").
    pub amount_lost: String,
    /// Platforms / websites / apps where it happened.
    pub platforms: String,
    /// Victim accounts involved (emails, usernames, account numbers).
    pub accounts_involved: String,
    /// Known suspect identifiers: handles, emails, phone numbers, wallet addresses, URLs.
    pub suspect_identifiers: String,
    /// Which evidence the victim has preserved (from the checklist).
    pub evidence_preserved: String,
    /// Victim contact info for the report (name / email / phone) — optional.
    pub contact_info: String,
    /// Whether any report has already been filed (IC3 ID, police report #, etc.).
    pub existing_reports: String,
}

impl ReportFields {
    fn is_empty(&self) -> bool {
        [
            &self.incident_dates,
            &self.description,
            &self.amount_lost,
            &self.platforms,
            &self.accounts_involved,
            &self.suspect_identifiers,
            &self.evidence_preserved,
        ]
        .iter()
        .all(|f| f.trim().is_empty())
    }
}

fn prompt<R: BufRead, W: Write>(input: &mut R, out: &mut W, label: &str) -> Result<String> {
    write!(out, "{}: ", label)?;
    out.flush()?;
    let mut line = String::new();
    input
        .read_line(&mut line)
        .with_context(|| format!("failed reading response for '{label}'"))?;
    Ok(line.trim().to_string())
}

/// Interactively collect report fields from stdin/stdout.
pub fn collect_interactive<R: BufRead, W: Write>(
    input: &mut R,
    out: &mut W,
    incident_type: IncidentType,
) -> Result<ReportFields> {
    writeln!(out)?;
    writeln!(
        out,
        "BEACON — building your {} report. Press Enter to skip any field.",
        incident_type.label()
    )?;
    if incident_type == IncidentType::Csam {
        writeln!(out)?;
        writeln!(
            out,
            "REMINDER: do NOT download, copy, or attach the material itself."
        )?;
        writeln!(
            out,
            "This report goes to the NCMEC CyberTipline (report.cybertip.org)."
        )?;
    }
    writeln!(out)?;
    Ok(ReportFields {
        incident_dates: prompt(input, out, "When did it happen (date or date range)")?,
        description: prompt(input, out, "Brief description of what happened")?,
        amount_lost: prompt(input, out, "Money lost, if any (e.g. 'USD 500' or 'none')")?,
        platforms: prompt(input, out, "Platforms / websites / apps involved")?,
        accounts_involved: prompt(input, out, "Your accounts involved (emails, usernames)")?,
        suspect_identifiers: prompt(
            input,
            out,
            "Suspect identifiers you know (handles, emails, phones, wallets, URLs)",
        )?,
        evidence_preserved: prompt(
            input,
            out,
            "Evidence you preserved (screenshots, .eml files, logs, receipts)",
        )?,
        contact_info: prompt(input, out, "Your contact info for investigators (optional)")?,
        existing_reports: prompt(
            input,
            out,
            "Existing report/case numbers, if any (IC3, police, FTC)",
        )?,
    })
}

fn or_na(value: &str) -> &str {
    if value.trim().is_empty() {
        "_Not provided._"
    } else {
        value.trim()
    }
}

/// Render the structured markdown report.
pub fn render_markdown(
    fields: &ReportFields,
    incident_type: IncidentType,
    generated_at: &chrono::DateTime<Local>,
) -> String {
    let mut md = String::new();
    md.push_str(&format!(
        "# Cybercrime Incident Report — {}\n\n",
        incident_type.label()
    ));
    md.push_str(&format!(
        "- **Generated:** {} by beacon (local, offline tool)\n",
        generated_at.format("%Y-%m-%d %H:%M:%S %Z")
    ));
    md.push_str("- **Intended recipients:** FBI IC3 (ic3.gov) / local police / FTC / NCMEC CyberTipline as applicable\n");
    md.push_str("- **Status:** Victim-prepared summary, unsworn\n\n");
    md.push_str("---\n\n");

    if incident_type == IncidentType::Csam {
        md.push_str("> **⚠️ CSAM — READ BEFORE HANDLING:** No abusive material is attached to\n");
        md.push_str(
            "> or embedded in this report, and none was downloaded or copied. This report\n",
        );
        md.push_str("> contains location metadata (URLs/usernames/dates) only. Route to the\n");
        md.push_str("> **NCMEC CyberTipline: https://report.cybertip.org / 1-800-843-5678**.\n\n");
        md.push_str("---\n\n");
    }

    md.push_str("## 1. Incident Summary\n\n");
    md.push_str(&format!("- **Incident type:** {}\n", incident_type.label()));
    md.push_str(&format!(
        "- **Date(s) of incident:** {}\n",
        or_na(&fields.incident_dates)
    ));
    md.push_str(&format!(
        "- **Date of report:** {}\n",
        generated_at.format("%Y-%m-%d")
    ));
    md.push_str(&format!(
        "- **Estimated financial loss:** {}\n\n",
        or_na(&fields.amount_lost)
    ));

    md.push_str("## 2. Description of What Happened\n\n");
    md.push_str(or_na(&fields.description));
    md.push_str("\n\n");

    md.push_str("## 3. Platforms and Accounts\n\n");
    md.push_str(&format!(
        "- **Platforms / websites / apps:** {}\n",
        or_na(&fields.platforms)
    ));
    md.push_str(&format!(
        "- **Victim accounts involved:** {}\n\n",
        or_na(&fields.accounts_involved)
    ));

    md.push_str("## 4. Suspect / Subject Identifiers Known\n\n");
    md.push_str(or_na(&fields.suspect_identifiers));
    md.push_str("\n\n");

    md.push_str("## 5. Evidence Preserved\n\n");
    md.push_str(or_na(&fields.evidence_preserved));
    md.push_str("\n\n");
    md.push_str("### Evidence-preservation checklist completed by victim\n\n");
    for (i, step) in checklist(incident_type).iter().enumerate() {
        md.push_str(&format!("{}. {}\n", i + 1, step));
    }
    md.push('\n');

    md.push_str("## 6. Prior Reports\n\n");
    md.push_str(or_na(&fields.existing_reports));
    md.push_str("\n\n");

    md.push_str("## 7. Victim Contact\n\n");
    md.push_str(or_na(&fields.contact_info));
    md.push_str("\n\n");

    md.push_str("---\n\n");
    md.push_str(
        "*Prepared locally with beacon. This document is a victim-prepared summary to\n\
         assist investigators; it is not legal advice and not a sworn statement.*\n",
    );
    md
}

/// Default output filename in the current directory.
pub fn default_output_path(incident_type: IncidentType) -> PathBuf {
    let stamp = Local::now().format("%Y%m%d-%H%M%S");
    PathBuf::from(format!(
        "beacon-report-{}-{}.md",
        incident_type.slug(),
        stamp
    ))
}

/// Write the rendered report to disk. Local-only: beacon never touches the network.
pub fn write_report(path: &Path, markdown: &str) -> Result<()> {
    std::fs::write(path, markdown)
        .with_context(|| format!("failed to write report to {}", path.display()))
}

/// Full report flow used by the CLI. Returns the path written.
pub fn run<R: BufRead, W: Write>(
    input: &mut R,
    out: &mut W,
    incident_type: IncidentType,
    non_interactive_fields: Option<ReportFields>,
    output: Option<PathBuf>,
) -> Result<PathBuf> {
    let fields = match non_interactive_fields {
        Some(f) => {
            if f.is_empty() {
                anyhow::bail!(
                    "--non-interactive was given but no field flags were provided \
                     (use --date, --description, --amount, --platform, --account, \
                     --suspect, --evidence)"
                );
            }
            f
        }
        None => collect_interactive(input, out, incident_type)?,
    };

    let now = Local::now();
    let md = render_markdown(&fields, incident_type, &now);
    let path = output.unwrap_or_else(|| default_output_path(incident_type));
    write_report(&path, &md)?;
    writeln!(
        out,
        "\nReport written to: {}\nAttach this file to your IC3 complaint or bring it to local police.",
        path.display()
    )?;
    if incident_type == IncidentType::Csam {
        writeln!(
            out,
            "For CSAM: file at https://report.cybertip.org — do NOT attach or send the material itself."
        )?;
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_fields() -> ReportFields {
        ReportFields {
            incident_dates: "2026-09-01 to 2026-09-03".into(),
            description: "Paid for goods never delivered; seller went silent.".into(),
            amount_lost: "USD 240".into(),
            platforms: "example-marketplace.test".into(),
            accounts_involved: "buyer-account-1234".into(),
            suspect_identifiers: "handle: seller-ghost-99, wallet: 0xDEADBEEF".into(),
            evidence_preserved: "screenshots of listing, .eml of confirmation, bank statement"
                .into(),
            contact_info: "jane-doe@example.com".into(),
            existing_reports: "none yet".into(),
        }
    }

    #[test]
    fn render_produces_structured_markdown_with_all_fields() {
        let now = Local::now();
        let md = render_markdown(&sample_fields(), IncidentType::OnlineFraud, &now);
        assert!(md.starts_with("# Cybercrime Incident Report — Online Fraud"));
        for header in [
            "## 1. Incident Summary",
            "## 2. Description of What Happened",
            "## 3. Platforms and Accounts",
            "## 4. Suspect / Subject Identifiers Known",
            "## 5. Evidence Preserved",
            "## 6. Prior Reports",
            "## 7. Victim Contact",
        ] {
            assert!(md.contains(header), "missing section: {header}");
        }
        let f = sample_fields();
        for value in [
            f.incident_dates,
            f.description,
            f.amount_lost,
            f.platforms,
            f.accounts_involved,
            f.suspect_identifiers,
            f.evidence_preserved,
            f.contact_info,
        ] {
            assert!(md.contains(&value), "missing field value: {value}");
        }
    }

    #[test]
    fn empty_fields_render_as_not_provided() {
        let now = Local::now();
        let md = render_markdown(&ReportFields::default(), IncidentType::Phishing, &now);
        assert!(md.contains("_Not provided._"));
        assert!(md.contains("## 5. Evidence Preserved"));
    }

    #[test]
    fn csam_report_carries_warning_and_cybertipline_routing() {
        let now = Local::now();
        let md = render_markdown(&sample_fields(), IncidentType::Csam, &now);
        assert!(md.contains("No abusive material is attached"));
        assert!(md.contains("report.cybertip.org"));
        assert!(md.contains("1-800-843-5678"));
        let lower = md.to_lowercase();
        assert!(lower.contains("do not download"));
    }

    #[test]
    fn interactive_collection_reads_all_fields_from_stdin() {
        let scripted = "2026-09-05\nclicked a fake login link\nnone\nwebmail\nuser@example.test\n\
                        phish-page.example/login\nsaved .eml with headers\n\nnone\n";
        let mut input = scripted.as_bytes();
        let mut out: Vec<u8> = Vec::new();
        let fields = collect_interactive(&mut input, &mut out, IncidentType::Phishing).unwrap();
        assert_eq!(fields.incident_dates, "2026-09-05");
        assert_eq!(fields.description, "clicked a fake login link");
        assert_eq!(fields.amount_lost, "none");
        assert_eq!(fields.suspect_identifiers, "phish-page.example/login");
        assert_eq!(fields.contact_info, "");
        assert_eq!(fields.existing_reports, "none");
    }

    #[test]
    fn run_writes_report_file_to_disk() {
        let dir = std::env::temp_dir().join(format!("beacon-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("report.md");
        let mut input = &b""[..];
        let mut out: Vec<u8> = Vec::new();
        let written = run(
            &mut input,
            &mut out,
            IncidentType::Ransomware,
            Some(sample_fields()),
            Some(path.clone()),
        )
        .unwrap();
        assert_eq!(written, path);
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("# Cybercrime Incident Report — Ransomware"));
        assert!(content.contains("seller-ghost-99"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn run_rejects_noninteractive_without_fields() {
        let mut input = &b""[..];
        let mut out: Vec<u8> = Vec::new();
        let result = run(
            &mut input,
            &mut out,
            IncidentType::Sextortion,
            Some(ReportFields::default()),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn default_output_path_is_markdown_and_slugged() {
        let p = default_output_path(IncidentType::IdentityTheft);
        let name = p.file_name().unwrap().to_string_lossy();
        assert!(name.starts_with("beacon-report-identity-theft-"));
        assert!(name.ends_with(".md"));
    }
}
