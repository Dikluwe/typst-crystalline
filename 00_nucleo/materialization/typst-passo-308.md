# typst-passo-308 — fechar `calc.erf` (paridade calc 100%)

**Tipo**: Passo de Execução (planeamento táctico)
**Data**: 2026-05-20
**Magnitude**: XS-S
**Pré-requisitos**: nenhum (sem ADR nova, sem crate externa)
**Posição na série**: complementa P306 (15 calc triviais) — fecha a categoria.

---

## 1. Objectivo do passo

Materializar a 41ª e última função `calc` ausente: `erf` (função
erro de Gauss).

Cobertura stdlib calc após P308:
- 40/41 = 97,6% → **41/41 = 100%**.

Esta é a primeira categoria da stdlib a atingir paridade
literal completa face ao vanilla.

---

## 2. Contexto matemático

`erf(x)` é a função erro de Gauss:

```
erf(x) = (2 / sqrt(π)) * ∫₀ˣ exp(-t²) dt
```

Propriedades relevantes:
- `erf(0) = 0`
- `erf(-x) = -erf(x)` (função ímpar)
- `lim x→+∞ erf(x) = 1`
- `lim x→-∞ erf(x) = -1`
- Não tem forma fechada elementar — implementação via aproximação.

Vanilla typst (`typst-library/src/foundations/calc.rs`) usa
aproximação polinomial inline. P308 reproduz o mesmo método para
paridade observable.

---

## 3. Decisão de aproximação — três caminhos

P308a (sub-passo diagnóstico) deve fixar qual destes:

### Caminho A — Abramowitz & Stegun 7.1.26

Aproximação polinomial clássica (5 coeficientes + constante):

```
erf(x) ≈ 1 - (a₁t + a₂t² + a₃t³ + a₄t⁴ + a₅t⁵) * exp(-x²)
onde t = 1 / (1 + p*|x|)
```

- Erro máximo: 1,5 × 10⁻⁷.
- ~10 linhas Rust.
- Determinístico sob IEEE 754.
- **Hipótese principal** — paridade vanilla literal provável.

### Caminho B — Série de Taylor truncada

```
erf(x) = (2/sqrt(π)) * Σ ((-1)ⁿ * x^(2n+1) / (n! * (2n+1)))
```

- Mais preciso para `|x|` pequeno.
- Convergência lenta para `|x|` grande.
- Requer corte adaptativo.
- ~20 linhas.

### Caminho C — Cody (Chebyshev rational approximations)

- Erro < 2⁻⁵³ (precisão IEEE 754 dupla completa).
- Implementação muito mais longa (~80 linhas, múltiplos branches).
- Excessivo para ADR-0054 perfil graded.

### Recomendação

**Caminho A** salvo se P308a detectar que vanilla usa outro.
Justificação:
- Paridade vanilla literal mais provável.
- Erro 1,5×10⁻⁷ está dentro da tolerância ADR-0054 graded.
- Volume mínimo (~10 linhas) consistente com magnitude XS-S.

A decisão final é parte do diagnóstico P308a, não desta spec.

---

## 4. Ficheiros tocados (previsão)

### 4.1 — L0 (Fase 2 do Protocolo de Nucleação)

- `00_nucleo/prompts/rules/stdlib/calc.md` (ou equivalente
  granular pós-P307c) — actualizar secção `calc`:
  - Cabeçalho: 40 → 41 funções.
  - Nova subsecção `erf(x)` com contrato observable.
  - Lista "Funções vanilla adiadas" — remover `erf`.
  - Critérios de verificação para `erf` (~6-10 casos).

### 4.2 — L1 (Fase 4-5 do Protocolo)

- `01_core/src/rules/stdlib/calc.rs` — adicionar:
  - `calc_erf` (função native ~15 linhas).
  - Entrada em `make_calc_module`: `dict.insert("erf", ...)`.

### 4.3 — Não tocados

- `entities/content.rs` — hash preservado (24º consecutivo após
  P306; P307 substituiu rastreio export.rs).
- `export/*` — bit-exact binário preservado (snapshot binário
  pós-P307 inalterado).
- Demais módulos L1.

---

## 5. Diagnóstico P308a — perguntas a responder

Sub-passo diagnóstico de magnitude XS-S antes do L0:

1. **Que aproximação vanilla usa?** Inspecção directa de
   `lab/typst-original/crates/typst-library/src/foundations/calc.rs`.
2. **Que precisão é aceitável?** ADR-0054 perfil graded define
   tolerância. Caminho A (1,5×10⁻⁷) provavelmente suficiente.
3. **Que casos de borda?** `erf(0) = 0`, simetria ímpar,
   `erf(∞) → 1`, `erf(NaN) = NaN`, `erf(±0)` sinal.
4. **Aceita Int ou só Float?** Vanilla provavelmente aceita
   ambos (coerce Int → Float).
5. **Argumentos múltiplos?** Pouco provável — `erf` é unária.
6. **Mensagem de erro para NaN/Inf?** Pelo padrão P306,
   `guard_float` no resultado captura ambos.

Output: secção curta em `diagnostico-erf-passo-308a.md` (~50 linhas)
ou inline no diagnóstico de P308 directamente, dependendo da
política de granularidade.

---

## 6. ADRs reusadas (zero novas)

| ADR | Decisão reusada |
|---|---|
| ADR-0017 | Estratégia gradual typst-library |
| ADR-0018 | `f64::*` aceite (`f64::exp`, `f64::abs`) até `libm` |
| ADR-0024 | `EcoString` para mensagens de erro |
| ADR-0033 | Paridade funcional observable |
| ADR-0036 | Atomização — `&Args` explícito |
| ADR-0037 | Coesão por domínio — entrada em `stdlib/calc.rs` |
| ADR-0054 | Perfil observacional graded — aproximação IEEE 754 padrão |
| ADR-0059 | `Args` como input vehicle |

---

## 7. Protocolo de Nucleação — sequência prevista

Conforme `CLAUDE.md` §"Protocolo de Nucleação":

| Fase | Acção | Quem | Estado P308 |
|---|---|---|---|
| 1 | Passo planeia tarefas | Humano + IA (este doc) | a redigir |
| 1.5 | P308a diagnóstico (opcional) | IA | aguarda Fase 1 |
| 2 | IA redige L0 (`calc.md` actualizado) | IA | aguarda 1.5 |
| 3 | Humano grava L0 + `crystalline-lint --fix-hashes` | Humano | aguarda Fase 2 |
| 4 | IA escreve testes (devem falhar) | IA | aguarda Fase 3 |
| 5 | IA escreve implementação | IA | aguarda Fase 4 |
| 6 | Linhagem `@prompt-hash` + `crystalline-lint .` | IA | aguarda Fase 5 |

**Trava arquitectural**: IA não pode prosseguir para Fase 3 sem
confirmação humana de que L0 está gravado e hash propagado.

Fase 1.5 (diagnóstico) é opcional — podes pedir que P308 vá
directo para Fase 2 se confiares na recomendação Caminho A.

---

## 8. Granularidade — fixada

P308 é **passo único**. Justificação:

- 1 função apenas; aditiva.
- Reusa helpers existentes (`coerce_to_f64`, `guard_float`,
  `expect_no_named` pós-P306).
- Sem decisão arquitectural nova.
- Precedente P306 inline pattern aplica-se: match-pattern em
  vez de helpers agregados.

Diagnóstico P308a pode estar inline no L0 directamente (sub-passo
opcional dispensável dada a magnitude XS-S).

---

## 9. Critérios de fecho do passo

P308 está fechado quando todas as fases do Protocolo de
Nucleação estiverem cumpridas:

- [ ] **Fase 1.5** (opcional): diagnóstico fixa aproximação A/B/C
- [ ] **Fase 2**: L0 `calc.md` actualizado (40 → 41 funções)
- [ ] **Fase 3**: Hash propagado; `crystalline-lint .` zero
      violations no L0
- [ ] **Fase 4**: ~6-10 testes unitários escritos e a falhar
      antes da implementação
- [ ] **Fase 5**: `calc_erf` em `stdlib/calc.rs`; testes verdes
- [ ] **Fase 6**: Headers `@prompt-hash` correctos;
      `crystalline-lint .` zero violations no workspace
- [ ] Relatório de passo P308 escrito após fecho
- [ ] L0 prompt: lista "Funções vanilla adiadas" remove `erf`
      (cobertura calc 40/41 → 41/41 anotada)

Invariantes a preservar:
- [ ] `entities/content.rs` hash inalterado (24º consecutivo)
- [ ] 9 snapshot binários export/ verdes (1º consecutivo pós-P307)
- [ ] Tests pré-existentes inalterados

---

## 10. Estimativa de tests

Casos de verificação propostos (a refinar em L0):

| Categoria | Casos |
|---|---|
| Identidade | `erf(0) = 0` |
| Simetria | `erf(-x) = -erf(x)` para vários x |
| Limites | `erf(5)` próximo de 1; `erf(-5)` próximo de -1 |
| Valores conhecidos | `erf(1) ≈ 0,8427`; `erf(0.5) ≈ 0,5205` |
| Tipo Int aceite | `erf(1)` (Int) = `erf(1.0)` (Float) |
| Float especiais | `erf(NaN)` → Err ou NaN; `erf(Inf)` → 1 |
| Args inválidos | `erf()` Err; `erf(1, 2)` Err; `erf(named: x)` Err |

Total: **~6-10 testes** unitários + **1-2 testes E2E** paridade
(em `lab/parity/corpus/`).

---

## 11. Cobertura pós-P308

**Stdlib calc**:
- Antes: 40/41 = 97,6%.
- Depois: 41/41 = **100%**.

**Stdlib agregada** (P306 §7):
- ~54% → ~54,5% (calc é 1 de 9 categorias; `erf` é 1/40 da
  categoria já quase fechada).

**Cobertura global** (per análise pós-P306):
- A.8 Foundations: ganho marginal (+0,1pp global).

O ganho material é categórico (primeira categoria a fechar),
não numérico.

---

## 12. Pendências adiadas explicitamente

- **`Length`/`Angle`/`Decimal`/`digits`** em funções existentes
  — magnitude M+, requer tipo `Angle` que não existe ainda. Não
  bloqueia P308.
- **`bb`/`cal`/`frak`/...** (12 math style) — categoria diferente
  (text style), candidato P309.
- **Data parsing** (`json`/`csv`/...) — requer I/O via World.
- **ADRs meta adiadas** — anti-padrão P273.17 §0 honrado 14ª vez
  consecutiva pós-P307 (ADR-0100 distinta); preservação contínua
  natural em P308 (passo aditivo trivial sem fricção meta).

---

## 13. Próxima acção concreta

**Aguardar confirmação humana** para a IA prosseguir.

Decisão a tomar pelo humano:

1. **Inclui Fase 1.5** (diagnóstico P308a) **ou vai directo
   para Fase 2** (L0)?
   - Opção α: Fase 1.5 inclusa — IA inspecciona vanilla
     `calc.rs`, fixa A/B/C, escreve diagnóstico curto, então L0.
   - Opção β: Fase 1.5 dispensada — IA assume Caminho A e vai
     directo para L0 (com nota no relatório se inspecção
     posterior revelar divergência face a vanilla).

2. **Granularidade do diagnóstico** se Opção α:
   - Ficheiro dedicado `diagnostico-erf-passo-308a.md`.
   - Ou inline no L0.

A IA não escreve código, testes, nem implementação até o L0
estar gravado e o hash propagado.
