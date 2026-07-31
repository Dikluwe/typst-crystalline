# Prompt L0 — `infra/fonts` — Gestão e Carregamento de Fontes
Hash do Código: d38fbbe3

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/fonts.rs`
**Criado em**: 2026-03-26 (Passo 11)
**Atualizado em**: 2026-07-23 (P875 — `Coverage` Unicode em `FontInfo` + partilha de bytes entre faces de `.ttc`)
**ADRs relevantes**: ADR-0019 (`ttf-parser` → L3 exclusivo), ADR-0022 (`FontInfo` — L1 recebe apenas campos primitivos), ADR-0107 (paridade com a linguagem)

---

## Contexto e Objetivo

O sistema precisa mapear pedidos de fonte ("Arial, Bold") para bytess binários
no disco ou memória. Este módulo gere o "livro de fontes" (*Font Book*) e é o
**sistema central de resolução tipográfica** — todo o I/O de ficheiros de fonte
está confinado aqui.

### Fronteira de Arquitectura

- `ttf_parser` **não escapa** a fronteira de L3: L1 recebe apenas
  `Font(Vec<u8>)` opaco (tipo de L1 em `world_types.rs`) e `FontInfo`
  (struct de L1 com campos primitivos — `String`, `FontVariant`, `FontFlags`).
- `OnceLock<Option<Font>>` garante carregamento **lazy** — os bytes da fonte
  são lidos do disco apenas na primeira chamada a `FontSlot::get()`.
- Validação de bytes acontece em `get()` (via `ttf_parser::Face::parse`),
  **não** na descoberta. Um slot pode ser criado para um ficheiro inválido;
  `get()` retornará `None`.

---

## Interface Pública

### `FontSlot` — carregamento lazy de uma face de fonte

```rust
pub struct FontSlot {
    pub path:  PathBuf,  // caminho do ficheiro no disco
    pub index: u32,      // índice da face num TrueType Collection (.ttc); 0 para fontes simples
    font:      OnceLock<Option<Font>>,  // interior mutável — thread-safe
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self


    /// Carrega e valida lazy. None se: ficheiro não existe, não legível,
    /// bytes inválidos, ou índice fora dos limites da colecção.
    /// OnceLock garante idempotência — resultado sempre igual para mesma instância.
    pub fn get(&self) -> Option<Font>
```

### `discover_fonts` — varrimento de paths

```rust
/// Descobre fontes nos paths fornecidos.
/// Cada path pode ser: ficheiro de fonte directamente, ou directório (varrido recursivamente).
/// Extensões aceites: .ttf, .otf, .ttc, .otc
/// Colecções .ttc → múltiplos slots (um por face via face_count / ttf_parser::fonts_in_collection)
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot>
```

### `font_info_from_bytes` — extrai metadados para o FontBook

```rust
/// Extrai FontInfo de bytes de fonte OpenType/TrueType (ADR-0022).
/// ttf_parser fica em L3 — L1 recebe apenas FontInfo com campos primitivos.
/// Retorna None se bytes inválidos ou índice inexistente.
///
/// P838 — flags.serif detectado via panose (OS/2 bytes 32..45):
/// `matches!(panose, [2, 2..=10, ..])` — critério idêntico ao vanilla
/// (typst-library/src/text/font/info.rs:131-138), lido via
/// `face.raw_face().table(Tag::from_bytes(b"OS/2"))`.
///
/// P839 — paridade de resolução de nome/estilo com o vanilla
/// (typst-library/src/text/font/info.rs), achados #25–#27 de P831:
/// família só do ID1 com aparo de sufixos; decode mac roman; estilo
/// inferido também do full name.
///
/// P840 — tabela de exceções de metadados (`find_exception`), port
/// integral do vanilla `text/font/exceptions.rs:46-342`, aplicada como
/// no vanilla `info.rs:60-61,73-77,80-112`: a lookup pelo name ID6
/// (POST_SCRIPT_NAME) e cada campo da exceção, quando presente,
/// **prevalece** sobre a extração normal (família, estilo, peso, stretch).
///
/// **P935** — `coverage` é extraído eager percorrendo a tabela `cmap` da face.
/// Segue o padrão do vanilla (`typst-library/src/text/font/info.rs:117-123`):
/// a cobertura fica disponível no `FontBook` para consultas O(1) durante o
/// fallback, evitando reabrir e reparsear fontes no caminho quente.
pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo>
```

Campos extraídos:
- **`family`**: exceção (`find_exception` pelo name ID6) se existir; senão
  name ID1 (`FAMILY`) com aparo iterativo de sufixos de estilo
  (`typographic_family` — port do vanilla `info.rs:206-267`: listas de
  sufixos/modificadores/separadores, case-insensitivo, repetido até
  fixpoint). O ID16 (TYPOGRAPHIC_FAMILY) é **ignorado de propósito** — o
  vanilla não o usa porque para algumas fontes agrupa mais do que
  variantes de estilo/peso/largura (`info.rs:62-72`). Nomes em registos
  Macintosh (plataforma 1, encoding 0) são decodificados via
  `decode_mac_roman` (tabela de 128 chars do vanilla `info.rs:185-203`) —
  `ttf_parser` 0.25 não os decodifica em `Name::to_string()`.
  Fallback residual **sem equivalente vanilla** (que descartaria a fonte):
  primeiro registo name decodificável, para fontes sem ID1.
- **`variant.style`**: exceção se existir; senão `infer_style` (port do
  vanilla `info.rs:80-103`) —
  `Italic` se `face.style() == Style::Italic` ou full name (minúsculas)
  contém "italic"; `Oblique` se `face.is_oblique()` ou full name contém
  "oblique"/"slanted"; italic tem precedência. **Não** usa
  `face.is_italic()` (consulta o ângulo → falsos positivos em oblique,
  typst/typst#7479).
- **`variant.weight`**: exceção se existir; senão `FontWeight(face.weight().to_number())` — escala OpenType 100–900
- **`variant.stretch`**: exceção se existir; senão `FontStretch::from_number(face.width().to_number())`
- **`flags.monospace`**: `face.is_monospaced()`
- **`flags.serif`**: panose OS/2 (bytes 32..45) com o critério do vanilla
  `[2, 2..=10, ..]` (P838) — antes era `false` fixo
- **`coverage`**: **extraído aqui** (P935 — padrão vanilla). Percorre a tabela
  `cmap` da face e preenche o bitmap por blocos de 256 codepoints.

### `find_exception` — tabela de exceções de metadados (P840)

```rust
/// Lookup pelo name ID6 (POST_SCRIPT_NAME) — port integral do vanilla
/// `text/font/exceptions.rs:46-342` (todas as entradas, sem scope-out).
/// Cada campo presente na exceção prevalece sobre a extração normal:
/// - family: nome documentado da linguagem (ex.: PS `NewCM10-Regular`
///   tem ID1 `NewComputerModern10`; a exceção regista a família
///   `New Computer Modern`, o nome da referência oficial do Typst);
/// - weight: corrige usWeightClass errado (ex.: PS `FandolHei-Bold`
///   marcado 400 → 700; `Arial-Black` 1996 → 900; Book da NewCM → 450);
/// - style: ex.: `NewCMSans10-Oblique` → Oblique (sem bits na fonte);
/// - stretch: ex.: `LMMonoLtCond10-Regular` → 666.
fn find_exception(postscript_name: &str) -> Option<FontException>
```

**Alcance do port**: **integral** — todas as entradas do vanilla
(`exceptions.rs:46-342` na revisão em `lab/typst-original`): Arial-Black,
Archivo Narrow, Fandol (Hei/Song), Noto (Naskh Arabic UI, Sora Sompeng,
Sans Display, Serif Display v2.007), New Computer Modern (08/10, Math,
Mono, Sans, Uncial), Latin Modern (Mono/Roman/Sans), SimSun-ExtB,
STKaiti. Sem entradas de fora; expansões futuras seguem o vanilla.

### `pair_slots_with_book` — emparelha slots com o FontBook (P839)

```rust
/// Lê os bytes de cada slot e extrai FontInfo via font_info_from_bytes.
/// Slots cuja extracção falha são DESCARTADOS: cada entrada do FontBook
/// corresponde ao slot de mesmo índice, como no vanilla
/// (typst-kit/src/fonts.rs:172-189 — filter_map produz o par (source, info)
/// e FontStore::push insere os dois juntos).
/// NOTA: duplica o I/O com FontSlot::get() — optimização futura (Passo 11).
pub fn pair_slots_with_book(slots: Vec<FontSlot>) -> (Vec<FontSlot>, FontBook)
```

**Motivo (bug medido em P831, achado #28/I4):** a API anterior
(`build_font_book(&[FontSlot]) -> FontBook`) criava o slot
incondicionalmente e fazia push no book só quando a info era extraída —
uma fonte sem nome decodificável entrava nos slots mas não no book, e
todos os índices seguintes do book apontavam para o slot errado (o
shaper indexa `font_slots` pelo índice do book; medido: fallback a
renderizar com a face errada, 11pt/22pt em vez de 16.5pt do vanilla).

### Extracção de faces de colecções (.ttc/.otc)

`FontSlot::get()` expõe sempre uma **fonte simples** (P609). Para ficheiros
TTC/OTC, `extract_collection_face(data, index)` **reconstrói o ficheiro** da
face pedida (P838): cabeçalho sfnt da face + directório de tabelas com
offsets reescritos para o novo ficheiro + bytes das tabelas copiados dos
offsets absolutos da colecção, com `checkSumAdjustment` do `head`
recalculado.

**Motivo (bug medido em P838):** os offsets do directório de tabelas de uma
face em TTC são **absolutos ao início da colecção** — a extracção original
(slice `data[start..end]`) produzia ficheiros com offsets fora de alcance e
`Face::parse` falhava. Efeito medido: todas as faces `Noto*CJK*.ttc` do
sistema ficavam incarregáveis e o fallback CJK caía sempre numa fonte
simples (`Droid Sans Fallback`).

---

## Funções Internas

| Função | Responsabilidade |
|--------|-----------------|
| `is_font_file(path)` | Verifica extensão: `.ttf`, `.otf`, `.ttc`, `.otc` |
| `face_count(path)` | Lê `ttf_parser::fonts_in_collection(data)` → n faces; fallback: 1 |
| `push_slots(path, slots)` | Cria `face_count` slots para o ficheiro (suporte a `.ttc`) |
| `discover_in_dir(dir, slots)` | Varredura recursiva de directório |
| `find_name(face, id)` | Procura + decodifica name (com fallback mac roman) — P839 |
| `decode_mac_roman(bytes)` | Tabela mac roman → char (port do vanilla) — P839 |
| `typographic_family(name)` | Aparo iterativo de sufixos de estilo do ID1 — P839 |
| `infer_style(italic, oblique, full)` | Estilo a partir de bits + full name — P839 |
| `find_exception(ps_name)` | Exceções de família/estilo/peso/stretch por PostScript name (port integral do vanilla) — P840 |

---

## Critérios de Verificação

```
// FontSlot
FontSlot::new("/nao/existe.ttf", 0).get() = None
FontSlot::new(path_bytes_invalidos, 0).get() = None

// Idempotência do OnceLock
slot.get() == slot.get()   // sempre igual para a mesma instância

// discover_fonts
discover_fonts(&[directorio_vazio])                   = []
discover_fonts(&[dir_com_readme_txt_e_data_bin])      = []  // extensões não-fonte ignoradas
discover_fonts(&[dir_com_fake_dot_ttf])               = [slot]  // slot criado
// slot de bytes inválidos → get() = None:
discover_fonts(&[dir_com_fake_dot_ttf])[0].get() = None

// font_info_from_bytes
font_info_from_bytes(b"not a font", 0) = None
// P839: ID1 "TriagX Bold" → family "TriagX"; ID16 ignorado
// P839: fonte só com nomes Macintosh → family "TriagRésumé" (decode_mac_roman)
// P839: full name "TriagSlant Oblique" sem bits → style Oblique
// P840: PS "NewCM10-Regular" (embutida) → family "New Computer Modern"
// P840: PS "NewCM10-Book" → weight 450; PS "NewCMSans10-Oblique" → style Oblique
// P840: PS "FandolHei-Bold" com usWeightClass=400 → weight 700
// P840: fonte sem exceção → extração inalterada (sem regressão)

// pair_slots_with_book (P839)
pair_slots_with_book([slot_invalido]) = ([], book vazio)
pair_slots_with_book([válido, inválido]): slots.len() == book.len() == 1
// com fonte real (.ttf fixture):
// FontInfo.family não vazio
// FontInfo.variant.weight ∈ [100, 900]
// FontInfo.flags.monospace: correcto para fonte monospace
```

---

## Relação com Outros Módulos

| Módulo | Como consome `fonts.rs` |
|--------|------------------------|
| `FontBookMetrics` (este crate, L3) | Consome `Font(Vec<u8>)` de `FontSlot::get()` para construir `Face` |
| `SystemWorld` (L3 — `world.rs`) | Chama `discover_fonts` na inicialização; chama `pair_slots_with_book` para emparelhar slots e `FontBook` |
| `MathLayouter` e `Layouter` (L1) | Recebem `&dyn FontMetrics` — nunca tocam em `FontSlot` |
| `FontBook` (L1) | Recebe `FontInfo` (primitivos) — nunca recebe `ttf_parser::Face` |

---
## Cobertura Unicode eager — P935

**Problema medido (P935):** o cristalino deixava `FontInfo::coverage` vazio (P880)
e computava a cobertura lazy na primeira consulta de fallback. Quando o fallback
CJK/emoji disparava, o scan de **todas** as fontes do sistema via
`std::fs::read` + `ttf_parser::Face::parse` repetia o I/O e o parsing no caminho
quente. O vanilla real (`typst-kit`/`fontdb-0.23.0`) faz o oposto: extrai a
cobertura **eager** durante a descoberta de fontes, usando o mmap do `fontdb`,
e depois consulta o bitmap em O(1) durante o fallback.

**Solução:** `font_info_from_bytes` preenche `coverage` eager, percorrendo a
tabela `cmap` da face e marcando os blocos de 256 codepoints cobertos. O
`FontBook` entrega `coverage` pronta; `World::candidates_for_char` delega
simplesmente a `FontBook::candidates_for_char`.

```rust
fn extract_coverage(face: &ttf_parser::Face) -> Coverage
```

Percorre a tabela `cmap` da face e marca o bloco de 256 codepoints de cada
caractere presente.

- Usar `ttf_parser::Face::tables().cmap` e iterar os subtables.
- Para cada subtable, iterar os codepoints cobertos (`subtable.iter()`) e
  inserir o bloco correspondente (`codepoint / 256`) no `Coverage`.
- O bitmap tem 4 096 bits (64×64 `u64`), cobrindo blocos 0..4095
  (U+0000..U+3FFFF). Codepoints acima de U+3FFFF são ignorados no bitmap
  (caso raro para fallback de texto).
- O `Coverage` é uma aproximação por bloco: um candidato pode aparecer em
  `candidates_for_char` sem cobrir o caractere exacto; o shaper continua a
  verificar `face.glyph_index(c)` antes de usar o candidato.

**Consequências:**

- O custo de iterar a `cmap` paga-se uma única vez no startup, amortizado por
  todas as consultas de fallback.
- Documentos latinos/gregos simples pagam esse custo upfront, mas a medição
  P935 mostra que, com o mmap do `fontdb`, o valor (~83 ms para ~1112 faces
  em cache quente) é comparável ao vanilla real (~274 ms total para um
  documento latino).
- O fallback CJK/emoji deixa de reparsear fontes no caminho quente.

## I/O de fontes — P935

**Princípio:** `FontSlot` guarda apenas `path` + `index` (e `embedded` para
fontes embutidas). Não mantém bytes de ficheiro de sistema em memória entre
`get()` e o startup: o vanilla descarta o `fontdb::Database` (e os seus mmap)
depois de extrair `FontInfo`, e só relê uma fonte específica quando `get()`
é chamado.

- `FontSlot::get()` lê o ficheiro com `std::fs::read` quando a fonte é
  realmente necessária, extrai a face de uma colecção se aplicável, valida
  com `Face::parse(..., 0)` e devolve `Font(Vec<u8>)`.
- `fontdb::load_system_fonts()` aproveita o mmap interno do `fontdb` para
  extrair `FontInfo` + coverage sem copiar o ficheiro inteiro para memória do
  processo. O `fontdb::Database` é descartado no final do carregamento.


## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-03-26 | Criação — Passo 11: `FontSlot`, `discover_fonts` (lazy I/O) | `fonts.rs` |
| 2026-04-12 | Restauro — expandido: `font_info_from_bytes` (ADR-0022), `build_font_book`, suporte `.ttc`, relação com SystemWorld | `fonts.md` |
| 2026-07-22 | P838 — `flags.serif` via panose OS/2 (critério vanilla `[2, 2..=10, ..]`), necessário ao scoring de `FontBook::select_fallback`; `extract_collection_face` reescreve os offsets do directório de tabelas (eram absolutos à colecção — faces .ttc ficavam incarregáveis) | `fonts.md`, `fonts.rs` |
| 2026-07-22 | P839 — achados #25–#28 de P831: família só do ID1 com `typographic_family` (ID16 ignorado), `decode_mac_roman` para registos Macintosh, `infer_style` pelo full name (sem `is_italic()`); `build_font_book` substituído por `pair_slots_with_book` (slots sem info descartados — índices book↔slots sempre alinhados, como no vanilla) | `fonts.md`, `fonts.rs`, `world.rs`, `integration_tests.rs`, fixtures `p839-*.ttf` |
| 2026-07-22 | P840 — achados #29/#30 de P831: `find_exception` + tabela de exceções portada integralmente do vanilla (`exceptions.rs:46-342`), aplicada em `font_info_from_bytes` por PostScript name (família/estilo/peso/stretch da exceção prevalecem) | `fonts.md`, `fonts.rs`, fixture `p840-fandolhei-bold.ttf` |
| 2026-07-23 | P875 — `Coverage` Unicode em `FontInfo` (extraído da `cmap`); partilha lazy de bytes entre faces do mesmo `.ttc` via `Arc<OnceLock<Arc<Vec<u8>>>>` | `fonts.md`, `fonts.rs`, `font_book.md`, `shaper.rs` |
| 2026-07-23 | P880 — `font_info_from_bytes` deixa `coverage` vazio; cobertura Unicode computada lazy por `World::candidates_for_char` com cache em `SystemWorld` | `fonts.md`, `fonts.rs`, `world.rs`, `font_book.md`, `shaper.rs`, `font_metrics.rs`, `contracts/world.md` |
| 2026-07-31 | P935 — cobertura Unicode eager em `font_info_from_bytes` (padrão vanilla); `FontSlot` volta a guardar apenas path/index, sem bytes de ficheiro de sistema persistentes | `fonts.md`, `fonts.rs`, `fontdb.md`, `world.rs` |
