//! Beacon desktop. Same local guide as the CLI. Nothing leaves the machine.
//!
//! Report text only. No file attachments, no screenshots of material, no uploads.

use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize)]
struct IncidentKind {
    slug: &'static str,
    label: &'static str,
}

fn parse_slug(slug: &str) -> Result<beacon::incident::IncidentType, String> {
    let slug = slug.trim();
    if slug.is_empty() {
        return Err("empty incident type".into());
    }
    beacon::incident::IncidentType::from_slug(slug)
        .ok_or_else(|| format!("unknown incident type: {slug}"))
}

#[tauri::command]
fn incident_types() -> Vec<IncidentKind> {
    beacon::incident::IncidentType::all()
        .into_iter()
        .map(|kind| IncidentKind {
            slug: kind.slug(),
            label: kind.label(),
        })
        .collect()
}

#[tauri::command]
fn checklist(slug: String) -> Result<Vec<String>, String> {
    let kind = parse_slug(&slug)?;
    Ok(beacon::incident::checklist(kind))
}

#[tauri::command]
fn resources() -> String {
    beacon::incident::resources_text().to_string()
}

#[tauri::command]
fn save_report(
    app: tauri::AppHandle,
    slug: String,
    incident_dates: String,
    description: String,
    amount_lost: String,
    platforms: String,
    accounts_involved: String,
    suspect_identifiers: String,
    evidence_preserved: String,
    contact_info: String,
    existing_reports: String,
    output_path: String,
) -> Result<String, String> {
    let kind = parse_slug(&slug)?;
    let fields = beacon::report::ReportFields {
        incident_dates,
        description,
        amount_lost,
        platforms,
        accounts_involved,
        suspect_identifiers,
        evidence_preserved,
        contact_info,
        existing_reports,
    };
    let now = beacon::report::local_now();
    let markdown = beacon::report::render_markdown(&fields, kind, &now);
    let path = if output_path.trim().is_empty() {
        let dir = app
            .path()
            .app_data_dir()
            .map_err(|e| e.to_string())?
            .join("exports");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        dir.join(beacon::report::default_output_path(kind))
    } else {
        PathBuf::from(output_path.trim())
    };
    beacon::report::write_report(&path, &markdown).map_err(|e| e.to_string())?;
    Ok(path.display().to_string())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            incident_types,
            checklist,
            resources,
            save_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running beacon");
}
