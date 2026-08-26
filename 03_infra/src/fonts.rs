//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fonts.md
//! @prompt-hash 4db12b47
//! @layer L3
//! @updated 2026-07-31

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use typst_core::entities::font_book::{
    Coverage, FontBook, FontFlags, FontInfo, FontStretch, FontStyle, FontVariant,
    FontWeight,
};
use typst_core::entities::world_types::Font;

/// Slot de fonte com carregamento lazy.
///
/// A fonte pode vir do disco (`path`) ou de bytes embutidos (`embedded`).
/// `ttf-parser` valida que os bytes são uma fonte OpenType/TrueType
/// válida antes de retornar `Some(Font)` — bytes inválidos retornam `None`.
/// `ttf-parser` não escapa a esta fronteira: L1 recebe apenas `Font` opaco.
pub struct FontSlot {
    pub path: PathBuf,
    /// Índice da face num TrueType Collection (.ttc). Sempre 0 para fontes simples.
    /// Ignorado quando `embedded` está presente.
    pub index: u32,
    /// Bytes embutidos (ex: vinda de `typst-assets`). Quando presentes,
    /// `get()` usa estes bytes em vez de ler do disco.
    embedded: Option<Vec<u8>>,
    /// **P937** — mmap lazy do ficheiro de fonte. Criado em L3 na primeira vez
    /// que os bytes são necessários; partilhado entre `source_bytes()` e `get()`.
    mmap: OnceLock<Option<Arc<memmap2::Mmap>>>,
    font: OnceLock<Option<Font>>,
}

/// DTO L3 para listagem pública de fontes. Não expõe tipos das bibliotecas
/// externas usadas na descoberta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontInventoryEntry {
    pub family: String,
    pub path: PathBuf,
    pub index: u32,
    pub style: String,
    pub weight: u16,
    pub stretch: u16,
    pub embedded: bool,
}

/// Constrói o inventário usado por `typst fonts`, com a mesma composição de
/// fontes embutidas, sistema e paths explícitos usada pelo World.
pub fn inventory_fonts(
    font_paths: &[PathBuf],
    include_system: bool,
) -> Vec<FontInventoryEntry> {
    fn extend(
        out: &mut Vec<FontInventoryEntry>,
        slots: Vec<FontSlot>,
        book: FontBook,
        embedded: bool,
    ) {
        for (slot, info) in slots.into_iter().zip(book.infos()) {
            out.push(FontInventoryEntry {
                family: info.family.clone(),
                path: slot.path,
                index: slot.index,
                style: format!("{:?}", info.variant.style).to_ascii_lowercase(),
                weight: info.variant.weight.0,
                stretch: info.variant.stretch.0,
                embedded,
            });
        }
    }

    let mut out = Vec::new();
    let embedded = crate::embedded_fonts::load_embedded_fonts();
    extend(&mut out, embedded.text_slots, embedded.text_book, true);
    extend(&mut out, embedded.math_code_slots, embedded.math_code_book, true);
    if include_system {
        let (slots, book) = crate::fontdb::load_system_fonts();
        extend(&mut out, slots, book, false);
    }
    let (slots, book) = pair_slots_with_book(discover_fonts(font_paths));
    extend(&mut out, slots, book, false);

    out.sort_by(|a, b| {
        a.family
            .to_lowercase()
            .cmp(&b.family.to_lowercase())
            .then_with(|| a.style.cmp(&b.style))
            .then_with(|| a.weight.cmp(&b.weight))
            .then_with(|| a.stretch.cmp(&b.stretch))
            .then_with(|| a.path.cmp(&b.path))
            .then_with(|| a.index.cmp(&b.index))
    });
    out.dedup_by(|a, b| a == b);
    out
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self {
        Self {
            path,
            index,
            embedded: None,
            mmap: OnceLock::new(),
            font: OnceLock::new(),
        }
    }

    /// Cria um slot a partir de bytes embutidos (P753).
    /// O path é mantido apenas para referência/depuração; não é lido.
    pub fn new_embedded(path: PathBuf, data: Vec<u8>) -> Self {
        Self {
            path,
            index: 0,
            embedded: Some(data),
            mmap: OnceLock::new(),
            font: OnceLock::new(),
        }
    }

    /// Garante que o mmap do ficheiro está criado.
    fn mmap(&self) -> Option<&Arc<memmap2::Mmap>> {
        self.mmap
            .get_or_init(|| {
                let file = std::fs::File::open(&self.path).ok()?;
                unsafe { memmap2::Mmap::map(&file) }.map(Arc::new).ok()
            })
            .as_ref()
    }

    /// Bytes fonte originais (sem extrair face de coleção).
    /// Preferência: embutidos → mmap do disco.
    ///
    /// P937/P938 — devolve `Cow::Borrowed` sobre o mmap para evitar cópias
    /// durante a construção do FontBook e a extração lazy de coverage.
    pub(crate) fn source_bytes(&self) -> Option<Cow<'_, [u8]>> {
        if let Some(bytes) = &self.embedded {
            return Some(Cow::Borrowed(bytes));
        }
        self.mmap().map(|mmap| Cow::Borrowed(&mmap[..]))
    }

    /// Carrega e valida a fonte (apenas na primeira chamada).
    /// Retorna `None` se os bytes não forem uma fonte OpenType/TrueType válida.
    ///
    /// **P609** — se o ficheiro for uma TrueType/OpenType Collection (.ttc/.otc),
    /// extrai a face correspondente a `self.index` para bytes independentes antes
    /// de expor. Isto permite que o export/subsetter receba uma fonte simples
    /// válida para `Face::parse(data, 0)` e `oxifont_subset`, em vez de falhar
    /// silenciosamente e embutir a coleção inteira.
    pub fn get(&self) -> Option<Font> {
        self.font
            .get_or_init(|| {
                if let Some(bytes) = &self.embedded {
                    let data = bytes.clone();
                    let data = extract_collection_face(&data, self.index).unwrap_or(data);
                    ttf_parser::Face::parse(&data, 0).ok()?;
                    return Some(Font::from_data(data));
                }

                let mmap = self.mmap()?;
                let data = mmap.as_ref();
                // P609: extrair face de uma coleção, se aplicável.
                let font =
                    if let Some(extracted) = extract_collection_face(data, self.index) {
                        ttf_parser::Face::parse(&extracted, 0).ok()?;
                        Font::from_data(extracted)
                    } else {
                        ttf_parser::Face::parse(data, self.index).ok()?;
                        Font::from_mmap(Arc::clone(mmap))
                    };
                Some(font)
            })
            .clone()
    }
}

/// Extrai a face de índice `index` de uma TrueType/OpenType Collection.
///
/// Devolve `Some(fonte_simples)` se `data` for uma coleção e o índice for
/// válido. Devolve `None` se não for uma coleção (incluindo quando só há
/// uma única face), mantendo o comportamento anterior para fontes normais.
///
/// Formato do cabeçalho `ttcf`:
/// - tag 'ttcf' (4 bytes)
/// - version (4 bytes)
/// - numFonts (4 bytes)
/// - offsetTableOffsets[ numFonts ] (4 bytes cada)
///
/// **P838** — a extracção anterior (P609) fazia slice dos bytes da face
/// (`data[start..end]`), mas os offsets do directório de tabelas de uma face
/// em TTC são **absolutos ao início da colecção** — o slice ficava com todos
/// os offsets fora de alcance e `Face::parse` falhava (medido: todas as
/// faces de `NotoSansCJK-*.ttc` ficavam incarregáveis; o fallback CJK caía
/// sempre em `Droid Sans Fallback`, um `.ttf` simples). A extracção correcta
/// **reconstrói o ficheiro**: cabeçalho sfnt da face + directório com
/// offsets recalculados + bytes das tabelas (copiados dos offsets absolutos
/// da colecção), com `checkSumAdjustment` do `head` recalculado no fim.
fn extract_collection_face(data: &[u8], index: u32) -> Option<Vec<u8>> {
    let count = ttf_parser::fonts_in_collection(data)?;
    if count <= 1 || index >= count {
        return None;
    }
    if data.len() < 12 || &data[..4] != b"ttcf" {
        return None;
    }
    let face_off = read_u32(data, 12 + 4 * index as usize)? as usize;

    // Cabeçalho sfnt da face (12 bytes) + directório de tabelas.
    let header = data.get(face_off..face_off.checked_add(12)?)?;
    let num_tables = u16::from_be_bytes([header[4], header[5]]) as usize;
    let dir_len = 16usize.checked_mul(num_tables)?;
    let dir = data.get(face_off + 12..face_off + 12 + dir_len)?;

    // Novo ficheiro: cabeçalho (12) + directório (16*n) + tabelas alinhadas a 4.
    let mut out = Vec::with_capacity(12 + dir_len);
    out.extend_from_slice(header);
    out.extend_from_slice(dir);

    let mut head_pos_in_out = None;
    for i in 0..num_tables {
        let rec = i * 16;
        let tag = &dir[rec..rec + 4];
        let abs_off = read_u32(dir, rec + 8)? as usize;
        let len = read_u32(dir, rec + 12)? as usize;
        let table = data.get(abs_off..abs_off.checked_add(len)?)?;

        let new_off = out.len();
        out.extend_from_slice(table);
        // Padding a 4 bytes (conta no offset seguinte, não no length).
        while out.len() % 4 != 0 {
            out.push(0);
        }
        let rec_out = 12 + rec;
        out[rec_out + 8..rec_out + 12].copy_from_slice(&(new_off as u32).to_be_bytes());
        if tag == b"head" {
            head_pos_in_out = Some(new_off);
        }
    }

    // Recalcular o checkSumAdjustment do `head`: zero no campo, soma do
    // ficheiro em u32 BE, ajuste = 0xB1B0AFBA - soma.
    let head = head_pos_in_out?;
    out[head + 8..head + 12].copy_from_slice(&0u32.to_be_bytes());
    let mut sum: u32 = 0;
    for chunk in out.chunks(4) {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum = sum.wrapping_add(u32::from_be_bytes(word));
    }
    let adj = 0xB1B0AFBAu32.wrapping_sub(sum);
    out[head + 8..head + 12].copy_from_slice(&adj.to_be_bytes());

    Some(out)
}

/// Lê um u32 big-endian de `data` em `off`, com verificação de limites.
fn read_u32(data: &[u8], off: usize) -> Option<u32> {
    let b = data.get(off..off.checked_add(4)?)?;
    Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// Descobre fontes nos paths fornecidos.
///
/// Cada path pode ser um ficheiro de fonte directamente ou um
/// directório (varrido recursivamente). Fontes TrueType Collection
/// (`.ttc`) produzem múltiplos slots — um por face.
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot> {
    let mut slots = Vec::new();
    for path in font_paths {
        if path.is_dir() {
            discover_in_dir(path, &mut slots);
        } else if is_font_file(path) {
            push_slots(path, &mut slots);
        }
    }
    slots
}

fn is_font_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("ttf" | "otf" | "ttc" | "otc")
    )
}

fn face_count(path: &Path) -> u32 {
    std::fs::read(path)
        .ok()
        .and_then(|data| ttf_parser::fonts_in_collection(&data))
        .unwrap_or(1)
}

fn push_slots(path: &Path, slots: &mut Vec<FontSlot>) {
    let count = face_count(path);
    for index in 0..count {
        slots.push(FontSlot::new(path.to_path_buf(), index));
    }
}

fn discover_in_dir(dir: &Path, slots: &mut Vec<FontSlot>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            discover_in_dir(&path, slots);
        } else if is_font_file(&path) {
            push_slots(&path, slots);
        }
    }
}

/// Extrai `FontInfo` de bytes de fonte OpenType/TrueType (ADR-0022).
///
/// `ttf_parser` fica em L3 — L1 recebe apenas `FontInfo` com campos primitivos.
/// Retorna `None` se os bytes não forem uma fonte válida ou o índice não existir.
///
/// **P838** — `flags.serif` é detectado via panose (OS/2 bytes 32..45) com o
/// critério idêntico ao vanilla (`typst-library/src/text/font/info.rs:131-138`):
/// `matches!(panose, [2, 2..=10, ..])`. Necessário ao scoring de similaridade
/// de `FontBook::select_fallback`.
///
/// **P839** — três correcções de paridade com o vanilla
/// (`typst-library/src/text/font/info.rs`), achados #25–#27 de P831:
/// - a família vem **só** do name ID1 (`FAMILY`) com aparo iterativo de
///   sufixos de estilo (`typographic_family`) — o ID16 é ignorado de
///   propósito pelo vanilla (`info.rs:62-77`);
/// - nomes em registos Macintosh (1,0,0) são decodificados via
///   `decode_mac_roman` (`info.rs:168-203`);
/// - o estilo italic/oblique é inferido também do full name
///   (`info.rs:80-103`), sem usar `is_italic()` (falsos positivos via
/// **P840** — tabela de exceções de metadados (`find_exception`, port
/// integral do vanilla `text/font/exceptions.rs:46-342`) aplicada por
/// PostScript name (`info.rs:60-61`): família/estilo/peso/stretch da
/// exceção prevalecem sobre a extração normal (achados #29/#30 de P831).
pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo> {
    let face = ttf_parser::Face::parse(data, index).ok()?;

    // P840 (#29/#30) — exceções de metadados por PostScript name (port do
    // vanilla `info.rs:60-61`): cada campo presente na exceção prevalece
    // sobre a extração normal abaixo.
    let ps_name = find_name(&face, ttf_parser::name_id::POST_SCRIPT_NAME);
    let exception = ps_name.as_deref().and_then(find_exception);

    // P839 (#25) — o vanilla não usa o ID16 (TYPOGRAPHIC_FAMILY): para
    // algumas fontes ele agrupa mais do que variantes de estilo/peso/largura
    // (ex.: variantes Display dos Noto) e essas variantes ficariam
    // inacessíveis. Usa o ID1 (FAMILY) com aparo de sufixos de estilo.
    let family = exception
        .and_then(|e| e.family.map(str::to_string))
        .or_else(|| {
            find_name(&face, ttf_parser::name_id::FAMILY)
                .map(|family| typographic_family(&family).to_string())
        })
        // Fallback sem equivalente vanilla (que descarta a fonte): primeiro
        // nome decodificável, para fontes sem ID1. Divergência residual
        // registada em P839.
        .or_else(|| face.names().into_iter().filter_map(|n| n.to_string()).next())?;

    // P839 (#27) — algumas fontes não têm os bits de italic/oblique; o
    // vanilla infere também do full name (minúsculas).
    let style = exception.and_then(|e| e.style).unwrap_or_else(|| {
        let full = find_name(&face, ttf_parser::name_id::FULL_NAME)
            .unwrap_or_default()
            .to_ascii_lowercase();
        infer_style(face.style() == ttf_parser::Style::Italic, face.is_oblique(), &full)
    });

    let weight = exception
        .and_then(|e| e.weight)
        .map(FontWeight)
        .unwrap_or_else(|| FontWeight(face.weight().to_number()));
    let stretch = exception
        .and_then(|e| e.stretch)
        .map(FontStretch)
        .unwrap_or_else(|| FontStretch::from_number(face.width().to_number()));

    // P838 — serif via panose (OS/2 bytes 32..45), critério do vanilla:
    // família de texto latino (2) com estilo serifado (2..=10).
    let serif = face
        .raw_face()
        .table(ttf_parser::Tag::from_bytes(b"OS/2"))
        .and_then(|os2| os2.get(32..45))
        .is_some_and(|panose| matches!(panose, [2, 2..=10, ..]));

    // P938 — coverage é computada lazy por SystemWorld::candidates_for_char
    // (cache por índice). Deixar vazio aqui evita iterar a cmap de todas as
    // fontes no arranque de documentos que não precisam de fallback.
    let coverage = Coverage::new();

    Some(FontInfo {
        family,
        variant: FontVariant { style, weight, stretch },
        flags: FontFlags { monospace: face.is_monospaced(), serif },
        coverage,
    })
}

/// **P937/P938** — extrai cobertura Unicode exacta da tabela `cmap`.
/// Percorre as subtables unicode e constrói `Coverage::from_codepoints`.
pub(crate) fn extract_coverage(face: &ttf_parser::Face) -> Coverage {
    let mut codepoints = Vec::new();
    let Some(cmap) = face.tables().cmap else { return Coverage::new() };
    for subtable in cmap.subtables {
        if subtable.is_unicode() {
            subtable.codepoints(&mut |codepoint| {
                codepoints.push(codepoint);
            });
        }
    }
    Coverage::from_codepoints(codepoints)
}

/// Procura e decodifica o nome com o id dado (port do vanilla
/// `info.rs:168-182`, P839 #26).
///
/// `ttf_parser` 0.25 não decodifica registos Macintosh em
/// `Name::to_string()`; para registos Macintosh (plataforma 1, encoding 0 =
/// mac roman) a decodificação é feita por `decode_mac_roman`.
fn find_name(face: &ttf_parser::Face, name_id: u16) -> Option<String> {
    face.names().into_iter().find_map(|entry| {
        if entry.name_id == name_id {
            if let Some(string) = entry.to_string() {
                return Some(string);
            }

            if entry.platform_id == ttf_parser::PlatformId::Macintosh
                && entry.encoding_id == 0
            {
                return Some(decode_mac_roman(entry.name));
            }
        }

        None
    })
}

/// Decodifica bytes mac roman para string (port verbatim da tabela do
/// vanilla `info.rs:185-203`, P839 #26).
fn decode_mac_roman(coded: &[u8]) -> String {
    #[rustfmt::skip]
    const TABLE: [char; 128] = [
        'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å', 'ç', 'é', 'è',
        'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô', 'ö', 'õ', 'ú', 'ù', 'û', 'ü',
        '†', '°', '¢', '£', '§', '•', '¶', 'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø',
        '∞', '±', '≤', '≥', '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø',
        '¿', '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', '\u{a0}', 'À', 'Ã', 'Õ', 'Œ', 'œ',
        '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄', '€', '‹', '›', 'ﬁ', 'ﬂ',
        '‡', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á', 'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô',
        '\u{f8ff}', 'Ò', 'Ú', 'Û', 'Ù', 'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ',
    ];

    fn char_from_mac_roman(code: u8) -> char {
        if code < 128 {
            code as char
        } else {
            TABLE[(code - 128) as usize]
        }
    }

    coded.iter().copied().map(char_from_mac_roman).collect()
}

/// Exceção de metadados de fonte, indexada por PostScript name
/// (port do vanilla `text/font/exceptions.rs:9-15`, P840).
///
/// Cada campo presente **prevalece** sobre a extração normal em
/// `font_info_from_bytes`, como no vanilla (`info.rs:73-77,80-112`).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct FontException {
    family: Option<&'static str>,
    style: Option<FontStyle>,
    weight: Option<u16>,
    stretch: Option<u16>,
}

impl FontException {
    const fn new() -> Self {
        Self {
            family: None,
            style: None,
            weight: None,
            stretch: None,
        }
    }

    const fn family(self, family: &'static str) -> Self {
        Self { family: Some(family), ..self }
    }

    const fn style(self, style: FontStyle) -> Self {
        Self { style: Some(style), ..self }
    }

    const fn weight(self, weight: u16) -> Self {
        Self { weight: Some(weight), ..self }
    }

    const fn stretch(self, stretch: u16) -> Self {
        Self { stretch: Some(stretch), ..self }
    }
}

/// Procura a exceção de metadados para um PostScript name (name ID6).
///
/// **P840** — port integral da tabela do vanilla
/// (`text/font/exceptions.rs:46-342`, comentários incluídos): todas as
/// entradas, sem scope-out. O vanilla usa um `phf::Map`; aqui um `match`
/// (mesma semântica de lookup exato por string, sem dependência nova).
fn find_exception(postscript_name: &str) -> Option<FontException> {
    let exception = match postscript_name {
        // The old version of Arial-Black, published by Microsoft in 1996 in their
        // "core fonts for the web" project, has a wrong weight of 400.
        // See https://corefonts.sourceforge.net/.
        "Arial-Black" => FontException::new().weight(900),
        // Archivo Narrow is different from Archivo and Archivo Black. Since Archivo Black
        // seems identical to Archivo weight 900, only differentiate between Archivo and
        // Archivo Narrow.
        "ArchivoNarrow-Regular" => FontException::new().family("Archivo Narrow"),
        "ArchivoNarrow-Italic" => FontException::new().family("Archivo Narrow"),
        "ArchivoNarrow-Bold" => FontException::new().family("Archivo Narrow"),
        "ArchivoNarrow-BoldItalic" => FontException::new().family("Archivo Narrow"),
        // Fandol fonts designed for Chinese typesetting.
        // See https://ctan.org/tex-archive/fonts/fandol/.
        "FandolHei-Bold" => FontException::new().weight(700),
        "FandolSong-Bold" => FontException::new().weight(700),
        // Noto fonts
        "NotoNaskhArabicUISemi-Bold" => {
            FontException::new().family("Noto Naskh Arabic UI").weight(600)
        }
        "NotoSansSoraSompengSemi-Bold" => {
            FontException::new().family("Noto Sans Sora Sompeng").weight(600)
        }
        "NotoSans-DisplayBlackItalic" => FontException::new().family("Noto Sans Display"),
        "NotoSans-DisplayCondensedBlackItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedBold" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedExtraBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedExtraLightItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedLightItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedMediumItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedSemiBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayCondensedThinItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedBlackItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedBold" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedExtraBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedExtraLightItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedLightItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedMediumItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedSemiBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraCondensedThinItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayExtraLightItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayLightItalic" => FontException::new().family("Noto Sans Display"),
        "NotoSans-DisplayMediumItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedBlackItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedBold" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedExtraBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedExtraLightItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedLightItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedMediumItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedSemiBoldItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplaySemiCondensedThinItalic" => {
            FontException::new().family("Noto Sans Display")
        }
        "NotoSans-DisplayThinItalic" => FontException::new().family("Noto Sans Display"),
        // The following three postscript names are only used in the version 2.007
        // of the Noto Sans font. Other versions, while have different postscript
        // name, happen to have correct metadata.
        "NotoSerif-DisplayCondensedBold" => {
            FontException::new().family("Noto Serif Display")
        }
        "NotoSerif-DisplayExtraCondensedBold" => {
            FontException::new().family("Noto Serif Display")
        }
        "NotoSerif-DisplaySemiCondensedBold" => {
            FontException::new().family("Noto Serif Display")
        }
        // New Computer Modern
        "NewCM08-Book" => {
            FontException::new().family("New Computer Modern 08").weight(450)
        }
        "NewCM08-BookItalic" => {
            FontException::new().family("New Computer Modern 08").weight(450)
        }
        "NewCM08-Italic" => FontException::new().family("New Computer Modern 08"),
        "NewCM08-Regular" => FontException::new().family("New Computer Modern 08"),
        "NewCM10-Bold" => FontException::new().family("New Computer Modern"),
        "NewCM10-BoldItalic" => FontException::new().family("New Computer Modern"),
        "NewCM10-Book" => FontException::new().family("New Computer Modern").weight(450),
        "NewCM10-BookItalic" => {
            FontException::new().family("New Computer Modern").weight(450)
        }
        "NewCM10-Italic" => FontException::new().family("New Computer Modern"),
        "NewCM10-Regular" => FontException::new().family("New Computer Modern"),
        "NewCMMath-Bold" => FontException::new().family("New Computer Modern Math"),
        "NewCMMath-Book" => {
            FontException::new().family("New Computer Modern Math").weight(450)
        }
        "NewCMMath-Regular" => FontException::new().family("New Computer Modern Math"),
        "NewCMMono10-Bold" => FontException::new().family("New Computer Modern Mono"),
        "NewCMMono10-BoldOblique" => {
            FontException::new().family("New Computer Modern Mono")
        }
        "NewCMMono10-Book" => {
            FontException::new().family("New Computer Modern Mono").weight(450)
        }
        "NewCMMono10-BookItalic" => {
            FontException::new().family("New Computer Modern Mono").weight(450)
        }
        "NewCMMono10-Italic" => FontException::new().family("New Computer Modern Mono"),
        "NewCMMono10-Regular" => FontException::new().family("New Computer Modern Mono"),
        "NewCMSans08-Book" => {
            FontException::new().family("New Computer Modern Sans 08").weight(450)
        }
        "NewCMSans08-BookOblique" => {
            FontException::new().family("New Computer Modern Sans 08").weight(450)
        }
        "NewCMSans08-Oblique" => {
            FontException::new().family("New Computer Modern Sans 08")
        }
        "NewCMSans08-Regular" => {
            FontException::new().family("New Computer Modern Sans 08")
        }
        "NewCMSans10-Bold" => FontException::new().family("New Computer Modern Sans"),
        "NewCMSans10-BoldOblique" => {
            FontException::new().family("New Computer Modern Sans")
        }
        "NewCMSans10-Book" => {
            FontException::new().family("New Computer Modern Sans").weight(450)
        }
        "NewCMSans10-BookOblique" => FontException::new()
            .family("New Computer Modern Sans")
            .weight(450)
            .style(FontStyle::Oblique),
        "NewCMSans10-Oblique" => FontException::new()
            .family("New Computer Modern Sans")
            .style(FontStyle::Oblique),
        "NewCMSans10-Regular" => FontException::new().family("New Computer Modern Sans"),
        "NewCMSansMath-Regular" => {
            FontException::new().family("New Computer Modern Sans Math")
        }
        "NewCMUncial08-Bold" => {
            FontException::new().family("New Computer Modern Uncial 08")
        }
        "NewCMUncial08-Book" => FontException::new()
            .family("New Computer Modern Uncial 08")
            .weight(450),
        "NewCMUncial08-Regular" => {
            FontException::new().family("New Computer Modern Uncial 08")
        }
        "NewCMUncial10-Bold" => FontException::new().family("New Computer Modern Uncial"),
        "NewCMUncial10-Book" => {
            FontException::new().family("New Computer Modern Uncial").weight(450)
        }
        "NewCMUncial10-Regular" => {
            FontException::new().family("New Computer Modern Uncial")
        }
        // Latin Modern
        "LMMono8-Regular" => FontException::new().family("Latin Modern Mono 8"),
        "LMMono9-Regular" => FontException::new().family("Latin Modern Mono 9"),
        "LMMono12-Regular" => FontException::new().family("Latin Modern Mono 12"),
        "LMMonoLt10-BoldOblique" => FontException::new().style(FontStyle::Oblique),
        "LMMonoLt10-Regular" => FontException::new().weight(300),
        "LMMonoLt10-Oblique" => {
            FontException::new().weight(300).style(FontStyle::Oblique)
        }
        "LMMonoLtCond10-Regular" => FontException::new().weight(300).stretch(666),
        "LMMonoLtCond10-Oblique" => FontException::new()
            .weight(300)
            .style(FontStyle::Oblique)
            .stretch(666),
        "LMMonoPropLt10-Regular" => FontException::new().weight(300),
        "LMMonoPropLt10-Oblique" => FontException::new().weight(300),
        "LMRoman5-Regular" => FontException::new().family("Latin Modern Roman 5"),
        "LMRoman6-Regular" => FontException::new().family("Latin Modern Roman 6"),
        "LMRoman7-Regular" => FontException::new().family("Latin Modern Roman 7"),
        "LMRoman8-Regular" => FontException::new().family("Latin Modern Roman 8"),
        "LMRoman9-Regular" => FontException::new().family("Latin Modern Roman 9"),
        "LMRoman12-Regular" => FontException::new().family("Latin Modern Roman 12"),
        "LMRoman17-Regular" => FontException::new().family("Latin Modern Roman 17"),
        "LMRoman7-Italic" => FontException::new().family("Latin Modern Roman 7"),
        "LMRoman8-Italic" => FontException::new().family("Latin Modern Roman 8"),
        "LMRoman9-Italic" => FontException::new().family("Latin Modern Roman 9"),
        "LMRoman12-Italic" => FontException::new().family("Latin Modern Roman 12"),
        "LMRoman5-Bold" => FontException::new().family("Latin Modern Roman 5"),
        "LMRoman6-Bold" => FontException::new().family("Latin Modern Roman 6"),
        "LMRoman7-Bold" => FontException::new().family("Latin Modern Roman 7"),
        "LMRoman8-Bold" => FontException::new().family("Latin Modern Roman 8"),
        "LMRoman9-Bold" => FontException::new().family("Latin Modern Roman 9"),
        "LMRoman12-Bold" => FontException::new().family("Latin Modern Roman 12"),
        "LMRomanSlant8-Regular" => FontException::new().family("Latin Modern Roman 8"),
        "LMRomanSlant9-Regular" => FontException::new().family("Latin Modern Roman 9"),
        "LMRomanSlant12-Regular" => FontException::new().family("Latin Modern Roman 12"),
        "LMRomanSlant17-Regular" => FontException::new().family("Latin Modern Roman 17"),
        "LMSans8-Regular" => FontException::new().family("Latin Modern Sans 8"),
        "LMSans9-Regular" => FontException::new().family("Latin Modern Sans 9"),
        "LMSans12-Regular" => FontException::new().family("Latin Modern Sans 12"),
        "LMSans17-Regular" => FontException::new().family("Latin Modern Sans 17"),
        "LMSans8-Oblique" => FontException::new().family("Latin Modern Sans 8"),
        "LMSans9-Oblique" => FontException::new().family("Latin Modern Sans 9"),
        "LMSans12-Oblique" => FontException::new().family("Latin Modern Sans 12"),
        "LMSans17-Oblique" => FontException::new().family("Latin Modern Sans 17"),
        // SimSun-ExtB is a CJK Extension B font, not an "ExtraBold" variant of
        // SimSun. Without this exception, `typographic_family()` strips the "ExtB"
        // suffix and merges it with SimSun, causing wrong font selection.
        "SimSun-ExtB" => FontException::new().family("SimSun-ExtB"),
        // STKaiti is a set of Kai fonts. Their weight values need to be corrected
        // according to their PostScript names.
        "STKaitiSC-Regular" => FontException::new().weight(400),
        "STKaitiTC-Regular" => FontException::new().weight(400),
        "STKaitiSC-Bold" => FontException::new().weight(700),
        "STKaitiTC-Bold" => FontException::new().weight(700),
        "STKaitiSC-Black" => FontException::new().weight(900),
        "STKaitiTC-Black" => FontException::new().weight(900),
        _ => return None,
    };
    Some(exception)
}

/// Apara sufixos de estilo de um nome de família e corrige nomes maus
/// (port verbatim do vanilla `info.rs:206-267`, P839 #25).
fn typographic_family(mut family: &str) -> &str {
    // Separadores entre nomes, modificadores e estilos.
    const SEPARATORS: [char; 3] = [' ', '-', '_'];

    // Modificadores que podem aparecer em combinação com sufixos.
    const MODIFIERS: &[&str] =
        &["extra", "ext", "ex", "x", "semi", "sem", "sm", "demi", "dem", "ultra"];

    // Sufixos de estilo.
    #[rustfmt::skip]
    const SUFFIXES: &[&str] = &[
        "normal", "italic", "oblique", "slanted",
        "thin", "th", "hairline", "light", "lt", "regular", "medium", "med",
        "md", "bold", "bd", "demi", "extb", "black", "blk", "bk", "heavy",
        "narrow", "condensed", "cond", "cn", "cd", "compressed", "expanded", "exp",
        "vf", "var", "variable",
    ];

    // Aparar espaços e pontos iniciais estranhos de fontes Apple.
    family = family.trim().trim_start_matches('.');

    // Minúsculas para o aparo ser case-insensitivo.
    let lower = family.to_ascii_lowercase();
    let mut len = usize::MAX;
    let mut trimmed = lower.as_str();

    // Aparar sufixos de estilo repetidamente.
    while trimmed.len() < len {
        len = trimmed.len();

        // Encontrar sufixo de estilo.
        let mut t = trimmed;
        let mut shortened = false;
        while let Some(s) = SUFFIXES.iter().find_map(|s| t.strip_suffix(s)) {
            shortened = true;
            t = s;
        }

        if !shortened {
            break;
        }

        // Aparar separador opcional.
        if let Some(s) = t.strip_suffix(SEPARATORS) {
            trimmed = s;
            t = s;
        }

        // Permitir um modificador extra, mas só se estiver separado do
        // texto anterior (para evitar falsos positivos).
        // (sem let-chains: este crate é edition 2021)
        if let Some(t) = MODIFIERS.iter().find_map(|s| t.strip_suffix(s)) {
            if let Some(stripped) = t.strip_suffix(SEPARATORS) {
                trimmed = stripped;
            }
        }
    }

    // Aplicar o aparo (o lowercase ASCII preserva o comprimento em bytes).
    family = &family[..len];

    family
}

/// Infere o `FontStyle` a partir dos bits da fonte e do full name em
/// minúsculas (port do vanilla `info.rs:80-103`, P839 #27).
///
/// `ttf_italic` deve ser `face.style() == ttf_parser::Style::Italic` — o
/// vanilla evita `is_italic()` porque também consulta o ângulo itálico, o
/// que dá falsos positivos em fontes oblique (typst/typst#7479).
fn infer_style(ttf_italic: bool, ttf_oblique: bool, full_lower: &str) -> FontStyle {
    let italic = ttf_italic || full_lower.contains("italic");
    let oblique =
        ttf_oblique || full_lower.contains("oblique") || full_lower.contains("slanted");

    match (italic, oblique) {
        (false, false) => FontStyle::Normal,
        (true, _) => FontStyle::Italic,
        (_, true) => FontStyle::Oblique,
    }
}

/// Emparelha slots de fonte com entradas do `FontBook` (P839, achado #28/I4
/// de P831; P937 — coverage exacta eager via mmap).
///
/// Lê os bytes de cada slot e extrai `FontInfo`. **Slots cuja extracção
/// falha são descartados** — cada entrada do book corresponde ao slot de
/// mesmo índice, como no vanilla (`typst-kit/src/fonts.rs:172-189`: o
/// `filter_map` só produz o par `(source, info)` quando a info é extraída,
/// e `FontStore::push` insere os dois juntos). Antes deste fix, o slot era
/// criado incondicionalmente e o push no book era condicional, desalinhando
/// os índices (o shaper indexa `font_slots` pelo índice do book).
///
/// Para slots embutidos, usa os bytes em memória. Para fontes do disco,
/// usa o mesmo mmap que `FontSlot::get()` — sem duplicação de I/O.
pub fn pair_slots_with_book(slots: Vec<FontSlot>) -> (Vec<FontSlot>, FontBook) {
    let mut kept = Vec::new();
    let mut book = FontBook::new();
    for slot in slots {
        let info = slot
            .source_bytes()
            .and_then(|data| font_info_from_bytes(&data, slot.index));
        if let Some(info) = info {
            book.push(info);
            kept.push(slot);
        }
    }
    (kept, book)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TempDir(PathBuf);
    impl TempDir {
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn tempdir() -> TempDir {
        let path = std::env::temp_dir().join(format!(
            "typst-fonts-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    #[test]
    fn font_slot_path_inexistente_retorna_none() {
        let slot = FontSlot::new(PathBuf::from("/nao/existe.ttf"), 0);
        assert!(slot.get().is_none());
    }

    #[test]
    fn font_slot_bytes_invalidos_retorna_none() {
        let dir = tempdir();
        let path = dir.path().join("invalid.ttf");
        std::fs::write(&path, b"not a font").unwrap();
        let slot = FontSlot::new(path, 0);
        assert!(slot.get().is_none());
    }

    #[test]
    fn discover_fonts_directorio_vazio() {
        let dir = tempdir();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        assert!(slots.is_empty());
    }

    #[test]
    fn discover_fonts_ignora_ficheiros_nao_fonte() {
        let dir = tempdir();
        std::fs::write(dir.path().join("readme.txt"), b"text").unwrap();
        std::fs::write(dir.path().join("data.bin"), b"binary").unwrap();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        assert!(slots.is_empty());
    }

    #[test]
    fn discover_fonts_cria_slot_para_ttf() {
        let dir = tempdir();
        // Ficheiro com extensão .ttf (conteúdo inválido — slot criado, get() retorna None)
        std::fs::write(dir.path().join("fake.ttf"), b"not a font").unwrap();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        // Slot é criado mesmo com bytes inválidos — validação acontece em get()
        assert_eq!(slots.len(), 1);
        assert!(slots[0].get().is_none());
    }

    #[test]
    fn font_slot_get_chamada_repetida_consistente() {
        let slot = FontSlot::new(PathBuf::from("/nao/existe.ttf"), 0);
        // OnceLock garante que o resultado é sempre o mesmo
        assert_eq!(slot.get(), slot.get());
    }

    #[test]
    fn font_info_bytes_invalidos() {
        assert!(font_info_from_bytes(b"not a font", 0).is_none());
    }

    #[test]
    fn pair_slots_with_book_slots_invalidos_descartados() {
        let dir = tempdir();
        std::fs::write(dir.path().join("fake.ttf"), b"not a font").unwrap();
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        let (slots, book) = pair_slots_with_book(slots);
        // Bytes inválidos → sem entradas no FontBook nem slots
        assert!(book.is_empty());
        assert!(slots.is_empty());
    }

    // ── P839 — achados #25–#28 de P831 (resolução de nome/estilo de fonte) ──
    //
    // Fixtures sintéticas em `fixtures/fonts/p839-*.ttf`, geradas por
    // fontTools 4.63.0 em P831 (`temp/p831/fonts/`) e copiadas para fixtures;
    // `p839-noname.ttf` derivada por `temp/p839/make_noname.py`.
    //
    // - `p839-triagx-bold.ttf`    — ID1 `TriagX Bold` (sem ID16), peso 700.
    // - `p839-triagdsp.ttf`       — ID16 `TriagDsp` + ID1 `TriagDsp Display Bold`.
    // - `p839-triagmac.ttf`       — tabela name só com registos Macintosh (1,0,0);
    //                               família `TriagRésumé` (é = byte mac roman 0x8E).
    // - `p839-triagslant-obl.ttf` — full name `TriagSlant Oblique` SEM bits
    //                               fsSelection nem ângulo itálico.
    // - `p839-noname.ttf`         — TTF válida sem nenhum registo name.

    /// **P839a (#25/I1)** — aparo de sufixos de estilo do name ID1
    /// (`typographic_family` do vanilla, `info.rs:73-77,206-267`): o ID1
    /// `TriagX Bold` regista a família base `TriagX`; o ID16 é ignorado
    /// (o vanilla não o usa — `info.rs:62-72`).
    #[test]
    fn p839a_aparo_sufixos_estilo_id1() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/p839-triagx-bold.ttf"
        ))
        .expect("fixture p839-triagx-bold.ttf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert_eq!(info.family, "TriagX", "sufixo ' Bold' aparado do ID1");

        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/p839-triagdsp.ttf"
        ))
        .expect("fixture p839-triagdsp.ttf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert_eq!(
            info.family, "TriagDsp Display",
            "ID16 'TriagDsp' ignorado; ID1 'TriagDsp Display Bold' aparado"
        );
    }

    /// Casos de aparo replicados do teste unitário do vanilla
    /// (`info.rs:352-367`, `test_trim_styles`).
    #[test]
    fn p839a_typographic_family_casos_vanilla() {
        assert_eq!(typographic_family("Atma Light"), "Atma");
        assert_eq!(typographic_family("eras bold"), "eras");
        assert_eq!(typographic_family("footlight mt light"), "footlight mt");
        assert_eq!(typographic_family("times new roman"), "times new roman");
        assert_eq!(typographic_family("noto sans mono cond sembd"), "noto sans mono");
        assert_eq!(typographic_family("noto serif SEMCOND sembd"), "noto serif");
        assert_eq!(typographic_family("crimson text"), "crimson text");
        assert_eq!(typographic_family("Noto Sans Light"), "Noto Sans");
        assert_eq!(typographic_family("Noto Sans Semicondensed Heavy"), "Noto Sans");
        assert_eq!(typographic_family("Familx"), "Familx");
        assert_eq!(typographic_family("Font Ultra"), "Font Ultra");
        assert_eq!(typographic_family("Font Ultra Bold"), "Font");
    }

    /// **P839b (#26/I2)** — `decode_mac_roman` (`info.rs:168-203`): fonte
    /// cujos únicos nomes são registos Macintosh (1,0,0) tem a família
    /// decodificada (`TriagRésumé`) em vez de falhar a extracção.
    #[test]
    fn p839b_decode_mac_roman() {
        assert_eq!(decode_mac_roman(b"abc"), "abc");
        assert_eq!(decode_mac_roman(&[0x8E]), "é");

        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/p839-triagmac.ttf"
        ))
        .expect("fixture p839-triagmac.ttf necessária");
        let info = font_info_from_bytes(&data, 0)
            .expect("fonte mac-only tem info extraível após decode_mac_roman");
        assert_eq!(info.family, "TriagRésumé");
    }

    /// **P839c (#27/I3)** — inferência de estilo italic/oblique a partir do
    /// full name (`info.rs:80-103`): a face `TriagSlant Oblique` não tem bits
    /// nem ângulo; o vanilla marca `Oblique` porque o full name contém
    /// "oblique". O vanilla evita `is_italic()` (falsos positivos via ângulo
    /// — typst/typst#7479) e usa `style() == Style::Italic`.
    #[test]
    fn p839c_estilo_inferido_do_full_name() {
        // Heurística pura (full name já em minúsculas).
        assert_eq!(infer_style(false, false, "triagslant oblique"), FontStyle::Oblique);
        assert_eq!(infer_style(false, false, "foo italic"), FontStyle::Italic);
        assert_eq!(infer_style(false, false, "foo slanted"), FontStyle::Oblique);
        assert_eq!(infer_style(false, false, "foo regular"), FontStyle::Normal);
        // Italic tem precedência sobre oblique (match do vanilla).
        assert_eq!(infer_style(false, false, "foo italic oblique"), FontStyle::Italic);
        assert_eq!(infer_style(true, false, "foo"), FontStyle::Italic);
        assert_eq!(infer_style(false, true, "foo"), FontStyle::Oblique);

        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/p839-triagslant-obl.ttf"
        ))
        .expect("fixture p839-triagslant-obl.ttf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert_eq!(
            info.variant.style,
            FontStyle::Oblique,
            "sem bits nem ângulo: estilo inferido do full name"
        );
    }

    /// **P839d (#28/I4)** — `FontBook` e `font_slots` sempre emparelhados:
    /// slots cuja extracção de info falha são descartados (comportamento do
    /// vanilla — `typst-kit/src/fonts.rs:172-189`, `filter_map` + push do par
    /// `(source, info)` junto). Antes do fix: `discover_fonts` criava o slot
    /// e `build_font_book` saltava o push → índices desalinhados.
    #[test]
    fn p839d_slots_e_book_emparelhados() {
        let dir = tempdir();
        let nimbus = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        std::fs::write(dir.path().join("valid.otf"), &nimbus).unwrap();
        std::fs::write(dir.path().join("fake.ttf"), b"not a font").unwrap();

        // A descoberta continua lazy: cria slots para os dois ficheiros.
        let slots = discover_fonts(&[dir.path().to_path_buf()]);
        assert_eq!(slots.len(), 2);

        // O emparelhamento descarta o slot sem info: book e slots alinhados.
        let (slots, book) = pair_slots_with_book(slots);
        assert_eq!(
            slots.len(),
            book.len(),
            "cada entrada do FontBook corresponde ao slot de mesmo índice"
        );
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].path.file_name().unwrap(), "valid.otf");
    }

    // ── P840 — achados #29/#30 de P831: tabela de exceções de metadados ──
    //
    // Port integral do vanilla `text/font/exceptions.rs:46-342`: lookup pelo
    // name ID6 (POST_SCRIPT_NAME); cada campo presente na exceção prevalece
    // sobre a extração normal (família, estilo, peso, stretch).
    //
    // Fixture `p840-fandolhei-bold.ttf` — sintética gerada por fontTools em
    // P831 (`temp/p831/fonts/fandolhei-bold.ttf`): PS `FandolHei-Bold` com
    // usWeightClass=400 errado; a exceção corrige para 700.

    /// Casos unitários da tabela (amostra representativa dos grupos
    /// portados: Arial, Fandol, Noto, NewCM, Latin Modern, SimSun, STKaiti).
    #[test]
    fn p840_find_exception_casos_tabela() {
        // Peso corrigido (usWeightClass errado na fonte).
        assert_eq!(find_exception("Arial-Black").unwrap().weight, Some(900));
        assert_eq!(find_exception("FandolHei-Bold").unwrap().weight, Some(700));
        assert_eq!(find_exception("FandolSong-Bold").unwrap().weight, Some(700));
        assert_eq!(find_exception("STKaitiSC-Black").unwrap().weight, Some(900));
        assert_eq!(find_exception("STKaitiTC-Regular").unwrap().weight, Some(400));

        // Família documentada que o ID1 cru não fornece.
        let e = find_exception("NewCM10-Regular").unwrap();
        assert_eq!(e.family, Some("New Computer Modern"));
        assert_eq!(e.weight, None);
        let e = find_exception("NewCM10-Book").unwrap();
        assert_eq!(e.family, Some("New Computer Modern"));
        assert_eq!(e.weight, Some(450));
        let e = find_exception("NewCM08-Regular").unwrap();
        assert_eq!(e.family, Some("New Computer Modern 08"));
        let e = find_exception("NewCMMath-Regular").unwrap();
        assert_eq!(e.family, Some("New Computer Modern Math"));
        let e = find_exception("NewCMMono10-Regular").unwrap();
        assert_eq!(e.family, Some("New Computer Modern Mono"));

        // Estilo e peso+estilo pela exceção (fontes sem bits de oblique).
        let e = find_exception("NewCMSans10-Oblique").unwrap();
        assert_eq!(e.family, Some("New Computer Modern Sans"));
        assert_eq!(e.style, Some(FontStyle::Oblique));
        assert_eq!(e.weight, None);
        let e = find_exception("NewCMSans10-BookOblique").unwrap();
        assert_eq!(e.weight, Some(450));
        assert_eq!(e.style, Some(FontStyle::Oblique));

        // Noto Display: o ID1 agrupa variantes Display inacessíveis.
        let e = find_exception("NotoSans-DisplayCondensedBold").unwrap();
        assert_eq!(e.family, Some("Noto Sans Display"));
        let e = find_exception("NotoSerif-DisplayCondensedBold").unwrap();
        assert_eq!(e.family, Some("Noto Serif Display"));

        // Latin Modern: famílias óticas + peso/stretch corrigidos.
        let e = find_exception("LMRoman7-Regular").unwrap();
        assert_eq!(e.family, Some("Latin Modern Roman 7"));
        let e = find_exception("LMMonoLtCond10-Regular").unwrap();
        assert_eq!(e.weight, Some(300));
        assert_eq!(e.stretch, Some(666));
        let e = find_exception("LMMonoLt10-BoldOblique").unwrap();
        assert_eq!(e.style, Some(FontStyle::Oblique));

        // SimSun-ExtB não é "ExtraBold" de SimSun.
        let e = find_exception("SimSun-ExtB").unwrap();
        assert_eq!(e.family, Some("SimSun-ExtB"));

        // Sem exceção → None.
        assert!(find_exception("NimbusSans-Regular").is_none());
        assert!(find_exception("Inexistente-Regular").is_none());
        assert!(find_exception("").is_none());
    }

    /// **P840 (#30/E2)** — a face com PS `FandolHei-Bold` tem
    /// usWeightClass=400 errado na OS/2; a exceção corrige para 700 (sem
    /// ela, `#set text(weight: "bold")` fica preso na regular — medido:
    /// `AAAA` a 22.0pt no cristalino vs 44.0pt no vanilla).
    #[test]
    fn p840_excecao_peso_fandol_hei_bold() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/p840-fandolhei-bold.ttf"
        ))
        .expect("fixture p840-fandolhei-bold.ttf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert_eq!(info.family, "FandolHei", "família vem do ID1 (sem override)");
        assert_eq!(
            info.variant.weight,
            FontWeight(700),
            "usWeightClass=400 errado corrigido para 700 pela exceção"
        );
    }

    /// **P840 (#29/E1)** — as NewCM embutidas via `typst-assets` têm ID1
    /// cru (`NewComputerModern10`); a exceção regista a família documentada
    /// na referência do Typst (`New Computer Modern`) — sem ela,
    /// `#set text(font: "New Computer Modern")` dava
    /// `warning: unknown font family` (medido).
    #[test]
    fn p840_excecao_familia_newcm_embutida() {
        let mut vistos = std::collections::HashMap::new();
        for data in typst_assets::fonts() {
            let face = match ttf_parser::Face::parse(data, 0) {
                Ok(face) => face,
                Err(_) => continue,
            };
            let Some(ps) = find_name(&face, ttf_parser::name_id::POST_SCRIPT_NAME) else {
                continue;
            };
            if ps.starts_with("NewCM") {
                let info = font_info_from_bytes(data, 0).expect("fonte embutida válida");
                vistos.insert(ps, (info.family, info.variant.weight));
            }
        }
        assert_eq!(
            vistos.get("NewCM10-Regular").map(|(f, _)| f.as_str()),
            Some("New Computer Modern"),
            "NewCM10-Regular regista a família documentada"
        );
        assert_eq!(
            vistos.get("NewCM10-Bold").map(|(f, _)| f.as_str()),
            Some("New Computer Modern")
        );
        assert_eq!(
            vistos.get("NewCMMath-Book").map(|(_, w)| *w),
            Some(FontWeight(450)),
            "NewCMMath-Book pesa 450 pela exceção"
        );
        assert!(
            vistos.contains_key("NewCM10-Regular"),
            "a fonte embutida NewCM10-Regular tem de existir (typst-assets)"
        );
    }

    /// Regressão: fonte sem entrada na tabela de exceções tem extração
    /// inalterada (NimbusSans-Regular não está na tabela).
    #[test]
    fn p840_fonte_sem_excecao_inalterada() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert_eq!(info.family, "Nimbus Sans");
        assert_eq!(info.variant.weight, FontWeight(400));
        assert_eq!(info.variant.style, FontStyle::Normal);
    }

    // ── P838 — flags.serif via panose OS/2 (critério do vanilla) ────────────
    /// Critério idêntico ao vanilla (`info.rs:131-138`):
    /// `matches!(panose, [2, 2..=10, ..])` sobre OS/2 bytes 32..45.
    #[test]
    fn p838_serif_detectado_via_panose() {
        // Fixture sans-serif: NimbusSans panose [2, 11, …] → 11 ∉ 2..=10.
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert!(!info.flags.serif, "NimbusSans-Regular é sans → serif=false");

        // Serif real do sistema (skip se ausente): DejaVu Serif panose
        // [2, 6, …] → serif=true.
        if let Ok(data) =
            std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf")
        {
            let info = font_info_from_bytes(&data, 0).expect("DejaVu Serif válida");
            assert!(info.flags.serif, "DejaVu Serif panose [2,6,…] → serif=true");
        }

        // Sans real do sistema (skip se ausente).
        if let Ok(data) = std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
        {
            let info = font_info_from_bytes(&data, 0).expect("DejaVu Sans válida");
            assert!(!info.flags.serif, "DejaVu Sans panose [2,11,…] → serif=false");
        }
    }

    // ── P838 — extracção de faces de colecções (.ttc) com offsets reescritos ──

    /// Constrói uma colecção TTC sintética com `n` faces, todas apontando
    /// para a mesma fonte (`font`), com offsets de tabela absolutos ao
    /// início da colecção (como num TTC real).
    fn build_synthetic_ttc(font: &[u8], n: usize) -> Vec<u8> {
        let face_off = 12 + 4 * n; // cabeçalho ttcf + tabela de offsets
        let num_tables = u16::from_be_bytes([font[4], font[5]]) as usize;
        let dir_len = 16 * num_tables;

        let mut ttc = Vec::new();
        ttc.extend_from_slice(b"ttcf");
        ttc.extend_from_slice(&0x00010000u32.to_be_bytes()); // version 1.0
        ttc.extend_from_slice(&(n as u32).to_be_bytes());
        for _ in 0..n {
            ttc.extend_from_slice(&(face_off as u32).to_be_bytes());
        }
        // Cabeçalho sfnt da fonte (12 bytes) tal qual.
        ttc.extend_from_slice(&font[..12]);
        // Directório com offsets convertidos para absolutos (+face_off).
        for i in 0..num_tables {
            let rec = 12 + i * 16;
            let mut record = font[rec..rec + 16].to_vec();
            let off = u32::from_be_bytes([record[8], record[9], record[10], record[11]]);
            record[8..12].copy_from_slice(&(off + face_off as u32).to_be_bytes());
            ttc.extend_from_slice(&record);
        }
        // Dados das tabelas (tudo o que segue o directório na fonte original).
        ttc.extend_from_slice(&font[12 + dir_len..]);
        ttc
    }

    /// **P838** — a extracção P609 fazia slice dos bytes da face, mas os
    /// offsets do directório de tabelas em TTC são absolutos ao início da
    /// colecção: o slice ficava corrompido e `Face::parse` falhava. A nova
    /// extracção reconstrói o ficheiro com offsets reescritos.
    #[test]
    fn p838_extract_collection_face_produz_fonte_valida() {
        let font = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let ttc = build_synthetic_ttc(&font, 2);

        let extracted = extract_collection_face(&ttc, 0)
            .expect("face 0 de colecção válida extrai-se");
        let face = ttf_parser::Face::parse(&extracted, 0).expect("face extraída parseia");
        let orig = ttf_parser::Face::parse(&font, 0).unwrap();

        // Mesma família e mesma cobertura que a fonte original.
        let name = |f: &ttf_parser::Face| {
            f.names()
                .into_iter()
                .filter(|n| n.name_id == ttf_parser::name_id::FAMILY)
                .filter_map(|n| n.to_string())
                .next()
        };
        assert_eq!(name(&face), name(&orig));
        assert_eq!(face.glyph_index('A'), orig.glyph_index('A'));
        assert!(face.glyph_index('A').is_some());

        // checkSumAdjustment consistente: soma do ficheiro ≡ 0xB1B0AFBA.
        let mut sum: u32 = 0;
        for chunk in extracted.chunks(4) {
            let mut word = [0u8; 4];
            word[..chunk.len()].copy_from_slice(chunk);
            sum = sum.wrapping_add(u32::from_be_bytes(word));
        }
        assert_eq!(sum, 0xB1B0AFBA, "checkSumAdjustment do head recalculado");
    }

    /// **P838** — regressão real: as faces CJK das colecções do sistema
    /// ficavam incarregáveis (`Face::parse` falhava nos bytes sliced), o que
    /// escondia todas as Noto CJK do fallback global (achado #24 de P831).
    #[test]
    fn p838_extract_collection_face_noto_cjk_real() {
        let Ok(data) =
            std::fs::read("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc")
        else {
            eprintln!("SKIP: NotoSansCJK-Regular.ttc não disponível");
            return;
        };
        let extracted =
            extract_collection_face(&data, 0).expect("face 0 (JP) extrai-se da colecção");
        let face = ttf_parser::Face::parse(&extracted, 0).expect("face JP parseia");
        assert!(face.glyph_index('日').is_some(), "Noto Sans CJK JP extraída cobre CJK");
        let info =
            font_info_from_bytes(&extracted, 0).expect("FontInfo da face extraída");
        assert_eq!(info.family, "Noto Sans CJK JP");
        assert!(!info.flags.serif, "Noto Sans CJK panose [2,11] → serif=false");
    }

    // ── P937 — cobertura Unicode exacta + mmap em FontSlot ────────────────

    /// P938 — `font_info_from_bytes` deixa `coverage` vazio; `extract_coverage`
    /// produz coverage exacta quando chamado.
    #[test]
    fn p938_font_info_coverage_vazia_extract_coverage_exacta() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let info = font_info_from_bytes(&data, 0).expect("fixture válida");
        assert!(
            info.coverage.is_empty(),
            "coverage deve estar vazia em font_info_from_bytes"
        );

        let face = ttf_parser::Face::parse(&data, 0).expect("fixture válida");
        let coverage = extract_coverage(&face);
        assert!(!coverage.is_empty(), "extract_coverage deve preencher coverage");
        assert!(coverage.contains('A' as u32), "Nimbus Sans cobre 'A'");
        assert!(coverage.contains('z' as u32), "Nimbus Sans cobre 'z'");
        // Exacta: codepoint do mesmo bloco que não está na cmap é falso negativo.
        // U+0370 é do bloco grego mas Nimbus Sans Regular não o cobre.
        assert!(
            !coverage.contains(0x0370),
            "coverage exacta: codepoint do bloco grego não coberto"
        );
    }

    /// Fonte sem cmap → coverage vazia; bytes inválidos → None.
    #[test]
    fn p937_font_info_bytes_invalidos() {
        assert!(font_info_from_bytes(b"not a font", 0).is_none());
    }

    /// Colecção TTC sintética: ambas as faces carregam com sucesso via mmap
    /// (cada slot tem o seu próprio mmap do mesmo ficheiro).
    #[test]
    fn p937_ttc_slots_carregam_via_mmap() {
        let dir = tempdir();
        let font = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let ttc = build_synthetic_ttc(&font, 2);
        let path = dir.path().join("test.ttc");
        std::fs::write(&path, &ttc).unwrap();

        let slots = discover_fonts(&[path]);
        assert_eq!(slots.len(), 2, "TTC com 2 faces produz 2 slots");

        // Ambas as faces carregam (são a mesma fonte repetida no TTC sintético).
        assert!(slots[0].get().is_some(), "face 0 carrega");
        assert!(slots[1].get().is_some(), "face 1 carrega");

        // Emparelhamento produz 2 entradas no book, coverage vazia (lazy).
        let (slots, book) = pair_slots_with_book(slots);
        assert_eq!(book.len(), 2, "book tem entrada para cada face válida");
        assert_eq!(slots.len(), 2);
        assert!(book.infos()[0].coverage.is_empty(), "coverage lazy começa vazia");
    }

    /// FontSlot cria mmap lazy e devolve Font::Mmap para fontes simples.
    #[test]
    fn p937_font_slot_mmap_simples() {
        let dir = tempdir();
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let path = dir.path().join("nimbus.otf");
        std::fs::write(&path, &data).unwrap();

        let slot = FontSlot::new(path, 0);
        let font = slot.get().expect("fonte válida carrega");
        // Fontes simples do disco devolvem Font::Mmap.
        match font {
            Font::Vec(_) => panic!("fonte simples do disco deve usar Mmap"),
            Font::Mmap(_) => {}
        }
        assert_eq!(font.as_slice().len(), data.len());
    }
}
