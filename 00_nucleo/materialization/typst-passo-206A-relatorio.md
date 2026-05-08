# Relatório do passo P206A

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-206A.md`.
**Natureza**: diagnóstico-primeiro (zero código tocado)
+ auditoria empírica de profundidade alta. 39ª aplicação
consecutiva do padrão.
**Sub-passo `A` da série P206** — primeiro de 5 (A–E).
**Magnitude planeada**: M (M auditoria + S diagnóstico).
**Magnitude real**: **M** (~45 min; 0 ficheiros código
+ 4 outputs documentais; sem refactor mid-execution).

---

## §1 O que foi feito

P206A executou diagnóstico-primeiro formal para
endereçar **DEBT-53/54** + **cond 9 ADR-0073 PARCIAL**
herdados de M8/F3. Auditoria empírica em 5 blocos (A1–A16)
+ decisões fixadas em 13 cláusulas (C1–C13) + ADR-0075
PROPOSTO produzido per C9 afirmativa.

Conclusões-chave da auditoria empírica:

- **Vanilla typst CLI 0.14.2 já instalado** em
  `/usr/local/bin/typst` — paridade exacta com
  `lab/typst-original/crates/typst-syntax v0.14.2`
  declared em `lab/parity/Cargo.toml`. Pre-built binário
  é divisor de águas — elimina necessidade de compilar
  vanilla na quarentena ou setup workspace.
- **2 breaks triviais** (1-line cada) em `lab/parity/tests/`
  identificados por P204F.div-1 confirmados
  empíricamente via `cargo check --all-targets`. Bin
  `parity-runner` permanece **funcional** (smoke test
  passou).
- **`typst query --format json` produz output
  directamente comparável** com `Introspector::query_*`
  cristalino. Smoke test em `query-metadata.typ`
  produziu match exacto.
- **Pixel-perfect PDF comparison é inviável por design**
  — cristalino `FixedMetrics` (ADR-0054) diverge
  estruturalmente de vanilla `FontBookMetrics`.
- **DEBT-54 obsoleta-se sem código** — workspace setup
  é desnecessário quando pre-built CLI funciona.

### Caminho fixado: A — Reactivar (não B nem C)

Per C1 + auditoria A12 vs A13 vs A14:

- **Caminho A (Reactivar)**: ~1-2h; reuso 5 src + 3
  tests + matriz histórica. Custo S-M.
- **Caminho B (Construir do zero)**: rejeitado.
  ~4-6h; clean slate descarta reuso sem benefício
  observable; risco re-introduzir bugs já fixados.
  Custo M-L.
- **Caminho C (Híbrido)**: rejeitado por equivalência
  semântica com A (Reactivar = "fix breaks + estender
  com vanilla CLI invocation").

### Output 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-206A-auditoria-vanilla.md`.

Conteúdo:
- §1 Bloco 1 — Estado actual do harness (A1-A4; 4/4
  CONFIRMADO).
- §2 Bloco 2 — Dependências vanilla (A5-A8; 4/4
  CONFIRMADO).
- §3 Bloco 3 — Corpus actual (A9-A11; 1 DIVERGÊNCIA
  empírica em A10 sobre companions).
- §4 Bloco 4 — Caminhos viáveis (A12-A14; tabela
  comparativa).
- §5 Bloco 5 — Estado pós-M8+F3 (A15-A16; cond 9
  PARCIAL preservada; DEBT-54 obsoleto).
- §6 Resumo final.

Tamanho: ~14 KB.

### Output 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-206A-diagnostico.md`.

Conteúdo:
- §1-§13 Cláusulas C1–C13 fixadas com valores concretos
  + justificação.
- §14 Decisões durante a leitura (D1–D7).
- §15 Resumo de métricas previstas.

Tamanho: ~13 KB.

### Output 3 — Relatório (este ficheiro)

### Output 4 — ADR-0075 PROPOSTO (per C9 afirmativa)

Localização:
`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`.

Estrutura paralela a ADR-0072/0073/0074:

- Contexto + Decisão + Mecanismo (per P206A C1, C5,
  C3, C4) + Escopo (C2) + DEBT-53/54 (C8).
- 7 alternativas consideradas e rejeitadas (B/C/D/E/F/G
  + híbrido).
- Consequências positivas/negativas/neutras.
- Plano de validação (7 condições para transitar
  ACEITE em P206E).
- Plano de materialização (P206A-E sub-passos).
- Cross-references + Pattern emergente.

Tamanho: ~10 KB.

---

## §2 Tempo de execução

~45 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList.
- ~12 min: A1-A4 (estrutura harness; cargo check
  empírico; smoke test parity-runner; layout_parity
  análise).
- ~8 min: A5-A8 (typst --version; tools PDF; smoke
  test typst query JSON match).
- ~8 min: A9-A11 (corpus 36 ficheiros; companions
  distribution; gaps de features).
- ~5 min: A12-A14 (estimativas de custo + comparação).
- ~3 min: A15-A16 (cond 9 ADR-0073; outras
  pendências).
- ~2 min: C1-C13 (decisões fixadas com base em
  auditoria).
- ~5 min: outputs documentais (auditoria + diagnóstico
  + ADR-0075 + relatório).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes (P206A) | 1860 |
| Tests workspace depois (P206A) | **1860** (∆ 0 — diagnóstico) |
| Tests P206A novos | 0 |
| Linter violations | 0 (sem alteração) |
| Ficheiros novos código | 0 |
| Ficheiros modificados código | 0 |
| Ficheiros novos docs | 4 (auditoria + diagnóstico + relatório + ADR-0075) |
| Ficheiros modificados docs | 0 |
| LOC novas (código) | 0 |
| LOC novas (docs) | ~3000+ (4 outputs P206A) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |

### Tests por crate (sem alteração)

- `typst_core` unit: 1584.
- `typst_infra` unit: 24.
- `typst_shell` unit: 21.
- `typst_wiring` unit: 2.
- Integration tests: 229.
- **Total**: 1860.

---

## §4 Decisões

### D1 — Vanilla CLI install é divisor de águas

Esperava-se compilar vanilla typst na quarentena
(custo M-L) ou setup workspace (custo XL). A5
mostrou pre-built binário em
`/usr/local/bin/typst v0.14.2`. Custo de vanilla
acesso colapsa de M-XL para zero (install ambiental).
Implicação directa: C5 = b (pre-built); DEBT-54
obsoleto.

### D2 — Pixel-perfect comparison rejeitada por design

ADR-0054 perfil graded já estabeleceu
`FixedMetrics` divergência face a `FontBookMetrics`
como **estrutural por construção**. P206 não inverte;
C3 = C (estrutural-only) formaliza explicitamente.
Pixel-perfect daria ~100% noise mesmo com semântica
idêntica.

### D3 — DEBT-54 obsoleto sem código

DEBT-54 ("vanilla workspace setup") era hipótese de
trabalho assumindo vanilla precisaria ser compilado no
workspace. A6 + A5 mostraram que **pre-built binário
torna workspace irrelevante**. Fechar por irrelevância
é solução honesta — não inflar trabalho para "fechá-lo
estruturalmente" se a hipótese inicial é obsoleta.

Pattern: DEBT pode fechar via 3 caminhos —
materialização (CLOSED), supersedure (REPLACED-BY),
ou irrelevância (OBSOLETED). Per spec C8.

### D4 — `here-locate.typ` referenciado mas inexistente

Spec mencionou "P204F SKIP here-locate.typ". A11
empírico mostra ficheiro **não existe** no corpus.
Hipótese spec foi precaução baseada em assumption
que stdlib `here()`/`locate()` materializariam P204F;
não materializaram. P206 não precisa SKIP. Stdlib
expansion é pendência separada.

### D5 — Caminho A vs B claramente disambiguado

A12 (S-M ~1-2h) vs A13 (M-L ~4-6h). Diferencial
2-3×. Reuso vs clean slate. Risco re-introduzir bugs
em B vs preservar fixes em A. Risco "subestimar A
por preguiça" (per spec §9): rejeitado por evidência
empírica — pre-existing breaks são literalmente 2
lines.

### D6 — Sub-passos `*B+` em série única (não múltiplas
séries)

C10 magnitude M agregado (~3.5-5h) é viável em série
P206A-E única. Sub-séries (P206/P207/...) seriam
inflação se C10 fosse XL — mas A12+A13+C10 não mostram
inflação. Hipótese spec C10 testada e rejeitada
empíricamente.

### D7 — Sem `P206A.div-N` sobre escopo

C13 explicita que pré-fixação do escopo (36 ficheiros)
não é absoluta — é guidance. Mas auditoria mostra
escopo viável com decisões fixas (Caminho A + C5=b +
C3=C + C4=D). Sem necessidade de reduzir a 6 ficheiros
introspection. Per spec §9 risco "inflar escopo para
honrar 36 ficheiros pré-fixado": rejeitado porque
empírico **suporta** 36 sem inflação.

### D8 — `markup/error.typ` SKIP é pré-existente

Linha 95 do `tests/layout_parity.rs` actual:
`if file.starts_with("error") { continue; }`. SKIP
documentado pré-P206. P206A confirma que C2 SKIP
literal é continuação, não nova decisão.

### D9 — Companions `[expectations.vanilla]` deferred
para P206C runtime

P204F adicionou `[expectations.cristalino]` em 5
ficheiros introspection. Hipótese era que P206 adicionaria
`[expectations.vanilla]` paralelo. **Mas** comparação
directa via `typst query` JSON em runtime dispensa
expectations explícitas — JSON output é dinamicamente
extraído. Decisão deferred para P206C runtime: se
JSON in-runtime suficiente, sem alteração de companions;
se for útil ter expectations explícitas para
documentação, P206C adiciona.

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §9 e cláusulas
de decisão:

| Hipótese | Resultado |
|----------|-----------|
| §9: "C1 = Caminho A (Reactivar) ou C1 = C (Híbrido)" | **CONFIRMADA** — C1 = A (Híbrido rejeitado por equivalência semântica) |
| §9: "Caminho B (Construir) só se A2 revelar lab/parity irrecuperável" | **CONFIRMADA** — A2 confirmou 2 breaks triviais; reactivar é viável |
| §9: "A5 pode revelar vanilla typst não compila trivialmente" | **PARCIALMENTE** — não foi necessário compilar; pre-built CLI já disponível |
| C9: "Hipótese mais provável: criar ADR-0075" | **CONFIRMADA** — ADR-0075 PROPOSTO criado |
| C10 hipótese: "se A12 ou A13 mostrarem custo XL, C10 obriga a sub-séries" | **REJEITADA** — A12 S-M; A13 M-L; nenhum XL; série única viável |
| C13: "se A12+A13+C10 mostrarem 36 inviável, registar div-N" | **REJEITADA** — escopo viável com decisões fixas |
| §9: "subestimar custo de Construir do zero por preguiça" | **EVITADO** — A13 estimado empiricamente, não estética |

7 hipóteses resolvidas pela auditoria empírica. A spec
previu correctamente os critérios; P206A executou-os
literalmente. Pattern alinhado com P204A/P205A
(diagnóstico-primeiro literal).

---

## §6 Sugestão para próximo sub-passo

P206A fechado per C12 (sem cláusulas condicionais)
com todos os critérios cumpridos:

- ✓ A1-A16 todos com etiqueta CONFIRMADO ou
  DIVERGÊNCIA (1 divergência empírica em A10 absorvida).
- ✓ C1-C12 instanciadas com valores concretos.
- ✓ ADR-0075 PROPOSTO escrito (C9 afirmativa).
- ✓ Magnitude calibrada (C10 = M agregado).
- ✓ Plano `*B+` sem condicionais.
- ✓ C13 sem divergência sobre escopo.

**Próximo sub-passo**: **P206B — Reactivar harness +
smoke vanilla CLI**.

P206B é magnitude S (~30-45 min):

- Fix `tests/layout_parity.rs:69` (1-line; remover
  `state` arg + linha `let state = introspect(...)`).
- Fix `src/value_dto.rs:83` (1-line; adicionar
  `Value::Location(_)` arm).
- Smoke test sentinel: `typst --version` available;
  abort gracefully se ausente.
- Confirmar `cargo check --manifest-path lab/parity --all-targets`
  passa.
- Outputs: 2 ficheiros (inventário + relatório).

---

## §7 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-206A.md`.
- **Outputs P206A**:
  - `00_nucleo/diagnosticos/typst-passo-206A-auditoria-vanilla.md`.
  - `00_nucleo/diagnosticos/typst-passo-206A-diagnostico.md`.
- **ADR produzida (PROPOSTO)**:
  `00_nucleo/adr/typst-adr-0075-vanilla-integration.md`
  (PROPOSTO 2026-05-07; transita ACEITE em P206E).
- **Pendências endereçadas**:
  - DEBT-53 (CLOSED em P206 via materialização).
  - DEBT-54 (OBSOLETED em P206 via irrelevância).
  - `P204F.div-1` (resolvido em P206).
  - Cond 9 ADR-0073 PARCIAL (transita "completo final"
    em P206E).
- **Predecessores série**:
  - P205E (F3 ACEITE final).
  - P204H (M8 estruturalmente fechado).
- **Pattern referência**: P204A diagnóstico-primeiro
  16 cláusulas A1-A16 (paralelo a P206A 16 cláusulas).
- **Vanilla typst v0.14.2**:
  `lab/typst-original/crates/typst-syntax v0.14.2`
  (path dep) + `/usr/local/bin/typst v0.14.2 (b33de9de)`
  (CLI binary).
