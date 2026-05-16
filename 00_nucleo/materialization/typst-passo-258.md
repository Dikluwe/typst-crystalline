# Passo 258 — Finalizar Model (auditoria Fase A + actualização docs L0 + decisão condicional B1/B2/B3)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada XS-L
conforme cenário Fase A.
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- `00_nucleo/diagnosticos/diagnostico-model-passo-256.md`
  (diagnóstico pai com 22 entradas P154A + cenários B1/B2/B3).
- `00_nucleo/diagnosticos/fase-a-checklist-model-passo-256.md`
  (checklist executável Fase A).
- ADR-0060 (Model structural roadmap; IMPLEMENTADO Fase 1).
- ADR-0061 (Layout Fase X; estado a confirmar Fase A).
- ADR-0062 (hayagriva PROPOSTO; reserva Bloco B Model).
- ADR-0033, ADR-0034, ADR-0054, ADR-0065.
- DEBT-55 (bibliography+cite).
- Relatórios precedentes: P255 (DEBT-8 Math ENCERRADO) e
  P257 (Color paridade vanilla) — padrões metodológicos
  estabelecidos.

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-model-fase-a-passo-258.md`
  (Fase A executada; tabelas A+B preenchidas; imutável per
  ADR-0034).
- Prompts L0 obsoletos actualizados (esperado: `entities/content.md`
  e possivelmente `entities/bib_entry.rs` se P159G fields não
  reflectidos); hashes propagados.
- DEBT-55 actualizada conforme cenário Fase B (CLOSED se B1;
  estado revisto se B2; preservada se B3).
- Eventual ADR-0062 promoção PROPOSTO → IMPLEMENTADO (cenário
  B2 Opção 2 hayagriva).
- Eventual código L1 novo (cenário B2 Opções 1/2/3) com prompts
  L0 actualizados primeiro per Regra de Ouro.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-258-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

1. **Regra de Ouro CLAUDE.md** — código L1 nunca antes de
   prompt L0. Order: Fase A audit → docs L0 → fix-hashes →
   (se materialização) testes-primeiro → código.
2. **ADR-0029 §"Diagnosticar primeiro"** — qualquer tipo
   vanilla materializado nesta passagem (footnote, hayagriva
   integration) obriga leitura literal vanilla antes de
   definir estrutura.
3. **ADR-0034 + ADR-0065 inventariar primeiro** — Fase A
   produz evidência factual antes de qualquer decisão.
4. **Ordem testes-primeiro** — para cada código novo: testes
   antes de implementação.
5. **`crystalline-lint .`** zero violations no fim do passo.
6. **Tests workspace** sem regressão (contagem ≥ baseline
   2334 pós-P257).
7. **Materialization é leitura proibida por iniciativa
   própria** — Claude Code não deve ler
   `00_nucleo/materialization/` excepto com path explícito.
8. **Política "sem novas reservas"** preservada — se descoberta
   nova obstrução durante audit, registar como achado no
   diagnóstico imutável, não criar DEBT/ADR reserva.

---

## §1 — Sub-passo P258.A: Fase A (auditoria empírica)

**Objectivo**: produzir evidência factual sobre estado real
das 22 entradas Model declaradas em P154A.

**Materialização**: zero código novo. Apenas leitura e
produção de diagnóstico imutável.

### Acções obrigatórias

Executar **integralmente** os 7 blocos de comandos listados em
`00_nucleo/diagnosticos/fase-a-checklist-model-passo-256.md`:

**Bloco 1 — Variants Content existentes**:
```bash
grep -n "^\s*[A-Z][a-zA-Z]*\s*[{(\s]" 01_core/src/entities/content.rs | head -80
grep -c "^\s*[A-Z][a-zA-Z]*\s*[{(\s]" 01_core/src/entities/content.rs
```

**Bloco 2 — Entradas `implementado` P154A** (heading, emph,
strong, outline):
```bash
grep -n "Content::Heading\b\|Content::Emph\b\|Content::Strong\b\|Content::Outline\b" \
  01_core/src/entities/content.rs
grep -n "native_heading\|native_emph\|native_strong\|native_outline" \
  01_core/src/rules/stdlib*
```

**Bloco 3 — Entradas `implementado⁺` P154A** (figure, ref,
numbering):
```bash
grep -n "Content::Figure\b\|Content::Ref\b\|Content::Labelled\b" \
  01_core/src/entities/content.rs
grep -n "Content::SetHeadingNumbering\b\|Content::SetEquationNumbering\b" \
  01_core/src/entities/content.rs
```

**Bloco 4 — Entradas `parcial` P154A** (link, list, enum,
par, caption inline):
```bash
grep -n "Content::Link\b\|Content::List\b\|Content::Enum\b\|Content::Par\b\|Content::Paragraph\b" \
  01_core/src/entities/content.rs
grep -n "caption" 01_core/src/entities/content.rs | head -10
```

**Bloco 5.1 — Esperadas materializadas** (terms, divider,
quote, table, bibliography, cite):
```bash
grep -n "Content::Terms\b\|Content::TermItem\b\|Content::Divider\b\|Content::Quote\b" \
  01_core/src/entities/content.rs
grep -n "Content::Table\b\|Content::TableCell\b\|Content::TableHeader\b\|Content::TableFooter\b" \
  01_core/src/entities/content.rs
grep -n "Content::Bibliography\b\|Content::Cite\b" 01_core/src/entities/content.rs
ls -la 01_core/src/entities/bib_entry.rs
grep -c "^\s*pub\s" 01_core/src/entities/bib_entry.rs
```

**Bloco 5.2 — Footnote (crítico)**:
```bash
grep -rn "Content::Footnote\b\|native_footnote" 01_core/src/
grep -n "footnote_area" 01_core/src/entities/layout_types.rs
grep -rn "footnote" 01_core/src/rules/layout/
```

**Bloco 5.3 — Fase 3 condicional** (document, title, asset):
```bash
grep -n "Content::Document\b\|Content::Title\b\|Content::Asset\b" \
  01_core/src/entities/content.rs
```

**Bloco 6 — DEBT-55 hayagriva integration**:
```bash
grep -rn "use hayagriva\|hayagriva::" 01_core/ 03_infra/ 02_shell/ 04_wiring/
grep "hayagriva" Cargo.toml */Cargo.toml
grep -rn "style.*csl\|BibStyle\|CitationStyle" 01_core/src/
grep "Status" 00_nucleo/adr/typst-adr-0062*
```

**Bloco 7 — Inconsistências documentais esperadas**:
```bash
view 00_nucleo/prompts/entities/content.md | head -100
ls -la 00_nucleo/prompts/entities/content.md
grep -A 30 "DEBT-55" 00_nucleo/DEBT.md
```

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-model-fase-a-passo-258.md`
com a seguinte estrutura (imutável após criação per ADR-0034;
modelo análogo a `diagnostico-math-fase-a-passo-255.md` e
`diagnostico-color-vanilla-passo-257.md`):

```markdown
# Diagnóstico Model Fase A — Passo 258 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0034 diagnóstico canónico + ADR-0065
inventariar primeiro critério #5.
**Diagnóstico pai**: `diagnostico-model-passo-256.md` +
`fase-a-checklist-model-passo-256.md`.
**Análogo estrutural**: `diagnostico-math-fase-a-passo-255.md`
(P255) e `diagnostico-color-vanilla-passo-257.md` (P257).

---

## §1 — Comandos executados e output literal

(Colar output literal completo dos 7 blocos. Sem
interpretação aqui — só registar.)

## §2 — Classificação por entrada (Tabela A)

| # | Entrada | P154A | Audit P258 | Hits literais | Justificação |
|---|---------|-------|------------|---------------|--------------|
| 1 | heading | implementado | _ | _ | _ |
| 2 | emph | implementado | _ | _ | _ |
| 3 | strong | implementado | _ | _ | _ |
| 4 | outline | implementado | _ | _ | _ |
| 5 | figure | implementado⁺ | _ | _ | _ |
| 6 | ref | implementado⁺ | _ | _ | _ |
| 7 | numbering | implementado⁺ | _ | _ | _ |
| 8 | heading (ressalva) | implementado⁺ | _ | _ | _ |
| 9 | link | parcial | _ | _ | _ |
| 10 | list | parcial | _ | _ | _ |
| 11 | enum | parcial | _ | _ | _ |
| 12 | par | parcial | _ | _ | _ |
| 13 | caption inline | parcial | _ | _ | _ |
| 14 | bibliography | ausente | _ | _ | _ |
| 15 | cite | ausente | _ | _ | _ |
| 16 | footnote | ausente | _ | _ | _ |
| 17 | quote | ausente | _ | _ | _ |
| 18 | terms | ausente | _ | _ | _ |
| 19 | table | ausente | _ | _ | _ |
| 20 | document | ausente | _ | _ | _ |
| 21 | divider | ausente | _ | _ | _ |
| 22 | asset | ausente | _ | _ | _ |
| 22' | title | ausente | _ | _ | _ |

## §3 — Estado agregado (Tabela B)

| Estado | P154A | Audit P258 | Δ |
|--------|-------|------------|---|
| implementado | 4 | _ | _ |
| implementado⁺ | 4 | _ | _ |
| parcial | 5 | _ | _ |
| ausente | 10 | _ | _ |
| scope-out | 0 | _ | _ |
| TOTAL | 22+1 | _ | _ |
| Cobertura agregada | ~45% | _% | _pp |

## §4 — DEBT-55 hayagriva integration

(Resultado Bloco 6: hits/zero hits em hayagriva crate;
ADR-0062 status actual; CSL refs existentes ou não.)

## §5 — Inconsistências documentais detectadas

(Listar achados Bloco 7: L0 prompts desactualizados; DEBT-55
congelada; etc. Análogo a precedente P255/P257.)

## §6 — Decisão cenário Fase B

**Contagem fechados/abertos**: _/22 fechados; _/22 abertos.

**Cobertura agregada empírica**: _%.

**Cenário escolhido**: ☐ B1 (≥75% — fecho conceptual) /
☐ B2 (55-70% — sub-passos prioritários) / ☐ B3 (≤50% —
re-classificação primeiro).

**Se B2, opção(ões) recomendada(s)**:
- ☐ Opção 1 — footnote materialização (M; +10-15 tests; +5pp).
- ☐ Opção 2 — ADR-0062 hayagriva promoção + bibliography
  hayagriva (L; +20-30 tests; +10pp).
- ☐ Opção 3 — refinos parcial→implementado para
  link/list/enum/par (S+ cada; cumulativo +15-20pp).

## §7 — Achados inesperados

(Qualquer descoberta não prevista em P256 — variants Model
materializados que não estavam na lista P154A; refactors
estruturais; etc.)

## §8 — Referências

- P154A diagnóstico Model original.
- P156C Layout Fase 1 desbloqueia footnote.
- P159A-G série bibliography+cite.
- P181D-H integração Introspector.
- P255 precedente "auditoria condicional".
- P257 precedente "Refactor cross-cutting entity primitivo".
```

### Critério de aceitação P258.A

- Ficheiro
  `diagnostico-model-fase-a-passo-258.md` criado em
  `00_nucleo/diagnosticos/`.
- §1-§8 preenchidos com **conteúdo literal** (hits/no-hits
  factuais), não interpretativos.
- Tabelas A+B preenchidas para todas as 22 entradas.
- Decisão cenário B1/B2/B3 explicitada em §6 com
  justificação factual.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações a prompts L0, ADRs ou DEBT.md (ainda — vem
  em P258.B+).

---

## §2 — Sub-passo P258.B: Reconciliação documental L0

**Objectivo**: actualizar prompts L0 obsoletos descobertos em
P258.A §5.

**Materialização**: edição de prompts L0 + `--fix-hashes`. Sem
código L1.

### Acções obrigatórias

#### B.1 — Identificar L0 prompts obsoletos

Baseado em P258.A §5. Esperado (per precedente P255/P257):
- `00_nucleo/prompts/entities/content.md` — listagem de
  variants provavelmente desactualizada vs enum real
  pós-M3-M9 + P199B.
- Eventuais outros prompts L0 que P258.A revele.

#### B.2 — Editar L0 prompts

Para cada prompt L0 desactualizado:
- Substituir secções obsoletas por estado actual descoberto
  na Fase A.
- Preservar histórico (não apagar; anotar como "actualizado
  por P258 reconciliação documental").
- Manter restrições arquitecturais.

#### B.3 — Propagar hashes

```bash
cargo run -p crystalline-lint -- --fix-hashes .
```

#### B.4 — Verificação

```bash
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
cargo test --workspace
# Esperado: contagem inalterada (≥2334)
```

### Critério de aceitação P258.B

- Prompts L0 reconciliados reflectem código real.
- Hashes propagados (zero violations V5 PromptStale).
- Tests workspace inalterados em contagem.
- Zero alterações a código L1/L2/L3/L4 (esta fase é
  puramente documental).

---

## §3 — Sub-passo P258.C: Materialização condicional

**Executar apenas** se P258.A §6 escolheu cenário **B2** ou
**B3**.

**Se P258.A escolheu B1**, saltar P258.C directamente para
P258.D.

### Cenário B1 — Fecho conceptual

Sem materialização. P258.D actualiza DEBT-55 (CLOSED se
hayagriva foi materializado fora de scope detectado em Fase A,
ou reclassificação como scope-out) + relatório.

### Cenário B2 — Sub-passos prioritários

**Decisão de qual(is) Opção(ões) materializar** registada em
P258.A §6. Materializar uma de cada vez (granularidade
preservada).

#### Opção 1 — footnote materialização

**Pré-requisitos** (Regra de Ouro):
1. Confirmar P258.A Bloco 5.2: `Page::footnote_area` existe
   (ou criar prompt L0 para extensão).
2. Criar/actualizar L0 prompt `entities/content.md` secção
   Footnote.
3. Diagnóstico vanilla per ADR-0029 §"Diagnosticar primeiro":
   leitura literal de `lab/typst-original/.../model/footnote.rs`.

**Materialização** (testes primeiro per CLAUDE.md):

1. **Testes E2E**:
   ```rust
   #[test]
   fn footnote_renderiza_em_footnote_area() { ... }
   #[test]
   fn footnote_numbering_sequential() { ... }
   #[test]
   fn footnote_marker_em_corpo() { ... }
   ```
   Executar `cargo test footnote::` — verificar falham.

2. **Variant `Content::Footnote { body, numbering }`** em
   `entities/content.rs`.

3. **Stdlib `native_footnote(body)`** em
   `rules/stdlib/structural.rs` ou similar.

4. **Layouter consumer** popula `footnote_area` do Page actual.

5. **Cobertura exaustiva pattern-match** (~7 sítios análogos a
   P154B/P155).

6. **Numbering**: `Content::SetFootnoteNumbering`? Confirmar
   paridade vanilla. Se sim, adicionar análogo a
   `SetHeadingNumbering` (P182C) / `SetEquationNumbering`
   (P199B).

7. Tests verdes; lint zero.

**Magnitude esperada**: M (~2-3h; +10-15 tests).
**Cobertura**: +5pp Model agregada; +1 ausente fechado.

#### Opção 2 — ADR-0062 hayagriva promoção + bibliography hayagriva

**Pré-requisitos críticos**:

1. **Passo administrativo XS pré** (análogo P160A/ADR-0062-create
   se já existe ficheiro PROPOSTO; ou criar): ADR-0062
   PROPOSTO → IMPLEMENTADO.
2. **Crate authorization**: `hayagriva = "0.9.1"` em
   `01_core/Cargo.toml` + `[l1_allowed_external.hayagriva]`
   em `crystalline.toml` (per DEBT-43 type-level whitelist
   futuro; usar crate-level por agora).
3. **Diagnóstico vanilla obrigatório** per ADR-0029.
4. **Prompt L0 actualizado** `entities/bib_entry.md` (ou
   criar `entities/hayagriva_integration.md`).

**Materialização** (testes primeiro):

1. **Testes** com 5+ entries reais (BibTeX-like).
2. **Parser BibEntry literal → `hayagriva::Entry`**.
3. **CSL styling** — pelo menos 2 styles (numeric, author-date).
4. **Cite forms via `hayagriva::CiteForm`**.
5. **Layouter integration**: substituir `format_bib_entry`
   manual (P159E-G) por chamada hayagriva.
6. **DEBT-55 transita para CLOSED** em P258.D.

**Magnitude esperada**: L (~6-10h; +20-30 tests).
**Cobertura**: +10pp Model; hayagriva entra em deps L1.

**Decisão de granularidade**: pode dividir-se em sub-passos
P258.C2.1 (ADR-0062 promoção + crate auth) + P258.C2.2
(parser BibEntry) + P258.C2.3 (CSL styling) + P258.C2.4
(Layouter integration). Decisão registada conforme magnitude
real percebida.

#### Opção 3 — Refinos parcial→implementado

**Pré-requisitos**: cada feature tem L0 prompt actualizado
ou criado com critérios refinados.

**Materialização granular** (1 feature/passo análogo
P157A-G):

**P258.C3.1 — `link` completar atributos vanilla**:
- Diagnóstico vanilla: confirmar atributos (`dest`, `body`,
  `fill`, `stroke`, etc.).
- L0 prompt secção Link expandida.
- Testes primeiro: +4-6.
- Magnitude: M.

**P258.C3.2 — `list`/`enum` refinos** (par simétrico
análogo P157C TableHeader/Footer):
- Atributos: `marker`, `tight`, `indent`.
- Par simétrico em pattern-match (subpadrão P157C #12).
- Testes primeiro: +6-8 (par).
- Magnitude: M.

**P258.C3.3 — `par` refinos**:
- Atributos: `leading`, `justify`, `first_line_indent`.
- Toca StyleChain (DEBT-1 ressuscitado parcialmente?).
- **Pré-requisito**: verificar se DEBT-stylechain-não-materializada
  bloqueia ou não.
- Testes primeiro: +5-8.
- Magnitude: M+.

**Magnitude por feature**: S+/M.
**Cobertura cumulativa**: +15-20pp Model agregada.

### Cenário B3 — Re-classificação primeiro

**Improvável**. Se materializar:

1. Re-classificar Tabela A conservadoramente em P258.A §2.
2. ADR-0060 anotação cumulativa de revisão para baixo.
3. Sub-passos de elevação prioritários como cenário B2.

### Critério de aceitação P258.C

- Cada Opção materializada (B2) respeita ordem
  testes-primeiro.
- Cada feature tem prompt L0 actualizado **antes** do código.
- Hashes propagados.
- Tests workspace +N (consoante Opções resolvidas).
- Zero violations linter.
- Paridade observable preservada.

---

## §4 — Sub-passo P258.D: Actualização cumulativa + relatório

**Objectivo**: actualizar DEBT-55, ADRs cumulativas, e
produzir relatório final.

### D.1 — Actualizar `00_nucleo/DEBT.md`

**Se cenário B1** ou **B2 Opção 2 hayagriva materializada**:
- DEBT-55 transita para **CLOSED**.
- Secção "Resolvido em Passo 258" com referência cruzada.

**Se B2 outras Opções** ou **B3**:
- DEBT-55 actualizada com lista revista (Opções
  materializadas marcadas; pendentes listadas).
- Preservar histórico Passo 154A.

### D.2 — ADRs cumulativas

**ADR-0060** (Model structural roadmap; IMPLEMENTADO):
- Anotação cumulativa P258 com cobertura empírica revista,
  Opções materializadas, estado Bloco A/B/C.
- Status `IMPLEMENTADO` preservado (Fase 1 fechada P155 não
  muda).

**ADR-0062** (hayagriva):
- **Se Opção 2 materializada**: PROPOSTO → IMPLEMENTADO no
  mesmo passo (subpadrão P257 "ADR PROPOSTO+IMPLEMENTADO no
  mesmo passo via Cenário B1" cresce **N=1 → N=2**).
- **Se não**: status preservado.

### D.3 — README ADRs

`00_nucleo/adr/README.md`:
- Entrada P258 nos passos-chave.
- Distribuição ADRs actualizada se transições ocorrerem.

### D.4 — Relatório do passo

`00_nucleo/materialization/typst-passo-258-relatorio.md`
estrutura canónica (análogo P255/P257):

- **§1 Sumário executivo** — cenário Fase A; opções
  materializadas; tests delta; ADRs tocadas; prompts L0
  actualizados.
- **§2 Sub-passo P258.A** — output Fase A resumido.
- **§3 Sub-passo P258.B** — prompts L0 editados; hashes.
- **§4 Sub-passo P258.C** — código materializado (se
  aplicável) com referências a ficheiros.
- **§5 Sub-passo P258.D** — DEBT-55 + ADRs cumulativas.
- **§6 Padrões metodológicos** — ADR-0065 critério #5
  aplicado; subpadrão "auditoria condicional" cresce **N=4
  → N=5**; eventuais subpadrões novos.
- **§7 Cobertura** — Model passa de ~45% (P154A) para X%
  (audit empírico).
- **§8 Limitações e trabalho futuro** — Opções não
  materializadas; Fase 3 condicional; scope-outs registados.
- **§9 Critério de aceitação global P258 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P258.D

- DEBT-55 reflecte estado real pós-passo.
- ADRs cumulativas anotadas.
- README ADRs actualizado.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P258

Ao fim do passo, todos os seguintes têm de ser verdadeiros:

- [ ] `cargo run -p crystalline-lint -- .` retorna
  `✓ No violations found`.
- [ ] `cargo test --workspace` retorna contagem ≥ baseline
  2334 (sem regressão).
- [ ] `diagnostico-model-fase-a-passo-258.md` existe com
  tabelas A+B preenchidas.
- [ ] Prompts L0 obsoletos reconciliados.
- [ ] Hashes propagados (zero violations V5).
- [ ] DEBT-55 actualizada conforme cenário.
- [ ] ADR-0060 anotação cumulativa adicionada.
- [ ] (Se B2 Opção 2) ADR-0062 promovida a IMPLEMENTADO.
- [ ] Relatório criado em `00_nucleo/materialization/`.
- [ ] Se cenário B2/B3 e materializaste código: cada Opção
  respeitou ordem testes-primeiro e teve L0 actualizado antes
  do código.

---

## §6 — Sequência operacional condensada

Para Claude Code seguir linearmente:

1. **Ler** `CLAUDE.md`, diagnóstico P256,
   checklist P256, ADRs 0060/0061/0062, DEBT-55, relatórios
   precedentes P255 + P257 (padrões metodológicos
   estabelecidos).
2. **Reportar** estado inicial: tests count (esperado 2334
   pós-P257) + lint baseline.
3. **P258.A** — Executar 7 blocos de comandos do checklist;
   criar diagnóstico Fase A imutável; tabelas A+B preenchidas;
   decisão B1/B2/B3 explícita em §6.
4. **P258.B** — Editar prompts L0 obsoletos descobertos;
   `--fix-hashes`; lint limpo; tests inalterados.
5. **P258.C condicional** — Se B2/B3, para cada Opção:
   verificar/criar L0 → diagnóstico vanilla se aplicável →
   testes primeiro → código → lint → tests.
6. **P258.D** — Actualizar DEBT-55, ADRs cumulativas, README
   ADRs; criar relatório.
7. **Verificação final** — todo o checklist §5 satisfeito.
8. **Reportar** ao utilizador: cenário escolhido, Opções
   materializadas, tests delta, ficheiros criados/editados.

---

## §7 — Política de paragem

Claude Code **deve parar e perguntar ao utilizador** se:

- P258.A revela que cobertura empírica é muito diferente do
  esperado (e.g. ≥80% — sugere Cenário B1 fecho conceptual mas
  pode esconder regressão ou re-classificação optimista).
- P258.A revela materialização de variants Model **não listados
  em P154A** (e.g. variants pós-M9 que pertencem a Model mas
  não estavam no inventário original) — exigem
  re-classificação Tabela A.
- P258.B descobre prompt L0 com critérios contraditórios face
  ao código real (não apenas desactualizados — exigem decisão
  arquitectural).
- P258.C Opção 1 (footnote) revela que `Page::footnote_area`
  ainda não foi materializado em Layout (Fase 1 P156C
  documentou desbloqueio mas implementação real pode estar
  ausente; pré-requisito não cumprido).
- P258.C Opção 2 (hayagriva) revela conflict com layout actual
  `format_bib_entry` (P159E-G) que exija refactor maior do
  esperado.
- P258.C Opção 3 (par refinos) revela que StyleChain DEBT
  bloqueia mais do que estimado.
- Decisão arquitectural nova exige ADR PROPOSTO independente
  (e.g. Footnote como variant separado vs sub-variant de
  Sequence; estrutura ColorSpace runtime).
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.

Em qualquer paragem, registar contexto no relatório parcial e
aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P257 (Color)

P257 fechou Color (Visualize) com 8 espaços + 4 scope-outs
documentados. Visualize ganhou expansão substantiva mas Model
ficou em pausa. P258 retoma Model.

### Subpadrão "auditoria condicional" cresce N=4 → N=5

Cumulativo:
- N=1 P192A (M7 fixpoint).
- N=2 P255 (DEBT-8 Math).
- N=3 P257 (Color — Fase A diagnóstico vanilla).
- N=4 P258 (este passo — Model).

**Patamar N=5 atinge limiar formalização clara** (política
consistente N=3-4 cumprida; promoção a ADR meta candidata em
passo administrativo XS futuro).

### Subpadrão "ADR PROPOSTO+IMPLEMENTADO mesmo passo"

Cumulativo:
- N=1 P257 (ADR-0083 Color).
- **Se P258 Opção 2 materializar**: N=2 (ADR-0062 hayagriva).

Patamar N=2 reforça subpadrão; formalização em ADR-meta
candidato pós-N=3.

### Política "sem novas reservas"

Preservada. Recomendações P256 são para validação humana;
opções B2 não são compromissos automáticos — decisão é da Fase
A §6.

### Visualize, Text restantes (pós-P258)

Visualize: Color expandido (P257); resto (paths, curves,
ShapeKind expansão, Gradient, Paint, Tiling) fica em roadmap
informal não-reservado per ADR-0029 §enumeração.

Text: StyleChain refino + DEBT-53 rustybuzz real — passos
dedicados futuros não cobertos por este passo.

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0017, ADR-0026, ADR-0033, ADR-0034, ADR-0038, ADR-0054,
  ADR-0060, ADR-0061, ADR-0062, ADR-0064, ADR-0065.
- ADR-0083 (P257 precedente Color paridade vanilla).
- DEBT-55 (bibliography+cite XL; alvo P258.D).
- `00_nucleo/diagnosticos/diagnostico-model-passo-256.md` —
  diagnóstico pai (planeamento Fase A/B).
- `00_nucleo/diagnosticos/fase-a-checklist-model-passo-256.md`
  — comandos exactos P258.A.
- P154A — diagnóstico Model original (origem 22 entradas).
- P154B → P159G — Bloco A Model materializado.
- P156C — Layout Fase 1 desbloqueia footnote.
- P181D-H — Bibliography integrado com Introspector.
- P182C, P199B — Set*Numbering variants.
- P255 — DEBT-8 Math ENCERRADO via audit condicional
  (precedente N=2 "auditoria condicional"; modelo P258).
- P257 — Color paridade vanilla via Cenário Fase A literal
  (precedente N=3 "auditoria condicional"; "ADR
  PROPOSTO+IMPLEMENTADO mesmo passo" N=1).
- P192A — N=1 "auditoria condicional".
