use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
struct GnomeApiResponse {
    count: i32,
    results: Vec<ExtensionVersion>,
}

#[derive(Deserialize, Debug)]
struct ExtensionVersion {
    version: i32,
}

const GNOME_EXTENSIONS: &[&str] = &[
    "dash-to-panel@jderose9.github.com",
    "blur-my-shell@aunetx",
    "tilingshell@ferrarodomenico.com",
    "arcmenu@arcmenu.com",
    "gtk4-ding@smedius.gitlab.com",
    "clipboard-history@alexsaveau.dev",
    "Vitals@CoreCoding.com",
];

pub async fn setup_desktop() -> Result<(), Box<dyn std::error::Error>> {
    let script_dir = std::env::current_dir()?;
    let project_dir = script_dir.parent().unwrap_or(&script_dir);
    let build_dir = project_dir.join("build");
    let download_dir = build_dir.join("downloads");

    let extract_dir = PathBuf::from("/usr/share/gnome-shell/extensions");

    fs::create_dir_all(&download_dir)?;

    if let Err(_) = fs::create_dir_all(&extract_dir) {
        eprintln!("Error: No write permission for {:?}.", extract_dir);
        eprintln!("Run the program again with root privileges.");
        std::process::exit(1);
    }

    let client = reqwest::Client::new();

    for &extension_id in GNOME_EXTENSIONS {
        let version = match get_latest_version(&client, extension_id).await {
            Some(v) => {
                println!("Detected the latest version: {}", v);
                v
            }
            None => {
                println!("Skipping: No version found for {}", extension_id);
                continue;
            }
        };

        let target_zip =
            match download_extension(&client, extension_id, version, &download_dir).await {
                Some(path) => path,
                None => continue,
            };

        if let Err(e) = extract_and_chmod(&target_zip, &extract_dir, extension_id) {
            println!("Error extracting {}: {}", extension_id, e);
        }
    }

    println!(
        "\nInstallation complete! Please restart GNOME Shell (Alt+F2, type 'r' on X11) or log out and log back in to apply the changes.."
    );
    Ok(())
}

async fn get_latest_version(client: &reqwest::Client, extension_id: &str) -> Option<i32> {
    let page_size = 20;
    let api_url = format!(
        "https://extensions.gnome.org/api/v1/extensions/{}/versions/?page_size={}",
        extension_id, page_size
    );

    let response = client.get(&api_url).send().await.ok()?;
    let data: GnomeApiResponse = response.json().await.ok()?;

    let count = data.count;
    if count == 0 {
        return None;
    }

    let total_pages = ((count as f64) / (page_size as f64)).ceil() as i32;

    let final_data = if total_pages > 1 {
        let last_page_url = format!(
            "https://extensions.gnome.org/api/v1/extensions/{}/versions/?page_size={}&page={}",
            extension_id, page_size, total_pages
        );
        let resp = client.get(&last_page_url).send().await.ok()?;
        resp.json::<GnomeApiResponse>().await.ok()?
    } else {
        data
    };

    final_data.results.last().map(|v| v.version)
}

async fn download_extension(
    client: &reqwest::Client,
    extension_id: &str,
    version: i32,
    download_dir: &Path,
) -> Option<PathBuf> {
    let clean_id = extension_id.replace('@', "");
    let full_url = format!(
        "https://extensions.gnome.org/extension-data/{}.v{}.shell-extension.zip",
        clean_id, version
    );
    let target_zip = download_dir.join(format!("{}.zip", extension_id));

    println!("Loading extension: {} (Version: {})", extension_id, version);
    let bytes = client
        .get(&full_url)
        .send()
        .await
        .ok()?
        .bytes()
        .await
        .ok()?;
    fs::write(&target_zip, bytes).ok()?;

    Some(target_zip)
}

fn extract_and_chmod(
    target_zip: &Path,
    extract_dir: &Path,
    extension_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Extracting extensions: {}", extension_id);
    let target_ext_dir = extract_dir.join(extension_id);
    fs::create_dir_all(&target_ext_dir)?;

    let file = fs::File::open(target_zip)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file_in_zip = archive.by_index(i)?;
        let outpath = target_ext_dir.join(file_in_zip.name());

        if file_in_zip.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file_in_zip, &mut outfile)?;
        }
    }

    for entry in walkdir::WalkDir::new(&target_ext_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "js" || ext == "sh" {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
                    }
                }
            }
        }
    }

    let schemas_dir = target_ext_dir.join("schemas");
    if schemas_dir.exists() && schemas_dir.is_dir() {
        println!("Compiling gschema for {}...", extension_id);
        let status = std::process::Command::new("glib-compile-schemas")
            .arg(&schemas_dir)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("Successfully compiled gschema!");
            }
            Ok(s) => {
                eprintln!(
                    "Warning: glib-compile-schemas exited with error code: {}",
                    s
                );
            }
            Err(e) => {
                eprintln!(
                    "Unable to run glib-compile-schemas (glib2 might not be installed): {}",
                    e
                );
            }
        }
    }

    Ok(())
}
