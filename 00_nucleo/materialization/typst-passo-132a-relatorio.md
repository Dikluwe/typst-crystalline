# Passo 132A — Relatório (diagnóstico `FontList` + ADR-0053 PROPOSTO)

**Data**: 2026-04-24
**Precondição**: Passo 131B encerrado; 1069 total tests; zero
violations; 52 ADRs activas (ADR-0052 `IMPLEMENTADO`); 11 DEBTs
abertos.
**Natureza**: passo L0-puro. **Sem código**. **Sem testes**.
Segunda aplicação do padrão "diagnóstico separado +
materialização" após 131A.
**ADR**: **ADR-0053** criada em status `PROPOSTO`.

---

## Sumário

Dois artefactos produzidos:

1. **Diagnóstico** em
   `00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md`
   com os 7 itens mínimos de ADR-0034 + decisão estratégica
   sobre `regex` + enumeração completa de impacto em testes.

2. **ADR proposta** em
   `00_nucleo/adr/typst-adr-0053-font-tipo-composto.md` em
   status `PROPOSTO`, 9 decisões numeradas, 5 alternativas.

**Zero código tocado**. **1069 total tests preservado**. Lint
zero violations.

---

## 132A.1 — Inventário vanilla

Leitura de `lab/typst-original/.../text/mod.rs:835-976`:

- `FontFamily { name: EcoString (lowercased), covers: Option<Covers> }`.
- `Covers { LatinInCjk, Regex(regex::Regex) }` — keyword +
  regex externo.
- `FontList(Vec<FontFamily>)` — newtype sobre Vec, non-empty
  obrigatório.
- `cast!` macros aceitam 3 formas: string, array, dict.
- Regex validado por AST (apenas classes + dot + literal).

## 132A.2 — Inventário cristalino

Findings críticos:

- **Nenhum tipo com nome colidido** em L1.
- **`regex` NÃO autorizada**: não está em `[l1_allowed_external]`
  do `crystalline.toml` nem declarada no workspace `Cargo.toml`.
  Lista autorizada: `thiserror`, `comemo`, `unicode_*`,
  `rustc_hash`, `time`, `indexmap`, `ecow`.
- Tipos relacionados já em L1: `FontWeight`/`FontStretch` em
  `font_book.rs` (catálogo, domínio diferente).

## 132A.3 — Inventário pool DEBT-49 e canary

**L3 tests afectados** (3):
- `debt49_set_text_font_emite_warning:2180` — input `font:"Arial"`.
- `debt49_set_text_multiplas_propriedades_desconhecidas:2229` —
  input `font:"A", alignment, stroke`.
- `debt49_dedup_warnings_identicos:2285+2295` — input `font:"A"`
  ×2.

**Pool actual**: `font, alignment, stroke` (pós-Passo 130).
**Substituto proposto**: `hyphenate` (vanilla: `Smart<bool>`;
cristalino sem captura). `#set text(hyphenate: true)` é
parseável, não colide com testes existentes.

## 132A.4 — Inventário L1 canary

**5 tests** em `01_core/src/rules/eval/tests.rs`:
- `eval_set_text_font_canary_passo_126`
- `eval_set_text_font_canary_passo_127`
- `eval_set_text_font_canary_passo_128`
- `eval_set_text_font_canary_passo_129`
- `eval_set_text_font_canary_passo_131b`

Todos usam `#set text(font: "X")` + assert warning `'font'`.

**L4 tests afectados** (2):
- `cli_sucesso_com_warning:65`.
- `disciplina_warnings_antes_de_errors:591`.

**Total canary migration**: **10 tests** (5 L1 + 3 L3 + 2 L4).

## 132A.5 — Diagnóstico escrito

Cobre os 7 itens mínimos ADR-0034 + 6 itens adicionais. Decisão
estratégica-chave documentada:

### Regex: Opção B (deferir `covers` por completo)

Três opções avaliadas:
- **A**: autorizar `regex` em L1 (com `regex_syntax` + 4-6 deps
  transitivas). Descartada — feature sem consumer.
- **B**: deferir `covers` inteiro, string + array apenas.
  **Escolhida**.
- **C**: suportar `LatinInCjk` keyword mas não regex.
  Descartada — parcialidade confusa.

**Resultado**: paridade com vanilla em 2 de 3 formas (string +
array). Dict form rejeitada com erro claro:
`"dict form of font not yet supported — use string or array of
strings"`.

## 132A.6 — ADR-0053 proposta

Status: **`PROPOSTO`**. 9 decisões numeradas. 5 alternativas
em tabela. Consequências positivas/negativas/neutras. Plano de
materialização em 8 steps. Referências a 7 ADRs/passos
relevantes.

### Divergência ADR-0033 documentada

**Primeira rejeição explícita de form suportada vanilla**.
Trade-off: rejeitar é menos grave que aceitar-sem-processar
(como 130 fazia com lang). Mensagem de erro clara guia
utilizador para forms suportadas.

---

## Verificação

1. ✓ Diagnóstico existe com 7 itens ADR-0034 preenchidos.
2. ✓ ADR-0053 existe com status `PROPOSTO`.
3. ✓ Decisão sobre `regex` documentada (Opção B — deferir).
4. ✓ Lista concreta de 10 testes canary identificada (5 L1 +
   3 L3 + 2 L4).
5. ✓ Pool DEBT-49 L3 com substituto `hyphenate` proposto.
6. ✓ Zero ficheiros L1/L2/L3/L4 tocados (git status confirma).
7. ✓ `cargo test --workspace` inalterado: 1069 total, 6
   ignorados.
8. ✓ `crystalline-lint` zero violations.

---

## Ficheiros criados

| Ficheiro | Natureza | Linhas |
|----------|----------|-------:|
| `00_nucleo/diagnosticos/diagnostico-font-list-passo-132a.md` | L0 diagnóstico ADR-0034 | ~380 |
| `00_nucleo/adr/typst-adr-0053-font-tipo-composto.md` | L0 ADR PROPOSTO | ~180 |
| `00_nucleo/materialization/typst-passo-132a-relatorio.md` | L0 relatório (este) | ~170 |

**Nenhum** ficheiro de código ou teste tocado.

---

## Lições

1. **`regex` não autorizado é decisão disciplinadora**: força
   escopo honesto. Autorizar seria 1 linha em toml + 1 ADR,
   mas traz 4-6 deps transitivas e abre precedente. Deferir
   mantém L1 limpo até haver consumer real.

2. **"Rejeitar explicitamente" > "aceitar silenciosamente"**:
   ADR-0033 prefere paridade mas, quando não é viável,
   mensagem de erro clara é melhor que captura raw sem
   processar. Utilizador recebe feedback preciso.

3. **Migração de canary em 10 testes é custo real**: 5 L1 + 3
   L3 + 2 L4 é mais do que passos anteriores. Enumerar
   antecipadamente permite 132B correr sem surpresas.

4. **Segunda aplicação do padrão 131A/B valida disciplina**:
   diagnóstico 132A reutiliza template 131A; ADR-0053 clona
   estrutura 0052. Produtividade em L0 ganha com repetição —
   próximas materializações (Stroke, Region, Dir) aceleram.

5. **Trade-off `covers`/`regex` resolvido limpo**: ao invés
   de meia-implementação (Option C), decisão foi "forma
   completa ou nenhuma". Quando consumer de shaping chegar
   num passo futuro, reabre-se ADR-0054 ou adição a 0053.
   Até lá, estado actual é **consistente**.

6. **Diagnóstico revelou complexidade escondida**: à primeira
   vista `font` parecia igual a `lang`; inventário revelou
   3 forms, tipo agregador, coverage filtering, regex AST
   validation, dependência externa não autorizada. 132A
   poupou horas em 132B.

---

## Estado pós-Passo 132A

### Ficheiros L0

- Diagnóstico `FontList` completo.
- ADR-0053 `PROPOSTO`.
- Roadmap DEBT-1 continua: 132A → 132B → 133 → 134 → 135.

### Código L1/L2/L3/L4

**Inalterado**. `font` continua a emitir warning (sem captura).

### Divergência ADR-0033

**Documentada** mas **não resolvida** neste passo. Resolução
parcial em 132B (string + array aceites; dict rejeitado).

### Pressão sobre regex

Descarregada. Autorização de `regex` em L1 passa a ser **nova
decisão explícita** quando consumer de shaping chegar — não
mais "requisito implícito para font".

---

## Próximo passo: 132B

Após aprovação deste diagnóstico, 132B enuncia-se com:

- Diagnóstico como fonte de verdade.
- ADR-0053 como contrato (transita para `IMPLEMENTADO` ao
  fechar 132B).
- Plano de materialização em 8 steps.
- 10 testes canary a adaptar (lista enumerada).
- 10-12 unit tests novos em `font_list.rs`.
- 4-5 integration tests novos em `rules/eval/tests.rs`.

**Estimativa 132B**: S-M, 2-3h.

### Progresso DEBT-1

| Propriedade | Estado | Passo |
|-------------|--------|------:|
| bold/italic/size/fill | ✓ captura | 30/102 |
| weight numérico + simbólico | ✓ captura | 126/129 |
| tracking | ✓ captura | 127 |
| leading (divergente) | ✓ captura em text | 128 |
| lang | ✓ tipo semântico Lang | 131B |
| **font** | ⏳ diagnosticado (132A) | **132B** |
| stroke/alignment/justify/... | ✗ | futuro |

Após 132B, DEBT-1 fica **quase completo** — falta apenas
migração de `leading` para `par` (133 + 134) e fecho formal
(135).
