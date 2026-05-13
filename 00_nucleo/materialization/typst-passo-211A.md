# Passo 211A — Diagnóstico-primeiro: Outline configurável

**Série**: 211 (sub-passo `A` = diagnóstico-primeiro
reduzido).
**Marco**: M9c (Bloco VII — Outline configurável).
**Tipo**: diagnóstico-primeiro reduzido (zero código
tocado).
**Magnitude**: S-M (~45 min).
**Pré-condição**: P210C concluído; série P210 fechada;
ADR-0076 PROPOSTO; ADR-0077 ACEITE; trait 26 métodos;
Selector 6 variants; stdlib funcs ~53; tests 1939
verdes; 0 violations; blueprint §3.0septies.
**Output**: 1 ficheiro (relatório curto consolidando
auditoria + decisões + plano P211B+).

---

## §1 Trabalho

Mapear empíricamente o gap entre `outline()` actual
cristalino e o target Bloco VII P207A.div-1 aprovado:

- **Item 55a** — `outline(target: ElementKind, depth,
  indent, fill)`. M-L (~4-6h estimado P207A).
- **Item 55b** — Outline params (depth, indent, fill)
  inline.

**Caveat crítico já documentado** (P207A.div-1 +
P206C/P206D D5): outline-toc divergência arquitectónica
existe (cristalino auto-toc; vanilla counts only
outline body). Bloco VII expande outline para outros
targets mas **não muda essa divergência principal**.

P211A produz:
1. Mapeamento empírico outline cristalino actual.
2. Decisão Caminho 1/2/3 com base em consumers reais.
3. Plano P211B+ (sub-passos sem ramos).

Reuso de dados toda a trajectória M9c:

- Pattern diagnóstico-primeiro reduzido (P208A/P209A/P210A).
- Pattern "Caminho 1 anti-inflação" 7 aplicações.
- Pattern "Caminho 3 honest subset" 1 aplicação (P210).
- Convenção L0 emergente para stdlib funcs P169+.

---

## §2 Cláusulas de auditoria (A1–A5)

### A1 — `outline()` cristalino actual

Localizar literalmente:

- Stdlib func `native_outline` em
  `01_core/src/rules/stdlib/`. Esperado: pre-M5 ou P200
  série.
- Assinatura actual: args aceites; default behavior.
- Show-rule ou eval-time? `outline()` em `.typ` source
  resolve quando?
- Tests E2E existentes (per P200 série).

Output: 6-10 linhas literais.

### A2 — `outline()` vanilla

Localizar literalmente em
`lab/typst-original/crates/typst-library/src/model/outline.rs`:

- Assinatura completa do constructor `outline()` +
  params (`target`, `depth`, `indent`, `fill`, etc.).
- Tipos exactos dos params.
- Show-rule expansão para conteúdo do outline.

Output: ~10-15 linhas literais.

### A3 — Outline-toc divergência arquitectónica
(re-confirmar)

Per P206C/P206D D5:

- Cristalino emite auto-toc para outline-toc.
- Vanilla counts only outline body.
- Item 55a **não muda** essa divergência principal.

Confirmar empíricamente que divergência ainda existe
em estado actual (pós-P210). Re-grep relevantes em
outline impl + show-rule.

### A4 — Consumers reais imediatos para outline
configurável

Re-grep em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/` por:
- Tests existentes que invocam outline com targets
  diferentes de heading default (figure, equation,
  etc.).
- `.typ` source fixtures com params customizados.

Esperado per pattern M9c: zero ou muito poucos
consumers.

Output: contagem literal.

### A5 — Custo empírico item 55a

Para cada sub-feature de item 55a:

- `target: ElementKind` — usa Selector ou string? Custo:
  pequeno (refactor signature).
- `depth: usize` — limita níveis. Custo: pequeno
  (filter no walk).
- `indent: Length` — formatação. Custo: pequeno
  (CSS-like).
- `fill: Content` — padrão entre numeração e título.
  Custo: pequeno (text element).

Esperado: cada sub-feature ~S; agregado M se 4 features.

---

## §3 Cláusulas de decisão (C1–C5)

Fixadas **depois** da auditoria.

### C1 — Forma do outline configurável

Per A5 — esperado: 4 sub-features triviais
independentes. Para cada uma:

- Manter assinatura actual `outline()` zero-arg como
  default (preserva tests existentes).
- Adicionar named args opcionais: `outline(target:
  ..., depth: ..., indent: ..., fill: ...)`.

C1 fixa assinatura literal final.

### C2 — Como expor `target`

Decisão crítica:

- **Opção α** — `target: ElementKind` (enum). Cristalino
  tem `ElementKind`; consumer passa via Value
  dispatch. Paridade vanilla.
- **Opção β** — `target: Selector`. Reusa P209 trabalho;
  paridade vanilla parcial; mais flexível mas custo
  ligeiramente maior.
- **Opção γ** — adiar `target` (só depth/indent/fill);
  default heading-only.

Hipótese provável: Opção α (mais simples) ou γ (anti-inflação
se zero consumers).

### C3 — Caminho 1/2/3

Decisão honesta com base em A4+A5:

- **Caminho 1 — Adiar série inteira P211**: zero
  consumers reais; pattern anti-inflação 8ª/9ª aplicação.
  P211 inteira documental (skip ou minimal anotações em
  ADR-0076 bloco P211).
- **Caminho 2 — Materializar item 55a literal**: 4
  sub-features + tests sintéticos. Honra
  `P207A.div-1` aprovado.
- **Caminho 3 — Subset minimal**: 1-2 sub-features
  triviais (ex: `depth` que é mais comum); outras
  deferred.

Critério: honesta + valor real adicionado vs custo.

Hipótese provável: **Caminho 1 ou 3**.

- Caminho 1 se A4 = zero consumers absoluto E outline
  actual cobre needs M5+P200 sem fricção visível.
- Caminho 3 se 1-2 sub-features tipicamente solicitadas
  (depth, fill).

Pattern P210 (Caminho 3) consolidado pode aplicar-se.

### C4 — Plano P211B+

Sub-passos sem ramos. Quantidade depende de C3.

- Se Caminho 1: só P211A + skip directo para P212.
- Se Caminho 2: P211B-E (4 sub-passos: 1 por sub-feature).
- Se Caminho 3: P211B (subset) + P211C encerramento.

### C5 — Magnitude agregada P211

- Caminho 1: 0 código (apenas P211A diagnóstico).
- Caminho 2: M-L (~4-6h per P207A estimativa).
- Caminho 3: S-M (~1.5-2h).

---

## §4 Output

1 ficheiro:
`00_nucleo/diagnosticos/typst-passo-211A-relatorio.md`.

Estrutura (~5-7 KB) com 6 §s padrão (paralelo a P208A /
P209A / P210A).

---

## §5 Não-objectivos

- Materializar params (P211B+ se Caminho 2/3).
- Resolver divergência outline-toc principal (P206C/D5;
  out-of-scope M9c).
- `outline.entry()` constructor func (vanilla; out of
  P207A.div-1).
- Trait method extensions.
- ADR nova ou transição.

---

## §6 Riscos a evitar

1. **Inflar para Caminho 2 por "completude vanilla"**:
   per `P205A.div-1` divergência arquitectónica legítima.
   Se zero consumers, anti-inflação aplica.
2. **Aceitar Caminho 2 sem critério**: pattern M9c —
   8 aplicações anti-inflação consecutivas. Justificar
   Caminho 2 exige evidência forte de consumer real.
3. **Confundir Bloco VII com resolução de divergência
   outline-toc principal**: item 55a apenas adiciona
   targets; divergência cristalino auto-toc vs vanilla
   outline-body **não** se resolve aqui.
4. **Pre-fixar Caminho 1 por consistência com pattern**:
   honestidade empírica vai em qualquer direcção. Se
   A4 revelar consumers reais (heading-only outline
   cobre 80% mas figure-outline pedido por 1-2 fixtures),
   Caminho 3 minimal justifica-se.
5. **Inflar diagnóstico**: P211A é reduzido.
6. **Esquecer regra P207B §5**: outline é stdlib func,
   não trait method. Trait mantém 26.

---

## §7 Hipótese provável

A1 mostrará `outline()` cristalino actual com
assinatura minimal (zero ou poucos args). A2 mostrará
vanilla com params completos. A3 confirmará divergência
ainda existe. **A4 confirmará zero consumers reais**.
A5 mostrará 4 sub-features triviais (~S cada).

C1 fixará assinatura com defaults preservando
backwards-compat.

C2 fixará Opção α (`target: ElementKind`) por
simplicidade — OU γ adiar se Caminho 1 escolhido.

C3 fixará **Caminho 1 puro** se A4 zero consumers
absoluto — 8ª aplicação anti-inflação cumulativa,
paralelo a P208D / P209D C6 / P209E C1.2 ("encerramento
sem materialização").

OU **Caminho 3** se A4 revelar uso real para 1-2
sub-features (ex: depth limit).

C4 fixará plano consoante C3.

C5 reportará 0 código (Caminho 1) ou S-M (Caminho 3).

Mas é hipótese, não decisão. C1-C5 fixam-se
empíricamente.

---

## §8 Nota sobre roadmap final M9c

Se P211A = Caminho 1:
- **P211 série fecha em 1 sub-passo** (apenas A
  diagnóstico).
- M9c restam só P211A documental + P212 encerramento
  M9c inteiro.
- Custo M9c final estimado: ~18.5h cumulativo (acima
  P210 ~17.5h + ~1h P211A + ~1h P212).

Se P211A = Caminho 3:
- **P211B + P211C** (2 sub-passos materialização +
  encerramento).
- Custo M9c final: ~20-21h.

Se P211A = Caminho 2:
- P211B-E (4 sub-passos).
- Custo M9c final: ~22-25h.

**Qualquer das opções está bem dentro do orçamento
~30-50h aprovado**. Honestidade empírica é critério
principal — não orçamento.
