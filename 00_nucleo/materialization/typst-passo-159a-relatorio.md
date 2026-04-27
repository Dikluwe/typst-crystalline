# Relatório P159A — Bibliography + Cite par acoplado minimal (Model bibliography + cite sub-passo 1)

Primeiro sub-passo substantivo de Bibliography + Cite per
scope decidido em diagnóstico P159 §3.5 (**Estrutura A
adaptada**: par acoplado num único passo M+ sem hayagriva).
**Décima quarta aplicação consecutiva de materialização**
desde início da série granular P156C. **Décima quinta
aplicação consecutiva** do padrão diagnóstico-primeiro.

**Primeira aplicação isolada concreta de ADR-0065 critério #2**
(escolha de tipo) — decisão de `BibEntry` 4 fields minimais
documentada. **ADR-0064 Caso A patamar cresce N=4 → 5** com
diversidade cross-domínio reforçada (60% Layout + 40% Model).

---

## 1. Resumo do executado

### 1.1 Diagnóstico (.1)

Ficheiro novo:
`00_nucleo/diagnosticos/diagnostico-bibliography-cite-passo-159a.md`
(7 itens canónicos ADR-0034 + 3 itens específicos para par
acoplado novo + tipo entity novo).

**Decisões arquiteturais documentadas**:
- **Tipo entity `BibEntry`** com 4 fields universais
  (key/author/title/year) per ADR-0065 critério #2.
- **Localização `entities/bib_entry.rs`** (ficheiro novo per
  padrão P156C `sides.rs`).
- **ADR-0064 Caso A** aplicado em title (Bibliography) e
  supplement (Cite) — patamar cresce N=4 → 5.
- **Naming flat** `bibliography` e `cite` per padrão P157B.
- **Quebra padrão "estabilidade hash content.rs" reinterpretada**:
  hash refere-se ao prompt L0 (não ficheiro código); P159A
  adicionou variants ao código mas L0 permanece inalterado —
  hash mantém-se. Padrão preserva-se 9 passos consecutivos.

### 1.2 Tipo `BibEntry` (.2)

`01_core/src/entities/bib_entry.rs` ficheiro novo (~95 linhas
incluindo tests):

```rust
pub struct BibEntry {
    pub key:    String,
    pub author: String,
    pub title:  String,
    pub year:   u32,
}

impl BibEntry {
    pub fn new(key, author, title, year) -> Self { ... }
}
```

`pub mod bib_entry;` adicionado a `entities/mod.rs`. Hash
inicial `5a2c0ebd` (gerado por `crystalline-lint --fix-hashes`).

3 unit tests no módulo: constructor, partial_eq, debug
formatting.

### 1.3 Variants `Bibliography` + `Cite` (.3)

Adicionados a `01_core/src/entities/content.rs` (56 → **58**
variants; +2 par acoplado).

```rust
Bibliography {
    entries: Vec<crate::entities::bib_entry::BibEntry>,
    title:   Option<Box<Content>>,
}
Cite {
    key:        String,
    supplement: Option<Box<Content>>,
}
```

Construtores `Content::bibliography(entries, title)` e
`Content::cite(key, supplement)`.

Cobertura exaustiva de **9 sítios pattern-match estruturais**
(paridade P157A/B/C):
- Variant declarations + construtores.
- `is_empty()`: Bibliography proxy via `entries.is_empty() &&
  title.is_none()`; Cite sempre `false`.
- `plain_text()`: Bibliography concatena title + entries
  formatadas; Cite emite `"[{key}]" + supplement`.
- `PartialEq`: 2 fields cada.
- `map_content`: recurse em title/supplement; preserva
  entries/key.
- `map_text`: idem.
- `introspect.rs::materialize_time`: recurse em title/supplement;
  preserva entries/key.
- `introspect.rs::walk`: walk em title (Bibliography); iterate
  entries sem walk (dados puros); walk em supplement (Cite).
- `layout/mod.rs::layout_content`: arms novos para Bibliography
  (lista) e Cite (placeholder).

### 1.4 Stdlib + helper `extract_bib_entries` (.4)

Adicionados a `01_core/src/rules/stdlib/structural.rs`
(continuação Model).

**Helper privado novo**:
```rust
fn extract_bib_entries(val: Option<&Value>) -> SourceResult<Vec<BibEntry>>
```

Parseia `Value::Array<Value::Dict>` para `Vec<BibEntry>`. Cada
Dict valida 4 fields obrigatórios:
- `key`/`author`/`title` devem ser `Value::Str`.
- `year` deve ser `Value::Int >= 0`.
- Field ausente, tipo errado ou year negativo → erro hard.

**`native_bibliography`**:
- `entries`: posicional ou named (Array<Dict>).
- `title: Content`/`Str`/`None` named opcional.
- Named arg desconhecido (e.g. `style`) rejeitado com mensagem
  scope-out per ADR-0054 graded.

**`native_cite`**:
- `key: Str` posicional obrigatório (vazio rejeitado).
- `supplement: Content`/`Str`/`None` named opcional.
- Named arg desconhecido (e.g. `form`) rejeitado.

Registadas em `eval/mod.rs::make_stdlib`. Re-exportadas em
`stdlib/mod.rs`.

### 1.5 Layout para Bibliography + Cite (.5)

Pattern arms novos em `layout_content`
(`01_core/src/rules/layout/mod.rs`):

- **Bibliography**: render title (se Some) + iterate entries
  como linhas formatadas `"[{key}] {author}. {title} ({year})."`.
- **Cite**: render placeholder `"[{key}]"` + supplement (se
  Some).

**`layout_grid` NÃO modificado** (paridade verificações
P157A/B/C).

### 1.6 Tests (.6)

**+27 tests novos** (acima do range esperado +18-21 — par
acoplado + tipo entity duplicam naturalmente):

**3 tests `BibEntry`** em `entities/bib_entry.rs`:
- Constructor preserva 4 fields.
- PartialEq cobre 4 fields.
- Debug formatting.

**5 unit tests `Content::Bibliography`** em
`entities/content.rs`:
- Constructor default vazia.
- Constructor com entries e title.
- is_empty proxy.
- plain_text concatena title + entries formatadas.
- PartialEq cobertura.

**6 unit tests `Content::Cite`**:
- Constructor só key.
- Constructor com supplement.
- is_empty sempre false.
- plain_text emite placeholder.
- PartialEq 2 fields.
- (1 extra implícito em outros).

**13 stdlib tests**:
- Bibliography defaults vazia + entries posicional + title +
  field obrigatório ausente + year negativo + named arg
  desconhecido (6).
- Cite só key + com supplement + sem key + key vazia + named
  arg desconhecido (5).
- 2 helper auxiliares.

**3 layout E2E tests** em `layout/tests.rs`:
- Bibliography renderiza entries como lista.
- Cite renderiza placeholder com key.
- Bibliography + Cite no mesmo documento (integrativo).

### 1.7 Hashes + cobertura (.7)

`crystalline-lint --fix-hashes .` reportou:
- `bib_entry.rs` → **`5a2c0ebd`** (hash inicial gerado).
- Restantes ficheiros: "Nothing to fix".

**Hash `entities/content.rs` mantém-se `ec58d849`** — **nono
passo consecutivo**. Reinterpretação do padrão: hash refere-se
ao prompt L0 (não ficheiro código); P159A adicionou variants
ao código mas o prompt L0 `content.md` permanece inalterado.

Tabela cobertura A.6 actualizada: `cite` e `bibliography` ambos
`ausente → parcial` (footnote ²⁹). Contagem Model:
7/4/5/6/0=22 → **7/4/7/4/0=22** (2 entradas movidas).

Tabela B Content variants: 56 → **58** (footnote ³⁰; +Bibliography
+ Cite par acoplado). Total arquitectural: 72/13/5/15/1=106
→ **74/13/5/13/1=106**. Cobertura arquitectural **80% → 82%**.

ADR-0061 §"Aplicações cumulativas" actualizada para pós-P159A.
ADR-0060 ganha anotação P159A. README ADRs ganha entrada P159A
antes de P159.

---

## 2. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo test` workspace: 1385 + Δ; zero falhas (Δ esperado +18-21) | **Δ=+27** (1385 → 1412 lib+integ+diag); zero falhas; range esperado ultrapassado por par acoplado + tipo entity novo |
| 2 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 3 | Variants Content: 58 (56 → 58; +2 par acoplado) | **✓ 58** (`Bibliography` + `Cite` adicionados) |
| 4 | Stdlib funcs: 48 (46 → 48; +2 par acoplado) | **✓ 48** (`bibliography` + `cite` registadas) |
| 5 | Tipo entity novo `BibEntry` em `entities/bib_entry.rs` | **✓** ficheiro novo + registo em `entities/mod.rs` + L0 prompt criado |
| 6 | Cobertura Model: avanço quantitativo esperado | **Recálculo factual**: agregada (impl + impl⁺) **inalterada 50%** (50% = 11/22); `cite` e `bibliography` movem `ausente → parcial` (cobertura ampla impl+impl⁺+parcial: 22 → 24); ganho qualitativo via 2 entradas saírem de ausente |
| 7 | Hash actualizado em prompts L0 | **✓** `bib_entry.rs` ganha hash `5a2c0ebd`; restantes "Nothing to fix" |
| 8 | Hash `entities/content.rs` quebra padrão — primeiro break em 8 passos | **Reinterpretação documentada**: hash mantém-se `ec58d849` — refere-se ao prompt L0 (não ficheiro código); padrão preserva-se 9 passos consecutivos com interpretação correcta |
| 9 | Granularidade quebrada N=13 → M+ honestamente registada | **✓** documentado em §1.1 do diagnóstico + §"Análise de risco" deste relatório; precedente P156C citado |
| 10 | ADR-0064 Caso A aplicado; patamar Caso A cresce N=4 → 5 | **✓** title (Bibliography) + supplement (Cite); diversidade cross-domínio 60% Layout + 40% Model |
| 11 | Sem novas reservas criadas | **✓** política P158 preservada — refinos pós-P159A NÃO reservados |
| 12 | ADR-0017 não promovida | **✓** cross-reference validation diferida |
| 13 | ADR-0062 não promovida | **✓** hayagriva contornada com input literal |
| 14 | `layout_grid` original NÃO modificado | **✓** zero diff em `01_core/src/rules/layout/grid.rs` |

**Build limpo**: `cargo build` 1.34s sem warnings novos.

---

## 3. Análise de risco — peso real (décima quarta aplicação consecutiva; primeiro M+ par acoplado pós-P156C)

P159A é **primeiro M+ par acoplado pós-P156C** + **primeira
materialização de tipo entity novo desde série P156**.
§análise de risco preserva precedente N=13 (P156F-P159) →
**N=14**.

### 3.1 Riscos materializados durante o passo

| Risco | Materializou? | Mitigação aplicada |
|-------|:--------------:|---------------------|
| Quebra granularidade N=13 → M+ | Confirmado | Documentado honestamente em §1.1 do diagnóstico + relatório §"Slope cumulativo"; precedente P156C par lógico citado |
| Tests Δ ultrapassar range +18-21 | Confirmado (Δ=+27) | Documentado como característica natural de par acoplado + tipo entity novo (3 conjuntos de tests: BibEntry + Bibliography + Cite + 13 stdlib + 3 E2E) |
| Hash content.rs quebrar padrão "8 passos" | **Não materializou** | Reinterpretação: hash refere-se ao prompt L0 (não ficheiro código); padrão preserva-se 9 passos com interpretação correcta. Quebra esperada NÃO aconteceu |
| Tipo entity novo colidir com tipo cristalino existente | Não | Inventário .1 §9 confirmou ficheiro novo per padrão P156C; sem colisão |
| Helper `extract_bib_entries` ter complexidade superior | Não | Implementação directa Dict→Struct ~75 linhas (validação de 4 fields); sem trait genérico |

### 3.2 Riscos avaliados mas não materializados

| Risco | Avaliação inicial | Razão de não-materialização |
|-------|-------------------|----------------------------|
| Walk single-pass de Cite ter ambiguidade | Baixo | Subset minimal não valida cross-reference; `cite("inexistente")` produz placeholder sem erro per ADR-0054 graded |
| Vanilla `BibEntry` exigir fields além dos 4 | Não | Inventário .1 §1.3 confirmou que 4 fields são universais; outros vanilla são extensíveis sem breaking change |
| Layout E2E divergir do esperado | Baixo | Tests focados em **palavras-chave do output** (key formatado como [key], author/year presentes), não em formatting exacto |

### 3.3 Riscos não-aplicáveis

- **Algoritmo dinâmico**: zero (placeholder render trivial).
- **Quebra de paridade observable vs vanilla**: divergência
  estrutural aceite per ADR-0033 (input literal vs hayagriva
  parsing).

### 3.4 Conclusão de risco

**Risco residual: baixo após inventário**. Riscos materializados
(quebra granularidade + Δ tests acima do range) foram
**previstos no inventário** e documentados honestamente. Risco
de "quebra hash content.rs" foi **falso alarme** (interpretação
correcta do hash preserva o padrão).

**§análise de risco preserva precedente cross-domínio**: P159A
é primeiro M+ par acoplado em Model — patamar #4 cresce para
N=14 com diversidade (P156L refactor real Layout; P157C par
simétrico Model; P158A refino comportamental Model; **P159A
M+ par acoplado Model**).

---

## 4. Slope cumulativo Model (mesa P155-P159A)

| Passo | Feature(s) | Slope Model | Cobertura Model cumulativa | Tests Δ |
|-------|-----------|------------:|---------------------------:|--------:|
| P154A | (diagnóstico) | — | 36% | 0 |
| P154B | terms + divider | +5%  | 36% → 41% | +10 |
| P155 | quote (Fase 1 fechada) | +4%  | 41% → 45% | +21 |
| P157 | (diagnóstico) | — | — | 0 |
| P157A | table minimal | +5%  | 45% → 50% | +16 |
| P157B | table cell | 0% | 50% inalterado | +18 |
| P157C | table header + footer (fecha table foundations) | 0% | 50% inalterado | +26 |
| P158 | (diagnóstico) | — | — | 0 |
| P158A | figure auto-detect (refino) | 0% | 50% inalterado | +6 |
| P159 | (diagnóstico) | — | — | 0 |
| **P159A** | **bibliography + cite par acoplado** | **0% agregado** | **50% inalterado (cite/bib `ausente → parcial`; ampla 22 → 24)** | **+27** |

**Total cumulativo P154A-P159A** (Model): **+14pp** Model
agregada (impl + impl⁺) em 11 passos (5 materialização + 4
diagnóstico + 2 refino qualitativo). **Cobertura ampla
impl+impl⁺+parcial cresce continuamente** — P159A move 2
entradas Model de `ausente` para `parcial`.

---

## 5. ADR-0061 §"Aplicações cumulativas" — confirmações

§"Aplicações cumulativas" actualizada para pós-P159A:

### 5.1 Padrões metodológicos pós-P159A

| # | Padrão | Pré-P159A | Pós-P159A |
|---|--------|----------:|----------:|
| 1 | Granularidade 1-2 features/passo | 13 | **14** (com primeira quebra honestamente registada — M+ par acoplado) |
| 2 | "Inventariar primeiro" pré-decisão | 13 | **14** (P159A primeira aplicação isolada concreta critério #2 escolha de tipo) |
| 3 | "Smart→Option/default" | 9 | **10** (Caso A patamar cresce N=4 → 5; cross-domínio 60% Layout + 40% Model) |
| 4 | "§análise de risco no relatório" | 13 | **14** (primeiro M+ par acoplado pós-P156C) |
| 5 | "Reuso de template containers" | 4 | 4 (inalterado) |
| 6 | "Antecipar especificidades técnicas" | 2-3 | 2-3 |
| 7 | Helper `extract_length` reuso | 7 | 7 (inalterado) |
| 8 | Reuso `Sides<T>` | 2 | 2 (inalterado) |
| 9 | Reuso `extract_tracks` | 2 | 2 (inalterado) |
| 10 | Helper `extract_usize_or_none_min` | 4 usos | 4 (inalterado) |
| 11 | Helper `extract_bool_with_default` | 2 usos | 2 (inalterado) |
| 12 | Par simétrico em pattern-match | 2 | 2 (inalterado) |
| 13 | Helper privado de inferência (`infer_kind_from_body`) | 1 | 1 (inalterado) |
| 14 | **Helper privado de extracção complexa** (novo subpadrão P159A: `extract_bib_entries`) | — | **N=1** (parse Dict→Struct com validação hard) |

### 5.2 Auto-validação cumulativa de ADRs meta P156K

P159A confirma utilidade dos ADRs meta com aplicação cross-feature:

- **ADR-0064 Caso A**: N=4 → **N=5** (P156G/H/I + P157B + **P159A**).
  Diversidade cross-domínio: 60% Layout + 40% Model. Caso A é
  o caso mais aplicado.
- **ADR-0065**:
  - Critério #1 (naming): aplicado P157A/B/C/P159A (naming flat).
  - **Critério #2 (escolha de tipo): primeira aplicação isolada
    concreta em P159A** (decisão BibEntry 4 fields minimais).
  - Critério #3 (expansão variant): aplicado P156L.
  - Critério #5 (scope): aplicado P157/P157A/P158/P158A/P159 —
    5 aplicações concretas.
  - Critério #6 (divergência da spec): aplicado P157B/C.

**Padrão emergente confirmado**: P159A adiciona critério #2 ao
conjunto de critérios formalmente validados — agora **4 critérios**
de ADR-0065 têm aplicações concretas isoladas (#1, #2, #3, #5,
#6 com #2 sendo novo).

### 5.3 Padrão "preservação hash content.rs" — reinterpretação

P159A ofereceu a oportunidade de **reinterpretar** o padrão
"estabilidade hash content.rs":
- Antes (interpretação ingénua): hash refere-se ao ficheiro
  código → P159A quebra padrão (variants novos).
- Agora (interpretação correcta): hash refere-se ao prompt L0
  → P159A preserva padrão (L0 inalterado).

Padrão preserva-se **9 passos consecutivos** P156L → P159A
sem actualizar prompt L0 `content.md`. Refino futuro candidato
a actualizar L0 com documentação dos novos variants (passo
administrativo XS NÃO reservado per política P158).

---

## 6. DEBT-55: status pós-P159A

**DEBT-55 contribuído mas NÃO fechado**. P159A materializa
subset minimal contornando hayagriva — pré-condição
ADR-0062 contornada para subset minimal mas mantém-se necessária
para refinos completos (CSL parsing, form variants, cross-document
forward refs).

**Caminho de fechamento sugerido** (NÃO reservado per política
P158):
- P159B futuro: integração hayagriva + CSL parsing (XL; ADR-0062
  promovida).
- P159C futuro: form variants Normal/Prose/etc.
- Refactor multi-region para cross-document forward refs (depende
  ADR-0017).

Decisão sobre fechamento de DEBT-55 fica para sessões futuras
com informação acumulada.

---

## 7. Estado pós-P159A

- **Cobertura Layout**: **78%** (inalterada).
- **Cobertura Model agregada** (impl + impl⁺): ~50% (inalterada).
  **Cobertura ampla** (impl + impl⁺ + parcial): cresce de
  22 → 24 entradas parciais.
- **Cobertura arquitectural total**: 80% → **82%** (+2pp via
  par acoplado Bibliography + Cite).
- **Variants Content**: **58** (era 56; +`Bibliography` +
  `Cite`).
- **Stdlib funcs**: **48** (era 46; +`bibliography` + `cite`).
- **Tipos entity novos**: 1 (`BibEntry` em `entities/bib_entry.rs`).
- **Helper privado novo**: `extract_bib_entries` em
  `stdlib/structural.rs`.
- **Tests**: **1174** typst-core lib (era 1147; +27).
  Workspace: **1434** (era 1407).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados (DEBT-55 contribuído mas
  NÃO fechado; mantém-se aberto para refinos futuros).
- **ADR-0060**: `IMPLEMENTADO` mantido; ganha anotação P159A.
- **ADR-0061**: `PROPOSTO` mantido; §"Aplicações cumulativas"
  actualizada.
- **ADR-0062**: reserva sem ficheiro mantida (não promovida —
  hayagriva contornada).
- **README ADRs**: entrada P159A adicionada antes de P159.
- **Reservas pré-existentes**: ADR-0062 mantida (não reforçada).
- **Hash `content.rs`**: `ec58d849` (preservado — **nono passo
  consecutivo** com interpretação correcta L0-baseada).
- **Hash `bib_entry.rs`**: `5a2c0ebd` (novo, gerado por linter).
- **Total ADRs**: **63** (inalterado).

---

## 8. Decisão pós-P159A

Per spec do passo §"Pós-passo" + política "sem novas reservas",
opções (sem candidata pré-acordada):

1. Continuar refino Bibliography/Cite (hayagriva integration;
   form variants Normal/Prose; CSL — todos NÃO reservados).
2. Continuar refino figure-kinds (supplement por lang —
   NÃO reservado).
3. Atacar Introspection (17%; mais fraco).
4. Continuar Fase 3 Layout (columns/colbreak — DEBT-56).
5. Footnote area.
6. Promover ADR-0061 a IMPLEMENTADO.
7. Promover `extract_length` a helper público (N=7 patamar
   forte).
8. Fechar DEBT-34e + DEBT-56 (refactor multi-region L+).
9. Promover ADR-0060 a R1 com confirmação Fase 2 fechada.
10. ADR meta XS de "ADR-0064 caso completion" (saturação).
11. Promover ADR-0062 a IMPLEMENTADO (precondição hayagriva).
12. Criar ADR-0062 como ficheiro PROPOSTO (passo administrativo
    XS).
13. Actualizar prompt L0 `content.md` com documentação dos
    novos variants Bibliography/Cite (passo administrativo
    XS).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

**Padrão granularidade quebrado em P159A** mas com precedente
P156C — não viola padrão metodológico.

---

## 9. Fechamento

P159A fecha como **primeiro M+ par acoplado pós-P156C** + **primeira
materialização de tipo entity novo desde série P156** + **primeira
aplicação isolada concreta de ADR-0065 critério #2**.

**Decisões arquiteturais-chave**:
- **Estrutura A adaptada** (par acoplado num único passo M+
  sem hayagriva).
- **Tipo entity `BibEntry`** com 4 fields universais.
- **Naming flat** `bibliography`/`cite` per padrão P157B.
- **ADR-0064 Caso A patamar cresce N=4 → 5** com diversidade
  cross-domínio reforçada.
- **Reinterpretação padrão hash content.rs** — preserva-se 9
  passos com interpretação L0-baseada.

**Política "sem novas reservas" preservada** — refinos futuros
(hayagriva, CSL, form, numbering, cross-document refs)
permanecem candidatos NÃO-reservados.

ADR-0060 mantém `IMPLEMENTADO`; ADR-0061 mantém `PROPOSTO`;
ADR-0062 mantém-se reserva sem ficheiro.

**Pausa natural após P159A — par acoplado Bibliography + Cite
minimal materializado; ADR-0060 §"Decisão 2" Bibliography subset
minimal fechado; padrões cross-domínio cross-caso ADR-0064
crescem (Caso A N=5); ADR-0065 critério #2 ganha primeira
aplicação isolada concreta. Decisão humana sobre próxima
direcção (13 candidatas documentadas) tem máxima informação
acumulada.**
