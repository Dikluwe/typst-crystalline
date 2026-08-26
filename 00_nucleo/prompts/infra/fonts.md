# Prompt L0 — `infra/fonts` — Gestão e Carregamento de Fontes
Hash do Código: 8b36be59

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/fonts.rs`
**Criado em**: 2026-03-26 (Passo 11)
**Atualizado em**: 2026-07-31 (P937 — coverage exacta eager + mmap em `FontSlot`)
**ADRs relevantes**: ADR-0019 (`ttf-parser` → L3 exclusivo), ADR-0022 (`FontInfo` — L1 recebe apenas campos primitivos), ADR-0107 (paridade com a linguagem), ADR-0123 (estrutura física portável do vanilla)

---

## Contexto e Objetivo

O sistema precisa mapear pedidos de fonte ("Arial, Bold") para bytes binários
no disco ou memória. Este módulo gere o "livro de fontes" (*Font Book*) e é o
**sistema central de resolução tipográfica** — todo o I/O de ficheiros de fonte
está confinado aqui.

### Fronteira de Arquitectura

- `ttf_parser` **não escapa** a fronteira de L3: L1 recebe apenas
  `Font` opaco (tipo de L1 em `world_types.rs`) e `FontInfo`
  (struct de L1 com campos primitivos — `String`, `FontVariant`, `FontFlags`,
  `Coverage`).
- `Font` (L1) pode ser `Font::Vec(Arc<Vec<u8>>)` ou `Font::Mmap(Arc<Mmap>)` —
  L3 escolhe a variante; L1 só vê `as_slice()`.
- `OnceLock<Option<Font>>` garante carregamento **lazy** da face final — a
  face simples é exposta apenas na primeira chamada a `FontSlot::get()`.
- O **mmap** do ficheiro de fonte é criado em L3 (sempre que possível) para
  evitar `std::fs::read` e cópias para memória do processo.
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
    embedded:  Option<Vec<u8>>,  // bytes embutidos (typst-assets); têm precedência sobre mmap
    mmap:      OnceLock<Option<Arc<memmap2::Mmap>>>,  // mmap lazy do ficheiro
    font:      OnceLock<Option<Font>>,  // face simples final (interior mutável — thread-safe)
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self
    pub fn new_embedded(path: PathBuf, data: Vec<u8>) -> Self

    /// Devolve os bytes fonte como slice sem extrair face de colecção.
    /// Usado por `pair_slots_with_book` e pelo cache de coverage.
    /// - Embutidos: devolve slice sobre `embedded`.
    /// - Disco: cria/mantém mmap do ficheiro e devolve slice sobre ele.
    fn source_bytes(&self) -> Option<Cow<'_, [u8]>>

    /// Carrega e valida lazy a face simples. None se: ficheiro não existe,
    /// não legível, bytes inválidos, ou índice fora dos limites da colecção.
    /// Para .ttc/.otc, extrai a face individual para bytes independentes.
    /// OnceLock garante idempotência — resultado sempre igual para mesma instância.
    pub fn get(&self) -> Option<Font>
}
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
/// **P937/P938** — `coverage` é deixado **vazio** aqui. A cobertura Unicode
/// exacta (runs de codepoints, sem falsos positivos) é computada **lazy** por
/// `SystemWorld::candidates_for_char` e cacheada por índice (ver
/// `infra/system-world.md`). Deixar `coverage` vazio no arranque evita iterar a
/// `cmap` de todas as fontes do sistema para documentos que não precisam de
/// fallback. Quando o fallback de facto dispara, a extração por fonte usa mmap
/// (barato) e `Coverage::from_codepoints`, preservando a exactidão que elimina
/// os falsos positivos do bitmap por bloco.
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
- **`coverage`**: **deixado vazio aqui** (P938). A cobertura Unicode exacta é
  computada lazy por `SystemWorld::candidates_for_char` (ver
  `infra/system-world.md`). A função `extract_coverage` continua a produzir
  `Coverage::from_codepoints` exacta, mas é chamada por fonte e só quando
  necessário, não para todas as fontes no arranque.

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
/// Extrai `FontInfo` de cada slot via `source_bytes()` (mmap quando possível)
/// e `font_info_from_bytes`, deixando `coverage` vazio para preenchimento lazy
/// por `SystemWorld::candidates_for_char`. Slots cuja extracção falha são
/// DESCARTADOS: cada entrada do FontBook corresponde ao slot de mesmo índice,
/// como no vanilla (`typst-kit/src/fonts.rs:172-189`, `filter_map` produz o par
/// `(source, info)` e insere os dois juntos).
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

// Coverage exacta (P937)
font_info_from_bytes(NimbusSans-Regular, 0).coverage.contains('A' as u32) == true
font_info_from_bytes(NimbusSans-Regular, 0).coverage.contains('你' as u32) == false
// P933 caso de falso positivo: bloco grego sem α — coverage exacta resolve
// (teste concreto com fixture se disponível)

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
| `FontBookMetrics` (este crate, L3) | Consome `Font` de `FontSlot::get()` para construir `Face` |
| `SystemWorld` (L3 — `world.rs`) | Chama `discover_fonts` na inicialização; chama `pair_slots_with_book` para emparelhar slots e `FontBook`; delega `candidates_for_char` a `FontBook` |
| `MathLayouter` e `Layouter` (L1) | Recebem `&dyn FontMetrics` — nunca tocam em `FontSlot` |
| `FontBook` (L1) | Recebe `FontInfo` (primitivos + `Coverage` exacta) — nunca recebe `ttf_parser::Face` |

---

## Coverage exacta lazy — P937/P938

**Problema:** o bitmap por bloco de 256 codepoints (P880) tem falsos
positivos reais. Durante o fallback, cada falso positivo obriga o shaper a
abrir a face para confirmar `face.glyph_index(c)`, tornando o tempo
proporcional ao número de falsos positivos do script (P936 mediu 22× mais
lento que o vanilla em emoji).

**Solução P937:** materializar a cobertura **exacta** (runs de codepoints) em
L1, de forma que `FontBook::candidates_for_char` nunca tenha falsos positivos.

**Solução P938:** a extração da coverage exacta move-se de *eager* (todas as
fontes no arranque) para *lazy* (por fonte, na primeira consulta). `SystemWorld`
consulta o `FontBook`; quando encontra uma entrada com `coverage` vazio, extrai
a coverage exacta via `extract_coverage` usando `FontSlot::source_bytes()` (mmap
quando possível), preenche o cache e devolve os candidatos. Texto dinâmico
(`context`, interpolações, `read()`) continua no caminho lazy original.

Para evitar pagar qualquer custo de coverage em documentos latinos simples,
`SystemWorld` pode opcionalmente fazer um **pré-scan condicional** do source
bruto (P927): se nenhum caractere escapar à cobertura das fontes embutidas,
não dispara a extração lazy das fontes do sistema.

### `extract_coverage(face)` — helper interno

```rust
fn extract_coverage(face: &ttf_parser::Face) -> Coverage
```

- Percorre a tabela `cmap` da face.
- Para cada subtable unicode (`subtable.is_unicode()`), itera os codepoints
  (`subtable.codepoints(...)`).
- Constrói `Coverage::from_codepoints(codepoints)` — representação exacta por
  runs de codepoints (ver `entities/font_book.md`).

### Consequências

- `FontBook::candidates_for_char` devolve apenas índices cuja coverage
  **exacta** contém o caractere.
- `SystemWorld::candidates_for_char` é responsável por garantir que a coverage
  de cada `FontInfo` consultado esteja preenchida (lazy + cache).
- O shaper pode confiar no filtro de coverage; a verificação repetida por
  `face_covers_char` durante o fallback deixa de ser necessária para
  candidatos do `FontBook`.

## Mmap em FontSlot — P937

**Problema:** `std::fs::read` copia o ficheiro inteiro para memória do
processo. Quando o shaping carrega várias faces (fallback CJK/emoji), a
soma dessas cópias é cara e enche a memória do processo. O vanilla usa mmap
via `fontdb`/`fontdb-0.23.0`; P935 mediu ~50 ms para extrair coverage de
~1112 faces por mmap vs ~4 s por `std::fs::read`.

**Solução:** `FontSlot` cria/mantém um `memmap2::Mmap` do ficheiro de fonte
(lazy, na primeira vez que os bytes são necessários). Tanto `source_bytes()`
(como usado por `pair_slots_with_book` e por `extract_coverage`) como
`get()` usam o mesmo mmap.

```rust
mmap: OnceLock<Option<Arc<memmap2::Mmap>>>
```

- `source_bytes()`: devolve `Some(Cow::Borrowed(&mmap))` (ou `&embedded`).
- `get()`:
  1. Se `embedded` presente, usa esses bytes.
  2. Senão, obtém o mmap; para fontes simples constrói `Font::Mmap(Arc<Mmap>)`.
  3. Para `.ttc/.otc`, `extract_collection_face` opera sobre o slice do mmap e
     devolve `Font::Vec(bytes_extraídos)` (a face simples resultante é um
     ficheiro independente).

### Partilha entre faces do mesmo `.ttc`

Com mmap, a partilha explícita de `Vec<u8>` entre faces do mesmo ficheiro
(P875) deixa de ser necessária: todas as faces de um `.ttc` partilham
automaticamente o mesmo mapa de memória do kernel (ou mapas separados que
apontam para as mesmas páginas físicas). O `mmap` é criado uma vez por
`FontSlot`, mas slots do mesmo path podem abrir o ficheiro separadamente —
o overhead é mínimo comparado com a cópia anterior. Se medições mostrarem
que abrir o mesmo `.ttc` várias vezes ainda é gargalo, reconsidera-se um
cache por path.

### Dependência

Adicionar `memmap2` a `03_infra/Cargo.toml` (versão compatível com a usada
pelo `fontdb-0.23.0`, tipicamente `0.9`).

## P1137-C001 — inventário para `typst fonts`

`inventory_fonts(font_paths, include_system)` reutiliza exactamente os mesmos
loaders de fontes embutidas, sistema e paths explícitos usados pelo World. O
retorno é um DTO L3 com `family`, `path`, `index`, `style`, `weight`, `stretch`
e `embedded`; tipos de `fontdb`/`ttf-parser` não escapam. Entradas inválidas são
descartadas, famílias/variantes são ordenadas deterministicamente e duplicados
idênticos são removidos. L3 não formata stdout.

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-03-26 | Criação — Passo 11: `FontSlot`, `discover_fonts` (lazy I/O) | `fonts.rs` |
| 2026-04-12 | Restauro — expandido: `font_info_from_bytes` (ADR-0022), `build_font_book`, suporte `.ttc`, relação com SystemWorld | `fonts.md` |
| 2026-07-22 | P838 — `flags.serif` via panose OS/2 (critério vanilla `[2, 2..=10, ..]`), necessário ao scoring de `FontBook::select_fallback`; `extract_collection_face` reescreve os offsets do directório de tabelas (eram absolutos à colecção — faces .ttc ficavam incarregáveis) | `fonts.md`, `fonts.rs` |
| 2026-07-22 | P839 — achados #25–#28 de P831: família só do ID1 com `typographic_family` (ID16 ignorado), `decode_mac_roman` para registos Macintosh, `infer_style` pelo full name (sem `is_italic()`); `build_font_book` substituído por `pair_slots_with_book` (slots sem info descartados — índices book↔slots sempre alinhados, como no vanilla) | `fonts.md`, `fonts.rs`, `world.rs`, `integration_tests.rs`, fixtures `p839-*.ttf` |
| 2026-07-22 | P840 — achados #29/#30 de P831: `find_exception` + tabela de exceções portada integralmente do vanilla (`exceptions.rs:46-342`), aplicada em `font_info_from_bytes` por PostScript name (família/estilo/peso/stretch da exceção prevalecem) | `fonts.md`, `fonts.rs`, fixture `p840-fandolhei-bold.ttf` |
| 2026-07-23 | P875 — `Coverage` Unicode em `FontInfo` (bitmap por bloco de 256); partilha lazy de bytes entre faces do mesmo `.ttc` | `fonts.md`, `fonts.rs`, `font_book.md`, `shaper.rs` |
| 2026-07-23 | P880 — `font_info_from_bytes` deixa `coverage` vazio; cobertura Unicode computada lazy por `World::candidates_for_char` com cache em `SystemWorld` | `fonts.md`, `fonts.rs`, `world.rs`, `font_book.md`, `shaper.rs`, `font_metrics.rs`, `contracts/world.md` |
| 2026-07-31 | P937 — coverage exacta eager (runs de codepoints) + mmap em `FontSlot`; remove partilha explícita P875 e cache lazy P880/P927 | `fonts.md`, `entities/font_book.md`, `fontdb.md`, `system-world.md`, `03_infra/src/fonts.rs`, `03_infra/src/world.rs`, `03_infra/Cargo.toml` |
| 2026-07-31 | P938 — coverage exacta volta a ser lazy; `font_info_from_bytes` deixa `coverage` vazio; `SystemWorld` preenche e cacheia por índice | `fonts.md`, `system-world.md`, `wiring.md`, `03_infra/src/fonts.rs`, `03_infra/src/world.rs`, `04_wiring/src/main.rs` |
| 2026-08-23 | P1137-C001 — DTO determinístico de inventário para o comando `typst fonts` | `fonts.md`, `shell/fonts-command.md`, `fonts.rs` |
