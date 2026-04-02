# Inventário de ADRs — Decisões Arquitecturais

**Data**: 2026-04-01
**Total**: 31 ADRs (0001–0031)

---

## ADRs do projecto typst-crystalline

| ADR | Título | Estado | Resumo |
|-----|--------|--------|--------|
| 0001 | Estratégia de migração Typst → Cristalina | PROPOSTO | `comemo` autorizado em L1; código original para `lab/`; sequência de 10 passos iniciais; `crystalline.toml` inicial |
| 0002 | Traits para contratos de regras | IMPLEMENTADO | Mocks locais para testes L1 |
| 0003 | *(não documentado no knowledge)* | — | — |
| 0004 | Descobertas do Passo 1 | IMPLEMENTADO | Zero-copy com lifetimes; FQN em L3; Fail-Fast V0; `PackageSpec` DTO pattern |
| 0005 | `PackageSpec` e `World` | IMPLEMENTADO | `Cow<'a, Path>` elimina `Box::leak`; B3 blanket impl para `TrackedWorld` |
| 0006 | `typst_timing` → no-op macro | IMPLEMENTADO | `timing_scope!` como stub; pontos de instrumentação registados em DEBT.md |
| 0007 | Remover `rustc_hash` do parser | **REVOGADO** por ADR-0018 | Erro: aplicou critério "externo = proibido" sem verificar se violava L1 |
| 0008 | `defer` inlinado de `typst_utils` | IMPLEMENTADO | Função RAII `defer()` copiada com licença Apache-2.0 |
| 0009 | *(linter: parsers por linguagem)* | IMPLEMENTADO | `ImportKind` semântico; V4 multi-linguagem |
| 0010 | `unicode_ident` em L1 | IMPLEMENTADO | Primeira crate Unicode autorizada |
| 0011 | `unicode_math_class` em L1 | IMPLEMENTADO | Classificação de caracteres matemáticos |
| 0012 | `unicode_script` em L1 | IMPLEMENTADO | Detecção de scripts Unicode |
| 0013 | `unicode_segmentation` em L1 | IMPLEMENTADO | UAX #29 — segmentação de graphemes |
| 0014 | V11 configurável | IMPLEMENTADO | `DanglingContract` pode ser suprimido por configuração |
| 0015 | `ecow`/`EcoString` → decisão | IMPLEMENTADO | Substituição de `ecow` avaliada; `String` usado onde possível |
| 0016 | Stubs para `Module` e `Value` | IMPLEMENTADO | Stubs opacos `Module(())` e `Value(())` até migração real |
| 0017 | Adiamento de `eval()` e estratégia typst-library | IMPLEMENTADO | `eval()` é L1 (não L3); depende de análise completa de `typst-library` |
| 0018 | `rustc_hash` → `[l1_allowed_external]` | IMPLEMENTADO | **Revoga ADR-0007**; `FxHashMap`/`FxHashSet` restaurados no parser |
| 0019 | `ttf-parser`/`rustybuzz` em L3 | IMPLEMENTADO | Parsing de fontes autorizado em L3 |
| 0020 | *(não documentado no knowledge)* | — | — |
| 0021 | Crate `time` em L1 | IMPLEMENTADO | `Datetime` real com validação via `time` |
| 0022 | `FontBook` real em L3 | IMPLEMENTADO | `FontInfo`, `FontVariant`, métodos de pesquisa |
| 0023 | `indexmap` em L1 | IMPLEMENTADO | Ordem de inserção em `Scope`; sem equivalente no std |
| 0024 | `ecow`/`EcoString` em L1 | IMPLEMENTADO | Strings eficientes para tipos de domínio |
| 0025–0027 | *(passos 25–27)* | — | Tipos tipográficos, correcções arquitecturais |
| 0028 | *(revogado)* | **REVOGADO** por ADR-0029 | Definição de pureza física — primeira tentativa |
| 0029 | Definição de pureza física | IMPLEMENTADO | Substitui ADR-0028; L1 purity = zero system I/O |
| 0030 | Correcção filosófica ADR-0004/0015 | IMPLEMENTADO | RAM performance não viola pureza L1 |
| 0031 | Early hashing para `Source` | IMPLEMENTADO | `content_hash: u64` via `FxHasher` no momento de criação |

---

## Lições registadas nos ADRs

### Erro corrigido: ADR-0007 → ADR-0018
O critério para `[l1_allowed_external]` **não é** "é externo?" mas "viola pureza funcional de L1?". Crates sem I/O, sem estado global mutável e sem efeitos colaterais são elegíveis independentemente de serem do std ou não.

### Padrão B3 (ADR-0005)
`TrackedWorld: World` como supertrait é incompatível com `#[comemo::track]`. A proc-macro gera tipos que não implementam automaticamente o supertrait. Solução: blanket implementation sem supertrait.

### `eval()` é L1, não L3 (ADR-0017)
`typst-library` está mal estratificado no original — `eval()` tem dependências de I/O porque os tipos em que depende misturam domínio e infra. Isso não muda a classificação: `eval()` é motor central do compilador, pertence a L1.

### Dívida antes de código (padrão geral)
Dívida técnica é registada em `DEBT.md` **antes** de escrever código de implementação, nunca depois. Cada item de dívida tem um plano de eliminação com passo estimado.

---

## ADRs por tema

### Crates autorizadas em L1
ADR-0001 (comemo), ADR-0010 (unicode_ident), ADR-0011 (unicode_math_class), ADR-0012 (unicode_script), ADR-0013 (unicode_segmentation), ADR-0018 (rustc_hash), ADR-0021 (time), ADR-0023 (indexmap), ADR-0024 (ecow)

### Decisões de camada
ADR-0017 (eval é L1), ADR-0019 (ttf-parser é L3), ADR-0022 (FontBook em L3)

### Padrões de implementação
ADR-0005 (B3 blanket impl), ADR-0006 (timing_scope no-op), ADR-0008 (defer inlinado), ADR-0031 (early hashing)

### Definição de pureza
ADR-0029 (pureza física = zero system I/O), ADR-0030 (RAM ≠ I/O)

### Revogações
ADR-0007 → revogado por ADR-0018; ADR-0028 → revogado por ADR-0029
