//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/font_selection.md
//! @prompt-hash 8e04e77b
//! @layer L3
//! @updated 2026-09-29

use super::bidi::BidiRun;
use super::cache::FaceCache;
use crate::fallback_fonts::{fallback_font_list_for, math_fallback_font_list, text_fallback_font_list};
use std::collections::HashMap;
use typst_core::contracts::world::World;
use typst_core::entities::font_book::{FontInfo, FontVariant};
use typst_core::entities::font_list::FontList;
use unicode_script::{Script, UnicodeScript};

pub(super) fn primary_candidates(
    world: &dyn World,
    font_list: &FontList,
    variant: &FontVariant,
    face_cache: &mut FaceCache,
    math: bool,
) -> Vec<FontCandidate> {
    let mut primary =
        resolve_candidates(world, font_list, variant, face_cache).unwrap_or_default();
    if primary.is_empty() {
        let first_family = font_list
            .as_slice()
            .first()
            .and_then(|f| f.name.as_str())
            .unwrap_or("");
        let fallback_list = fallback_font_list_for(first_family);
        for family in fallback_list {
            let fallback_font_list = FontList::single(ecow::EcoString::from(*family));
            if let Some(cands) =
                resolve_candidates(world, &fallback_font_list, variant, face_cache)
            {
                if !cands.is_empty() {
                    primary = cands;
                    break;
                }
            }
        }
    } else {
        // **P783/P784** — adicionar a cadeia de fallback matemático como
        // primárias adicionais, para que glifos ausentes na primária sejam
        // buscados em fontes math dedicadas antes do fallback global (todo
        // o FontBook em ordem de índice). Dois gatilhos independentes
        // (OR): `math` (P784 — sempre que o texto vem do motor de
        // layout matemático, `layout/equation.rs`, independentemente de a
        // fonte de corpo por omissão ter ou não tabela MATH própria — ela
        // não tem, `Libertinus Serif` não tem MATH, então este era o único
        // caso que realmente falhava e motivou este passo) e
        // `primary_has_math` (P783 original — a fonte já resolvida declara
        // MATH própria, útil se o utilizador define `font:
        // "New Computer Modern Math"` explicitamente fora de `$...$`).
        let primary_has_math = primary
            .first()
            .and_then(|cand| face_cache.get(world, cand.slot_idx))
            .map_or(false, |cached| cached.face().tables().math.is_some());
        if math || primary_has_math {
            for family in math_fallback_font_list() {
                let math_font_list = FontList::single(ecow::EcoString::from(*family));
                if let Some(cands) =
                    resolve_candidates(world, &math_font_list, variant, face_cache)
                {
                    for cand in cands {
                        if !primary.iter().any(|p| p.slot_idx == cand.slot_idx) {
                            primary.push(cand);
                        }
                    }
                }
            }
        }
    }

    // Cadeia textual declarativa do vanilla: famílias escolhidas primeiro,
    // depois fallback comum, só então busca global por cobertura.
    for family in text_fallback_font_list() {
        let list = FontList::single(ecow::EcoString::from(*family));
        if let Some(candidates) = resolve_candidates(world, &list, variant, face_cache) {
            for candidate in candidates {
                if !primary.iter().any(|item| item.slot_idx == candidate.slot_idx) {
                    primary.push(candidate);
                }
            }
        }
    }

    primary
}

/// Candidata a fonte para shaping/fallback.
#[derive(Clone, Copy)]
pub(super) struct FontCandidate {
    pub(super) slot_idx: usize,
    pub(super) units_per_em: u16,
}

/// Resolve as fontes declaradas na `FontList` (primárias), na ordem do
/// utilizador. Se nenhuma resolver, `try_shape` cai no fallback `Text`.
pub(super) fn resolve_candidates(
    world: &dyn World,
    font_list: &FontList,
    variant: &FontVariant,
    face_cache: &mut FaceCache,
) -> Option<Vec<FontCandidate>> {
    let book = world.book();
    let mut candidates = Vec::new();
    for family in font_list.as_slice() {
        if let Some(idx) = book.select_pattern(&family.name, &variant) {
            if let Some(cached) = face_cache.get(world, idx) {
                let units_per_em = cached.face().units_per_em().max(1) as u16;
                candidates.push(FontCandidate { slot_idx: idx, units_per_em });
            }
        }
    }
    if candidates.is_empty() {
        None
    } else {
        Some(candidates)
    }
}

/// Conjunto de candidatas primárias + fallback lazy sobre todo o `FontBook`.
///
/// P534: quando as famílias declaradas não cobrem um caractere, procura-se no
/// resto do catálogo na ordem de descoberta. O fallback é lazy para evitar
/// carregar todas as fontes do sistema em documentos que não precisam.
pub(super) struct CandidateSet<'a> {
    world: &'a dyn World,
    face_cache: &'a mut FaceCache,
    primary: Vec<FontCandidate>,
    /// Índices de slot das fontes de fallback. Mantidos como `usize` (não
    /// `FontCandidate`) para evitar carregar/parsear faces apenas durante a
    /// filtragem de candidatos — protótipo P932.
    fallback: Vec<Option<usize>>,
    /// Cache local dos resultados de `World::candidates_for_char` por
    /// caractere. Evita re-allocar o mesmo `Vec<usize>` enquanto se estende
    /// um run de fallback (protótipo P932, variante lazy).
    candidates_cache: HashMap<char, Vec<usize>>,
    /// **P838** — `FontInfo` da primeira primária (o `like` / `ctx.first()`
    /// do vanilla) e a variante activa, para o scoring de similaridade do
    /// fallback global (`FontBook::select_fallback`).
    like: Option<FontInfo>,
    variant: FontVariant,
}

impl<'a> CandidateSet<'a> {
    pub(super) fn new(
        world: &'a dyn World,
        primary: Vec<FontCandidate>,
        face_cache: &'a mut FaceCache,
        like: Option<FontInfo>,
        variant: FontVariant,
    ) -> Self {
        Self {
            world,
            face_cache,
            primary,
            fallback: Vec::new(),
            candidates_cache: HashMap::new(),
            like,
            variant,
        }
    }

    /// Todos os candidatos que cobrem `c`, em ordem de prioridade (primárias
    /// primeiro, depois fallback lazy na ordem do FontBook).
    /// **P875** — o fallback global é filtrado pelo bitmap `Coverage` do
    /// `FontInfo` antes de carregar a face; só se abrem faces cujo bloco de
    /// 256 codepoints cobre `c`.
    pub(super) fn covering_all(&mut self, c: char) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, cand) in self.primary.iter().enumerate() {
            if face_covers_char(self.world, self.face_cache, cand.slot_idx, c) {
                result.push(i);
            }
        }

        // P880 — iterar só os candidatos cujo bitmap cobre o bloco de `c`.
        // SystemWorld calcula coverage lazy; MockWorlds usam o FontBook via default.
        let candidates = self.world.candidates_for_char(c);
        self.candidates_cache.insert(c, candidates.clone());
        for slot_idx in candidates {
            // Primárias já foram tratadas pelo índice interno.
            if let Some(pos) =
                self.primary.iter().position(|cand| cand.slot_idx == slot_idx)
            {
                result.push(pos);
                continue;
            }

            let fallback = self.load_fallback(slot_idx);
            let fb_idx = intern_fallback_slot(&mut self.fallback, fallback);
            if self.fallback[fb_idx].is_some() {
                // Protótipo P932 — confiamos no filtro de `Coverage` já pago
                // por `World::candidates_for_char`; não reabrimos a face aqui.
                result.push(self.primary.len() + fb_idx);
            }
        }
        result
    }

    /// Escolhe o candidato que cobre o maior trecho contíguo de `text`
    /// começando em `start`. As primárias têm prioridade: só se nenhuma
    /// primária cobrir o primeiro caractere é que se recai no fallback global.
    /// Devolve `(candidate_idx, end_byte)`. Se nenhuma fonte cobrir o primeiro
    /// caractere, devolve `None`.
    pub(super) fn covering_run(
        &mut self,
        text: &str,
        start: usize,
    ) -> Option<(usize, usize)> {
        let first_char = text[start..].chars().next()?;

        // 1. Tentar primárias primeiro.
        let mut primary_candidates = Vec::new();
        for (i, cand) in self.primary.iter().enumerate() {
            if face_covers_char(self.world, self.face_cache, cand.slot_idx, first_char) {
                primary_candidates.push(i);
            }
        }
        if let Some(result) = self.best_covering_run(text, start, &primary_candidates) {
            return Some(result);
        }

        // 2. Recair no fallback global. **P838** — o vencedor do scoring de
        // similaridade do vanilla (`FontBook::select_fallback`, com
        // `like` = primeira primária) entra na frente da lista: em empate de
        // comprimento de run (caso CJK típico — todas as candidatas cobrem o
        // run inteiro), vence o scoring em vez da ordem de índice do book.
        // O critério primário continua a ser o run mais longo (P543).
        let mut fallback_candidates = self.covering_all(first_char);
        if let Some(best) = self.world.book().select_fallback(
            self.like.as_ref(),
            &self.variant,
            fallback_candidates.iter().filter_map(|&idx| self.candidate_slot(idx)),
        ) {
            if let Some(pos) = fallback_candidates
                .iter()
                .position(|&idx| self.candidate_slot(idx) == Some(best))
            {
                fallback_candidates.swap(0, pos);
            }
        }
        self.best_covering_run(text, start, &fallback_candidates)
    }

    fn candidate_slot(&self, idx: usize) -> Option<usize> {
        if idx < self.primary.len() {
            self.primary.get(idx).map(|candidate| candidate.slot_idx)
        } else {
            self.fallback.get(idx - self.primary.len()).copied().flatten()
        }
    }

    /// Dado um conjunto de índices de candidatos, escolhe o que cobre o maior
    /// trecho contíguo a partir de `start`.
    fn best_covering_run(
        &mut self,
        text: &str,
        start: usize,
        candidates: &[usize],
    ) -> Option<(usize, usize)> {
        if candidates.is_empty() {
            return None;
        }

        let mut best_idx = candidates[0];
        let mut best_end = start;

        for &idx in candidates {
            let mut end = start;
            for c in text[start..].chars() {
                let covers = if idx < self.primary.len() {
                    face_covers_char(
                        self.world,
                        self.face_cache,
                        self.primary[idx].slot_idx,
                        c,
                    )
                } else {
                    self.fallback
                        .get(idx - self.primary.len())
                        .and_then(|f| *f)
                        .map_or(false, |slot_idx| self.slot_covers_char(slot_idx, c))
                };
                if !covers {
                    break;
                }
                end += c.len_utf8();
            }
            if end > best_end {
                best_end = end;
                best_idx = idx;
            }
        }

        Some((best_idx, best_end))
    }

    fn load_fallback(&mut self, slot_idx: usize) -> Option<usize> {
        // Protótipo P932 — não carrega a face durante a filtragem; só valida
        // que o índice existe no FontBook.
        self.world.book().infos().get(slot_idx).map(|_| slot_idx)
    }

    /// Verifica se `slot_idx` cobre `c` usando o cache de `candidates_for_char`.
    ///
    /// No `SystemWorld`, a primeira consulta de um caractere preenche o cache
    /// lazy de coverage de todas as fontes; as seguintes são apenas scans do
    /// bitmap. No protótipo P932 isto substitui o carregamento de faces para
    /// confirmar cobertura.
    fn slot_covers_char(&mut self, slot_idx: usize, c: char) -> bool {
        let world = self.world;
        self.candidates_cache
            .entry(c)
            .or_insert_with(|| world.candidates_for_char(c))
            .contains(&slot_idx)
    }

    pub(super) fn get(&mut self, idx: usize) -> Option<FontCandidate> {
        if idx < self.primary.len() {
            self.primary.get(idx).copied()
        } else {
            let slot_idx =
                self.fallback.get(idx - self.primary.len()).copied().flatten()?;
            let cached = self.face_cache.get(self.world, slot_idx)?;
            Some(FontCandidate {
                slot_idx,
                units_per_em: cached.face().units_per_em().max(1) as u16,
            })
        }
    }
}

/// Interna um slot global numa posição local densa. O valor numérico do slot
/// não pode determinar o tamanho do vetor (P1404, controle U+2060).
pub(super) fn intern_fallback_slot(
    fallback: &mut Vec<Option<usize>>,
    slot: Option<usize>,
) -> usize {
    if let Some(position) = fallback.iter().position(|candidate| *candidate == slot) {
        position
    } else {
        fallback.push(slot);
        fallback.len() - 1
    }
}

fn face_covers_char(
    world: &dyn World,
    face_cache: &mut FaceCache,
    slot_idx: usize,
    c: char,
) -> bool {
    let Some(cached) = face_cache.get(world, slot_idx) else { return false };
    cached.face().glyph_index(c).is_some()
}

/// Sub-run dentro de um BidiRun: todos os caracteres partilham o mesmo script
/// efectivo e a mesma fonte candidata.
pub(super) struct SubRun {
    pub(super) text: String,
    pub(super) candidate_idx: usize,
}

/// P534/P543 — divide um BidiRun em sub-runs por (a) mudança de script
/// Unicode e (b) mudança de fonte necessária para cobertura do caractere.
///
/// **P543**: em vez de escolher a primeira fonte que cobre o caractere
/// actual, escolhe-se a fonte que cobre o maior trecho contíguo a partir da
/// posição actual. Isto evita fragmentação excessiva (ex.: "Hello" → "H" +
/// "ello") quando o FontBook começa por fontes especializadas de cobertura
/// parcial.
pub(super) fn split_run_by_font(
    run: &BidiRun,
    candidates: &mut CandidateSet,
) -> Vec<SubRun> {
    let text = run.text.as_str();
    if text.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut pos = 0usize;
    let mut current_script = text
        .chars()
        .map(|c| c.script())
        .find(|&script| !is_generic_script(script))
        .unwrap_or(Script::Unknown);
    let mut current: Option<SubRun> = None;

    while pos < text.len() {
        let c = text[pos..].chars().next().unwrap();
        // P1404/P1412 — controles de formato não consultam cobertura global.
        // Conservá-los no sub-run permite ao shaper/ToUnicode preservar o
        // codepoint invisível sem reabrir a alocação causada pelo slot global.
        if is_p1404_format_control(c) {
            if let Some(ref mut cur) = current {
                cur.text.push(c);
            } else {
                current = Some(SubRun { text: c.to_string(), candidate_idx: 0 });
            }
            pos += c.len_utf8();
            continue;
        }
        let script = effective_script(c.script(), current_script);
        let script_changed = pos > 0 && !is_compatible(script, current_script);

        // Próxima fronteira de script dentro do run.
        let script_end = next_script_boundary(text, pos, script);

        // P543 — fonte que cobre o maior trecho contíguo a partir de pos.
        let (candidate_idx, font_end) =
            candidates.covering_run(text, pos).unwrap_or((0, pos + c.len_utf8()));

        let end_byte = font_end.min(script_end);

        if script_changed {
            if let Some(cur) = current.take() {
                result.push(cur);
            }
            current_script = script;
        }

        if let Some(ref mut cur) = current {
            if cur.candidate_idx == candidate_idx {
                // Mesma fonte: estender o sub-run actual.
                cur.text.push_str(&text[pos..end_byte]);
                pos = end_byte;
                continue;
            }
            // Fonte diferente: fechar o actual e iniciar novo.
            result.push(current.take().unwrap());
        }

        current = Some(SubRun {
            text: text[pos..end_byte].to_owned(),
            candidate_idx,
        });
        pos = end_byte;
    }

    if let Some(cur) = current {
        result.push(cur);
    }

    result
}

pub(super) fn is_p1404_format_control(c: char) -> bool {
    matches!(c, '\u{2060}' | '\u{202b}' | '\u{202c}')
}

/// Script efectivo de `c` dado o script do segmento actual: scripts genéricos
/// herdam o script corrente para evitar fragmentação por pontuação/espaço.
fn effective_script(script: Script, current: Script) -> Script {
    if is_generic_script(script) && !is_generic_script(current) {
        current
    } else {
        script
    }
}

/// Byte offset imediatamente após a maior sequência de caracteres a partir de
/// `start` que partilham o mesmo script efectivo `current_script`.
fn next_script_boundary(text: &str, start: usize, current_script: Script) -> usize {
    let mut end = start;
    for (off, c) in text[start..].char_indices() {
        let script = effective_script(c.script(), current_script);
        if off > 0 && !is_compatible(script, current_script) {
            break;
        }
        end = start + off + c.len_utf8();
    }
    end
}

fn is_generic_script(script: Script) -> bool {
    matches!(script, Script::Unknown | Script::Common | Script::Inherited)
}

fn is_compatible(a: Script, b: Script) -> bool {
    is_generic_script(a) || is_generic_script(b) || a == b
}
