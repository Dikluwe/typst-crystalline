# Relatório de Paridade — P753

**Passo:** 753  
**Data:** 2026-07-14  
**Foco:** Fonte por defeito do cristalino deve bater com a do vanilla (`Libertinus Serif`).  
**Hash do commit:** 25ffa4629

---

## Resumo Executivo

P752 encontrou que o cristalino usava `Liberation Serif` como fonte por defeito
enquanto o vanilla usa `Libertinus Serif`. P753 confirmou que a causa era a
indisponibilidade de `Libertinus Serif` no ambiente de testes (não instalada no
sistema) e corrigiu o problema embutindo o mesmo conjunto de fontes que o
vanilla CLI usa via `typst-assets`.

A fonte por defeito do cristalino passou a ser `Libertinus Serif`, batendo
exactamente com o vanilla. A validação confirma que o PDF de saída embute
`Libertinus Serif` e que o resíduo de posicionamento de ~0.03pt reportado em
P752 desapareceu.

---

## Sonda

### Lista de fontes por defeito do vanilla

Confirmada no código vanilla (`lab/typst-original/crates/typst-library/src/text/mod.rs`):

- Fonte por defeito de `text`: `Libertinus Serif`.
- Fontes embutidas pelo CLI via `typst-assets`:
  - `LibertinusSerif-Regular/Bold/Italic/BoldItalic/Semibold/SemiboldItalic`
  - `NewCMMath-Bold/Book/Regular`
  - `NewCM10-Regular/Bold/Italic/BoldItalic`
  - `DejaVuSansMono-Bold/BoldOblique/Oblique/Regular`

### Disponibilidade de `Libertinus Serif` no ambiente

- `fc-list | grep -i libertinus` — nenhum resultado (não instalada no sistema).
- Ficheiros OTF disponíveis na cache do cargo provenientes de `typst-assets`
  (`~/.cargo/git/checkouts/typst-assets-...`) e em
  `lab/krilla-reference/assets/fonts/LibertinusSerif-Regular.otf`.
- Conclusão: a fonte existe mas não está registada no sistema; o cristalino
  precisa de a carregar explicitamente.

### Lista actual do cristalino

- Default em `01_core/src/entities/style_chain.rs:621`: `Liberation Serif`.
- Fallbacks serif em `03_infra/src/fallback_fonts.rs:20-25`:
  `Liberation Serif`, `DejaVu Serif`, `Bitstream Vera Serif`, `FreeSerif`.
- Razão histórica (P558): `Liberation Serif` substituiu `FreeSerif` para evitar
  acentos trocados causados pela decomposição base + mark do `FreeSerif`.

---

## Implementação

### Mudanças efectuadas

1. **Fontes embutidas via `typst-assets`** (`03_infra/src/embedded_fonts.rs`)
   - Nova função `load_embedded_fonts()` que itera sobre
     `typst_assets::fonts()` e cria `FontSlot` a partir dos bytes embutidos.

2. **`FontSlot` suporta bytes embutidos** (`03_infra/src/fonts.rs`)
   - Adicionado campo `embedded: Option<Vec<u8>>` e construtor
     `FontSlot::new_embedded(path, data)`.
   - `get()` usa os bytes embutidos quando presentes; caso contrário, lê do
     disco.
   - `build_font_book()` também consulta `embedded` antes de reler do disco.

3. **`SystemWorld` carrega embutidas por defeito** (`03_infra/src/world.rs`)
   - Novo builder `with_embedded_fonts()`.
   - `with_fonts_and_system()` agora carrega: (1) embutidas; (2) sistema;
     (3) projecto (`--font-path`).

4. **Fonte por defeito `Libertinus Serif`**
   - `01_core/src/entities/style_chain.rs` — alterado o default de
     `Liberation Serif` para `Libertinus Serif`.
   - Comentários actualizados em `01_core/src/rules/layout/text.rs` e
     `03_infra/src/shaper.rs`.

5. **Dependências**
   - `Cargo.toml` (workspace) e `03_infra/Cargo.toml` — adicionada
     `typst-assets` com a mesma revisão git do vanilla 0.15.0
     (`c0ae970`).

6. **Prompts L0 actualizados**
   - `00_nucleo/prompts/entities/style_chain.md` — decisão P753.
   - `00_nucleo/prompts/infra/system-world.md` — builders de fontes embutidas.
   - `00_nucleo/prompts/infra/embedded_fonts.md` — novo prompt L0 para o
     módulo de fontes embutidas.

### Ficheiros alterados

```text
Cargo.toml
03_infra/Cargo.toml
01_core/src/entities/style_chain.rs
01_core/src/rules/layout/text.rs
03_infra/src/fonts.rs
03_infra/src/lib.rs
03_infra/src/shaper.rs
03_infra/src/world.rs
03_infra/src/embedded_fonts.rs       (novo)
00_nucleo/prompts/entities/style_chain.md
00_nucleo/prompts/infra/system-world.md
00_nucleo/prompts/infra/embedded_fonts.md (novo)
```

---

## Validação

### Comparação de PDFs (documento mínimo `X`)

| Aspecto | Vanilla | Cristalino (após P753) |
|---------|---------|------------------------|
| BaseFont | `PWQVDX+LibertinusSerif-Regular-Identity-H` | `AAAAAA+CrystallineFont` |
| Família real embutida | `Libertinus Serif` | `Libertinus Serif` |
| Posição Y do texto | `763.78564` pt | `763.785` pt |
| Diferença Y | — | ~0.0006 pt (anteriormente ~0.03 pt em P752) |

A família real foi confirmada extraindo o stream de fonte do PDF cristalino e
consultando a tabela `name` via `fontTools`: `Family: Libertinus Serif`.

### Testes

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram** (incluindo o novo teste
`p753_embedded_fonts_contain_libertinus_serif`).

### Linter

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

---

## Decisões e Notas

- A paridade é ao nível da **linguagem** (semântica da fonte por defeito),
  não ao nível mecânico: o vanilla gera `/BaseFont /PWQVDX+LibertinusSerif-Regular`
  (CFF/Type 0C), enquanto o cristalino gera `/BaseFont /AAAAAA+CrystallineFont`
  (TrueType/CIDFontType2). A forma do exportador difere, mas a fonte real
  embutida é a mesma (ADR-0107).
- A ordem de carregamento de fontes é agora: embutidas → sistema → projecto.
  Isto preserva o comportamento de `--font-path` enquanto garante que o
  conjunto vanilla-like está sempre presente.
- A salvaguarda de fallback para `DEFAULT_FALLBACK_FONTS_SERIF` (P558) mantém-se:
  se `Libertinus Serif` não resolver, o shaper tenta `Liberation Serif`,
  `DejaVu Serif`, `Bitstream Vera Serif` e `FreeSerif`.

---

## Critérios de Fecho

- [x] Sonda completa, cadeia de fallback confirmada, disponibilidade confirmada.
- [x] Fonte por defeito corrigida para bater com o vanilla (`Libertinus Serif`).
- [x] Resíduo de ~0.03pt (P752) verificado — eliminado (diferença residual
      ~0.0006 pt, dentro da tolerância de arredondamento).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p753.md`.
