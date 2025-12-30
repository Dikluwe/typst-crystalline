use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let root = find_project_root().expect("⌠Não foi possível encontrar a raiz do projeto (.git)");
    println!("🔎 Project Root found at: {}", root.display());

    let layers = vec!["00_nucleo", "01_core", "02_shell", "03_infra", "04_wiring", "_lab"];

    let mut global_map = String::from("# 🗺️ Project Atlas (Crystalline)\n\n");
    global_map.push_str("> Map of architectural layers and their active modules.\n\n");

    // Lista arquivos na raiz do projeto
    let root_files = list_project_root_files(&root)?;
    if !root_files.is_empty() {
        global_map.push_str("## 📄 Project Files\n\n");
        global_map.push_str("| File | Purpose |\n|---|---|\n");
        for (name, desc) in &root_files {
            global_map.push_str(&format!("| `{}` | {} |\n", name, desc));
        }
        global_map.push_str("\n");
    }

    // Lista as camadas
    global_map.push_str("## 📂 Architectural Layers\n\n");

    for layer in &layers {
        let layer_path = root.join(layer);

        if layer_path.exists() {
            global_map.push_str(&format!("### {}\n", layer));
            generate_layer_map(&layer_path, layer)?;
            global_map.push_str(&format!("[View Layer Map]({}/{})\n\n",
                                         layer, format!("{}_MAP.md", layer)));
        }
    }

    fs::write(root.join("PROJECT_MAP.md"), global_map)?;
    println!("✅ Atlas updated: {}/PROJECT_MAP.md", root.display());
    Ok(())
}

/// Lista arquivos na raiz do projeto
fn list_project_root_files(root: &Path) -> io::Result<Vec<(String, String)>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(root)?;
    let mut paths: Vec<_> = entries
    .filter_map(|r| r.ok())
    .map(|e| e.path())
    .filter(|p| p.is_file())
    .collect();
    paths.sort();

    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        // Ignora arquivos gerados, ocultos e específicos do sistema
        if name == "PROJECT_MAP.md"
            || name.starts_with(".")
            || name == "Cargo.lock"
            || name.ends_with(".o")
            || name.ends_with(".exe") {
                continue;
            }

            let desc = extract_doc_comment(&path)
            .unwrap_or_else(|| "—".to_string());

        files.push((name, desc));
    }

    Ok(files)
}

fn generate_layer_map(layer_path: &Path, layer_name: &str) -> io::Result<()> {
    let mut layer_map = format!("# 🗺️ Layer: {}\n\n", layer_name);
    layer_map.push_str("> Modules in this architectural layer.\n\n");

    // Lista arquivos na raiz da camada
    let root_files = list_layer_root_files(layer_path)?;
    if !root_files.is_empty() {
        layer_map.push_str("## 📄 Layer Files\n\n");
        layer_map.push_str("| File | Purpose |\n|---|---|\n");
        for (name, desc) in &root_files {
            layer_map.push_str(&format!("| `{}` | {} |\n", name, desc));
        }
        layer_map.push_str("\n");
    }

    // Lista os módulos (subdiretórios)
    let entries = fs::read_dir(&layer_path)?;
    let mut subdirs: Vec<_> = entries
    .filter_map(|e| e.ok())
    .map(|e| e.path())
    .filter(|p| p.is_dir())
    .collect();
    subdirs.sort();

    let mut has_modules = false;

    if !subdirs.is_empty() {
        layer_map.push_str("## 📂 Modules\n\n");
    }

    for subdir in subdirs {
        let module_name = subdir.file_name().unwrap().to_string_lossy();

        if module_name.starts_with(".") || module_name == "target" || module_name == "node_modules" {
            continue;
        }

        has_modules = true;

        let module_has_files = generate_module_map(&subdir, &module_name)?;

        if module_has_files {
            layer_map.push_str(&format!(
                "- **[{}]** - [View Map]({}/{})\n",
                                        module_name,
                                        module_name,
                                        format!("{}_MAP.md", module_name)
            ));
        } else {
            layer_map.push_str(&format!("- **[{}]** *(empty)*\n", module_name));
        }
    }

    if !has_modules && root_files.is_empty() {
        layer_map.push_str("*No content yet / Nenhum conteúdo ainda*\n");
    }

    let map_path = layer_path.join(format!("{}_MAP.md", layer_name));
    fs::write(map_path, layer_map)?;
    println!("   📄 Layer map generated: {}/{}_MAP.md", layer_name, layer_name);

    Ok(())
}

fn list_layer_root_files(layer_path: &Path) -> io::Result<Vec<(String, String)>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(layer_path)?;
    let mut paths: Vec<_> = entries
    .filter_map(|r| r.ok())
    .map(|e| e.path())
    .filter(|p| p.is_file())
    .collect();
    paths.sort();

    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        if name.ends_with("_MAP.md") || name.starts_with(".") {
            continue;
        }

        let desc = extract_doc_comment(&path)
        .unwrap_or_else(|| "—".to_string());

        files.push((name, desc));
    }

    Ok(files)
}

fn generate_module_map(dir: &Path, module_name: &str) -> io::Result<bool> {
    let mut content = format!("# 🗺️ Module: {}\n\n", module_name);

    // Primeiro, lista arquivos na raiz do módulo
    let root_files = collect_files_in_dir(dir, "")?;

    // Depois, verifica se existe um diretório src/ e lista recursivamente
    let src_path = dir.join("src");
    let src_files = if src_path.exists() && src_path.is_dir() {
        collect_files_recursive(&src_path, "src")?
    } else {
        Vec::new()
    };

    // Combina todos os arquivos
    let mut all_files = root_files;
    all_files.extend(src_files);

    if all_files.is_empty() {
        return Ok(false);
    }

    content.push_str("| File | Purpose |\n|---|---|\n");

    for (path, desc) in &all_files {
        content.push_str(&format!("| `{}` | {} |\n", path, desc));
    }

    let map_file = dir.join(format!("{}_MAP.md", module_name));
    fs::write(map_file, content)?;
    println!("      📗 Module map: {}/{}_MAP.md ({} files)",
             dir.file_name().unwrap().to_string_lossy(),
             module_name,
             all_files.len());

    Ok(true)
}

/// Coleta arquivos em um diretório específico (não recursivo)
fn collect_files_in_dir(dir: &Path, prefix: &str) -> io::Result<Vec<(String, String)>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(dir)?;
    let mut paths: Vec<_> = entries
    .filter_map(|r| r.ok())
    .map(|e| e.path())
    .filter(|p| p.is_file())
    .collect();
    paths.sort();

    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy();

        if name.ends_with("_MAP.md") || name.starts_with(".") {
            continue;
        }

        let desc = extract_doc_comment(&path)
        .unwrap_or_else(|| "—".to_string());

        let display_path = if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{}/{}", prefix, name)
        };

        files.push((display_path, desc));
    }

    Ok(files)
}

/// Coleta arquivos recursivamente dentro de um diretório
fn collect_files_recursive(dir: &Path, prefix: &str) -> io::Result<Vec<(String, String)>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(dir)?;
    let mut paths: Vec<_> = entries
    .filter_map(|r| r.ok())
    .map(|e| e.path())
    .collect();
    paths.sort();

    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy();

        if name.starts_with(".") || name == "target" || name == "node_modules" {
            continue;
        }

        if path.is_file() {
            if name.ends_with("_MAP.md") {
                continue;
            }

            let desc = extract_doc_comment(&path)
            .unwrap_or_else(|| "—".to_string());

            let display_path = format!("{}/{}", prefix, name);
            files.push((display_path, desc));
        } else if path.is_dir() {
            // Recursivamente processa subdiretórios
            let subdir_prefix = format!("{}/{}", prefix, name);
            let subdir_files = collect_files_recursive(&path, &subdir_prefix)?;
            files.extend(subdir_files);
        }
    }

    Ok(files)
}

fn find_project_root() -> io::Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Root not found"));
        }
    }
}

fn extract_doc_comment(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_string_lossy();
    let content = fs::read_to_string(path).ok()?;

    match ext.as_ref() {
        "md" => {
            for line in content.lines().take(20) {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    continue;
                }

                if trimmed.starts_with("#") {
                    let title = trimmed.trim_start_matches('#').trim();
                    if title.contains(" / ") {
                        let clean = title.split(" / ").next().unwrap_or(title);
                        return Some(clean.trim().to_string());
                    }
                    return Some(title.to_string());
                }

                if !trimmed.starts_with(">") && !trimmed.starts_with("-") && !trimmed.starts_with("*") {
                    let first_sentence = trimmed.split('.').next().unwrap_or(trimmed);
                    return Some(first_sentence.trim().to_string());
                }
            }
            Some("Documentation".to_string())
        }
        "rs" => {
            for line in content.lines().take(10) {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with("//!") {
                    return Some(trimmed.replace("//!", "").trim().to_string());
                }
                if trimmed.starts_with("///") {
                    return Some(trimmed.replace("///", "").trim().to_string());
                }
                if trimmed.starts_with("//") {
                    return Some(trimmed.replace("//", "").trim().to_string());
                }
            }
            None
        }
        "ts" | "js" => {
            for line in content.lines().take(10) {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with("//") {
                    return Some(trimmed.replace("//", "").trim().to_string());
                }
            }
            None
        }
        _ => None
    }
}
