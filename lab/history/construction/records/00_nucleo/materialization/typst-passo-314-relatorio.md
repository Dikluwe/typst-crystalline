# Relatório P314 — Duas ADRs do modelo de elemento + conserto do imposto de hash (L0 fino)

**Data**: 2026-06-10
**Tipo**: materialização de processo e linhagem (zero lógica de produto).
**Fonte de decisão**: `diagnostico-modelo-elemento-passo-313.md` (§2.1, §5, §6) +
decisão do dono (recomendação primária aceita; conserto do imposto puxado para
antes do D).
**Resultado**: ✅ entregue e validado. Imposto de hash **18 → 1**.

---

## 1. ADRs criadas

| ADR | Título | Status |
|-----|--------|--------|
| **ADR-0104** | Atomicidade para agentes — força arquitetural de 1.ª classe | `EM VIGOR` |
| **ADR-0105** | Modelo de elemento — D incremental agora, F (PropMap) como destino | `EM VIGOR` |

- ADR-0104: três cláusulas (hubs concentradores = anti-padrão; linhagem fina;
  verificação mecânica não depende de memória — precedente F4) + corolário de
  sequência (custo-por-elemento = métrica de saúde).
- ADR-0105: adota **D** incremental (variante `Nome(Arc<nome::Nome>)`, trait
  `Element`, absorve `ElementPayload`), declara **F** como destino junto com o
  DEBT StyleChain (99.E), trava gravada (teste-varre-tabela ou regra lint antes
  de F), compatibilidade `impl Element`→descritor, descarta E e F-direto.
  **Complementa ADR-0026, não revoga.** Cruz-ref adicionada ao
  `debt-stylechain-nao-materializada.md`.

## 2. Partição dos prompts L0 (tabela de mapeamento)

### `rules/stdlib.md` (era 12 `.rs`) → 6 finos + índice

| `.rs` | → prompt fino |
|---|---|
| `calc.rs` | `stdlib/calc.md` |
| `foundations.rs` | `stdlib/foundations.md` |
| `figure_image.rs` | `stdlib/figure_image.md` |
| `text.rs` | `stdlib/text.md` |
| `math_style.rs` | `stdlib/math_style.md` |
| `mod.rs`, `assert.rs`, `layout.rs`, `shapes.rs`, `transforms.rs`, `gradients.rs`, `structural.rs` | `stdlib/_comum.md` |

### `rules/math/layout.md` (era 10 `.rs`) → 9 finos + índice

| `.rs` | → prompt fino |
|---|---|
| `mod.rs`, `tests.rs` | `math/layout/_comum.md` |
| `attach.rs` | `math/layout/attach.md` |
| `root.rs` | `math/layout/root.md` |
| `frac.rs` | `math/layout/frac.md` |
| `matrix.rs` | `math/layout/matrix.md` |
| `cases.rs` | `math/layout/cases.md` |
| `stretchy.rs` | `math/layout/stretchy.md` |
| `assembly.rs` | `math/layout/assembly.md` |
| `delimited.rs` | `math/layout/delimited.md` |

**Content-preserving**: a união dos finos = o prompt velho. Regras partilhadas
(convenção de assinatura, helpers, IEEE 754 transversal, promoção de tipos;
`MathLayouter`/`MathBox`/baseline/primes/handler MathStyled) → `_comum.md`.
Os prompts velhos viraram **índices** de uma página com a tabela de mapeamento.

**Decisão honesta de partição (deriva F4 registada, não corrigida)**:
`stdlib.md` estava desatualizado — `layout.rs`(17 fn)/`shapes.rs`(6)/
`transforms.rs`(4)/`assert.rs`/`gradients.rs`(3)/`structural.rs`(21) e funções
de `foundations.rs` (`oklab`/`oklch`/`cmyk`/`hsl`/`hsv`/`state*`/`counter*`/
`query`/`here`/`locate`) **não** estavam specadas. **Não inventei spec**: esses
`.rs` apontam para `_comum.md` (a convenção partilhada era o único conteúdo que
o prompt velho lhes dava); ficam registados como **candidatos a spec dedicada**.

## 3. Medição do imposto de hash (critério de aceitação)

| | antes (P311 tocou `stdlib.md`) | depois (P314 tocou `stdlib/math_style.md`) |
|---|---:|---:|
| `.rs` re-hasheados | **18** | **1** |

Simulação: linha trivial em `math_style.md` → `crystalline-lint --fix-hashes .`
→ re-hasheou **só** `stdlib/math_style.rs`. Revertido depois (0 drift).

## 4. Validação

- **`cargo build`**: ✅ verde (`Finished dev` em 2.69s).
- **`cargo test --workspace`**: ✅ **2462** tests typst-core (= contagem
  documentada P311) + restantes crates = **2981 passed / 0 failed** (com stack
  adequado). Nenhum teste novo nem removido.
  - **Ressalva pré-existente (não-P314)**: com a stack default (2 MB) de thread
    de teste em debug, o teste `rules::eval::tests::recursao_infinita_retorna_
    err_sem_crash` estoura a stack (SIGABRT). É artefacto de **tamanho de
    stack** (passa com `RUST_MIN_STACK` maior), **não** regressão: o diff do
    P314 é exclusivamente linhas `@prompt`/`@prompt-hash` (comentários), que não
    alteram o binário. Registado, fora de escopo.
- **`crystalline-lint .`**:
  - **3 V9** (`ForbiddenImport`/encapsulamento) em `03_infra` — **pré-existentes
    e inalteradas** (registradas no mapa de migração; fora de escopo).
  - **2 V7** (`Prompt órfão`, **warnings**) — **consequência intencional de
    A.3.3**: os índices `stdlib.md` e `math/layout.md` deixaram de ser
    referenciados por `.rs`. Ver §5.
  - **0 drift** após `--fix-hashes`.
- **`git diff` dos `.rs`**: exclusivamente linhas `@prompt`/`@prompt-hash`
  (22 ficheiros, +44/−44 = 2 linhas cada). Qualquer outra mudança seria bug —
  nenhuma encontrada.

## 5. Tensão A.3.3 ↔ V7 (decisão para o dono)

A.3.3 manda manter o prompt velho como **índice** (trilha anti-F4). Mas o linter
marca prompt não-referenciado como **V7 órfão** (warning). Logo os 2 índices
geram 2 V7 — **warnings, não erros** (os 3 erros V9 não mudaram). Mantive os
índices (honra A.3.3; a trilha é exatamente o valor anti-F4 do projeto). **Opção
do dono**: aceitar os 2 warnings, OU apagar os 2 índices (a trilha continua em
ADR-0104 §"Prompts Afetados" + este relatório + histórico git) para zerar V7.

## 6. Candidatos a fatiamento futuro (medidos, NÃO fatiados — fora de escopo)

| prompt | `.rs` que o referenciam |
|---|---:|
| `engine/layout.md` | 11 |
| `rules/eval.md` | 10 |
| `rules/parse.md` | 7 |

## 7. Fora de escopo (confirmado intocado)

- Trait `Element` / código do D — é o **P316**.
- As 3 violações V9 pré-existentes em `03_infra` — não tocadas.
- Spec das funções não-specadas (deriva F4) — registadas, não inventadas.

---

**Sequência gravada (ADR-0105)**: P314 (este) → **P316+** (D por lotes; primeiro
lote no arranque) → **F** junto com o DEBT StyleChain 99.E.
