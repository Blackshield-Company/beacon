//! Incident types, evidence-preservation checklists, and official resources.

use clap::ValueEnum;
use std::fmt;

/// The category of cybercrime incident.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IncidentType {
    /// Online fraud, scams, non-delivery, marketplace fraud
    OnlineFraud,
    /// Identity theft, account takeover, fraudulent accounts in your name
    IdentityTheft,
    /// Cyberstalking, online harassment, threats, doxxing
    Cyberstalking,
    /// Phishing, smishing, credential-harvesting messages
    Phishing,
    /// Ransomware or extortion demanding payment to unlock data
    Ransomware,
    /// Sextortion: threats to release intimate images unless paid
    Sextortion,
    /// Discovery of child sexual abuse material online
    Csam,
}

impl IncidentType {
    pub fn label(&self) -> &'static str {
        match self {
            IncidentType::OnlineFraud => "Online Fraud",
            IncidentType::IdentityTheft => "Identity Theft",
            IncidentType::Cyberstalking => "Cyberstalking / Harassment",
            IncidentType::Phishing => "Phishing",
            IncidentType::Ransomware => "Ransomware",
            IncidentType::Sextortion => "Sextortion",
            IncidentType::Csam => "CSAM Discovery",
        }
    }

    /// Stable slug used in report filenames.
    pub fn slug(&self) -> &'static str {
        match self {
            IncidentType::OnlineFraud => "online-fraud",
            IncidentType::IdentityTheft => "identity-theft",
            IncidentType::Cyberstalking => "cyberstalking",
            IncidentType::Phishing => "phishing",
            IncidentType::Ransomware => "ransomware",
            IncidentType::Sextortion => "sextortion",
            IncidentType::Csam => "csam",
        }
    }

    /// All incident types. Used by tests to guarantee per-type coverage.
    #[cfg(test)]
    pub fn all() -> [IncidentType; 7] {
        [
            IncidentType::OnlineFraud,
            IncidentType::IdentityTheft,
            IncidentType::Cyberstalking,
            IncidentType::Phishing,
            IncidentType::Ransomware,
            IncidentType::Sextortion,
            IncidentType::Csam,
        ]
    }
}

impl fmt::Display for IncidentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// Steps that apply to nearly every incident type.
fn core_steps() -> Vec<&'static str> {
    vec![
        "Do NOT delete any messages, emails, posts, or accounts related to the incident — even if they are upsetting. Deleting destroys evidence.",
        "Take screenshots with timestamps and URLs/usernames visible. Capture full context, not just fragments.",
        "Export or save chat logs where the platform allows it (download data export, save .html/.txt copies).",
        "Save original emails WITH full headers (in most clients: 'Show original' or 'View message source', then save as .eml).",
        "Write down every URL, username, handle, email address, phone number, and profile ID involved — exactly as shown.",
        "Record a timeline while your memory is fresh: dates, times, and what happened, in order.",
        "Do NOT pay any money or send gift cards/crypto, and do NOT confront or warn the suspect — both can destroy the investigation.",
        "Change passwords on affected and related accounts from a clean device, and enable two-factor authentication.",
        "Keep the original devices powered on and stop using them for anything related to the incident until advised otherwise.",
        "Back up all preserved evidence to a second location (external drive or cloud) so a single failure can't erase it.",
    ]
}

/// Type-specific steps appended after the core steps.
fn type_steps(t: IncidentType) -> Vec<&'static str> {
    match t {
        IncidentType::OnlineFraud => vec![
            "Save all payment records: receipts, invoices, bank/card statements, wire or crypto transaction IDs, and gift card numbers.",
            "Screenshot the original listing/ad, the seller's profile, and the storefront URL before it is taken down.",
            "Preserve all shipping/tracking communications and any order confirmations.",
            "Contact your bank or card issuer immediately to report fraud and ask about chargebacks — note who you spoke with and when.",
        ],
        IncidentType::IdentityTheft => vec![
            "Pull your credit reports (annualcreditreport.com) and save copies showing fraudulent accounts or inquiries.",
            "Save every letter, bill, or collection notice for accounts you did not open.",
            "Place a fraud alert or credit freeze with Equifax, Experian, and TransUnion — record confirmation numbers.",
            "File an identity theft report at IdentityTheft.gov and save the affidavit; police and creditors will ask for it.",
            "Keep a log of every call you make to banks and agencies: date, time, name, and reference number.",
        ],
        IncidentType::Cyberstalking => vec![
            "Do NOT block the harasser yet if evidence is still arriving — mute instead, and capture everything first.",
            "Screenshot every message, post, comment, and threat with timestamps, URLs, and the sender's profile visible.",
            "Save voicemails and call logs; note dates/times of calls even if unanswered.",
            "Document any escalation to real-world contact (showing up, contacting family/work) with dates and witnesses.",
            "If there are credible threats of violence, treat it as an emergency: call 911, then preserve evidence.",
        ],
        IncidentType::Phishing => vec![
            "Save the original phishing message as .eml/.txt WITH full headers — do not just forward it (forwarding strips headers).",
            "Screenshot the message and the fake login page URL exactly as it appeared in the address bar.",
            "If you entered credentials, change that password everywhere it is reused, starting with email and banking.",
            "If you ran an attachment or link, note the time, disconnect the device from the network, and note any filenames.",
            "Record where you reported it internally (employer IT, bank fraud line) with reference numbers.",
        ],
        IncidentType::Ransomware => vec![
            "Do NOT pay the ransom. Payment funds crime, is no guarantee of decryption, and can violate sanctions law.",
            "Photograph/screenshot the ransom note and any countdown screens; save the note file itself.",
            "Disconnect the infected device from the network (unplug Ethernet / disable Wi-Fi) but do NOT power it off — memory may hold keys.",
            "Isolate other devices on the same network and check backups; do not connect backup drives to the infected machine.",
            "Note the ransomware variant/extension and any contact addresses or wallet addresses in the note.",
            "Check nomoreransom.org for free decryptors before considering anything wiped.",
        ],
        IncidentType::Sextortion => vec![
            "Do NOT pay and do NOT send more images. Payment almost never ends the demands — it escalates them.",
            "Do NOT delete the conversation. Screenshot all threats, demands, usernames, and payment instructions with timestamps.",
            "Preserve the suspect's profile URLs, handles, phone numbers, emails, and any crypto wallet or payment handles.",
            "After evidence is captured, block the account and tighten privacy settings, but keep your saved copies.",
            "If you are under 18, or the images involve a minor, report to NCMEC's CyberTipline (report.cybertip.org) and use Take It Down (takeitdown.ncmec.org).",
            "If threats are imminent or you feel unsafe, call 911 or a local crisis line — your safety comes before evidence.",
        ],
        // CSAM is handled separately — it must NOT follow the normal pattern of
        // "screenshot/copy the material", which would be a crime in itself.
        IncidentType::Csam => vec![
            "STOP. Do NOT download, copy, screenshot, forward, or save the material itself — possessing or distributing it is a serious federal crime, even for evidence.",
            "Do NOT investigate further, do not 'collect more proof', and do not visit the page/profile again.",
            "DO write down the exact URL(s), usernames, platform name, and the date/time you encountered it — location metadata only, never the material.",
            "Close the page. Do not delete your browser history or account — investigators may need the access records.",
            "Report IMMEDIATELY to the NCMEC CyberTipline at https://report.cybertip.org or call 1-800-843-5678 (THE-LOST). This is the correct and required channel in the U.S.",
            "If a child is in immediate danger, call 911 first, then file the CyberTipline report.",
            "If the material arrived via message/email, do not forward it to anyone (including police by email) — describe it and give them the location details; law enforcement will instruct you.",
        ],
    }
}

/// Full numbered checklist for an incident type.
pub fn checklist(t: IncidentType) -> Vec<String> {
    let mut steps: Vec<String> = Vec::new();
    if t == IncidentType::Csam {
        // For CSAM, the type-specific steps ARE the checklist. The generic
        // "screenshot everything" core steps would instruct a crime.
        steps.extend(type_steps(t).into_iter().map(String::from));
    } else {
        steps.extend(core_steps().into_iter().map(String::from));
        steps.extend(type_steps(t).into_iter().map(String::from));
    }
    steps
}

/// Print the evidence-preservation checklist for an incident type.
pub fn print_checklist(t: IncidentType) {
    println!("===================================================================");
    println!(" BEACON — Evidence Preservation Checklist");
    println!(" Incident type: {}", t.label());
    println!("===================================================================");
    if t == IncidentType::Csam {
        println!();
        println!(" !!! CRITICAL WARNING — READ FIRST !!!");
        println!(" Do NOT download, copy, screenshot, or forward the material.");
        println!(" Possessing or distributing it is a federal crime, even to");
        println!(" 'help' an investigation. Report the LOCATION, not the files.");
        println!(" Route: NCMEC CyberTipline — https://report.cybertip.org");
        println!();
    }
    println!("Complete these steps BEFORE filing any report. Most victims");
    println!("accidentally destroy evidence in the first hours — don't.\n");
    for (i, step) in checklist(t).iter().enumerate() {
        println!(" {:>2}. {}", i + 1, step);
    }
    println!();
    println!(
        "Next: run `beacon report --type {}` to build your report.",
        t.slug()
    );
    println!("Then: run `beacon resources` for official reporting channels.");
}

/// Official reporting channels and what each handles.
pub fn resources_text() -> &'static str {
    "===================================================================
 BEACON — Official Reporting Channels
===================================================================

 1. FBI Internet Crime Complaint Center (IC3) — https://www.ic3.gov
    The primary U.S. channel for: online fraud, phishing, ransomware,
    identity theft (cyber-enabled), sextortion, business email
    compromise, and most internet-facilitated crime.
    File online; attach your beacon report and evidence list.
    You will receive a complaint ID — keep it for follow-ups.

 2. NCMEC CyberTipline — https://report.cybertip.org / 1-800-843-5678
    THE correct channel for child sexual abuse material (CSAM),
    online enticement of minors, and sextortion involving minors.
    Do NOT send the material itself — report URLs/usernames only.
    NCMEC routes reports to the appropriate law enforcement agency.

 3. FTC — https://reportfraud.ftc.gov and https://IdentityTheft.gov
    Consumer fraud, scams, imposter schemes, and identity theft.
    IdentityTheft.gov generates an official FTC Identity Theft
    Affidavit and a personal recovery plan — creditors and police
    accept it as your formal report.

 4. Local Police (non-emergency line)
    Required for: in-person threats, stalking with a local suspect,
    theft where you need a police report number for banks/insurance,
    and anything involving immediate physical safety.
    Find the non-emergency number on your city/county PD website.
    Bring: printed beacon report, evidence list, and any IC3/FTC
    complaint IDs you already have.
    EMERGENCY — anyone in immediate danger: call 911.

 5. Platform reporting (in parallel, not instead of police)
    Report the account/content to the platform (fraud, harassment,
    impersonation). Platforms can preserve server-side logs that
    vanish quickly — report early.

 TIP: File with IC3 first for internet crime; it is free, takes
 ~15 minutes, and creates a federal record other agencies can see.
==================================================================="
}

pub fn print_resources() {
    println!("{}", resources_text());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_incident_type_has_a_nonempty_checklist() {
        for t in IncidentType::all() {
            let steps = checklist(t);
            assert!(!steps.is_empty(), "checklist missing for {:?}", t);
            for step in &steps {
                assert!(!step.trim().is_empty(), "blank step in {:?}", t);
            }
        }
    }

    #[test]
    fn checklists_include_core_evidence_preservation_guidance() {
        for t in IncidentType::all() {
            if t == IncidentType::Csam {
                continue; // CSAM intentionally diverges — tested separately
            }
            let joined = checklist(t).join("\n").to_lowercase();
            assert!(
                joined.contains("do not delete"),
                "{:?} missing do-not-delete",
                t
            );
            assert!(
                joined.contains("screenshot"),
                "{:?} missing screenshot step",
                t
            );
            assert!(
                joined.contains("headers"),
                "{:?} missing email-headers step",
                t
            );
        }
    }

    #[test]
    fn csam_checklist_warns_not_to_download_and_routes_to_cybertipline() {
        let joined = checklist(IncidentType::Csam).join("\n").to_lowercase();
        assert!(
            joined.contains("do not download"),
            "CSAM checklist must emphatically forbid downloading"
        );
        assert!(
            joined.contains("cybertipline"),
            "CSAM checklist must route to NCMEC CyberTipline"
        );
        assert!(joined.contains("report.cybertip.org"));
        // It must NOT instruct the user to screenshot/copy the material.
        assert!(!joined.contains("screenshot the material with"));
    }

    #[test]
    fn resources_include_ic3_ftc_ncmec_and_local_pd() {
        let r = resources_text().to_lowercase();
        assert!(r.contains("ic3.gov"));
        assert!(r.contains("reportfraud.ftc.gov") || r.contains("identitytheft.gov"));
        assert!(r.contains("cybertip"));
        assert!(r.contains("non-emergency"));
    }

    #[test]
    fn slugs_are_filesystem_safe() {
        for t in IncidentType::all() {
            let s = t.slug();
            assert!(s.chars().all(|c| c.is_ascii_lowercase() || c == '-'));
        }
    }
}
