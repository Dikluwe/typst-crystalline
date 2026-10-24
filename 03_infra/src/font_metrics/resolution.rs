//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics/resolution.md
//! @prompt-hash cd68cced
//! @layer L3
//! @updated 2026-10-24

use super::FallbackFontMetrics;
use crate::fallback_fonts::{fallback_font_list_for, text_fallback_font_list};
use crate::font_variant::text_style_to_font_variant;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::{FontList, FontNamePattern};
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::layout_types::TextStyle;

/// Candidata a fonte para medição.
#[derive(Clone, Copy)]
pub(super) struct FontCandidate {
    pub(super) slot_idx: usize,
    pub(super) units_per_em: u16,
}

/// Latest complete base chain; bounded to one entry and shared by metrics clones.
pub(super) struct PrimaryResolution {
    font: Option<FontList>,
    variant: FontVariant,
    candidates: Vec<FontCandidate>,
}

impl FallbackFontMetrics<'_> {
    /// Resolve as fontes primárias declaradas no estilo. Se o estilo não
    /// declarar fonte (ou nenhuma resolver), replica o fallback padrão do
    /// shaper (`DEFAULT_FALLBACK_FONTS`) para manter as métricas alinhadas
    /// com a face que será efectivamente usada no shaping/PDF.
    pub(super) fn resolve_primary(&self, style: &TextStyle) -> Vec<FontCandidate> {
        let variant = text_style_to_font_variant(style);
        {
            let cache = self.primary_resolution.lock().unwrap();
            if let Some(entry) = cache.as_ref() {
                if entry.font == style.font && entry.variant == variant {
                    return entry.candidates.clone();
                }
            }
        }
        let mut primary = Vec::new();
        let mut complete = true;

        // Nome da primeira família para decidir a classe de fallback (P555).
        let first_family: Option<&str> = style
            .font
            .as_ref()
            .and_then(|fl| fl.as_slice().first())
            .and_then(|f| f.name.as_str());

        if let Some(font_list) = &style.font {
            for family in font_list.as_slice() {
                let Some(idx) = self.world.book().select_pattern(&family.name, &variant)
                else {
                    continue;
                };
                let Some(cached) = self.cached_face(idx) else {
                    complete = false;
                    continue;
                };
                primary.push(FontCandidate {
                    slot_idx: idx,
                    units_per_em: cached.face().units_per_em().max(1),
                });
            }
        }

        if primary.is_empty() {
            let fallback_list = fallback_font_list_for(first_family.unwrap_or(""));
            for family in fallback_list {
                let pattern = FontNamePattern::Literal(ecow::EcoString::from(*family));
                let Some(idx) = self.world.book().select_pattern(&pattern, &variant)
                else {
                    continue;
                };
                let Some(cached) = self.cached_face(idx) else {
                    complete = false;
                    continue;
                };
                primary.push(FontCandidate {
                    slot_idx: idx,
                    units_per_em: cached.face().units_per_em().max(1),
                });
                break;
            }
        }

        // Mesma cadeia textual explícita consultada pelo shaper, antes do
        // fallback global. Preservar a prioridade das famílias declaradas.
        for family in text_fallback_font_list() {
            let pattern = FontNamePattern::Literal(ecow::EcoString::from(*family));
            let Some(idx) = self.world.book().select_pattern(&pattern, &variant) else {
                continue;
            };
            if primary.iter().any(|candidate| candidate.slot_idx == idx) {
                continue;
            }
            let Some(cached) = self.cached_face(idx) else {
                complete = false;
                continue;
            };
            primary.push(FontCandidate {
                slot_idx: idx,
                units_per_em: cached.face().units_per_em().max(1),
            });
        }

        if complete && !primary.is_empty() {
            *self.primary_resolution.lock().unwrap() = Some(PrimaryResolution {
                font: style.font.clone(),
                variant,
                candidates: primary.clone(),
            });
        }
        primary
    }

    /// **P890** — como `resolve_primary`, mas injecta a cadeia de fallback
    /// matemático (`math_fallback_font_list()`) como primárias adicionais
    /// quando `style.math` é verdadeiro — mesma condição e cadeia que
    /// `shaper.rs::try_shape`/`shaped_width` (P783/P784) já aplicam antes de
    /// chamar `covering`/`covering_run`. Extraída para uso partilhado entre
    /// **todos** os métodos de `FontMetrics` que resolvem cobertura
    /// carácter-a-carácter (`advance`, `text_ink_bounds`) — antes de P890,
    /// só `text_ink_bounds` tinha esta injecção; `advance` chamava
    /// `resolve_primary` puro e, por correr primeiro em
    /// `layout_equation_measured`, era sempre quem disparava o scan caro de
    /// `World::candidates_for_char` para glifos matemáticos (letras
    /// itálicas de variável, letras gregas) só cobertos pela fonte MATH, não
    /// pela fonte de corpo — mesmo quando a fonte certa já está na cadeia
    /// dedicada, na primeira posição (`typst-passo-890-relatorio.md`).
    pub(super) fn resolve_primary_with_math_fallback(
        &self,
        style: &TextStyle,
        variant: &FontVariant,
    ) -> Vec<FontCandidate> {
        let mut primary = self.resolve_primary(style);
        if style.math {
            for family in crate::fallback_fonts::math_fallback_font_list() {
                let pattern = FontNamePattern::Literal(ecow::EcoString::from(*family));
                let Some(idx) = self.world.book().select_pattern(&pattern, variant)
                else {
                    continue;
                };
                if primary.iter().any(|p| p.slot_idx == idx) {
                    continue;
                }
                let Some(cached) = self.cached_face(idx) else { continue };
                primary.push(FontCandidate {
                    slot_idx: idx,
                    units_per_em: cached.face().units_per_em().max(1),
                });
            }
        }
        primary
    }

    /// Encontra a fonte que cobre `c`: primárias primeiro (a primeira que
    /// cobre); se nenhuma cobrir, recolhe **todas** as fontes do `FontBook`
    /// que cobrem o caractere e escolhe via `FontBook::select_fallback`
    /// (**P838** — scoring de similaridade do vanilla, com `like` = `FontInfo`
    /// da primeira primária). Isto alinha a fonte usada na medição com a
    /// usada no shaping, que aplica o mesmo scoring.
    pub(super) fn covering_text(
        &self,
        c: char,
        primary: &[FontCandidate],
        variant: &FontVariant,
    ) -> Option<FontCandidate> {
        primary
            .iter()
            .find(|candidate| {
                self.cached_face(candidate.slot_idx)
                    .is_some_and(|face| face.face().glyph_index(c).is_some())
            })
            .copied()
            .or_else(|| self.covering(c, primary, variant))
    }

    pub(super) fn covering(
        &self,
        c: char,
        primary: &[FontCandidate],
        variant: &FontVariant,
    ) -> Option<FontCandidate> {
        // Primeiro passe (P912): dar prioridade a qualquer face em `primary` que
        // possua a tabela OpenType MATH e cubra `c`. Em contexto matemático, isto
        // garante que glifos comuns como `(`, `)`, `{` resolvam para a fonte MATH
        // (ex: New Computer Modern Math) em vez de pararem na primeira fonte de
        // corpo sem tabela MATH.
        for cand in primary {
            if let Some(cached) = self.cached_face(cand.slot_idx) {
                let face = cached.face();
                if face.tables().math.is_some() && face.glyph_index(c).is_some() {
                    return Some(*cand);
                }
            }
        }

        for cand in primary {
            let cached = self.cached_face(cand.slot_idx)?;
            if cached.face().glyph_index(c).is_some() {
                return Some(*cand);
            }
        }

        let book = self.world.book();
        let like = primary.first().and_then(|cand| book.infos().get(cand.slot_idx));
        let mut ids = Vec::new();
        // **P880** — usar o filtro de coverage via World::candidates_for_char.
        // SystemWorld calcula coverage lazy; MockWorlds usam o FontBook via default.
        // Isso evita carregar dezenas de faces CJK só para verificar cobertura
        // de caracteres que elas não cobrem.
        //
        // **P942** — a coverage exacta (P937/P938, runs de codepoints) já é
        // definitiva: o vanilla confia nela em `select_fallback` (book.rs:106-114)
        // e não re-verifica `glyph_index`. A versão anterior carregava a face de
        // cada candidata que cobria o caractere (dezenas de `.ttc` de CJK,
        // ~10-20 MB cada) só para re-verificar `glyph_index` — redundante e
        // medido como o custo dominante do fallback de layout (~480 ms em
        // `utf8-cjk`). Carrega-se agora só a face da fonte vencedora.
        for slot_idx in self.world.candidates_for_char(c) {
            if primary.iter().any(|cand| cand.slot_idx == slot_idx) {
                continue;
            }
            ids.push(slot_idx);
        }

        let best = book.select_fallback(like, variant, ids)?;
        let cached = self.cached_face(best)?;
        Some(FontCandidate {
            slot_idx: best,
            units_per_em: cached.face().units_per_em().max(1),
        })
    }

    /// **P906** — resolve o combo `(FontList, FontVariant, FontVariations)`
    /// que efectivamente cobre `c`, mesmo mecanismo de `covering` +
    /// `resolve_primary_with_math_fallback` (inclui a cadeia de fallback
    /// matemático quando `style.math`), mas devolve a IDENTIDADE da fonte
    /// (via `FontInfo` no `slot_idx` resolvido) em vez de dados de glifo.
    ///
    /// Usado por `pipeline::collect_fonts_in_items` para que `FrameItem::
    /// Glyph` (glifos de esticamento matemático, sem `style.font` já
    /// apontado à fonte certa como `FrameItem::TextShaped` tem após
    /// `shaper.rs`) entre correctamente na selecção Cidfont-vs-Multifont —
    /// achado: antes deste passo, `collect_fonts_in_items` ignorava
    /// `FrameItem::Glyph` por completo (braço `FrameItem::Glyph { .. } =>
    /// {}`), fazendo equações cujo ÚNICO conteúdo a precisar da fonte MATH
    /// fosse um glifo de esticamento (`underbracket`/`underbrace`/etc. sem
    /// texto itálico à volta) escolherem só a fonte de corpo como candidata
    /// Cidfont única — os glyph_ids de esticamento, correctos na fonte MATH,
    /// caíam fora do subset embutido (glifo `.notdef`). Ver `pipeline.md`
    /// §P906.
    pub(crate) fn resolve_font_combo(
        &self,
        c: char,
        style: &TextStyle,
    ) -> Option<(FontList, FontVariant, FontVariations)> {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let cand = self.covering(c, &primary, &variant)?;
        let info = self.world.book().infos().get(cand.slot_idx)?;
        Some((
            FontList::single(ecow::EcoString::from(info.family.as_str())),
            info.variant,
            style.variations.clone().unwrap_or_default(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font_metrics::FallbackFontMetrics;
    use typst_core::compiler::layout::FontMetrics;
    use typst_core::contracts::world::World;
    use typst_core::entities::layout_types::{Pt, TextStyle};

    /// **P838** (achado #24 de P831) — o fallback global de `covering` usa o
    /// scoring de similaridade do vanilla (`FontBook::select_fallback`) em vez
    /// da primeira fonte por ordem de índice: para `like` sans-serif
    /// (DejaVu Sans), CJK resolve para "Noto Sans CJK JP" e não para
    /// "Droid Sans Fallback", alinhando a fonte medida com a do shaping.
    #[test]
    fn p838_covering_fallback_scoring_vanilla() {
        use crate::world::SystemWorld;
        use typst_core::contracts::world::World;

        let dir = std::env::temp_dir().join(format!(
            "typst-fontmetrics-test-p838-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.typ"), "text").unwrap();
        let Ok(world) = SystemWorld::new(&dir, "main.typ").map(|w| w.with_system_fonts())
        else {
            return;
        };
        let book = world.book();
        if !book.infos().iter().any(|i| i.family == "Noto Sans CJK JP") {
            eprintln!("SKIP: Noto Sans CJK JP não instalada");
            return;
        }
        if !book.infos().iter().any(|i| i.family == "Droid Sans Fallback") {
            eprintln!("SKIP: Droid Sans Fallback não instalada");
            return;
        }

        let metrics = FallbackFontMetrics::new(&world);
        let style = TextStyle {
            font: Some(typst_core::entities::font_list::FontList::single(
                ecow::EcoString::from("DejaVu Sans"),
            )),
            ..TextStyle::default()
        };
        let primary = metrics.resolve_primary(&style);
        assert!(!primary.is_empty(), "DejaVu Sans resolve como primária");

        let cand = metrics
            .covering('日', &primary, &FontVariant::default())
            .expect("alguma fonte do book cobre CJK");
        let chosen = &book.infos()[cand.slot_idx];
        assert_eq!(
            chosen.family, "Noto Sans CJK JP",
            "scoring vanilla escolhe Noto Sans CJK JP, não {:?}",
            chosen.family
        );
        assert_eq!(
            chosen.variant.weight.to_number(),
            400,
            "a face Regular (w400) vence a Bold (w700) por distância de variante, \
             como no vanilla (NotoSansCJKjp-Regular)"
        );
    }

    /// **P880** — `FallbackFontMetrics::covering` deve respeitar o conjunto
    /// devolvido por `World::candidates_for_char` e NUNCA carregar faces cujo
    /// índice não esteja nesse conjunto. Teste de regressão do loop corrigido
    /// em P879/P880.
    #[test]
    fn p880_covering_nao_carrega_faces_fora_de_candidates_for_char() {
        use std::num::NonZeroU16;
        use typst_core::contracts::world::World;
        use typst_core::entities::file_id::FileId;
        use typst_core::entities::font_book::{
            Coverage, FontBook, FontFlags, FontInfo, FontStretch, FontStyle, FontVariant,
            FontWeight,
        };
        use typst_core::entities::source::Source;
        use typst_core::entities::world_types::{
            Bytes, Datetime, FileError, FileResult, Font, Library,
        };

        struct FilteredWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
            allowed: Vec<usize>,
        }

        impl World for FilteredWorld {
            fn library(&self) -> &Library {
                &self.library
            }
            fn book(&self) -> &FontBook {
                &self.book
            }
            fn main(&self) -> FileId {
                FileId::from_raw(NonZeroU16::new(1).unwrap())
            }
            fn source(&self, _: FileId) -> FileResult<Source> {
                Err(FileError::NotFound)
            }
            fn file(&self, _: FileId) -> FileResult<Bytes> {
                Err(FileError::NotFound)
            }
            fn font(&self, idx: usize) -> Option<Font> {
                assert!(
                    self.allowed.contains(&idx),
                    "covering carregou face fora de candidates_for_char: idx={}",
                    idx
                );
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(
                &self,
                _: Option<typst_core::entities::duration::Duration>,
            ) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                self.allowed.clone()
            }
        }

        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let nimbus = Font::from_data(data);

        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "Nimbus Sans".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        // Duas entradas fictícias que NÃO estão em `allowed`.
        for family in ["Dummy One", "Dummy Two"] {
            book.push(FontInfo {
                family: family.into(),
                variant: FontVariant {
                    style: FontStyle::Normal,
                    weight: FontWeight::REGULAR,
                    stretch: FontStretch::NORMAL,
                },
                flags: FontFlags::default(),
                coverage: Coverage::default(),
            });
        }

        let world = FilteredWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(nimbus), None, None],
            allowed: vec![0],
        };

        let metrics = FallbackFontMetrics::new(&world);
        let style = TextStyle {
            font: Some(typst_core::entities::font_list::FontList::single(
                ecow::EcoString::from("Nimbus Sans"),
            )),
            ..TextStyle::default()
        };
        let primary = metrics.resolve_primary(&style);
        assert!(!primary.is_empty(), "Nimbus Sans deve resolver como primária");

        // '你' (CJK) não é coberto por Nimbus Sans. O fallback global só deve
        // considerar os índices devolvidos por `candidates_for_char` (apenas 0),
        // e portanto não deve carregar os slots 1 e 2.
        let _ = metrics.covering('你', &primary, &FontVariant::default());
        // Se `covering` tentasse carregar slot 1 ou 2, `FilteredWorld::font`
        // daria panic.
    }

    /// **P890** — `advance()` deve encontrar um carácter coberto só pela
    /// cadeia de fallback matemático (`math_fallback_font_list()`, primeira
    /// entrada "New Computer Modern Math") **sem** chamar
    /// `World::candidates_for_char` — o scan caro só é aceitável quando a
    /// cadeia dedicada genuinamente não cobre o carácter, não quando cobre
    /// mas não foi consultada (a causa exacta confirmada por instrumentação
    /// em `typst-passo-890-relatorio.md`). Antes da correcção, `advance()`
    /// chamava só `resolve_primary` (sem a cadeia math) e este teste falha.
    #[test]
    fn p890_advance_math_nao_chama_candidates_for_char_quando_math_fallback_cobre() {
        use std::num::NonZeroU16;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use typst_core::contracts::world::World;
        use typst_core::entities::file_id::FileId;
        use typst_core::entities::font_book::{
            Coverage, FontBook, FontFlags, FontInfo, FontStretch, FontStyle, FontVariant,
            FontWeight,
        };
        use typst_core::entities::source::Source;
        use typst_core::entities::world_types::{
            Bytes, Datetime, FileError, FileResult, Font, Library,
        };

        struct SpyWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
            candidates_for_char_calls: AtomicUsize,
        }

        impl World for SpyWorld {
            fn library(&self) -> &Library {
                &self.library
            }
            fn book(&self) -> &FontBook {
                &self.book
            }
            fn main(&self) -> FileId {
                FileId::from_raw(NonZeroU16::new(1).unwrap())
            }
            fn source(&self, _: FileId) -> FileResult<Source> {
                Err(FileError::NotFound)
            }
            fn file(&self, _: FileId) -> FileResult<Bytes> {
                Err(FileError::NotFound)
            }
            fn font(&self, idx: usize) -> Option<Font> {
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(
                &self,
                _: Option<typst_core::entities::duration::Duration>,
            ) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                // O scan caro em si (custo de P880/candidates_for_char) não é
                // o que este teste verifica — o que importa é que NÃO seja
                // chamado neste cenário. Regista a chamada em vez de dar
                // panic para a asserção poder reportar a contagem exacta.
                self.candidates_for_char_calls.fetch_add(1, Ordering::SeqCst);
                vec![]
            }
        }

        // Nimbus Sans (fonte de corpo declarada) NÃO tem 'Ə' (U+018F).
        let nimbus_data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        // Cantarell TEM 'Ə' (U+018F) — usada aqui a fazer de stand-in para
        // "New Computer Modern Math" (não precisamos da fonte MATH real de
        // produção só para provar que a cadeia é consultada; qualquer fonte
        // registada sob esse nome exacto serve para o mecanismo).
        let cantarell_data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/Cantarell-VF.otf"
        ))
        .expect("fixture Cantarell-VF.otf necessária");

        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "Nimbus Sans".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        book.push(FontInfo {
            family: "New Computer Modern Math".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });

        let world = SpyWorld {
            library: Library::new(),
            book,
            fonts: vec![
                Some(Font::from_data(nimbus_data)),
                Some(Font::from_data(cantarell_data)),
            ],
            candidates_for_char_calls: AtomicUsize::new(0),
        };

        let metrics = FallbackFontMetrics::new(&world);
        let style = TextStyle {
            font: Some(typst_core::entities::font_list::FontList::single(
                ecow::EcoString::from("Nimbus Sans"),
            )),
            math: true,
            ..TextStyle::default()
        };

        let _ = metrics.advance("\u{018F}", Pt(11.0), &style);

        assert_eq!(
            world.candidates_for_char_calls.load(Ordering::SeqCst),
            0,
            "advance() não deveria ter caído no scan caro de candidates_for_char — \
             'Ə' (U+018F) já está coberto pela cadeia de fallback matemático \
             ('New Computer Modern Math'), só faltava consultá-la antes do scan global"
        );
    }

    // P912 — `covering()` deve preferir a fonte com tabela MATH quando `style.math` é
    // verdadeiro, para que delimitadores como `(`, `)` resolvam para a fonte MATH.
    #[test]
    fn p912_covering_prefere_fonte_math_para_glifos_comuns() {
        use crate::world::SystemWorld;

        let dir = std::env::temp_dir().join(format!(
            "typst-fontmetrics-test-p912-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.typ"), "text").unwrap();
        let Ok(world) = SystemWorld::new(&dir, "main.typ").map(|w| w.with_system_fonts())
        else {
            return;
        };
        let book = world.book();
        if !book.infos().iter().any(|i| i.family.contains("Math")) {
            eprintln!("SKIP: Nenhuma fonte MATH instalada no sistema");
            return;
        }

        let metrics = FallbackFontMetrics::new(&world);
        let style = TextStyle {
            font: Some(typst_core::entities::font_list::FontList::single(
                ecow::EcoString::from("DejaVu Sans"),
            )),
            math: true,
            ..TextStyle::default()
        };

        let variant = text_style_to_font_variant(&style);
        let primary = metrics.resolve_primary_with_math_fallback(&style, &variant);
        let cand = metrics
            .covering('(', &primary, &variant)
            .expect("alguma fonte cobre '('");

        let chosen = metrics.cached_face(cand.slot_idx).expect("cached face");
        assert!(
            chosen.face().tables().math.is_some(),
            "covering('(') com style.math=true deve escolher uma face com tabela OpenType MATH"
        );
    }
}

#[cfg(test)]
mod primary_reuse_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use typst_core::{
        compiler::layout::FontMetrics,
        contracts::world::World,
        entities::{
            duration::Duration,
            file_id::FileId,
            font_book::{Coverage, FontBook, FontFlags, FontInfo, FontStyle, FontWeight},
            font_list::FontFamily,
            layout_types::Pt,
            regex::Regex,
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
    };

    struct ProbeWorld {
        library: Library,
        book: FontBook,
        fonts: Vec<Font>,
        book_reads: AtomicUsize,
        blocked_slot: AtomicUsize,
    }
    impl World for ProbeWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            self.book_reads.fetch_add(1, Ordering::Relaxed);
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, id: usize) -> Option<Font> {
            if self.blocked_slot.load(Ordering::Relaxed) == id {
                None
            } else {
                self.fonts.get(id).cloned()
            }
        }
        fn today(&self, _: Option<Duration>) -> Option<Datetime> {
            None
        }
    }
    fn world(reverse: bool) -> ProbeWorld {
        let load = |name: &str| {
            Font::from_data(
                std::fs::read(
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join("fixtures/fonts")
                        .join(name),
                )
                .unwrap(),
            )
        };
        let regular = load("NimbusSans-Regular.otf");
        let mut entries = vec![
            ("Alpha", FontVariant::default(), regular.clone()),
            (
                "Alpha",
                FontVariant { weight: FontWeight::BOLD, ..Default::default() },
                regular.clone(),
            ),
            (
                "Alpha",
                FontVariant { style: FontStyle::Italic, ..Default::default() },
                regular.clone(),
            ),
            ("Beta", FontVariant::default(), regular),
            (
                "New Computer Modern Math",
                FontVariant::default(),
                load("NewCMMath-Book.otf"),
            ),
            ("Noto Sans", FontVariant::default(), load("NotoSans-Regular.ttf")),
        ];
        if reverse {
            entries.reverse();
        }
        let mut book = FontBook::new();
        let mut fonts = Vec::new();
        for (family, variant, font) in entries {
            book.push(FontInfo {
                family: family.into(),
                variant,
                flags: FontFlags::default(),
                coverage: Coverage::default(),
            });
            fonts.push(font);
        }
        ProbeWorld {
            library: Library::new(),
            book,
            fonts,
            book_reads: AtomicUsize::new(0),
            blocked_slot: AtomicUsize::new(usize::MAX),
        }
    }
    fn style(names: &[&str]) -> TextStyle {
        TextStyle {
            font: Some(
                FontList::new(
                    names
                        .iter()
                        .map(|s| FontFamily::new(ecow::EcoString::from(*s)))
                        .collect(),
                )
                .unwrap(),
            ),
            ..TextStyle::regular(Pt(10.0))
        }
    }
    fn slots(candidates: Vec<FontCandidate>) -> Vec<(usize, u16)> {
        candidates.into_iter().map(|c| (c.slot_idx, c.units_per_em)).collect()
    }

    #[test]
    fn primary_reuse_repeated_metrics_share_resolution_across_clones() {
        let world = world(false);
        let metrics = FallbackFontMetrics::new(&world);
        let style = style(&["Alpha"]);
        let expected = slots(metrics.resolve_primary(&style));
        let book_reads = world.book_reads.load(Ordering::Relaxed);
        assert!(book_reads > 0);
        let clone = metrics.clone();
        assert_eq!(slots(clone.resolve_primary(&style)), expected);
        let edge = metrics.text_edges(Pt(10.0), &style);
        assert_eq!(clone.text_edges(Pt(10.0), &style), edge);
        let vertical = metrics.vertical_metrics(Pt(10.0), &style);
        assert_eq!(clone.vertical_metrics(Pt(10.0), &style), vertical);
        assert_eq!(
            world.book_reads.load(Ordering::Relaxed),
            book_reads,
            "unchanged metrics should reuse catalog resolution"
        );
    }

    #[test]
    fn primary_reuse_selection_changes_match_fresh_metrics_in_both_orders() {
        let world = world(false);
        let cached = FallbackFontMetrics::new(&world);
        let regular = style(&["Alpha"]);
        let bold = TextStyle { bold: true, ..regular.clone() };
        let italic = TextStyle { italic: true, ..regular.clone() };
        let explicit_weight = TextStyle { weight: Some(400), ..bold.clone() };
        let regex = TextStyle {
            font: Some(
                FontList::new(vec![FontFamily::new_regex(
                    Regex::new("^Beta$").unwrap(),
                    vec![],
                )])
                .unwrap(),
            ),
            ..regular.clone()
        };
        let cases = vec![
            regular,
            bold,
            italic,
            explicit_weight,
            regex,
            style(&["Beta", "Alpha"]),
            style(&["Alpha", "Beta"]),
            style(&["Missing"]),
        ];
        for styles in [cases.clone(), cases.into_iter().rev().collect()] {
            for style in styles {
                let fresh = FallbackFontMetrics::new(&world);
                let expected = slots(fresh.resolve_primary(&style));
                assert_eq!(slots(cached.resolve_primary(&style)), expected);
                assert_eq!(slots(cached.resolve_primary(&style)), expected);
                assert_eq!(
                    cached.text_edges(Pt(17.0), &style),
                    fresh.text_edges(Pt(17.0), &style)
                );
            }
        }
        assert_eq!(slots(cached.resolve_primary(&style(&["Alpha"])))[0].0, 0);
    }

    #[test]
    fn primary_reuse_failed_face_is_retried_and_worlds_are_independent() {
        let world = world(false);
        world.blocked_slot.store(0, Ordering::Relaxed);
        let metrics = FallbackFontMetrics::new(&world);
        let style = style(&["Alpha"]);
        assert_eq!(slots(metrics.resolve_primary(&style))[0].0, 5);
        world.blocked_slot.store(usize::MAX, Ordering::Relaxed);
        assert_eq!(
            slots(metrics.resolve_primary(&style))[0].0,
            0,
            "partial fallback resolution must not hide newly readable primary"
        );
        let other = super::primary_reuse_tests::world(true);
        assert_eq!(
            slots(FallbackFontMetrics::new(&other).resolve_primary(&style))[0].0,
            5
        );
    }

    #[test]
    fn primary_reuse_math_and_variations_are_applied_after_base_reuse() {
        let world = world(false);
        let metrics = FallbackFontMetrics::new(&world);
        let text = style(&["Alpha"]);
        assert!(!slots(metrics.resolve_primary(&text)).iter().any(|c| c.0 == 4));
        let math = TextStyle {
            math: true,
            variations: Some(FontVariations(vec![(*b"wght", 550.0)])),
            ..text.clone()
        };
        for style in [&math, &text, &math] {
            let variant = text_style_to_font_variant(style);
            let fresh = FallbackFontMetrics::new(&world);
            let actual =
                slots(metrics.resolve_primary_with_math_fallback(style, &variant));
            assert_eq!(
                actual,
                slots(fresh.resolve_primary_with_math_fallback(style, &variant))
            );
            assert_eq!(actual.iter().any(|c| c.0 == 4), style.math);
            assert_eq!(
                metrics.resolve_font_combo('a', style),
                fresh.resolve_font_combo('a', style)
            );
        }
    }
}
