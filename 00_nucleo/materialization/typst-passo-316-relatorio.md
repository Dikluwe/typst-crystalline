# Relatório P316 — Lote piloto do modelo D (trait `Element`: Divider + Heading + MathStyled)

**Data**: 2026-06-10
**Tipo**: primeira implementação do candidato D (ADR-0105) — refatoração de
**comportamento idêntico**. Nenhuma semântica muda; a lógica muda de morada.
**Nota de número**: o ficheiro de passo estava nomeado `typst-passo-315.md` mas
descrevia a Tarefa **P316** (P315 foi absorvido pelo P314); renomeado para
`typst-passo-316.md`.
**Resultado**: ✅ entregue e validado. 2992 testes verdes (0 failed); lint 0 V7.

---

## 1. Pré-tarefas (commit `Passo 316 — pré-tarefas`)

- **Pre-1** ✅ `git rm` dos 2 índices L0 do P314 → `crystalline-lint` **0 V7**.
- **Pre-2** ✅ **DEBT-57** (specs L0 ausentes para ~70 funções stdlib).

## 2. Decisões de desenho (A.1)

1. **Trait `Element`** (`entities/elements/mod.rs`, depende só de `entities/`):
   absorve 5 dos 6 matches do hub — `plain_text`, `is_empty`, `map_content`,
   `map_text`, `get_field` (métodos genéricos, despacho estático; o trait
   não-object-safe é aceitável).
2. **`eq`/`hash`**: `eq` por `#[derive(PartialEq)]` no `NomeElem` (estrutural via
   `Arc<T>: PartialEq` compara o valor). `hash` continua via `content_hash`
   (Debug) — **interação registrada**: valores absolutos das 3 variantes mudam,
   relação preserva-se. **Verificado em Fase B**: nenhum teste fixa hash
   absoluto; `payload_diferente_produz_hash_diferente` (relacional) passa.
3. **Layout fica em `rules/`** (não no trait — topologia); só muda destructuring
   de `Arc<Elem>`. ⇒ `rules/layout.md` **não foi fatiado** (o imposto não morde
   neste passo).
4. **Absorção do locatável (Heading)**: `element_kind()` + `to_payload()` no
   trait; `extract_payload.rs` arm Heading → `h.to_payload()`. Enums
   `ElementKind`/`ElementPayload` permanecem para as outras variantes (estado
   misto, ADR-0105).
5. **Compat-F**: campos agrupados no `NomeElem` (props do descritor futuro).

## 3. Implementação (ordem piso→teto)

`Content::Divider` → `Divider(Arc<DividerElem>)`; `Content::MathStyled {…}` →
`MathStyled(Arc<MathStyledElem>)`; `Content::Heading {…}` →
`Heading(Arc<HeadingElem>)`. Construtores ergonómicos `Content::divider()` e
`Content::math_styled(...)` adicionados (Heading já tinha `Content::heading`).
Os 6 matches do hub despacham as 3 migradas ao trait; restantes 74 variantes
inline (estado misto esperado).

## 4. Validação

- **`cargo build --workspace`**: ✅ verde.
- **`cargo test --workspace`**: ✅ **2992 passed / 0 failed** (typst-core
  2462→**2473**, +11 dos testes unitários novos dos 3 elementos). **Comportamento
  idêntico**: nenhuma asserção de teste existente foi alterada — só a *sintaxe*
  de construção/match (`Content::Heading {…}` → `Content::heading(…)`). Ressalva
  da stack do P314 reaparece com stack default (teste `recursao_infinita…`;
  passa com `RUST_MIN_STACK` maior — artefacto de stack, não regressão).
- **`crystalline-lint .`**: **0 V7**, **0 drift** (pós `--fix-hashes`); as **3
  V9** pré-existentes em `03_infra` (`font_metrics.rs`, `layout.rs`) inalteradas
  (fora de escopo).
- **Warnings**: lib `typst-core` 5→5 (zero novos; as 3 `unreachable pattern` em
  `foundations.rs` + `Paint` são pré-existentes).

## 5. Medições (a métrica da ADR-0104)

### Encolhimento do hub (`content.rs`)

| | linhas |
|---|---|
| `content.rs` HEAD | 5782 |
| `content.rs` pós-P316 | **5785** (+3 líquido) |
| diff | +60 / −57 |

**Leitura honesta**: o hub **não encolheu ainda** no piloto (+3). Os braços
das 3 variantes encolheram (ex.: `map_content` MathStyled 8→2 linhas; `eq`
MathStyled 4→2), mas isso foi **compensado pelo custo fixo de setup**: 4 linhas
de import + 2 construtores (`divider`/`math_styled`, ~16 linhas). **O custo
marginal por variante adicional é negativo** — cada lote seguinte remove braços
sem repagar o setup, logo `content.rs` começa a encolher a partir daqui.

### Parte atómica (a morada nova da lógica)

| módulo | linhas (inclui testes) |
|---|---|
| `elements/mod.rs` (trait) | 71 |
| `elements/divider.rs` | 70 |
| `elements/heading.rs` | 117 |
| `elements/math_styled.rs` | 93 |
| **total** | **351** (≈105 são +11 testes unitários) |

### Custo-por-elemento medido (toque fora do módulo próprio)

| | valor |
|---|---|
| ficheiros `.rs` tocados (sites de construção/match) | **15** |
| linhas | **+153 / −264** (líquido **−111**) |
| Divider | ~5 ficheiros de produção |
| Heading (locatável) | ~7 de produção |
| MathStyled | ~7 de produção |

**O líquido é negativo (−111)**: as construções `Content::X {…}` → chamadas de
construtor são mais curtas. O custo **dominante e menos compressível** é
actualizar os **sites de construção/match** — proporcional a **quão larga é a
utilização** da variante (Heading é construído em ~15 sítios; uma variante rara
toca poucos). Isto é mensurável por `grep` **antes** de migrar cada variante.

### Projeção honesta (~74 restantes)

- Por variante: **1 módulo atómico** (~50–120 linhas; locatável custa mais —
  Heading 117) + **N sites** de construção/match (N = largura de uso, 3–15).
- O hub `content.rs` **encolhe marginalmente** por variante agora que o setup
  está pago.
- Custo por **lote** de ~5–8 variantes ≈ **M** (fatiável, alinhado DEBT-57).
  74 variantes ≈ **10–12 lotes**.

## 6. `rules/layout.md` — NÃO fatiado

A decisão A.1.2 (layout fica em `rules/`, só destructuring) significou que
nenhuma spec de layout mudou; `rules/layout.md` (11 `.rs`) **não foi editado**,
logo o imposto não mordeu e não se fatiou (princípio ADR-0104: fatiar quando
morde).

## 7. Proposta de composição do **Lote 2** (decisão humana)

Critério: minimizar o custo-por-elemento do lote (largura de uso baixa) e/ou
fechar uma família. Duas opções:

- **(a) Singletons triviais** — `Empty`, `Space`, `MathAlignPoint`, `Linebreak`,
  `Outline`: como `Divider`, sem campos; provam o mecanismo em massa com toque
  mínimo. Lote rápido (S).
- **(b) Família math restante** — `MathOp`, `MathAccent`, `MathCancel`,
  `MathUnderover`, `MathFrac`, `MathAttach`, `MathRoot`, …: prompts já finos
  desde P314; coesão de domínio; custo médio (M). **Recomendação primária**:
  (b), porque consolida o domínio onde o P314 já preparou o terreno L0.

A decisão é do dono.

## Fora de escopo (confirmado intocado)

- Os outros ~74 elementos (lotes futuros).
- F / PropMap / StyleChain (entra com o DEBT 99.E, ADR-0105).
- As 3 V9 pré-existentes em `03_infra`.
- As specs ausentes do DEBT-57 (registradas, não escritas).
- `rules/eval.md` / `rules/parse.md` (não mordem neste passo).
