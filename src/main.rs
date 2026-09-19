//! beacon — an interactive cybercrime reporting guide for victims.
//!
//! beacon helps victims preserve evidence correctly BEFORE reporting, then
//! generates a structured markdown report suitable for an IC3 complaint or a
//! local police visit. Local-first: zero network, zero telemetry.

mod incident;
mod report;

use anyhow::Result;
use clap::{Parser, Subcommand};
use incident::IncidentType;
use report::ReportFields;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "beacon",
    version,
    about = "Interactive cybercrime reporting guide for victims: preserve evidence first, then report.",
    long_about = "beacon walks cybercrime victims through preserving evidence correctly BEFORE \
                  reporting (most victims accidentally destroy evidence), then generates a \
                  structured markdown report to attach to an IC3 complaint or hand to local \
                  police. Local-first: zero network, zero telemetry. Not legal advice."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the evidence-preservation checklist for an incident type
    Start {
        /// Incident type: online-fraud, identity-theft, cyberstalking, phishing,
        /// ransomware, sextortion, or csam
        #[arg(long, value_enum)]
        r#type: IncidentType,
    },
    /// Build a structured incident report (interactive prompts, or flags with --non-interactive)
    Report {
        /// Incident type: online-fraud, identity-theft, cyberstalking, phishing,
        /// ransomware, sextortion, or csam
        #[arg(long, value_enum)]
        r#type: IncidentType,

        /// Do not prompt; take all fields from flags (for scripting)
        #[arg(long)]
        non_interactive: bool,

        /// When it happened (date or date range)
        #[arg(long)]
        date: Option<String>,
        /// Brief description of what happened
        #[arg(long)]
        description: Option<String>,
        /// Money lost, e.g. "USD 500" or "none"
        #[arg(long)]
        amount: Option<String>,
        /// Platforms / websites / apps involved
        #[arg(long)]
        platform: Option<String>,
        /// Your accounts involved (emails, usernames)
        #[arg(long)]
        account: Option<String>,
        /// Known suspect identifiers (handles, emails, phones, wallets, URLs)
        #[arg(long)]
        suspect: Option<String>,
        /// Evidence you preserved (screenshots, .eml files, logs, receipts)
        #[arg(long)]
        evidence: Option<String>,
        /// Your contact info for investigators (optional)
        #[arg(long)]
        contact: Option<String>,
        /// Existing report/case numbers (IC3, police, FTC)
        #[arg(long)]
        existing_report: Option<String>,

        /// Output file path (default: beacon-report-<type>-<timestamp>.md in current dir)
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
    /// Print official reporting channels (IC3, FTC, NCMEC CyberTipline, local PD)
    Resources,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start { r#type } => incident::print_checklist(r#type),
        Commands::Resources => incident::print_resources(),
        Commands::Report {
            r#type,
            non_interactive,
            date,
            description,
            amount,
            platform,
            account,
            suspect,
            evidence,
            contact,
            existing_report,
            output,
        } => {
            let stdin = io::stdin();
            let stdout = io::stdout();
            let mut input = BufReader::new(stdin.lock());
            let mut out = BufWriter::new(stdout.lock());

            let fields = non_interactive.then(|| ReportFields {
                incident_dates: date.unwrap_or_default(),
                description: description.unwrap_or_default(),
                amount_lost: amount.unwrap_or_default(),
                platforms: platform.unwrap_or_default(),
                accounts_involved: account.unwrap_or_default(),
                suspect_identifiers: suspect.unwrap_or_default(),
                evidence_preserved: evidence.unwrap_or_default(),
                contact_info: contact.unwrap_or_default(),
                existing_reports: existing_report.unwrap_or_default(),
            });

            report::run(&mut input, &mut out, r#type, fields, output)?;
        }
    }
    Ok(())
}
