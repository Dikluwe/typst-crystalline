//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/layout_bidi.md
//! @prompt-hash b8767182
//! @layer L3
//! @updated 2026-07-04
//!
//! **P562/P564/P565** — Reordenação visual bidireccional de linhas e
//! reflow de blocos RTL. Passagem posterior pura sobre `PagedDocument`,
//! inserida entre layout (L1) e shaping (L3). Corrige a ordem visual de
//! palavras RTL (árabe, hebraico), recalcula as posições x com base nas
//! larguras reais e funde blocos de linhas RTL adjacentes quando o texto
//! total cabe na largura útil.

#![allow(deprecated)] // FrameItem::Text é o input legítimo desta passagem

use typst_core::compiler::layout::FontMetrics;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};
use unicode_bidi::{bidi_class, BidiClass, BidiInfo};

/// **P591/P593** — largura de um item de texto para reordenação bidi.
/// Delegado para `FontMetrics::text_width`, a fonte única do nível palavra
/// (shaping + tracking).
fn text_width_for_bidi(
    metrics: &dyn FontMetrics,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
) -> f64 {
    metrics.text_width(text, style.size, style).0
}

/// Tolerância para agrupar items na mesma linha visual (baseline y).
const Y_TOLERANCE_PT: f64 = 0.01;

/// Reordena os `FrameItem::Text` dentro de cada linha visual que tenha
/// direcção base RTL, recalcula as coordenadas x a partir das larguras
/// reais devolvidas por `metrics`, e funde blocos de linhas RTL
/// adjacentes quando isso corrigir quebras de linha provocadas pelo
/// layout LTR.
pub fn reorder_bidi_document(
    mut doc: PagedDocument,
    metrics: &dyn FontMetrics,
) -> PagedDocument {
    for page in &mut doc.pages {
        reorder_bidi_page(page, metrics);
    }
    doc
}

fn reorder_bidi_page(page: &mut Page, metrics: &dyn FontMetrics) {
    if page.items.is_empty() {
        return;
    }

    // Agrupar items por linha visual (baseline y dentro de tolerância).
    // Usa clustering em vez de agrupamento sequencial, para que items
    // não-texto (ex.: rectângulo de highlight) com y ligeiramente
    // diferente não separem textos da mesma linha.
    let mut lines: Vec<(f64, Vec<usize>)> = Vec::new();
    for i in 0..page.items.len() {
        let y = item_baseline_y(&page.items[i]).0;
        if let Some((_, indices)) = lines
            .iter_mut()
            .find(|(line_y, _)| (y - line_y).abs() <= Y_TOLERANCE_PT)
        {
            indices.push(i);
        } else {
            lines.push((y, vec![i]));
        }
    }

    // Detectar quais linhas são RTL (antes de qualquer reordenação, para
    // que o reflow trabalhe com os textos originais do Layouter).
    let line_is_rtl: Vec<bool> = lines
        .iter()
        .map(|(_, line)| detect_rtl_line(&page.items, line))
        .collect();

    // Reflow primeiro: fundir linhas consecutivas que formam um parágrafo
    // RTL, mesmo quando alguma linha intermédia contém texto LTR (ex.:
    // números dentro de texto árabe). O shaper é aplicado depois desta
    // passagem, por isso trabalhamos com os items Text do Layouter na sua
    // ordem lógica original.
    let fused_lines = reflow_rtl_paragraphs(page, &lines, &line_is_rtl, metrics);

    // Reordenar visualmente as linhas que não foram fundidas.
    for (i, (_, line)) in lines.iter().enumerate() {
        if line_is_rtl[i] && !fused_lines.contains(&i) {
            reorder_bidi_line(&mut page.items, line, metrics);
        }
    }

    // P569 — separar sufixos LTR (pontuação, dígitos) do final de items
    // RTL. Esta passagem insere novos items no vector da página, pelo que
    // é feita no fim, depois de todas as reordenações e reflows.
    split_ltr_suffixes_page(page, &mut lines, &line_is_rtl, &fused_lines, metrics);
}

/// Devolve a baseline y de um item para efeitos de agrupamento por linha.
fn item_baseline_y(item: &FrameItem) -> Pt {
    match item {
        FrameItem::Text { pos, .. } => pos.y,
        FrameItem::TextShaped { pos, .. } => pos.y,
        FrameItem::Line { start, .. } => start.y,
        FrameItem::Glyph { pos, .. } => pos.y,
        FrameItem::Image { pos, .. } => pos.y,
        FrameItem::Shape { pos, .. } => pos.y,
        FrameItem::Group { pos, .. } => pos.y,
        FrameItem::Link { pos, .. } => pos.y,
    }
}

/// Devolve a coordenada x de um item.
fn item_x(item: &FrameItem) -> f64 {
    match item {
        FrameItem::Text { pos, .. } => pos.x.0,
        FrameItem::TextShaped { pos, .. } => pos.x.0,
        FrameItem::Line { start, .. } => start.x.0,
        FrameItem::Glyph { pos, .. } => pos.x.0,
        FrameItem::Image { pos, .. } => pos.x.0,
        FrameItem::Shape { pos, .. } => pos.x.0,
        FrameItem::Group { pos, .. } => pos.x.0,
        FrameItem::Link { pos, .. } => pos.x.0,
    }
}

/// Reordena visualmente (RTL) os `FrameItem::Text` de uma linha,
/// assumindo que a linha já foi detectada como RTL. Recalcula as
/// posições x usando as larguras reais.
fn reorder_bidi_line(items: &mut [FrameItem], line: &[usize], metrics: &dyn FontMetrics) {
    let text_indices: Vec<usize> = line
        .iter()
        .copied()
        .filter(|&idx| matches!(items[idx], FrameItem::Text { .. }))
        .collect();

    if text_indices.len() <= 1 {
        return;
    }

    let widths: Vec<f64> = text_indices
        .iter()
        .map(|&idx| {
            if let FrameItem::Text { text, style, .. } = &items[idx] {
                text_width_for_bidi(metrics, text.as_str(), style)
            } else {
                0.0
            }
        })
        .collect();

    let x_min = item_x(&items[text_indices[0]]);
    // **P592/P593** — usar `FontMetrics::line_content_right`, a fonte única do
    // nível linha. O limite direito real do conteúdo (right edge do item mais
    // à direita) substitui o left edge do último item, evitando que espaços
    // finais sem item visual desloquem a linha RTL para a esquerda.
    let line_refs: Vec<&FrameItem> = line.iter().map(|&idx| &items[idx]).collect();
    let content_right = metrics.line_content_right(&line_refs);
    let total_width: f64 = widths.iter().sum();
    let gap = if text_indices.len() > 1 {
        ((content_right - x_min - total_width) / (text_indices.len() - 1) as f64).max(0.0)
    } else {
        0.0
    };

    let target_y = item_baseline_y(&items[text_indices[0]]).0;
    reorder_indices(items, &text_indices, metrics, x_min, gap, target_y);

    // P569 — depois de reposicionar visualmente, ordenar os items no vector
    // da página por x crescente (ordem visual esquerda→direita). Extratores
    // de texto como `pdftotext` seguem a ordem dos operadores no stream;
    // sem esta ordenação, caracteres RTL ficam na ordem lógica e espaços
    // entre palavras podem desaparecer na extração.
    sort_line_items(items, line);

    // P569 — itens de espaço soltos são neutros no bidi e ficam presos à
    // pontuação LTR. Coalescer cada espaço com o item de texto seguinte
    // na ordem visual (maior x), transformando-o em trailing space desse
    // item; o shaper `visual_runs` coloca trailing spaces do lado correcto
    // do run RTL.
    coalesce_space_items(items, line);
}

/// Aplica a ordem visual RTL a uma lista de índices, recalculando x e y.
fn reorder_indices(
    items: &mut [FrameItem],
    indices: &[usize],
    metrics: &dyn FontMetrics,
    x_min: f64,
    gap: f64,
    target_y: f64,
) {
    if indices.is_empty() {
        return;
    }

    // 1. Concatenar texto e registrar intervalos
    let mut line_text = String::new();
    let mut item_ranges = Vec::new();
    for &idx in indices {
        if let FrameItem::Text { text, .. } = &items[idx] {
            let start = line_text.len();
            line_text.push_str(text.as_str());
            let end = line_text.len();
            item_ranges.push((idx, start..end));
            line_text.push(' ');
        }
    }

    // 2. Determinar nível padrão do parágrafo
    let mut has_rtl_dir = false;
    for &idx in indices {
        if let FrameItem::Text { style, .. } = &items[idx] {
            if style.dir == Some(typst_core::entities::dir::Dir::RTL) {
                has_rtl_dir = true;
                break;
            }
        }
    }
    let default_level = if has_rtl_dir { Some(unicode_bidi::Level::rtl()) } else { None };

    // 3. Executar Unicode Bidi
    let bidi = BidiInfo::new(&line_text, default_level);
    if bidi.paragraphs.is_empty() {
        // Fallback para reversão total se não houver parágrafos (improvável)
        let mut pairs: Vec<(
            ecow::EcoString,
            typst_core::entities::layout_types::TextStyle,
        )> = indices
            .iter()
            .filter_map(|&idx| {
                if let FrameItem::Text { text, style, .. } = &items[idx] {
                    Some((text.clone(), style.clone()))
                } else {
                    None
                }
            })
            .collect();
        pairs.reverse();
        let mut current_x = x_min;
        for (&idx, (text, style)) in indices.iter().zip(pairs.into_iter()) {
            if let FrameItem::Text { text: t, style: s, pos } = &mut items[idx] {
                *t = text;
                *s = style;
                pos.x = Pt(current_x);
                pos.y = Pt(target_y);
                let w = text_width_for_bidi(metrics, t.as_str(), s);
                current_x += w + gap;
            }
        }
        return;
    }
    let para = &bidi.paragraphs[0];
    let (levels, runs) = bidi.visual_runs(para, para.range.clone());

    // 4. Mapear itens para runs visuais e reordenar
    let mut visual_item_order = Vec::new();
    for run_range in runs {
        let is_run_rtl = levels.get(run_range.start).map(|l| l.is_rtl()).unwrap_or(false);

        // Encontrar todos os itens que caem dentro deste run visual
        let mut run_items = Vec::new();
        for &(idx, ref range) in &item_ranges {
            let mid = (range.start + range.end) / 2;
            if run_range.contains(&mid) {
                run_items.push(idx);
            }
        }

        if is_run_rtl {
            run_items.reverse();
        }
        visual_item_order.extend(run_items);
    }

    // Adicionar quaisquer itens não mapeados
    for &idx in indices {
        if !visual_item_order.contains(&idx) {
            visual_item_order.push(idx);
        }
    }

    // 5. Coletar os pares (texto, estilo) originais correspondentes aos índices em ordem visual
    let mut pairs = Vec::new();
    for &idx in &visual_item_order {
        if let FrameItem::Text { text, style, .. } = &items[idx] {
            pairs.push((text.clone(), style.clone()));
        }
    }

    // 6. Posicionar os itens em ordem visual nas posições originais da linha
    let mut current_x = x_min;
    for (&idx, (text, style)) in indices.iter().zip(pairs.into_iter()) {
        if let FrameItem::Text { text: t, style: s, pos } = &mut items[idx] {
            *t = text;
            *s = style;
            pos.x = Pt(current_x);
            pos.y = Pt(target_y);
            let w = text_width_for_bidi(metrics, t.as_str(), s);
            current_x += w + gap;
        }
    }
}

/// Reordena os items de uma linha no vector `items` para a ordem visual
/// esquerda→direita (x crescente). Preserva os conteúdos já reposicionados.
fn sort_line_items(items: &mut [FrameItem], line: &[usize]) {
    if line.len() <= 1 {
        return;
    }
    let mut sorted: Vec<usize> = line.to_vec();
    sorted.sort_by(|&a, &b| {
        item_x(&items[a])
            .partial_cmp(&item_x(&items[b]))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    use typst_core::entities::layout_types::{Point, TextStyle};
    let dummy = FrameItem::Text {
        pos: Point::ZERO,
        text: ecow::EcoString::default(),
        style: TextStyle::default(),
    };

    let mut extracted: Vec<FrameItem> = Vec::with_capacity(line.len());
    for &idx in &sorted {
        extracted.push(std::mem::replace(&mut items[idx], dummy.clone()));
    }
    for (&idx, item) in line.iter().zip(extracted.into_iter()) {
        items[idx] = item;
    }
}

/// Junta items de espaço ao item de texto seguinte na ordem visual.
/// Modifica os textos dos items e remove os items de espaço do vector.
fn coalesce_space_items(items: &mut [FrameItem], line: &[usize]) {
    if line.len() <= 1 {
        return;
    }

    let mut sorted: Vec<usize> = line.to_vec();
    sorted.sort_by(|&a, &b| {
        item_x(&items[a])
            .partial_cmp(&item_x(&items[b]))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut to_remove: std::collections::BTreeSet<usize> =
        std::collections::BTreeSet::new();
    for i in 0..sorted.len() {
        if !is_space_item(&items[sorted[i]]) {
            continue;
        }
        // P569 — anexar o espaço como *trailing space* do item de texto
        // anterior na ordem visual (menor x). Assim o espaço fica entre
        // as duas palavras na ordem do stream e não é capturado por
        // sufixos LTR.
        for j in (0..i).rev() {
            if is_space_item(&items[sorted[j]]) {
                continue;
            }
            if let FrameItem::Text { text, .. } = &mut items[sorted[j]] {
                let mut merged = ecow::EcoString::with_capacity(text.len() + 1);
                merged.push_str(text.as_str());
                merged.push(' ');
                *text = merged;
            }
            to_remove.insert(sorted[i]);
            break;
        }
    }

    if to_remove.is_empty() {
        return;
    }

    // Remover items coalescidos substituindo por dummy e depois filtrando.
    // Como `line` contém índices no slice `items`, não podemos alterar
    // comprimentos; pomos texto vazio e posição fora da página para que
    // o export os ignore.
    for &idx in &to_remove {
        if let FrameItem::Text { text, .. } = &mut items[idx] {
            *text = ecow::EcoString::default();
        }
    }
}

fn is_space_item(item: &FrameItem) -> bool {
    matches!(item, FrameItem::Text { text, .. } if text.trim().is_empty())
}

/// P569 — separa sufixos com direcção forte LTR (pontuação, dígitos,
/// etc.) do final de cada item `FrameItem::Text` numa linha RTL.
/// O sufixo é colocado num item independente imediatamente à esquerda
/// do item original na ordem visual, evitando que o espaço neutro entre
/// palavras seja capturado pelo ponto na extracção sequencial.
fn split_ltr_suffixes_page(
    page: &mut Page,
    lines: &mut [(f64, Vec<usize>)],
    line_is_rtl: &[bool],
    fused_lines: &std::collections::HashSet<usize>,
    metrics: &dyn FontMetrics,
) {
    for (i, (_, line)) in lines.iter_mut().enumerate() {
        if !line_is_rtl[i] || fused_lines.contains(&i) {
            continue;
        }
        split_ltr_suffixes_line(&mut page.items, line, metrics);
    }
}

fn split_ltr_suffixes_line(
    items: &mut Vec<FrameItem>,
    line: &mut Vec<usize>,
    metrics: &dyn FontMetrics,
) {
    if line.is_empty() {
        return;
    }

    // Processar da direita para a esquerda (maior x primeiro). Cada
    // inserção acontece *antes* do índice do item base, pelo que não
    // afecta os índices dos items já processados (que têm x maior).
    let mut sorted: Vec<usize> = line.clone();
    sorted.sort_by(|&a, &b| {
        item_x(&items[b])
            .partial_cmp(&item_x(&items[a]))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for &idx in &sorted {
        let (text, style, x, y) = match &items[idx] {
            FrameItem::Text { text, style, pos } => {
                (text.clone(), style.clone(), pos.x.0, pos.y.0)
            }
            _ => continue,
        };

        let (base_prefix, suffix, trailing) = match split_ltr_suffix(text.as_str()) {
            Some(parts) => parts,
            None => continue,
        };

        let suffix_width = text_width_for_bidi(metrics, suffix, &style);
        let mut base_text =
            ecow::EcoString::with_capacity(base_prefix.len() + trailing.len());
        base_text.push_str(base_prefix);
        base_text.push_str(trailing);

        let suffix_item = FrameItem::Text {
            pos: Point { x: Pt(x - suffix_width), y: Pt(y) },
            text: suffix.into(),
            style: style.clone(),
        };

        items.insert(idx, suffix_item);

        // Ajustar todos os índices da linha que se deslocaram com a inserção.
        for n in line.iter_mut() {
            if *n >= idx {
                *n += 1;
            }
        }

        // O índice original agora aponta para o sufixo; a base passou
        // para idx + 1. Substituímos a referência no vector da linha para
        // apontar à base e adicionamos o sufixo como item próprio.
        for n in line.iter_mut() {
            if *n == idx {
                *n = idx + 1;
                break;
            }
        }
        line.push(idx);

        if let FrameItem::Text { text, .. } = &mut items[idx + 1] {
            *text = base_text;
        }
    }
}

/// Divide `s` em `(base_prefix, suffix, trailing)` onde `suffix` é o
/// maior sufixo final composto por caracteres LTR fortes (letras L,
/// dígitos EN/AN, e pontuação ASCII). `base_prefix` é a parte antes do
/// sufixo; `trailing` são os espaços finais originais do item (que
/// devem ser recolados ao base). Espaços iniciais do sufixo são
/// descartados. Devolve `None` quando não há sufixo a separar.
fn split_ltr_suffix(s: &str) -> Option<(&str, &str, &str)> {
    // Ignorar trailing spaces: eles pertencem ao base como separação
    // visual entre palavras.
    let trimmed_end = s.trim_end_matches(|c: char| c.is_whitespace());
    if trimmed_end.is_empty() {
        return None;
    }

    let mut split = trimmed_end.len();
    for (idx, ch) in trimmed_end.char_indices().rev() {
        if is_ltr_suffix_char(ch) {
            split = idx;
        } else {
            break;
        }
    }
    if split == trimmed_end.len() {
        return None;
    }

    let suffix = &trimmed_end[split..];
    let trimmed_suffix = suffix.trim_start();
    let suffix_leading_spaces = suffix.len() - trimmed_suffix.len();

    // Remover apenas os bytes do sufixo; os espaços à frente do sufixo
    // e os espaços finais originais permanecem no base.
    let suffix_start = split + suffix_leading_spaces;
    let suffix_end = suffix_start + trimmed_suffix.len();
    let base_prefix = &s[..suffix_start];
    let trailing = &s[suffix_end..];

    if base_prefix.trim().is_empty() {
        // Não dividir um item que é puramente sufixo (ex.: "42").
        return None;
    }
    Some((base_prefix, trimmed_suffix, trailing))
}

fn is_ltr_suffix_char(c: char) -> bool {
    if c.is_ascii() && !c.is_ascii_alphabetic() && !c.is_ascii_whitespace() {
        return true;
    }
    matches!(bidi_class(c), BidiClass::L | BidiClass::EN | BidiClass::AN)
}

/// Detecta se uma linha tem direcção base RTL, mesmo que tenha apenas
/// um item de texto.
fn detect_rtl_line(items: &[FrameItem], line: &[usize]) -> bool {
    let mut buffer = String::new();
    for &idx in line {
        if let FrameItem::Text { text, .. } = &items[idx] {
            buffer.push_str(text.as_str());
        }
    }
    if buffer.is_empty() {
        return false;
    }
    // Usa o nível de parágrafo determinado pelo algoritmo Unicode Bidi.
    // Linhas com prefixo LTR (ex.: "Arabic: \u0645\u0631\u062d\u0628\u0627") ficam
    // com nível LTR e NÃO são tratadas como RTL. Só parágrafos
    // iniciados com caracteres RTL fortes (\u00e1rabe, hebraico puro) têm
    // nível base RTL e entram no reflow.
    let bidi = BidiInfo::new(&buffer, None);
    bidi.paragraphs
        .first()
        .map(|para| para.level.is_rtl())
        .unwrap_or(false)
}

/// Identifica e funde linhas consecutivas que formam um parágrafo RTL,
/// mesmo quando alguma linha intermédia contém texto LTR. Devolve o
/// conjunto de índices de linhas que foram fundidas.
fn reflow_rtl_paragraphs(
    page: &mut Page,
    lines: &[(f64, Vec<usize>)],
    line_is_rtl: &[bool],
    metrics: &dyn FontMetrics,
) -> std::collections::HashSet<usize> {
    let mut fused_lines: std::collections::HashSet<usize> =
        std::collections::HashSet::new();

    let mut i = 0;
    while i < lines.len() {
        if !line_is_rtl[i] {
            i += 1;
            continue;
        }

        // Estender a run enquanto as linhas seguintes fizerem parte do
        // mesmo parágrafo (y próximo), independentemente de direcção.
        let mut end = i + 1;
        while end < lines.len() && same_paragraph(page, lines, end - 1, end, metrics) {
            end += 1;
        }

        // Ajustar os limites para que a run comece e termine em linhas RTL,
        // removendo linhas LTR soltas no início ou no fim.
        let mut run_start = i;
        let mut run_end = end;
        while run_start < run_end && !line_is_rtl[run_start] {
            run_start += 1;
        }
        while run_end > run_start && !line_is_rtl[run_end - 1] {
            run_end -= 1;
        }

        if run_end - run_start > 1
            && is_predominantly_rtl(line_is_rtl, run_start..run_end)
        {
            if try_fuse_paragraph(page, lines, run_start, run_end, metrics) {
                for k in run_start..run_end {
                    fused_lines.insert(k);
                }
            }
        }

        i = end;
    }

    fused_lines
}

/// Heurística de "mesmo parágrafo": duas linhas consecutivas estão
/// separadas por no máximo 1.5× a altura da linha seguinte.
fn same_paragraph(
    page: &Page,
    lines: &[(f64, Vec<usize>)],
    a: usize,
    b: usize,
    metrics: &dyn FontMetrics,
) -> bool {
    let y_diff = lines[b].0 - lines[a].0;
    let max_height = lines[b]
        .1
        .iter()
        .filter_map(|&idx| item_height(&page.items[idx]))
        .fold(0.0, f64::max);
    if y_diff > 1.5 * max_height.max(1.0) {
        return false;
    }

    // Heurística para evitar fundir linhas separadas por quebra manual (\ ou parágrafo).
    // O espaço disponível na linha `a` é medido até ao limite direito real do
    // contento (content_right), não até à largura da página — isso permite que
    // colunas e outros sub-layouts com largura reduzida sejam tratados
    // correctamente (P625).
    let line_a_refs: Vec<&FrameItem> =
        lines[a].1.iter().map(|&idx| &page.items[idx]).collect();
    let content_right = metrics.line_content_right(&line_a_refs);

    let line_a_end_x = lines[a]
        .1
        .iter()
        .map(|&idx| {
            let item = &page.items[idx];
            let x = item_x(item);
            let w = match item {
                FrameItem::Text { text, style, .. } => {
                    text.len() as f64 * style.size.0 * 0.5
                }
                _ => 0.0,
            };
            x + w
        })
        .max_by(|x1, x2| x1.partial_cmp(x2).unwrap())
        .unwrap_or(0.0);

    let remaining = content_right - line_a_end_x;

    if let Some(&first_b_idx) = lines[b]
        .1
        .iter()
        .find(|&&idx| matches!(page.items[idx], FrameItem::Text { .. }))
    {
        if let FrameItem::Text { text, style, .. } = &page.items[first_b_idx] {
            let word_w = text.len() as f64 * style.size.0 * 0.5;
            if remaining > word_w + 30.0 {
                return false;
            }
        }
    }

    true
}

/// Devolve a altura de um item, se tiver dimensão tipográfica.
fn item_height(item: &FrameItem) -> Option<f64> {
    match item {
        FrameItem::Text { style, .. } => Some(style.size.0),
        FrameItem::TextShaped { style, .. } => Some(style.size.0),
        _ => None, // neutro: FrameItem não-textual retorna None na extracção de altura tipográfica
    }
}

/// Verifica se a maioria das linhas do intervalo é RTL.
fn is_predominantly_rtl(line_is_rtl: &[bool], range: std::ops::Range<usize>) -> bool {
    let rtl_count = range.clone().filter(|&i| line_is_rtl[i]).count();
    rtl_count * 2 > range.len()
}

/// Tenta fundir as linhas [start, end) numa única linha. Se conseguir,
/// reposiciona os items e devolve true.
fn try_fuse_paragraph(
    page: &mut Page,
    lines: &[(f64, Vec<usize>)],
    start: usize,
    end: usize,
    metrics: &dyn FontMetrics,
) -> bool {
    // Apenas fundir se todas as linhas contiverem apenas items de texto.
    let has_non_text = lines[start..end].iter().any(|(_, line)| {
        line.iter()
            .any(|&idx| !matches!(page.items[idx], FrameItem::Text { .. }))
    });
    if has_non_text {
        return false;
    }

    // Coletar todos os items de texto na ordem original do Layouter
    // (ordem lógica do texto).
    let mut text_indices: Vec<usize> = Vec::new();
    for (_, line) in &lines[start..end] {
        for &idx in line {
            if matches!(page.items[idx], FrameItem::Text { .. }) {
                text_indices.push(idx);
            }
        }
    }

    if text_indices.len() <= 1 {
        return false;
    }

    let widths: Vec<f64> = text_indices
        .iter()
        .map(|&idx| {
            if let FrameItem::Text { text, style, .. } = &page.items[idx] {
                text_width_for_bidi(metrics, text.as_str(), style)
            } else {
                0.0
            }
        })
        .collect();

    let sum_widths: f64 = widths.iter().sum();
    let x_min = text_indices
        .iter()
        .map(|&idx| item_x(&page.items[idx]))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    // **P625** — usar o limite direito real do conteúdo (content_right) em vez
    // da largura da página. Isto faz com que sub-layouts com largura reduzida
    // (colunas, caixas, células) usem a sua própria largura útil ao decidir se
    // cabem numa única linha visual.
    let content_right = lines[start..end]
        .iter()
        .map(|(_, line)| {
            let line_refs: Vec<&FrameItem> =
                line.iter().map(|&idx| &page.items[idx]).collect();
            metrics.line_content_right(&line_refs)
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(page.width);
    let available_width = content_right - x_min;

    if sum_widths > available_width {
        return false;
    }

    let target_y = lines[start].0;
    let gap = if text_indices.len() > 1 {
        (available_width - sum_widths) / (text_indices.len() - 1) as f64
    } else {
        0.0
    };

    reorder_indices(
        page.items.as_mut_slice(),
        &text_indices,
        metrics,
        x_min,
        gap,
        target_y,
    );

    // P569 — manter ordem visual esquerda→direita no vector de items para
    // extratores de texto que seguem a ordem dos operadores PDF.
    let mut line_indices: Vec<usize> = lines[start..end]
        .iter()
        .flat_map(|(_, l)| l.iter().copied())
        .collect();
    sort_line_items(page.items.as_mut_slice(), &line_indices);

    // P569 — coalescer espaços e separar sufixos LTR na linha fundida.
    coalesce_space_items(page.items.as_mut_slice(), &line_indices);
    split_ltr_suffixes_line(&mut page.items, &mut line_indices, metrics);

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::compiler::layout::FixedMetrics;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::{
        Color, FrameItem, Page, PagedDocument, Point, Pt, TextStyle,
    };

    fn text_item(x: f64, y: f64, text: &str) -> FrameItem {
        text_item_with_size(x, y, text, Pt(12.0))
    }

    fn text_item_with_size(x: f64, y: f64, text: &str, size: Pt) -> FrameItem {
        FrameItem::Text {
            pos: Point { x: Pt(x), y: Pt(y) },
            text: text.into(),
            style: TextStyle::regular(size),
        }
    }

    fn page_with(items: Vec<FrameItem>) -> Page {
        Page {
            width: 595.0,
            height: 842.0,
            numbering: None,
            items,
        }
    }

    #[test]
    fn p565_latin_no_change() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "Hello"),
            text_item(150.0, 100.0, "world"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "Hello");
        assert_eq!(extract_text(&items[1]), "world");
        assert!((item_x(&items[0]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[1]) - 150.0).abs() < 0.001);
    }

    #[test]
    fn p565_reorder_arabic_line() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            text_item(200.0, 100.0, "على"),
            text_item(280.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Textos invertidos (ordem visual RTL).
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "الكتاب");

        // Larguras com FixedMetrics (size * 0.6 por codepoint):
        // الكتاب  = 6 chars → 43.2 pt
        // على     = 3 chars → 21.6 pt
        // الطاولة = 7 chars → 50.4 pt
        // width_before_last = 64.8; span = 280 - 100 = 180; gap = 57.6
        // Posições recalculadas a partir de x_min = 100:
        // الطاولة: 100.0
        // على:     100.0 + 50.4 + 57.6 = 208.0
        // الكتاب:  208.0 + 21.6 + 57.6 = 287.2
        assert!((item_x(&items[0]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[1]) - 208.0).abs() < 0.001);
        assert!((item_x(&items[2]) - 287.2).abs() < 0.001);
    }

    #[test]
    fn p565_mixed_latin_arabic() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            text_item(200.0, 100.0, "42"),
            text_item(240.0, 100.0, "على"),
            text_item(300.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "42");
        assert_eq!(extract_text(&items[3]), "الكتاب");

        // Larguras: 43.2 + 14.4 + 21.6 + 50.4 = 129.6
        // x_min = 100, x_max = 300, span = 200
        // width_before_last = 79.2; gap = (200 - 79.2) / 3 = 40.2666...
        let gap = (200.0 - 79.2) / 3.0;
        let mut expected_x = 100.0;
        assert!((item_x(&items[0]) - expected_x).abs() < 0.001);
        expected_x += 50.4 + gap;
        assert!((item_x(&items[1]) - expected_x).abs() < 0.001);
        expected_x += 21.6 + gap;
        assert!((item_x(&items[2]) - expected_x).abs() < 0.001);
        expected_x += 14.4 + gap;
        assert!((item_x(&items[3]) - expected_x).abs() < 0.001);
    }

    #[test]
    fn p565_empty_text_unchanged() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, ""),
            text_item(150.0, 100.0, "x"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "");
        assert_eq!(extract_text(&items[1]), "x");
    }

    #[test]
    fn p565_line_with_shape_unchanged() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            FrameItem::Shape {
                pos: Point { x: Pt(180.0), y: Pt(90.0) },
                kind: ShapeKind::Rect,
                width: 10.0,
                height: 10.0,
                fill: Some(Color::rgb(0, 0, 0)),
                stroke: None,
                parent_bbox_at_emit: None,
            },
            text_item(200.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert!(matches!(items[1], FrameItem::Shape { .. }));
        assert_eq!(extract_text(&items[2]), "الكتاب");

        // O shape não participa; apenas os dois textos entram no cálculo.
        // x_min = 100, x_max = 200, width_before_last = 43.2, gap = 56.8.
        assert!((item_x(&items[0]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[2]) - 207.2).abs() < 0.001);
    }

    #[test]
    fn p565_reflow_merges_rtl_lines_when_fits() {
        // Simular o que o Layouter LTR produziu para "الكتاب 42 على الطاولة"
        // a 40 pt, mas com margens artificiais que fazem o texto caber.
        // Larguras FixedMetrics a 40 pt:
        //   الكتاب = 144, 42 = 48, على = 72, الطاولة = 168
        // Total = 432. Com gap = 10, total = 462.
        // Página 595 x 842; margem = 66.5 → available = 595 - 133 = 462.
        // Cabe exactamente.
        // **P625** — o limite direito da primeira linha tem de tocar na
        // margem direita (content_right = 595 - 66.5 = 528.5), porque o
        // reflow agora usa o limite real do conteúdo, não a largura da
        // página. O último item da primeira linha foi reposicionado para
        // x = 528.5 - 72 = 456.5.
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item_with_size(66.5, 100.0, "الكتاب", Pt(40.0)),
            text_item_with_size(220.5, 100.0, "42", Pt(40.0)),
            text_item_with_size(456.5, 100.0, "على", Pt(40.0)),
            // Layouter colocou الطاولة numa segunda linha
            text_item_with_size(66.5, 130.0, "الطاولة", Pt(40.0)),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Todas as palavras devem estar na mesma linha (y = 100).
        assert_eq!(items.len(), 4);
        for item in items {
            if let FrameItem::Text { pos, .. } = item {
                assert!((pos.y.0 - 100.0).abs() < 0.001);
            }
        }

        // Ordem visual: الطاولة, على, 42, الكتاب
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "42");
        assert_eq!(extract_text(&items[3]), "الكتاب");

        // Posições: começam em x_min = 66.5, gap = 10.
        assert!((item_x(&items[0]) - 66.5).abs() < 0.001);
        assert!((item_x(&items[1]) - (66.5 + 168.0 + 10.0)).abs() < 0.001);
        assert!((item_x(&items[2]) - (66.5 + 168.0 + 10.0 + 72.0 + 10.0)).abs() < 0.001);
        assert!(
            (item_x(&items[3]) - (66.5 + 168.0 + 10.0 + 72.0 + 10.0 + 48.0 + 10.0)).abs()
                < 0.001
        );
    }

    #[test]
    fn p565_reflow_does_not_merge_when_too_wide() {
        // Mesmo texto, mas página mais estreita: não deve fundir.
        let mut page = page_with(vec![
            text_item_with_size(66.5, 100.0, "الكتاب", Pt(40.0)),
            text_item_with_size(220.5, 100.0, "42", Pt(40.0)),
            text_item_with_size(278.5, 100.0, "على", Pt(40.0)),
            text_item_with_size(66.5, 130.0, "الطاولة", Pt(40.0)),
        ]);
        page.width = 400.0; // available = 400 - 133 = 267 < 432
        let doc = PagedDocument::new(vec![page]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Ainda deve haver duas linhas.
        let mut y_values: Vec<f64> = items
            .iter()
            .filter_map(|item| {
                if let FrameItem::Text { pos, .. } = item {
                    Some((pos.y.0 * 100.0).round() / 100.0)
                } else {
                    None
                }
            })
            .collect();
        y_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y_values.dedup_by(|a, b| (*a - *b).abs() < 0.001);
        assert_eq!(y_values.len(), 2);
    }

    #[test]
    fn p569_ltr_suffix_split() {
        // Um único item RTL que termina com ponto LTR deve ser dividido.
        // FixedMetrics a 12 pt: 0.6 * 12 = 7.2 pt por caractere.
        // "قيمة" = 4 chars → 28.8 pt; "." = 1 char → 7.2 pt.
        let doc =
            PagedDocument::new(vec![page_with(vec![text_item(100.0, 100.0, "قيمة.")])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Sufixo à esquerda do texto base na ordem visual.
        assert_eq!(items.len(), 2);
        assert_eq!(extract_text(&items[0]), ".");
        assert_eq!(extract_text(&items[1]), "قيمة");
        assert!((item_x(&items[0]) - 92.8).abs() < 0.001);
        assert!((item_x(&items[1]) - 100.0).abs() < 0.001);
    }

    #[test]
    fn p569_trailing_space_coalesced() {
        // Linha RTL com espaço solto entre duas palavras. O espaço deve
        // tornar-se trailing space da palavra visualmente mais à esquerda.
        // "معلومات" = 8 chars → 57.6 pt; " " = 1 char → 7.2 pt;
        // "قيمة" = 4 chars → 28.8 pt.
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "معلومات"),
            text_item(200.0, 100.0, " "),
            text_item(208.0, 100.0, "قيمة"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Ordem visual x crescente: قيمة + espaço, (espaço vazio), معلومات.
        assert_eq!(extract_text(&items[0]), "قيمة ");
        assert_eq!(extract_text(&items[1]), "");
        assert_eq!(extract_text(&items[2]), "معلومات");

        // "معلومات" = 7 chars → 50.4 pt; " " = 1 char → 7.2 pt;
        // "قيمة" = 4 chars → 28.8 pt.
        // x_min = 100; x_max = 208; widths_before_last = 50.4 + 7.2 = 57.6.
        // gap = (208 - 100 - 57.6) / 2 = 25.2.
        assert!((item_x(&items[0]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[1]) - 154.0).abs() < 0.001);
        assert!((item_x(&items[2]) - 186.4).abs() < 0.001);
    }

    #[test]
    fn p569_suffix_and_space() {
        // Combinação: espaço solto + sufixo LTR no final.
        // "معلومات" = 7 → 50.4; " " = 7.2; "قيمة." = 5 → 36.0.
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "معلومات"),
            text_item(200.0, 100.0, " "),
            text_item(208.0, 100.0, "قيمة."),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Ordem visual x crescente: ".", "قيمة ", (vazio), "معلومات".
        assert_eq!(extract_text(&items[0]), ".");
        assert_eq!(extract_text(&items[1]), "قيمة ");
        assert_eq!(extract_text(&items[2]), "");
        assert_eq!(extract_text(&items[3]), "معلومات");

        // widths_before_last = 50.4 + 7.2 = 57.6.
        // gap = (208 - 100 - 57.6) / 2 = 25.2.
        // "قيمة." começa em x=100, largura 36.0; após split o ponto fica
        // em 100 - 7.2 = 92.8 e o base "قيمة " permanece em 100.
        assert!((item_x(&items[0]) - 92.8).abs() < 0.001);
        assert!((item_x(&items[1]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[3]) - 193.6).abs() < 0.001);
    }

    fn extract_text(item: &FrameItem) -> String {
        match item {
            FrameItem::Text { text, .. } => text.to_string(),
            _ => panic!("expected Text"),
        }
    }
}
