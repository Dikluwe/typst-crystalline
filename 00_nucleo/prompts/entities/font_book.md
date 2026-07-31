# Prompt L0 — `entities/font_book` — `FontBook`, `FontInfo` e cobertura Unicode exacta
Hash do Código: ea4b6fcd

**Camada**: L1  
**Ficheiros alvo**: `01_core/src/entities/font_book.rs`, `01_core/src/entities/world_types.rs`  
**Criado em**: 2026-07-31 (P937)  
**ADRs relevantes**: ADR-0022 (`FontInfo` — L1 recebe apenas campos primitivos), ADR-0107 (paridade com a linguagem), ADR-0123 (fórmula/estrutura de dados física portável do vanilla quando justificada)

---

## Contexto

O `FontBook` (L1) é o catálogo de metadados de fontes disponíveis. É consultado
pelo shaper para resolver famílias declaradas e para o fallback global. A
qualidade do fallback depende da precisão da cobertura Unicode guardada em
cada `FontInfo`.

Até P880 o `Coverage` era um bitmap por bloco de 256 codepoints: barato de
construir, mas com falsos positivos reais (ex.: uma fonte que cobre o bloco
grego U+0300–U+03FF mas não contém α U+03B1). Esses falsos positivos forçavam
o shaper a abrir faces durante o fallback para confirmar candidatos, o que se
tornou o gargalo em CJK/emoji (P936).

P936 confirmou a estrutura do vanilla: coverage **exacta** (runs de codepoints),
extraída eager no arranque sobre mmap barato, elimina completamente a
necessidade de abrir faces durante `select_fallback`.

---

## `Coverage` — conjunto exacto de codepoints

Portar a representação do vanilla (`typst-library/src/text/font/info.rs:269-318`):

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Coverage(Vec<u32>);
```

O `Vec<u32>` codifica runs alternadas de codepoints **fora** e **dentro** do
conjunto, começando por "fora" desde `0`. Para cada codepoint `c`, se
`c == next` estende a run dentro actual; senão empurra `c - next` (fora) e
`1` (dentro), e `next = c + 1`:

- Exemplo do vanilla: `{2,3,4,9,10,11,15,18,19}` → `[2, 3, 4, 3, 3, 1, 2, 2]`.

### Construtores

```rust
impl Coverage {
    /// Coverage vazia.
    pub fn new() -> Self;

    /// Constrói a partir de um iterador de codepoints.
    /// Ordena, remove duplicados e codifica em runs.
    pub fn from_codepoints(codepoints: impl IntoIterator<Item = u32>) -> Self;
}
```

### Métodos

```rust
impl Coverage {
    /// Verifica se o codepoint está coberto. O(log n) via busca binária nas runs.
    pub fn contains(&self, codepoint: u32) -> bool;

    /// True se não cobre nenhum codepoint.
    pub fn is_empty(&self) -> bool;

    /// Itera todos os codepoints cobertos (útil para testes e debug).
    pub fn iter(&self) -> impl Iterator<Item = u32>;
}
```

### Regras

- Codepoints acima de `u32::MAX` não existem; a representação suporta todo o
  espaço Unicode (`0..=0x10FFFF`).
- `contains` deve ser **exacta**: `false` para codepoints que não estão na
  tabela `cmap` da fonte. Sem falsos positivos.
- A estrutura é imutável depois de construída. Clone é O(1) (Arc interno ou Vec
  pequeno).

### Proveniência

A representação por runs e o algoritmo `contains` são portados do vanilla
(`info.rs:269-318`). A decisão de reaproveitar a estrutura segue ADR-0123: é a
fórmula/estrutura de dados que resolve o problema físico real (falsos positivos
de bitmap), não mecânica a reinventar livremente.

---

## `FontInfo`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontInfo {
    pub family: String,
    pub variant: FontVariant,
    pub flags: FontFlags,
    pub coverage: Coverage,  // exacta a partir de P937
}
```

- `coverage` passa a ser preenchido **eager** por L3 durante a construção do
  `FontBook` (ver `infra/fonts.md`).
- L1 continua a não tocar em bytes de fonte; `FontInfo` contém apenas tipos
  primitivos + `Coverage`.

---

## `FontBook::candidates_for_char`

```rust
impl FontBook {
    pub fn candidates_for_char(&self, c: char) -> impl Iterator<Item = usize> + '_ {
        let codepoint = c as u32;
        self.infos.iter().enumerate()
            .filter(move |(_, info)| info.coverage.contains(codepoint))
            .map(|(index, _)| index)
    }
}
```

- Devolve os índices cujo `coverage` **exacto** contém o caractere.
- O(n) no número de fontes; como a coverage já está materializada e a
  consulta é O(log n), o custo é aceitável e comparável ao vanilla.
- Não abre faces, não lê disco, não usa `ttf_parser`.

---

## `Font` (L1) — suporte a bytes mmap-backed

Até P937 `Font` era `Font(Vec<u8>)`. Para evitar copiar bytes de um mmap para
um `Vec<u8>` sempre que uma fonte do disco é carregada no shaping, `Font` passa
a ser um enum que pode conter ou um `Arc<Vec<u8>>` (embutidas / faces extraídas
de `.ttc`) ou um `Arc<Mmap>` (fontes simples do disco).

```rust
#[derive(Clone)]
pub enum Font {
    Vec(Arc<Vec<u8>>),
    Mmap(Arc<memmap2::Mmap>),
}
```

**Nota:** `memmap2::Mmap` é um tipo de L3. Para manter L1 pura (zero I/O),
`Font` não cria mmap — apenas recebe o `Arc<Mmap>` já construído por L3. A
variante `Vec` cobre todos os casos que não vêm directamente de um ficheiro
mmap-ado (fontes embutidas, faces reconstruídas de coleções).

### Interface pública

```rust
impl Font {
    /// Constrói a partir de um Vec<u8> (fontes embutidas, faces extraídas).
    pub fn from_data(data: Vec<u8>) -> Self;

    /// Constrói a partir de um mmap já criado por L3.
    pub fn from_mmap(mmap: Arc<memmap2::Mmap>) -> Self;

    /// Devolve os bytes como slice.
    pub fn as_slice(&self) -> &[u8];
}
```

### Traits

- `Debug`, `PartialEq`, `Eq`, `Hash`: implementados manualmente comparando
  `as_slice()`. Duas fontes são iguais se os seus bytes forem iguais,
  independentemente da variante interna.
- `Clone`: clone O(1) do Arc interno.

### Restrição de pureza L1

L1 não importa `memmap2` nem abre ficheiros. A variante `Mmap` existe apenas
para que L3 possa passar uma referência estável de bytes sem cópia. A criação
de `Mmap` fica estritamente em `03_infra/src/fonts.rs`.

---

## Critérios de Verificação

```
// Coverage exacta
Coverage::from_codepoints([0x03B1]).contains(0x03B1) == true
Coverage::from_codepoints([0x03B2]).contains(0x03B1) == false
Coverage::new().is_empty() == true
Coverage::from_codepoints([1,2,3]).iter().collect() == [1,2,3]

// FontBook candidates_for_char com coverage exacta
book.infos()[i].coverage = Coverage::from_codepoints([0x41, 0x03B1])
book.candidates_for_char('A').collect() == [i]
book.candidates_for_char('α').collect() == [i]
book.candidates_for_char('你').collect() == []

// Font
Font::from_data(vec![1,2,3]).as_slice() == [1,2,3]
// (variante Mmap testada em L3, ver infra/fonts.md)
```

---

## Relação com Outros Módulos

| Módulo | Como consome |
|--------|-------------|
| `infra/fonts.rs` (L3) | Constrói `FontInfo` + `Coverage` exacta a partir de bytes; cria `Font::Mmap` ou `Font::Vec` |
| `infra/shaper.rs` (L3) | Usa `Font::as_slice()` para `ttf_parser::Face::parse` e `rustybuzz::Face::from_slice` |
| `infra/font_metrics.rs` (L3) | Idem |
| `infra/fontdb.rs` (L3) | Usa `db.with_face_data` para extrair `FontInfo` + coverage no arranque |
| `SystemWorld` (L3) | Delega `candidates_for_char` a `FontBook::candidates_for_char` |
| `World` trait (L1) | Mantém a assinatura `candidates_for_char(&self, c: char) -> Vec<usize>` |

---

## Histórico de Revisões

| Data | Motivo | Ficheiros afetados |
|------|--------|--------------------|
| 2026-07-31 | P937 — cobertura Unicode exacta (runs) e `Font` mmap-backed | `entities/font_book.md`, `01_core/src/entities/font_book.rs`, `01_core/src/entities/world_types.rs` |
