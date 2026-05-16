# Passo 266 — Text audit Fase A (primeiro consumo directo ADR-0084 + ADR-0085 EM VIGOR)

**Data**: 2026-05-15
**Tipo**: passo composto sequencial; magnitude estimada
**XS-L** conforme cenário Fase A.
**Pré-requisito leitura obrigatória** (CLAUDE.md Regra de Ouro):
- `CLAUDE.md` (Regra de Ouro + Protocolo de Nucleação + Ordem
  testes-primeiro).
- `00_nucleo/diagnosticos/diagnostico-text-passo-266.md`
  (diagnóstico pai com 40 entradas + cenários B1/B2/B3).
- `00_nucleo/diagnosticos/fase-a-checklist-text-passo-266.md`
  (checklist executável Fase A).
- **ADR-0084** (auditoria condicional EM VIGOR P260 — **este
  passo é primeiro consumo directo formal pós-formalização**).
- **ADR-0085** (diagnóstico imutável EM VIGOR P260 — Fase A
  produz diagnóstico sob esta ADR).
- ADR-0029, ADR-0033, ADR-0034, ADR-0054, ADR-0065.
- ADR-0038 (Content::Styled), ADR-0039 (TextStyle SR).
- ADR-0052 (Lang tipo), ADR-0053 (font dict deferido),
  ADR-0054 (perfil graded), ADR-0055 (font consumer CIDFont),
  ADR-0057 (lang hyphenation).
- DEBT-53 (rustybuzz shaping; em aberto candidato XL — **fora
  de scope ADR-0054**).
- Relatórios precedentes: **P259 (Visualize Fase A)** —
  **template literal directo deste passo** (mesmo pattern;
  P259 foi pré-formalização P260).

**Outputs canónicos esperados** ao fim do passo:
- `00_nucleo/diagnosticos/diagnostico-text-fase-a-passo-266.md`
  (Fase A executada; tabelas A+B preenchidas; imutável per
  **ADR-0085 — primeiro consumo directo formal**).
- Prompts L0 obsoletos actualizados (esperado per padrão
  P255-P259); hashes propagados.
- ADR-0054 anotação cumulativa P266 com cobertura Text
  empírica.
- DEBT-53 anotação cumulativa (em aberto; cross-reference
  P266 audit confirma estado).
- Eventual ADR nova (cenário B2 — ADR-0055bis variant-aware
  ou ADR-0056 subsetting).
- Eventual código L1 novo (cenário B2) com prompts L0
  actualizados primeiro per Regra de Ouro.
- Relatório do passo em
  `00_nucleo/materialization/typst-passo-266-relatorio.md`.

---

## §0 — Princípios vinculativos para este passo

0. **Vanilla read-first explicitamente autorizado** —
   `lab/typst-original/` disponível; Claude Code lê literal
   estrutura vanilla quando necessário (especialmente Bloco 1
   Fase A: `lab/typst-original/.../text/mod.rs` + raw.rs +
   lang.rs + font.rs).
1. **Regra de Ouro CLAUDE.md** — código L1 nunca antes de
   prompt L0. Ordem: Fase A audit → docs L0 → fix-hashes →
   (se materialização) testes-primeiro → código.
2. **ADR-0084 primeiro consumo directo** — este passo cumpre
   estrutura ADR-0084 literalmente (sub-passo .A audit + .B
   reconciliação docs + .C materialização condicional + .D
   fecho). Validação retrospectiva da formalização P260.
3. **ADR-0085 primeiro consumo directo** — diagnóstico Fase A
   produzido cumpre estrutura ADR-0085 literalmente (imutável,
   localização canónica, nome, marcador, tabelas, decisão).
4. **ADR-0029 §"Diagnosticar primeiro"** — qualquer tipo
   vanilla materializado neste passo (improvável dado cenário
   B1 esperado) obriga leitura vanilla.
5. **ADR-0034 + ADR-0065 inventariar primeiro** — Fase A
   produz evidência factual.
6. **Ordem testes-primeiro** — para qualquer código novo.
7. **`crystalline-lint .`** zero violations no fim do passo.
8. **Tests workspace** sem regressão (baseline 2393 pós-P265).
9. **Materialization é leitura proibida por iniciativa
   própria**.
10. **Política "sem novas reservas"** preservada — se
    descoberta nova obstrução, registar como achado no
    diagnóstico imutável, não criar DEBT/ADR reserva.
11. **DEBT-53 fora de scope ADR-0054** — qualquer cenário B
    preserva DEBT-53 como candidato XL futuro; não é gatilho
    de B3.

---

## §1 — Sub-passo P266.A: Fase A (auditoria empírica)

**Objectivo**: produzir evidência factual sobre estado real
dos 40+ subsistemas/entradas Text.

**Materialização**: zero código novo. Apenas leitura e
produção de diagnóstico imutável per ADR-0085.

### Acções obrigatórias

Executar **integralmente** os 9 blocos de comandos listados
em `fase-a-checklist-text-passo-266.md`:
- **Bloco 1** Vanilla Text módulo (leitura literal).
- **Bloco 2** Variants Content text-related (cristalino).
- **Bloco 3** StyleChain + StyleDelta campos.
- **Bloco 4** Font rendering helpers L3 (3 caminhos).
- **Bloco 5** Lang features.
- **Bloco 6** Markup secundários.
- **Bloco 7** Refinos text features.
- **Bloco 8** Inconsistências documentais.
- **Bloco 9** Cross-features arquitecturais.

### Output exigido — ficheiro novo

Criar
`00_nucleo/diagnosticos/diagnostico-text-fase-a-passo-266.md`
com estrutura análoga
`diagnostico-visualize-fase-a-passo-259.md` (P259):

```markdown
# Diagnóstico Text Fase A — Passo 266 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0084 + ADR-0085 (primeiro consumo directo
formal pós-P260) + ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `diagnostico-text-passo-266.md` +
`fase-a-checklist-text-passo-266.md`.
**Análogo estrutural directo**:
`diagnostico-visualize-fase-a-passo-259.md` (P259).

---

## §1 — Comandos executados e output literal

(Colar output literal completo dos 9 blocos. Sem
interpretação aqui — só registar.)

## §2 — Classificação por subsistema/entrada (Tabela A — 40 entradas)

| # | Subsistema/Entrada | Pré-audit | Audit P266 | Hits | Justificação |
|---|---|---|---|---|---|
(40 entradas conforme checklist Tabela A)

## §3 — Estado agregado (Tabela B)

| Estado | Pré-P266 estimado | Audit P266 | Δ |
|--------|---------------------|------------|---|

## §4 — Achados inesperados

(Materializações fora do esperado; subsistemas com expansão
não documentada; inconsistências documentais detectadas;
features secundárias confirmadas/ausentes — Raw, escape,
shorthands, linebreak, parbreak.)

## §5 — Decisão cenário Fase B

**Contagem fechados/abertos**: _/40 fechados; _/40 abertos.

**Cobertura agregada empírica**: _%.

**Cenário escolhido**: ☐ B1 (≥75% — provável) / ☐ B2 / ☐ B3.

**Se B2, opção(ões) recomendada(s)**:
- ☐ Opção 1 — ADR-0055bis variant-aware (M).
- ☐ Opção 2 — ADR-0056 font subsetting (M-L).
- ☐ Opção 3 — Refinos Raw (S+).
- ☐ Opção 4 — Shaping pre-rustybuzz (não recomendado).
- ☐ Opção 5 — Refinos lang (S+).

## §6 — Referências

(P255/P257/P258/P259 precedentes; ADR-0084/0085/0054; DEBT-53.)
```

### Critério de aceitação P266.A

- Ficheiro `diagnostico-text-fase-a-passo-266.md` criado.
- §1-§5 preenchidos com conteúdo literal.
- Tabelas A+B preenchidas (40+ entradas).
- Decisão cenário B1/B2/B3 explicitada em §5.
- **Marcador "Imutável após criação per ADR-0085"** explícito.
- Zero alterações em código L1/L2/L3/L4.
- Zero alterações a prompts L0, ADRs ou DEBT.md.

---

## §2 — Sub-passo P266.B: Reconciliação documental L0

**Objectivo**: actualizar prompts L0 obsoletos descobertos em
P266.A §4.

**Materialização**: edição L0 + `--fix-hashes`. Sem código L1.

### Acções obrigatórias

#### B.1 — Identificar L0 prompts obsoletos

Baseado em P266.A §4. Esperado (per precedente P255/P257/P258/
P259):
- `entities/style_chain.md` — DEBT-1 fechado + 9 refinos
  cumulativos; provavelmente desactualizado.
- `entities/font_book.md` — multi-font P146 cumulativos;
  provavelmente desactualizado.
- `rules/layout.md` — hyphenation P144 cumulativos;
  provavelmente desactualizado.
- `rules/lang.md` — smart-quotes P155 cumulativos.
- Eventuais outros descobertos em audit.

#### B.2 — Editar L0 prompts

Para cada prompt L0 desactualizado:
- Substituir secções obsoletas por estado actual.
- Preservar histórico (não apagar; anotar como "actualizado
  por P266 reconciliação documental").
- Manter restrições arquitecturais.
- **Decisão arquitectural** (paridade P258.B/P259.B): preservar
  representação base como histórico cumulativo; secções
  subsequentes cobrem materializações reais. **Não
  reconciliação destructiva**.

#### B.3 — Propagar hashes

```bash
cargo run -p crystalline-lint -- --fix-hashes .
```

#### B.4 — Verificação

```bash
cargo run -p crystalline-lint -- .
# Esperado: ✓ No violations found
cargo test --workspace --release
# Esperado: 2393 preservado
```

### Critério de aceitação P266.B

- Prompts L0 reconciliados.
- Hashes propagados.
- Tests workspace inalterados.
- Zero violations.

---

## §3 — Sub-passo P266.C: Materialização condicional

**Executar apenas** se P266.A §5 escolheu cenário **B2** ou
**B3**. **Cenário B1 esperado provável** — saltar P266.C.

### Cenário B1 — Fecho conceptual (provável)

Sem materialização. P266.D actualiza DEBT-53 + ADR-0054 +
relatório.

### Cenário B2 — Sub-passos prioritários (improvável)

**Decisão Opção** registada em P266.A §5. Materializar uma
de cada vez.

#### Opção 1 — ADR-0055bis variant-aware font selection

Pré-requisitos:
1. ADR nova `00_nucleo/adr/typst-adr-0089-variant-aware-font-selection.md`
   (próximo slot livre pós-ADR-0088 P264) — PROPOSTO em
   P266.C, IMPLEMENTADO em P266.D.
2. Diagnóstico vanilla per ADR-0029:
   `lab/typst-original/.../text/font.rs`.
3. L0 prompt `entities/font_book.md` actualizado.

Materialização:
- `FontBook::select_variant(family, variant)` aceita
  `FontVariant { weight, style }`.
- `resolve_font` usa variant correcto em vez de
  `FontVariant::default()`.
- Substitui faux-bold P139 onde font-file dedicado existe.

Magnitude: M (~2-3h; +10-15 tests).

#### Opção 2 — ADR-0056 font subsetting

Pré-requisitos:
1. ADR-0056 nova (slot reservado per blueprint).
2. Crate L3 autorização (`ttf-subset` ou similar; ADR-0018).
3. Diagnóstico vanilla.
4. L0 prompt `infra/export.md` actualizado.

Materialização:
- PDF embeds só glyphs usados em vez de TTF completa.
- Reduz PDF substancialmente para fonts com glyph count
  grande.

Magnitude: M-L (~4-6h; +15-20 tests).

#### Opção 3 — Refinos Raw

Pré-requisitos:
1. Confirmar Fase A Bloco 6 — Raw shape actual.
2. L0 prompt `entities/content.md` secção Raw expandida.

Materialização:
- `Content::Raw { body, lang, block }` campos completos
  (paridade vanilla mínima).
- **Syntax highlighting scope-out** (preservaria ADR-0054
  graded).

Magnitude: S+ (~1-2h; +5 tests).

#### Opção 4 — Não recomendado (Shaping pre-rustybuzz)

Pré-requisitos: ADR explícita (ADR-0054 graded scope-out a
revisitar?).

Materialização: PDF `Tc`/`Tw` operators avançados; sem
rustybuzz seria aproximação fragmentária.

**Não recomendado** — DEBT-53 endereça shaping completo
futuro.

#### Opção 5 — Refinos lang (mais idiomas)

Materialização:
- Smart-quotes ja/zh/ar/he adicionais.
- Tabela 6 idiomas + default → 8-10 idiomas.

Magnitude: S+ (~30 min por idioma).

### Cenário B3 — Re-classificação primeiro (improvável)

1. Re-classificar Tabela A conservadoramente.
2. ADR-0054 anotação cumulativa de revisão para baixo.
3. Sub-passos elevação prioritários como B2.

### Critério de aceitação P266.C

- Cada Opção materializada (B2) respeita ordem
  testes-primeiro.
- Cada feature tem prompt L0 actualizado antes do código.
- Hashes propagados.
- Tests workspace +N.
- Zero violations.
- Paridade observable preservada.

---

## §4 — Sub-passo P266.D: Actualização cumulativa + relatório

### D.1 — Actualizar `00_nucleo/DEBT.md`

**DEBT-53 anotação cumulativa**:
- Confirmar estado actual (em aberto candidato XL).
- Cross-reference P266 audit confirma estado preservado.
- Sem alteração de status (paridade pattern ADR-0080
  §"refactor aditivo").

### D.2 — ADR-0054 anotação cumulativa

`00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` ganha
secção "Anotação cumulativa P266 — Cobertura Text empírica
confirmada":
- Cobertura empírica Text Fase A.
- Pendências preservadas (DEBT-53; variant-aware; subsetting).
- Status `EM VIGOR` preservado literal.

### D.3 — Eventual ADR nova (cenário B2)

Se Opção 1 (ADR-0055bis ou ADR-0089) ou Opção 2 (ADR-0056)
escolhidas:
- ADR nova criada PROPOSTO em P266.C.
- Promovida IMPLEMENTADO em P266.D.
- README ADRs actualizado.

### D.4 — README ADRs

`00_nucleo/adr/README.md`:
- Entrada P266 nos passos-chave.
- Distribuição ADRs actualizada se transições ocorrerem.

### D.5 — Relatório do passo

`00_nucleo/materialization/typst-passo-266-relatorio.md`
estrutura canónica (paridade P258 fecho conceptual):

- **§1 Sumário executivo** — cenário Fase A; opções
  materializadas (se B2); tests delta; ADRs tocadas.
- **§2 P266.A** — output Fase A resumido.
- **§3 P266.B** — prompts L0 editados; hashes.
- **§4 P266.C** — código materializado (se B2/B3).
- **§5 P266.D** — DEBT-53 + ADR-0054 anotação cumulativa.
- **§6 Padrões metodológicos** — **primeiro consumo directo
  ADR-0084 + ADR-0085**; subpadrão "auditoria condicional"
  cresce N=5 → 6; "diagnóstico imutável" cresce N=6 → 7.
- **§7 Cobertura** — Text passa de ~52% citado para X%
  empírico.
- **§8 Limitações e trabalho futuro** — DEBT-53 preservada;
  ADR-0055bis + ADR-0056 candidatos.
- **§9 Critério de aceitação global P266 — Checklist final**.
- **§10 Referências**.

### Critério de aceitação P266.D

- DEBT-53 anotação cumulativa.
- ADR-0054 anotação cumulativa.
- README ADRs actualizado.
- Relatório criado.
- Cross-references coerentes.

---

## §5 — Critério de aceitação global P266

- [ ] `cargo run -p crystalline-lint -- .` retorna `✓ No
  violations found`.
- [ ] `cargo test --workspace --release` retorna ≥ 2393 (sem
  regressão).
- [ ] `diagnostico-text-fase-a-passo-266.md` existe com
  tabelas A+B preenchidas (40+ entradas) e marcador
  "Imutável após criação per ADR-0085".
- [ ] Prompts L0 obsoletos reconciliados.
- [ ] Hashes propagados (zero violations V5).
- [ ] DEBT-53 anotação cumulativa.
- [ ] ADR-0054 anotação cumulativa.
- [ ] (Se B2) ADR nova criada+promovida (ADR-0055bis/0089/
  0056).
- [ ] Relatório criado em `00_nucleo/materialization/`.
- [ ] Se cenário B2/B3 e materializaste código: cada Opção
  respeitou ordem testes-primeiro e L0 actualizado antes do
  código.
- [ ] **Primeiro consumo directo ADR-0084 + ADR-0085
  documentado em §6 relatório**.

---

## §6 — Sequência operacional condensada

1. **Ler** `CLAUDE.md`, diagnóstico P266, checklist P266,
   ADRs 0033/0034/0038/0039/0054/0055/0057/0084/0085, DEBT-53,
   relatórios precedentes P255+P257+P258+P259.
2. **Reportar** estado inicial: tests 2393 + lint baseline +
   ADRs 75.
3. **P266.A** — Executar 9 blocos comandos checklist; criar
   diagnóstico Fase A imutável (primeiro consumo directo
   ADR-0085); tabelas A+B preenchidas; decisão B1/B2/B3
   explícita em §5.
4. **P266.B** — Editar prompts L0 obsoletos descobertos;
   `--fix-hashes`; lint limpo; tests inalterados.
5. **P266.C condicional** — Se B2/B3, para cada Opção:
   verificar/criar L0 → diagnóstico vanilla se aplicável →
   testes primeiro → código → lint → tests.
6. **P266.D** — Anotar DEBT-53 + ADR-0054 cumulativamente;
   eventual ADR nova promovida; README ADRs; criar relatório
   destacando primeiro consumo directo ADR-0084/0085.
7. **Verificação final** — checklist §5 satisfeito.
8. **Reportar** ao utilizador: cenário escolhido, Opções
   materializadas, tests delta, ficheiros criados/editados,
   recomendação P267+.

---

## §7 — Política de paragem

**Nota preliminar**: discrepância palpite-vs-vanilla não é
gatilho de paragem por si — Fase A regista o vanilla literal
e adapta classificação. Política aplica-se a decisões
arquitecturais não-óbvias.

Claude Code **deve parar e perguntar ao utilizador** se:

- P266.A revela cobertura empírica muito diferente do
  esperado (e.g. ≥95% — extremo positivo) ou ≤40% (extremo
  negativo).
- P266.A revela materialização de subsistemas Text **não
  previstos** no diagnóstico pai (e.g. shaping rustybuzz
  parcialmente materializado contra DEBT-53; algoritmos
  bidi presentes).
- P266.A revela que **DEBT-53 está parcialmente fechado**
  (e.g. ligatures via PDF `Tj` array) — re-classificação
  necessária.
- P266.A revela que `lab/typst-original/.../text/` tem
  estrutura significativamente diferente da esperada (e.g.
  module split novo, tipos novos).
- P266.B descobre prompt L0 com critérios contraditórios
  face ao código real (não apenas desactualizados — exigem
  decisão arquitectural).
- P266.C Opção 1 (variant-aware) revela que `FontVariant`
  cristalino tem semântica diferente do vanilla
  (e.g. weight ranges).
- P266.C Opção 2 (subsetting) revela que crate disponível
  tem licença/dependências incompatíveis com ADR-0018.
- P266.C Opção 3 (Raw) revela que vanilla Raw tem syntax
  highlighting integrado obrigatório (significaria scope-out
  ADR-0054 expandir).
- Decisão arquitectural nova exige ADR PROPOSTO independente.
- `crystalline-lint` reporta violations não-triviais.
- Tests regridem sem causa óbvia.

Em qualquer paragem, registar contexto no relatório parcial
e aguardar instrução.

---

## §8 — Notas estratégicas

### Relação com P259 (Visualize Fase A) — template directo

P259 foi último audit Fase A pré-formalização P260. P266 é
**primeiro audit Fase A formal pós-P260** — consome ADR-0084
+ ADR-0085 directamente.

**Validação retrospectiva ADR-0084 + 0085** cumprida via
exercício real num módulo grande (Text).

### Subpadrão "auditoria condicional" cresce N=5 → N=6

Cumulativo:
- N=1 P192A.
- N=2 P255 (Math).
- N=3 P257 (Color).
- N=4 P258 (Model).
- N=5 P259 (Visualize).
- **N=6 P266** (Text — primeiro consumo directo formal).

**Patamar N=6 excede limiar formalização clara**. Pattern
sólido confirmado retroactivamente; ADR-0084 EM VIGOR
validada.

### Subpadrão "diagnóstico imutável precedente à acção" N=6 → N=7

Cumulativo:
- N=1-4 P255/P257/P258/P259 (audit Fase A).
- N=5 P262 (Gradient Linear vanilla).
- N=6 P264 (Gradient Radial vanilla).
- **N=7 P266** (se Fase A produzir diagnóstico Text Fase A
  imutável — primeiro consumo directo formal ADR-0085).

**Patamar N=7 reforça pattern sólido**.

### Política "sem novas reservas"

Preservada. Recomendações P266.C Opções são para validação
humana.

### Comparação cumulativa audits

| Audit | Módulo | Cobertura pré | Cobertura empírica | Δ | Status pós |
|-------|--------|---------------|--------------------|----|-----------|
| P255 | Math (DEBT-8) | parcial | fechado | + | ENCERRADO |
| P257 | Color | 25% | 100% estrutural | +75pp | 8/8 |
| P258 | Model | ~48% declarado | ~73% empírico | +25pp | B1 fecho |
| P259 | Visualize | ~60-65% estim | ~52% factual | -8 a -13pp | B2 sub-passos |
| **P266** | **Text** | **~52% citado** | **? esperado ~80-85%** | **? esperado +30pp** | **? esperado B1** |

**Hipótese auditável**: Text segue padrão Model/Color
(empírica > citada).

### Pós-P266 — sequência lógica recomendada

Conforme decisão Fase B:
- **B1 provável**: pivot para outro módulo:
  - P-Gradient-Conic (último Gradient variant).
  - P-Footnote-N (Model pendência P258).
  - DEBT-33 (Bézier bbox + Stroke<Length>).
- **B2 Opção 1**: P267 ADR-0055bis variant-aware (M).
- **B2 Opção 2**: P267 ADR-0056 subsetting (M-L).
- **B2 Opção 3**: P267 Refinos Raw (S+).

---

## §9 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0019, ADR-0027, ADR-0029, ADR-0033, ADR-0034, ADR-0038,
  ADR-0039, ADR-0052, ADR-0053, ADR-0054, ADR-0055, ADR-0057,
  ADR-0065, ADR-0080.
- **ADR-0084, ADR-0085** (P260 — primeiro consumo directo
  formal aqui).
- DEBT-1 (fechado P142; preservado), DEBT-52 (fechado P142;
  preservado), DEBT-53 (em aberto XL; anotado P266).
- `diagnostico-text-passo-266.md` — diagnóstico pai
  (planeamento Fase A/B).
- `fase-a-checklist-text-passo-266.md` — comandos exactos
  P266.A.
- P21, P30, P99, P100, P126-P139, P140B, P141, P142, P144,
  P146, P155 — materializações Text cumulativas.
- P192A, P255, P257, P258, P259 — precedentes "auditoria
  condicional".
- P260 — ADRs meta (formaliza ADR-0084/0085 consumidos
  directamente por este passo).
- P262, P264 — diagnósticos vanilla Gradient (precedentes
  diagnóstico imutável).
- Vanilla `lab/typst-original/crates/typst-library/src/text/`
  — fonte canónica (leitura Bloco 1 Fase A obrigatória).
