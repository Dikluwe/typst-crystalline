# Relatório P329 — Triagem DEBT-58 + Lote 14 (Labelled, Boxed)

**Pré-condição**: Lote 13 (P328) fechado — lint 0, suíte verde (typst-core 2685);
dossiê em `dossie-triagem-debt-58.md`. ✅ Verificado.
**Commits**: `Passo 329 — triagem DEBT-58` (registro, tree limpo) e
`Passo 329 — lote 14`.

---

## Parte 1 — Triagem DEBT-58 (decisões do dono, registro)

**Critério do dono** (cabeçalho): fidelidade ao typst vanilla é **de
comportamento** (saída renderizada + semântica da linguagem), **não de estrutura
Rust**. A estrutura interna decide-se por atomicidade, performance e manutenção
por IA.

### A verificação mecânica (única pendência factual)

**Comando**: `grep -nE "Self::(Sequence|Empty|Block|Space|MathSequence)\b"
01_core/src/entities/content.rs | grep -v "=>"`.

**Resultado**: a álgebra do `Content` é `sequence()` (`content.rs:1560`) — constrói
`Empty` (0 partes), passthrough (1), `Sequence` (n). **`Block` aparece só no seu
construtor ergonómico `block(...)`** (`content.rs:~1369`, campos de utilizador,
como todo elemento denso) → **NÃO é álgebra**.

**Classificação resultante**: `Block` é **lote tardio** (L15), não primitivo.
Binário resolvido **sem surpresa**.

### As decisões (gravadas)

| Classe | Variantes | Destino |
|--------|-----------|---------|
| Primitivos **definitivos** (4) | `Sequence`, `MathSequence`, `Empty`, `Space` | hub por desenho declarado (arm deixa de ser dívida) |
| Primitivos **provisórios** (3) | `Text`, `MathText`, `MathIdent` | hub, **revisita no diagnóstico do F** (campos = StyleChain) |
| **Lote tardio** (3) | `Labelled`(57), `Boxed`(69), `Block`(121) | L14 (`Labelled`+`Boxed`), L15 (`Block`) |
| **Transferido ao F** (1) | `Styled` | diagnóstico do F (carrega `Styles`) + `Set*` |

Conta: 7 primitivos + 3 lote-tardio + 1 ao F = 11 ✓ (62 + 4 `Set*` + 11 = 77 ✓).

### Onde foi gravado

- **DEBT-58** (`00_nucleo/DEBT.md`): **triado** — parte "primitivos" encerra;
  `Styled`→F; `Block`/`Boxed`/`Labelled`→lotes.
- **L0 do content** (`prompts/entities/content.md`): os 7 primitivos como
  **desenho declarado do hub**.
- **Contabilidade** (`modelo-lote-migracao-d.md`): roteiro L14/L15 + perf + F.
- **Dossiê** (`dossie-triagem-debt-58.md`): nota de fecho.
- `content.rs`: só `@prompt-hash` (sync da edição da L0) — **zero código**.

---

## Parte 2 — Lote 14 (`Labelled` + `Boxed`)

**Composição: `Labelled`(57) + `Boxed`(69)** = 126 sites. Modo: 2 variantes com
validação intermediária.

### Forma das 2

| Variante | Forma | Locatável | `is_empty` | Hash |
|----------|-------|-----------|------------|------|
| `Labelled` `{target, label}` | wrapper (recurse target) | não (ver achado) | `target.is_empty()` | **derive** (`Label: Eq+Hash` + `Content`) |
| `Boxed` `{body + 9 cosm.}` | contentor denso (recurse body) | não | `body.is_empty()` | **manual** (`Length`/`Sides`/`Corners`/f64) |

### O achado — como a introspecção consome labels

`Labelled` **não é locatável no sentido do trait** (sem `element_kind`/
`to_payload`; `extract_payload` não o matcheia — não é pre-recursion). **MAS** a
introspecção **consome `Content::Labelled` directamente** num arm de walk
(`introspect.rs:944`) que emite `ElementPayload::Labelled` em **pós-recursão**
(P195D — `resolved_text` depende de state mutado durante o walk recursivo). A
migração **não muda esse mecanismo**; só converte o lado `Content::Labelled` dos
arms (walk, materialize, fixpoint ×2, layout `layout_labelled`) de struct-pattern
para `(e)`. Plano de toque cumprido.

### `Boxed` Arc-wrap (C2)

O construtor `boxed(body, width, height, inset, baseline)` cobre **5 de 10**
campos (defaults outset/radius/clip/fill/stroke) → as construções de
campo-completo (materialize `..(**e).clone()`, `native_box`, testes) usam
**`Arc::new(BoxedElem{…})`** (regra C2). `Styled` **não tocado** — não há
`|`-combinado com binding entre `Styled`/`Boxed`/`Labelled`.

### Validação intermediária

| Após variante | typst-core | build |
|---|---|---|
| Labelled | 2689 | ✅ |
| Boxed | 2693 | ✅ |

### C1…C2 — sites tratados à mão (zero conversões indevidas)

Skip-list: **40 sites**, aos dois transformadores. Tratados à mão: arms de
produção (introspect materialize/walk/fixpoint, layout ×2, eval construção);
**1 nested** `Content::Labelled { target, label: Label(s) }` (matches! com
destruturação interna do `Label`) → `(e) if … e.label.0 == …`; Arc move-outs
(`e.stroke.expect()` → `.clone().expect()`). **Achado de tooling**: o
transformador de construções não tinha `Boxed` no dict (crash) → adicionado +
re-rodado (Arc-wrap). **Verificação pós-passada**: ✓ interseção vazia nas 2.

---

## Medições (ADR-0104)

- **`cargo build`**: limpo. **Suíte**: typst-core **2693** (era 2685; **+8**),
  0 failed; `typst-infra` 472, restantes verdes.
- **`content.rs`**: **5135 → 5072** (−63). Trajetória desde P313 (5782):
  **−710 acumulado**. **Parte atómica**: `labelled.rs` + `boxed.rs` =
  **247 linhas**.
- **`crystalline-lint`**: 0 drift; **✓ No violations found**.

## Contabilidade atualizada

- Migradas **62 → 64**. **DEBT-58 encerrado** (7 primitivos declarados + `Styled`
  → F; `Block` é o único lote-tardio que resta).
- Conta: **64 migradas** + 4 `Set*` + 9 (4 def. + 3 prov. + `Block` + `Styled`)
  = **77** ✓.

## Roteiro restante (decisão humana)

1. **L15 = `Block`** (~121) — o último lote-tardio (confirmado pela verificação).
2. **Verificação de performance** (`hyperfine`; caveat C1/P319).
3. **Diagnóstico do F** — absorve `Set*` + `Styled` + revisita `Text`/`MathText`/
   `MathIdent`.

## Fora de escopo (confirmado)

Lote 15 (`Block`); verificação de performance e diagnóstico do F (nessa ordem);
qualquer mudança em `Styled`/`Set*`/`Text`/`MathText`/`MathIdent` (escopo do F);
otimizações sugeridas por medição. Caveat: stack default em `recursao_infinita_*`
— não é regressão (`RUST_MIN_STACK=33554432`).
