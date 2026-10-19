# Passo 209A — Diagnóstico-primeiro: `Selector` enum extension design

**Série**: 209 (sub-passo `A` = diagnóstico-primeiro
reduzido).
**Marco**: M9c (Bloco VI — Selector extensions).
**Tipo**: diagnóstico-primeiro reduzido (zero código
tocado).
**Magnitude**: S-M (~45 min).
**Pré-condição**: P208D concluído; série P208 fechada;
trait 26 métodos; stdlib funcs ~52; `Selector` cristalino
P175 minimal (`Kind(ElementKind)` only); tests 1907
verdes; 0 violations; ADR-0076 PROPOSTO anotado §P208D;
blueprint §3.0quinquies; Q-decisões P207A fixadas
(Q2=γ Where adiar; Q3=α Regex+Location materializar).
**Output**: 1 ficheiro (relatório curto consolidando
auditoria + decisões + plano P209B+).

---

## §1 Trabalho

Mapear empíricamente o gap entre `Selector` cristalino
actual (`Kind` only) e o target M9c per Q-decisões:

- **Materializar**: `Label`, `And`, `Or`, `Regex`,
  `Location` (5 variants novos).
- **Adiar**: `Where` (Q2=γ explicito), `Before`,
  `After`.

Decidir estrutura, dependências e plano P209B+.

Reuso de dados P207A + P208A:

- `Selector` em `01_core/src/entities/selector.rs` (per
  P207A A12 + P208A A4) com 1 variant `Kind(ElementKind)`.
- `Introspector::query(&Selector) -> Vec<Location>` (per
  P204B + P175).
- `ElementKind::from_name` (per P207B usage).
- `Value::Location(Location)` (P179).
- `native_query` + `native_locate` parsing patterns
  (P208C).

---

## §2 Cláusulas de auditoria (A1–A5)

Executada **primeiro**. Output empírico alimenta C1+.

### A1 — `Selector` cristalino actual

Localizar literalmente:

- `01_core/src/entities/selector.rs` — definição enum
  + derives.
- Hash impl (derived ou manual)?
- Consumers de `Selector` em `01_core/src/`,
  `02_shell/`, `03_infra/` — esperado: zero externos
  além de `Introspector::query`.
- L0 prompt existente
  `00_nucleo/prompts/entities/selector.md` — confirma
  estado P175 minimal.

Output: 4-6 linhas literais.

### A2 — `Selector` vanilla

Localizar literalmente em
`lab/typst-original/crates/typst-library/src/foundations/selector.rs`
ou similar:

- Enum completo com variants (`Kind`, `Label`, `And`,
  `Or`, `Where`, `Regex`, `Before`, `After`, etc.).
- Estrutura de cada variant (campos, tipos).
- Como `And`/`Or` são representados (`EcoVec<Self>` per
  P207A A13).
- Como `Regex` é representado (struct interno?
  pattern? bounds).

Output: ~10-15 linhas literais.

### A3 — `Introspector::query` impl actual

Como o impl cristalino reage a Selector com variants
novos?

- Esperado: `match selector { Kind(k) => ... }` exhaustivo
  hoje; precisa de arms para os 5 variants novos.
- Identificar literalmente onde se manifestaria a
  extensão (`TagIntrospector::query`).

Output: 3-5 linhas + edição mínima esperada.

### A4 — Regex dependência

Per P207A C9 + Q3=α: cristalino precisa de dep `regex`
em L1.

- `01_core/Cargo.toml` — deps actuais; existência de
  regex/regex-lite?
- ADR-0029 (allowlist deps L1) — política para nova
  dep.
- Alternativas: `regex` (full Unicode), `regex-lite`
  (sem Unicode; subset; ~30% mais pequeno).

Output: estado actual + 2 opções + custo de ADR
adicional.

### A5 — Parsing args de selector em stdlib

Per P208C: `native_locate` aceita arg `Value::Str(kind)`
e converte via `ElementKind::from_name`. Para `Label`,
`And`, `Or`, `Regex`, `Location`:

- `Label`: arg `Value::Str("<label-name>")` → parse →
  `Selector::Label`.
- `And`/`Or`: ?
- `Regex`: arg `Value::Str("pattern")`.
- `Location`: arg `Value::Location(loc)`.

Decisão sobre como expor compósitos (`And`/`Or`) em
stdlib args é não-trivial. Vanilla usa `selector(...)`
constructor func. Cristalino?

Output: comparação stdlib API cristalino vs vanilla.

---

## §3 Cláusulas de decisão (C1–C5)

Fixadas **depois** da auditoria. Sem condicionais.

### C1 — Estrutura dos 5 variants novos

Decisão com base em A1+A2:

- `Label(Label)` — trivial.
- `And(EcoVec<Self>)` — composição N-ária.
- `Or(EcoVec<Self>)` — composição N-ária.
- `Regex(Regex)` — depende de C2.
- `Location(Location)` — trivial.

Hash impl: derive se todos os fields são Hash; manual
para Regex se Regex crate não derivar Hash (verificar
em A4).

C1 fixa estrutura literal de cada variant.

### C2 — Dep regex L1

Decisão fixada com base em A4:

- **Caminho A — `regex` full**: Unicode; binário
  maior; pattern Rust standard.
- **Caminho B — `regex-lite`**: subset; sem Unicode;
  binário menor.
- **Caminho C — sem regex; `Selector::Regex` mantém
  pattern como string**: parsing diferido para consumer.
  Magnitude trivial mas semântica degenerada.

Critério: simplicidade + uso esperado. Hipótese
provável: Caminho A (`regex` full) per paridade vanilla.

C2 fixa **uma**. ADR-0077 nova ou anotação em ADR-0076
para dep adicional.

### C3 — Stdlib args API

Decisão com base em A5:

- `Label`: `Value::Str("<name>")`.
- `And`/`Or`: ?
  - **Opção (a)** — array de strings:
    `Value::Array([Value::Str, ...])` interpretado
    consoante context.
  - **Opção (b)** — constructor func stdlib
    `selector(...)` análoga a vanilla.
  - **Opção (c)** — adiar (`And`/`Or` apenas via Rust
    API; sem expressão em `.typ` source).
- `Regex`: `Value::Str("pattern")` ou novo
  `Value::Regex`.
- `Location`: `Value::Location(loc)` (já existe).

Critério: cobertura mínima viável para users `.typ`.

C3 fixa **uma** por variant.

### C4 — Plano P209B+ — desdobramento

Decisão sobre quantidade de sub-passos:

- **Caminho 1** — série única P209A-D (4 sub-passos):
  - P209B: `Label` + `Location` (trivial).
  - P209C: `And` + `Or` (composição).
  - P209D: encerramento série.
  - Regex fica para sub-passo dedicado P209-Regex
    pós-encerramento.

- **Caminho 2** — desdobrado P209A-E (5 sub-passos):
  - P209B: `Label` + `Location`.
  - P209C: `And` + `Or`.
  - P209D: `Regex` + ADR-0077 (ou anotação ADR-0076).
  - P209E: encerramento série.

- **Caminho 3** — sub-série dedicada P209 + P209-bis
  (Regex):
  - P209A-D fecha 4 variants triviais/compostos.
  - P209-bis (depois): Regex em série dedicada com
    ADR.

C4 fixa **uma**. Critério: magnitude por sub-passo
balanceada + ADR como passo formal vs anotação.

### C5 — Magnitude agregada P209

Reportar com base em C1-C4.

Range plausível:
- Caminho 1 + Caminho A (regex full): M (~4-5h).
- Caminho 2 + Caminho B (regex-lite): M (~4-5h).
- Caminho 3: S-M para P209 + S para P209-bis (~4-5h
  total).

---

## §4 Output

1 ficheiro:
`00_nucleo/diagnosticos/typst-passo-209A-relatorio.md`.

Estrutura (~5-8 KB) com 6 §s:

- §1 O que foi auditado (sumário 3-5 linhas).
- §2 Auditoria A1-A5 (tabelas compactas).
- §3 Decisões C1-C5.
- §4 Magnitude agregada P209.
- §5 Plano P209B+.
- §6 Próximo sub-passo (P209B).

---

## §5 Não-objectivos

- Materializar variants (P209B+).
- Adicionar deps a `Cargo.toml`.
- Tocar em código produção.
- `Selector::Where` (adiado per Q2=γ).
- `Selector::Before/After` (fora roadmap M9c).
- Trait method extensions.
- ADR transição PROPOSTO → ACEITE.

---

## §6 Riscos a evitar

1. **Inflar `Where` por simetria com vanilla**: Q2=γ
   fixou. Documentar como deferred; não materializar.
2. **Subestimar custo de Regex + ADR**: dep nova em L1
   exige ADR. Magnitude pode subir se ADR-0077 for
   formal. C2 + C4 endereçam.
3. **Trade-off `regex` vs `regex-lite`**: full Unicode
   é Rust standard mas binário maior. C2 fixa
   empíricamente.
4. **`And`/`Or` stdlib API confusion**: vanilla tem
   constructor func `selector(...)`; cristalino pode
   adiar via Rust API only. C3 fixa.
5. **Inflar diagnóstico**: P209A é reduzido per
   reformulação humana. 1 ficheiro; ~5-8 KB.
6. **Esquecer regra empírica P207B §5**: `Selector`
   extension **não toca trait `Introspector`** —
   apenas estende variants enum + arms em `query`
   impl. Trait mantém 26 métodos.

---

## §7 Hipótese provável

C1 fixará estrutura paralela a vanilla com Hash manual
para Regex (`Regex` crate não deriva Hash; pattern
string usada como key).

C2 fixará Caminho A (`regex` full) per pattern Rust
standard.

C3 fixará Opção (a) ou (c) para `And`/`Or` — Opção (b)
(constructor func stdlib) tem custo M+ desproporcional
sem caller real imediato.

C4 fixará Caminho 2 (desdobrado P209A-E) — Regex com
ADR merece sub-passo dedicado; magnitude balanceada.

C5 reportará M (~4-5h) total série P209.

Mas é hipótese, não decisão. C1-C5 fixam-se com base
em A1-A5.
