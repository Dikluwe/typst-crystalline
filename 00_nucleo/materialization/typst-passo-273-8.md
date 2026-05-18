# Passo P273.8 — Cleanup `unused_variable: parent_bbox_at_emit` em export.rs

**Tipo**: cleanup XS literal — substituir 4 bindings em pattern-match por `_`.
**Magnitude estimada**: XS (4 substituições; ~5 min de execução real; ≤8 LOC alterados; 0 tests novos).
**Pré-requisitos**: P273.7 fechado (cascade `FrameItem::Shape.parent_bbox_at_emit` consolidado).
**Cluster**: Visualize / Gradient (cleanup pós-cluster).
**Aplica ADRs**: nenhuma nova; herda contexto P273.6+P273.7.

---

## §0 — Contexto

P273.6 introduziu o campo `parent_bbox_at_emit: Option<Rect>` em `FrameItem::Shape` com cascade ~86 sites bulk-patched. Os sítios consumidores em `03_infra/src/export.rs` são pattern-matches que destructuram a shape para extrair fields relevantes.

Relatório P273.6 §2.5 mostra que **`scan_all_gradients`** destructura `parent_bbox_at_emit` e consome-o:

```rust
if let FrameItem::Shape {
    stroke: Some(Stroke { paint: Paint::Gradient(g), .. }),
    parent_bbox_at_emit,
    ..
} = item {
    // ...
    grad_objs.push(GradientObject {
        // ...
        parent_bbox_at_emit: *parent_bbox_at_emit,  // CONSUMIDO
    });
}
```

Mas relatório P273.7 §9 documenta que existem **4 outros pattern-matches** em `export.rs` que extraem o campo `parent_bbox_at_emit` mas **não o consomem**:

> 4 warnings pré-existentes `unused variable: parent_bbox_at_emit` em `03_infra/src/export.rs` (linhas 2003/2275/2521/2705) — vêm do cascade P273.6 (binding em destructures sem uso); NÃO introduzidos por P273.7. Candidato cleanup XS futuro: ignorar via pattern `parent_bbox_at_emit: _`.

São pattern-matches que processam a shape para outro propósito (e.g. detectar fill/stroke não-gradient, bbox de página, dimensões, etc.) e o binding `parent_bbox_at_emit` é capturado pelo cascade automático mas nunca lido no corpo do `if let` / `match`.

P273.8 substitui esses 4 bindings por `_` para eliminar os warnings de build.

### Por que não foi feito em P273.6 directamente

O cascade ~86 sites foi bulk-patched via Python script com regra uniforme (acrescentar campo + default `None`). Não havia análise per-sítio para decidir consume vs ignore. P273.6 priorizou correctness do cascade (todos os sítios compilam + default `None` preserva semantic); cleanup dos sítios não-consumidores ficou registado mas não tratado.

P273.7 documentou explicitamente o resíduo e propôs o cleanup como candidato XS. P273.8 executa.

---

## §1 — Sub-passo P273.8.A — Fase A diagnóstico

**Magnitude**: XS-XXS documental (~5-10 min).
**Output**: `00_nucleo/diagnosticos/typst-passo-273-8A-diagnostico.md`.

### §A.1 — Localização literal dos 4 sítios

Listar literal em `03_infra/src/export.rs` (path:linha):

- **Linha 2003** — confirmar contexto: pattern-match em `if let FrameItem::Shape { ... }` ou `match`? Qual o propósito do bloco (detecção fill não-gradient? bbox calculation? outro)?
- **Linha 2275** — idem.
- **Linha 2521** — idem.
- **Linha 2705** — idem.

Output esperado: tabela com 4 linhas × 3 colunas (linha, propósito do bloco, snippet do pattern-match).

### §A.2 — Confirmar que `scan_all_gradients` (linhas P273.6 §2.5) NÃO está entre os 4 sítios

`scan_all_gradients` consome `parent_bbox_at_emit` (relatório P273.6 §2.5 literal); não é candidato a cleanup. Confirmar empíricamente cruzando linha de `scan_all_gradients` com lista §A.1.

### §A.3 — Decisão de forma do cleanup

Opções:

- **Opção α — `parent_bbox_at_emit: _` no pattern** — explícito; preserva legibilidade do que se está a ignorar; idiomático Rust.
- **Opção β — `_parent_bbox_at_emit` no binding** — prefixo `_` silencia warning sem mudar pattern syntax; menos invasivo; mas o binding fica reservado se algum dia for preciso usar (precisa renomear).
- **Opção γ — Remover binding do pattern** — `..` rest pattern já existe na maioria; remover a linha do `parent_bbox_at_emit` deixa o cascade automático ignorá-lo via `..`. Possível apenas se o pattern original tem `..`.

Recomendação spec: **Opção α** — explícito em todos os 4 sítios. Razões:

1. Sinala intenção (`_` declara "consciência do campo + decisão de o ignorar").
2. Não cria binding-fantasma com prefixo `_` que possa colidir futuramente.
3. Mais robusto a refactors — se o campo desaparecer da struct, o pattern parte de imediato sinalizando que o ignore precisa ser reavaliado.

Alternativa Opção γ (rest pattern `..`) considerada mas rejeitada se o pattern original já é específico — diferente significado declarativo.

Decisão final na Fase A.

### §A.4 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Warning continua | Sítio mal identificado (linhas alteradas pós-P273.7) | §A.1 confirma path:linha empírico antes do patch |
| Quebra compile | `_` em pattern com side-effect (impossível em destructure puro) | Destructure puro — risco zero |
| Regressão tests | Cleanup altera comportamento | Cleanup é warning-only — comportamento idêntico bit-exact |
| Sítio é consumidor real (parent_bbox_at_emit deveria ser usado mas foi esquecido) | Análise §A.1 superficial | §A.1 examina propósito do bloco; se um sítio precisa do campo, abrir DEBT separado em vez de fazer cleanup |

### §A.5 — Decisões a fixar na Fase A

1. **Decisão 1** (forma cleanup): α / β / γ. Recomendação spec: α.
2. **Decisão 2** (sítios candidatos a consumir o campo): identificar se algum dos 4 sítios é "consumidor esquecido" em vez de "ignorante consciente". Se sim, abrir DEBT separado e tratar como fora de escopo P273.8.

### §A.6 — Critério de aceitação Fase A

- §A.1 confirma path:linha empírico (linhas exactas podem ter shift após P273.7 ~20 LOC adicionados a layout/mod.rs — mas export.rs não foi tocado em P273.7, logo linhas 2003/2275/2521/2705 devem estar inalteradas; confirmar).
- §A.2 confirma `scan_all_gradients` exclusivo.
- §A.5 decisões 1+2 fixadas.

---

## §2 — Sub-passo P273.8.B — Sem anotação ADR

**Magnitude**: zero documental.

Cleanup XS sem decisão arquitectural; sem padrão metodológico novo; sem ADR para anotar.

A ADR-0091 nona anotação (P273.7) é a anotação final do cluster Gradient. P273.8 não merece anotação separada — é trabalho mecânico residual.

---

## §3 — Sub-passo P273.8.C — Materialização

**Magnitude**: XS literal (~5 min execução).

### Ordem literal

1. Fase A §1 produzida (rápida — 5-10 min).
2. **`cargo build` antes do patch** — confirmar empírico que exactamente 4 warnings `unused_variable: parent_bbox_at_emit` aparecem (nem mais nem menos).
3. **Patch** — substituir nos 4 sítios identificados conforme Decisão 1.
4. **`cargo build` pós-patch** — confirmar 0 warnings `unused_variable: parent_bbox_at_emit`.
5. **`cargo test --workspace`** — confirmar 2620 verdes preserved bit-exact.
6. `crystalline-lint .` — confirmar lint zero.
7. Verificar que **nenhum outro warning aparece** (e.g. patch não introduz warning de outra natureza).

### Cap LOC (ADR-0094 Pattern 1)

- **L3 hard cap**: ≤ 8 LOC (4 substituições × ~2 LOC cada).
- **L3 soft cap**: ≤ 4 LOC (substituições puras sem comentário).
- **Tests hard cap**: 0 novos.
- **Tests soft cap**: 0.

### Alterações esperadas no código

```rust
// 03_infra/src/export.rs — 4 sítios

// Linha ~2003 (exemplo):
- if let FrameItem::Shape {
-     fill: Some(...),
-     parent_bbox_at_emit,
-     ..
- } = item {
+ if let FrameItem::Shape {
+     fill: Some(...),
+     parent_bbox_at_emit: _,
+     ..
+ } = item {

// Análogo para linhas ~2275, ~2521, ~2705 (forma exacta confirmada na Fase A §A.1).
```

### Verificação final

- 4 warnings desapareceram (`cargo build 2>&1 | grep "unused_variable: parent_bbox_at_emit"` retorna zero linhas).
- Cap LOC respeitado (~8 LOC L3 hard / ~4 LOC soft).
- `cargo test --workspace` verde — 2620 preserved bit-exact (cleanup é puramente cosmético).
- `crystalline-lint .` zero violations.
- Hashes L0 preserved (export.rs não tem L0 directo; touch L3 puro).
- Tests P262-P273.7 inalterados bit-exact.
- DEBT saldo 10 preserved.

---

## §4 — Sub-padrões cumulativos pós-P273.8

Cleanup XS não inaugura sub-padrão novo e não cresce os existentes de forma material. Excepções:

| Sub-padrão | Pós-P273.7 | Pós-P273.8 |
|---|---|---|
| Cap LOC hard vs soft explícito | 10 | 11 (mantido per Pattern 1 documentação) |
| Diagnóstico imutável | 23 (18º consumo) | 24 (19º consumo) — Fase A directa |
| Aplicação meta-ADR (ADR-0094) | 6 | 7 (Pattern 1 cap LOC) |

Sub-padrão emergente "Sub-passos decimais consecutivos do mesmo cluster" cresce N=3 → 4 (P273.5+P273.6+P273.7+P273.8) — limiar formalização N=3-4 atingido com folga. Candidato meta-ADR futura permanece NÃO reservado.

### Sub-padrão NÃO aplicado (anotado por contraste)

- **Pattern DEBT-37 `cell_origin_*` replicado**: P273.8 não toca padrão Layouter save/restore; pattern preserved N=3.
- **Template-passo replicado literal**: P273.8 não replica template P273.6/P273.7; sub-padrão preserved N=1.
- **Reutilização literal helpers cross-passos**: P273.8 não reusa helper; preserved N=16.
- **Anotação cumulativa em vez de ADR nova**: P273.8 não anota ADR (§2 explícito); preserved N=16.

---

## §5 — Limitações conscientes P273.8

- Decisão α (`parent_bbox_at_emit: _`) é puramente declarativa — comportamento runtime idêntico a binding actual com warning. Cleanup é "build output hygiene".
- Se algum dos 4 sítios for "consumidor esquecido" detectado em §A.5 Decisão 2, esse sítio NÃO é tratado em P273.8 — DEBT separado é aberto. P273.8 só limpa "ignorantes conscientes".
- Cleanup não revisita a decisão estrutural P273.6 de cascade total — é cosmético dentro da decisão existente.

---

## §6 — Workflow operacional

1. Utilizador lê esta spec.
2. Utilizador executa Fase A em Claude Code → diagnóstico XS rápido (5-10 min).
3. Utilizador upload do diagnóstico.
4. Claude web valida critério §A.6 + revê Decisões 1+2.
5. Utilizador executa P273.8.C em Claude Code → relatório XS.
6. Utilizador upload do relatório.
7. Claude web analisa + propõe próximo passo (cluster Gradient ficará pronto para saída definitiva sem warnings de build).

**Nota operacional**: P273.8 é pequeno o suficiente para potencialmente unir Fase A + materialização num único sub-passo executado de uma vez em Claude Code. Spec preserva separação por consistência metodológica, mas se preferires single-shot, é viável.

---

## §7 — Pendências preservadas pós-P273.8

Inalteradas vs P273.7:

- P-Gradient-CMYK-ICC (S-M).
- ADR-0055bis variant-aware fonts (M).
- P-Footnote-N (M).
- DEBT-33 Bézier bbox (S+M).
- Stroke\<Length\> / Curve / Polygon (S+M).
- Tiling activação.
- Outro cluster — saída Visualize/Gradient.

Pendências específicas pós-P273.7 inalteradas:
- P273.9 — Stack/Pad/Group/Grid cell save/restore.
- P273.X-bis — Bbox medido pós-layout.
- P273.X-bis2 — Bbox.y topo-exacto inline.
- Dedup bbox-aware.

**Pós-P273.8 cluster Gradient ficará totalmente pronto para saída**: feature-complete user-facing + adaptive N qualitativo + refino estrutural Block+Boxed + build output limpo.

---

## §8 — Critério de fecho do passo

P273.8 fecha com **IMPLEMENTADO** quando:

- Fase A produzida + critério §A.6 cumprido.
- 4 warnings `unused_variable: parent_bbox_at_emit` desaparecem (verificação `cargo build` empírica).
- L3 alterado dentro do cap LOC.
- Tests workspace verdes 2620 preserved bit-exact.
- Lint zero.
- Nenhum warning novo introduzido.
- Tests P262-P273.7 inalterados.
- DEBT saldo 10 preserved.
- Sub-padrões §4 atualizados (mínimos — passo XS).

---

## §9 — Numeração — nota

Spec usa **P273.8** em numeração consecutiva. Justificação:

- P273.8 é cleanup XS directamente derivado de pendência específica documentada em P273.7 §9 quinto bullet.
- Não introduz feature nova nem decisão arquitectural — mas ocupa o número consecutivo conforme decisão do utilizador (preserva consistência com sub-passos anteriores P273.5/P273.6/P273.7).
- A reserva original em P273.7 §7 para "Stack/Pad/Group/Grid cell save/restore" desloca-se para **P273.9** (registado em §7 deste passo e nos relatórios P273.7 spec/relatório).
- Sub-padrão emergente "P\<X\>.\<Y\>.1 = cleanup XS derivado" considerado inicialmente foi **abandonado** com este rename — cleanup XS passa a usar numeração consecutiva normal, igual a qualquer outro sub-passo.
