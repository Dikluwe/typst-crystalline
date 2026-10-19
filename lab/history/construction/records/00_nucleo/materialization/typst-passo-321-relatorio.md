# Relatório P321 — Lote 6 (família state/counter + Metadata)

**Pré-condição**: Lote 5 (P320) fechado — lint 0, suíte verde. ✅ Verificado.
**Commit**: `Passo 321 — lote 6` (lote único; sem caronas neste passo).

---

## Lote 6 — state/counter + Metadata (o 1º lote locatável desde o piloto)

**Composição (escolha do dono no checkpoint): 7 variantes**, ordem por largura
crescente: `CounterDisplay`(15) · `Metadata`(16) · `CounterDisplayCallback`(19) ·
`State`(21) · `StateDisplay`(21) · `StateUpdate`(25) · `CounterUpdate`(39) =
**~172 sites**. Todas são **leaves/markers**: `plain_text` vazio; `is_empty`
default `false`; `map_content`/`map_text` **terminais**; `get_field` default.

### Fronteira de locatabilidade declarada (o cerne do lote)

- **6 locatáveis** (queryable) — absorvem o braço de `extract_payload` no trait
  (precedente Heading P316): cada `…Elem` implementa `element_kind()` →
  `Some(ElementKind::X)` e `to_payload()` → `Some(ElementPayload::X{…})`.
  - `Metadata`→Metadata · `State`→State · `StateUpdate`→StateUpdate ·
    `StateDisplay`→StateDisplay · `CounterUpdate`→CounterUpdate ·
    `CounterDisplayCallback`→**CounterDisplay** (kind partilhado com o legacy).
  - O hub passou a despachar `extract_payload.rs` (`Content::X(e) =>
    e.to_payload()`) e `locatable.rs` (`Content::X(_) => true`).
- **1 não-locatável** — `CounterDisplay { kind }` legacy single-pass (DEBT-10):
  `element_kind`/`to_payload` no default `None`.
- **Chave da tratabilidade**: o **consumo por `ElementPayload`** (`from_tags` +
  walk de payload em `introspect.rs`) matcheia o **payload**, não o `Content` —
  **inalterado** pela migração. Só os sites `Content::X`-side mudaram.

### Achados content-preserving preservados

- **Quirk de `eq`**: `Metadata`/`State`/`StateUpdate` **não têm arm de `eq` no
  hub** → caem em `_ => false` (sempre desiguais — marcadores efectivos).
  **Preservado** (não adicionado dispatch). As outras 4 despacham `(X(a),X(b)) =>
  a == b`. Os `…Elem` derivam `PartialEq` pelo contrato do trait (não usado pelo
  hub nas 3 do quirk).
- **`Hash` manual via Debug** (regra do modelo, tipos sem `Hash`): `Metadata`
  (`Value`/`f64`), `State` (`Value`), `StateUpdate` (`state_update::StateUpdate`,
  sem `Hash` nem `PartialEq` derivados — tem `impl` manual de `PartialEq`),
  `StateDisplay`/`CounterDisplayCallback` (`Func`). **Derivam `Hash`**:
  `CounterDisplay` (só `String`), `CounterUpdate` (`CounterAction` deriva `Hash`).
- `Box<Value>` mantido (paridade `ElementPayload`).

### Medições (métrica ADR-0104) — vs preditor

| medição | valor | comando |
|---|---|---|
| `content.rs` antes/depois | **5603 → 5573 (−30)** | `wc -l`; diff `+98 / −128` |
| parte atómica (7 módulos novos, c/ testes) | **575 linhas** | `wc -l elements/{…}.rs` |
| suíte typst-core | **2566 → 2583 (+17)** | só os 17 testes unitários novos |
| `crystalline-lint .` | **0 violations** | gate binário |

**Trajetória `content.rs`**: 5782 (P313) → … → 5603 (L5) → **5573** (L6).
Custo-por-módulo: Metadata 96 · CDCallback 89 · StateUpdate 85 · StateDisplay 83 ·
State 82 · CounterUpdate 77 · CounterDisplay 63. **Preditor validado** mesmo num
lote locatável (custo ∝ largura; bulk em `introspect.rs` tests e `layout/tests.rs`).

**Plano de toque (realidade)**: `content.rs` hub (6 matches → dispatch; quirk de
eq preservado) · `extract_payload.rs` (6 locatáveis → `e.to_payload()`) ·
`locatable.rs` (`X(_) => true`) · `introspect.rs` (materialize/walk de
CounterDisplay/CounterUpdate; terminais; muitos testes) · `layout/mod.rs`
(handlers) · `stdlib/foundations.rs`+`eval/bindings.rs` (construção) · muitos
sites de teste. **Nenhuma asserção existente alterada.**

> **Nota de ferramenta** (registo): o transformador de construções (balanced-
> brace) **sobre-removeu** `Box::new` aninhado em `StateUpdate::Set(Box<Value>)`
> e **converteu padrões `{ field: bind }` em chamadas de construtor** (inválido
> em posição de padrão). Ambos detectados pelo compilador e corrigidos à mão.
> Lição: o script só serve construções com `Box` externo único; padrões e
> `Box` aninhados são manuais.

### Contabilidade atualizada (item obrigatório)

Migradas **31 → 38**; element-shaped restantes **~31 → ~24**; estimativa
**3–5 lotes** restantes.

### Proposta do Lote 7 (do mapa — decisão humana)

Por largura: `Raw`(9) · `Align`(11) · `Image`(17) · `Hide`(18) · `Repeat`(18) ·
`Quote`(20) · `Columns`(22) · `Ref`(23) · `Outline`(24) · … — ou o **bloco
grid/table cell** (`TableCell`/`GridCell`/`Table`/`Grid`, ~193 sites) como lote
próprio pesado (a sequência sugerida pelo dono: largura → grid/table como
penúltimo → fim dos element-shaped dispara o gatilho do DEBT-58).

---

## Encerramento

- `git log --oneline -1`: `Passo 321 — lote 6`.
- `crystalline-lint .` → **0 violations**. `git status` limpo (só cruft `lab/`).
- Caveat conhecida: stack default estoura em `recursao_infinita_*` — não é
  regressão (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lotes 7+ e o bloco grid/table cell; DEBT-58 (gatilho não disparou — ainda há
element-shaped); F / `Set*` / 99.E; otimizações.
