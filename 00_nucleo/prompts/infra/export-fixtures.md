# Prompt L0 — `infra/export-fixtures` — Corpus canónico de snapshot binário
Hash do Código: 22983866

**Camada**: L3 (apoio — fixtures de teste)
**Diretório alvo**: `03_infra/fixtures/p307b/`
**Criado em**: 2026-05-19 (Passo 307a)
**ADRs relevantes**: ADR-0098 (SSoT, anotada P307a), ADR-0100 (coesão L3, PROPOSTA P307a), ADR-0037 (coesão por domínio L1), ADR-0033 (paridade funcional observable)

---

## Contexto e Objetivo

Pós-P306, `03_infra/src/export.rs` foi avaliado como candidato à
decomposição (ADR-0100 PROPOSTA; ver `diagnostico-export-passo-307a.md`).
A decomposição quebra o invariante textual da ADR-0098 ("hash de
`export.rs` preservado bit-exact"), por construção: o ficheiro deixa
de existir como unidade.

**Substituição do invariante** (per ADR-0098 §"Evolução pós-P307"):

> Em vez de hash textual de um único `.rs`, validar que os **bytes
> PDF gerados para um corpus canónico de inputs** permanecem
> bit-exact pós-decomposição.

Este L0 documenta o corpus canónico, o critério de escolha de cada
fixture, e o protocolo de regeneração se a invariante mudar
intencionalmente em passo futuro.

---

## Restrições Estruturais

- **Camada alvo L3 (apoio)**: fixtures são inputs/outputs para
  testes; não são código L3 production. Não têm `@prompt-hash`
  textual nos ficheiros `.typ` (não é mecanismo aplicável a
  ficheiros não-Rust); a rastreabilidade é assegurada por:
  - Este prompt L0 (cabeçalho hash propagado por
    `crystalline-lint`).
  - Referência cruzada ao diagnóstico imutável
    `diagnostico-export-passo-307a.md` §6.
  - `MANIFEST.md` no directório que lista bytes esperados + md5.

- **Não é teste**: o teste consumidor vive em
  `03_infra/src/integration_tests.rs` (módulo `p307b_snapshot`,
  criado em P307b.1).

- **Não é parity corpus** (`lab/parity/corpus/`): este corpus é
  específico do P307. Existe paralelo conceptual mas propósitos
  são distintos:
  - `lab/parity/corpus/`: paridade vanilla vs cristalino.
  - `03_infra/fixtures/p307b/`: paridade cristalino vs cristalino
    (pré vs pós decomposição interna).

---

## Estrutura

```
03_infra/fixtures/p307b/
    sources/             # Inputs .typ + assets (1 .jpg fixture)
        01-markup-plain.typ
        02-markup-heading.typ
        03-text-styling.typ
        04-shapes.typ
        05-gradient-linear.typ
        06-gradient-conic.typ
        07-multi-feature.typ
        08-image-jpeg.typ + tiny.jpg
        09-cidfont.typ
    reference/           # Outputs .pdf de baseline (gerados em P307a.2)
        01-markup-plain.pdf
        02-markup-heading.pdf
        ...
        09-cidfont.pdf
    MANIFEST.md          # Documentação operacional: bytes, md5, cluster cobertura
```

---

## Inventário de fixtures

Cada fixture exercita primariamente um cluster identificado no
diagnóstico:

| # | Fixture | Cluster primário | Determinismo |
|---:|---|---|---|
| 01 | `01-markup-plain.typ` | API + Helvetica + escape_pdf_string | ✓ |
| 02 | `02-markup-heading.typ` | Helvetica + emit_text | ✓ |
| 03 | `03-text-styling.typ` | Bold/italic via Styled → /F2/F3 | ✓ |
| 04 | `04-shapes.typ` | Shape kinds (rect, circle) + paint solid | ✓ |
| 05 | `05-gradient-linear.typ` | Linear scan + pattern_resources (P263) | ✓ |
| 06 | `06-gradient-conic.typ` | Conic Coons (P272) + bezier_control_points | ✓ |
| 07 | `07-multi-feature.typ` | Integração (heading + gradient + multi-página) | ✓ |
| 08 | `08-image-jpeg.typ` | JPEG XObject + dedup + zlib | ✓ |
| 09 | `09-cidfont.typ` | CIDFont + Type0 + Identity-H + ToUnicode CMap | ✓ |

**Cobertura por cluster** (per diagnóstico §2):

| Cluster | LOC produção | Coberto por | Status |
|---|---:|---|---|
| PageContext + emit (P281) | 668 | 01, 02, 03 | ✓ |
| Builder | 630 | todos | ✓ |
| Conic Coons (P272) | 445 | 06 | ✓ |
| Gradient Linear (P263) | 390 | 05, 07 | ✓ |
| Imagens | 311 | **08** | ✓ |
| CIDFont helpers | 118 | **09** | ✓ |
| Gradient CMYK (P270.2) | 83 | (indirecto via Builder) | ⚠ parcial |
| Gradient relative (P273) | 53 | (indirecto via Builder) | ⚠ parcial |
| Adaptive N (P274) | 64 | (indirecto via Builder) | ⚠ parcial |

**Cobertura efectiva directa**: 6/9 clusters. Os 3 restantes
(CMYK/relative/adaptive) são features menores (200 LOC totais) com
testes inline em `export.rs::tests` que migram para
`export/tests.rs` em P307b.1 — cobertura redundante via snapshot
binário seria sub-prioritária.

---

## Critério de escolha (instrução de manutenção)

Quando adicionar/remover fixtures pós-P307:

1. **Adicionar fixture novo só se**:
   - Cobre cluster L3 novo introduzido em passo posterior; OU
   - Reflecte regressão histórica que P307b.1 deixaria escapar.
2. **Remover fixture** apenas se:
   - O cluster que exercita foi removido do código L3; OU
   - Outro fixture cobre o mesmo cluster de forma equivalente
     ou superior (decisão registada no relatório do passo).
3. **Regenerar bytes de referência** apenas se a invariante muda
   intencionalmente (passo dedicado documenta a razão).

---

## Determinismo

Pré-requisito validado empiricamente em P307a.2: para cada fixture,
2 invocações consecutivas de `typst <src> <dst>` produzem bytes
idênticos (verificação md5 documentada em MANIFEST.md).

**Garantias internas que asseguram determinismo**:

- `PdfBuilder` não usa `SystemTime` nem RNG (verificável via grep).
- `flate2::Compression::default()` é determinístico em toolchain
  fixa.
- Ordering em `HashMap` interno do exporter não afecta output
  textual final (objects emitidos em ordem fixa por ID).
- Discovery de fontes via `--font-path` é determinístico em FS
  estável.

**Quebras conhecidas de determinismo a evitar**:

- Não usar `SystemTime` em `export/`.
- Não usar `rand` ou similar.
- Não introduzir parallel iteration sobre estruturas com ordem
  emergente.

---

## Protocolo de regeneração

Quando o invariante muda intencionalmente:

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
cargo build --bin typst
FONTDIR=lab/krilla-reference/assets/fonts
for f in 03_infra/fixtures/p307b/sources/*.typ; do
  bn=$(basename "$f" .typ)
  # Fixture 09 requer --font-path
  if [ "$bn" = "09-cidfont" ]; then
    ./target/debug/typst --font-path "$FONTDIR" "$f" "03_infra/fixtures/p307b/reference/${bn}.pdf"
  else
    ./target/debug/typst "$f" "03_infra/fixtures/p307b/reference/${bn}.pdf"
  fi
done
md5sum 03_infra/fixtures/p307b/reference/*.pdf
```

A mudança deve ser justificada num passo dedicado (e.g. "P3XX
altera escape_pdf_string para suportar caracteres Unicode acima
de \\x7F — regenera fixtures 01/02/07/09").

---

## Uso em P307b.1

Test harness consumidor (a criar em P307b.1):

```rust
// 03_infra/src/integration_tests.rs (extensão)
#[cfg(test)]
mod p307b_snapshot {
    use super::*;

    #[test]
    fn p307b_snapshot_01_markup_plain() {
        let pre = include_bytes!("../fixtures/p307b/reference/01-markup-plain.pdf");
        let post = compile_to_pdf_from_path("../fixtures/p307b/sources/01-markup-plain.typ");
        assert_eq!(&post[..], &pre[..], "PDF binário regrediu: 01-markup-plain");
    }
    // ... 9 testes análogos
}
```

Cada teste lê bytes de referência embebidos no binário via
`include_bytes!` e compara byte-a-byte. Falha → diff bin diff
revela onde divergiu.

---

## Não-objectivos

- **Não** valida correctude visual do PDF (só bit-exactness vs
  baseline P306).
- **Não** mede performance (compile time, render time).
- **Não** garante compatibilidade entre toolchains Rust diferentes
  (pinned ao rustc actual via `rust-toolchain.toml`).
- **Não** substitui testes unitários inline em `export/tests.rs`
  (que cobrem caminhos específicos com asserts sobre conteúdo
  textual do PDF).

---

## Estado V7 transiente

Este L0 dispara warning V7 (PromptOrphan) em P307a porque
nenhum ficheiro `.rs` em L1-L4 declara `@prompt
00_nucleo/prompts/infra/export-fixtures.md`. Isto é **estrutural
e esperado**:

- A regra V7 só varre `.rs` por header `@prompt`.
- Fixtures são `.typ`/`.pdf`/MANIFEST.md (não `.rs`); não têm header.
- O **consumidor `.rs`** dos fixtures será criado em P307b.1 —
  extensão de `03_infra/src/integration_tests.rs` com `mod
  p307b_snapshot` declarando `@prompt
  00_nucleo/prompts/infra/export-fixtures.md` no header.

Pós-P307b.1, V7 fecha-se automaticamente. Até lá, warning é
sinal correcto de que o trabalho consumidor está pendente.

Alternativa rejeitada em P307a: criar stub `.rs` com header
referencia mas sem corpo. Seria gaming do linter para silenciar
um sinal legítimo ("este L0 ainda não foi materializado").

---

## Cross-references

- `00_nucleo/diagnosticos/diagnostico-export-passo-307a.md` §6:
  identificação inicial do corpus.
- `00_nucleo/adr/typst-adr-0100-coesao-por-dominio-l3.md`:
  motiva existência destes fixtures.
- `00_nucleo/adr/typst-adr-0098-...md` §"Evolução pós-P307":
  formaliza substituição proxy textual → binário.
- `03_infra/fixtures/p307b/MANIFEST.md`: tabela operacional
  bytes/md5 actualizada.
- `00_nucleo/materialization/typst-passo-307.md` §6: contexto
  inicial sobre snapshot binário.
