        let saved_scans = parts.as_ref().and_then(|parts| parts.scans.clone());
        let font_hit = saved_scans.is_some();
        let mut same_pages = None;
        let scans = saved_scans.unwrap_or_else(|| {
            // The full views are needed by Type3 emission. Keep that path
            // on the existing collector, including for unusual raster tables.
            let allow_pages = faces.iter().all(|face| {
                let tables = face.tables();
                tables.bdat.is_none() && tables.cbdt.is_none()
                    && tables.ebdt.is_none() && tables.sbix.is_none()
            });
            let Some(parts) = parts.as_mut().filter(|parts| parts.mode.fonts && allow_pages) else {
                return Arc::new(collect_document_fonts(doc, fonts, faces));
            };
            let old_scans = std::mem::take(&mut parts.page_fonts);
            let trace = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
            let mut recomputed = Vec::new();
            let mut matches = Vec::with_capacity(doc.pages.len());
            for (i, page) in doc.pages.iter().enumerate() {
                let unchanged = parts.previous_document.as_ref().and_then(|old| old.pages.get(i))
                    .is_some_and(|prior| reuse::same_page(prior, page));
                matches.push(unchanged);
                let cached = unchanged.then(|| old_scans.get(i)).flatten();
                parts.page_fonts.push(match cached {
                    Some(cached) => cached.clone(),
                    None => {
                        if trace { recomputed.push(i + 1); }
                        collect_page_fonts(page, fonts, faces)
                    }
                });
            }
            if trace {
                eprintln!("pdf-page-fonts pages={} recomputed={recomputed:?}", doc.pages.len());
            }
            same_pages = Some(matches);
            let merged = reuse::merge_page_fonts(&parts.page_fonts, fonts.len());
            if std::env::var_os("TEKT_LAB_FONT_SCAN_CHECK").is_some() {
                let mut exact = collect_document_fonts(doc, fonts, faces);
                for scan in &mut exact.faces {
                    scan.shaped_cluster_texts = reuse::compact_clusters(
                        std::mem::take(&mut scan.shaped_cluster_texts)
                    );
                }
                assert!(merged.faces == exact.faces && merged.used == exact.used);
            }
            Arc::new(merged)
        });
        if let Some(parts) = &mut parts {
            if parts.mode.fonts {
                parts.scans = Some(scans.clone());
            }
        }
