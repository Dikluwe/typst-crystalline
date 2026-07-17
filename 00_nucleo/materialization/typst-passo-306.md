# typst-passo-306 — fechar 15 funções `calc` triviais

**Tipo**: Passo de Execução (planeamento táctico)
**Data**: 2026-05-19
**Magnitude**: S+ agregado
**Pré-requisitos**: nenhum (sem ADR nova, sem crate externa)
**`erf` adiada**: passo separado posterior (P307+)

---

## 1. Objectivo do passo

Fechar 15 das 16 funções `calc` ausentes identificadas na
auditoria empírica de 2026-05-19. A 16ª (`erf`) fica para passo
posterior porque exige aproximação polinomial (decisão separada).

Cobertura stdlib calc após P306:
- 25/41 = 61% → 40/41 = **97,6%**.

---

## 2. Lista das 15 funções

Agrupadas semanticamente para servir de referência ao L0 que a IA
vai redigir na Fase 2 do Protocolo de Nucleação:

### 2.1 — Aritmética inteira e partes (6)

`trunc`, `fract`, `rem`, `rem_euclid`, `div_euclid`, `quo`

### 2.2 — Predicados inteiros (2)

`even`, `odd`

### 2.3 — Teoria dos números (2)

`gcd`, `lcm`

### 2.4 — Combinatória (3)

`fact`, `perm`, `binom`

### 2.5 — Normas e raízes (2)

`norm`, `root`

---

## 3. Ficheiros tocados (previsão)

### 3.1 — L0 (Fase 2 do Protocolo)

- `00_nucleo/prompts/engine/stdlib.md` — actualizar secção
  `calc` (9 → 24 funções), estender DEBT ADR-0018 (`f64::*`),
  reduzir lista de "Funções vanilla adiadas".

### 3.2 — L1 (Fase 4 do Protocolo)

- `01_core/src/engine/stdlib/calc.rs` — adicionar 15 funções
  `calc_*` + 15 entradas em `make_calc_module`.

Não tocados:
- `entities/content.rs` — hash preservado (23º passo
  consecutivo P282-P306).
- `export.rs` — bit-exact preservado.
- Demais módulos L1.

---

## 4. ADRs reusadas (zero novas)

Per pesquisa em `00_nucleo/adr/`:

| ADR | Decisão reusada |
|---|---|
| ADR-0017 | Estratégia gradual typst-library |
| ADR-0018 | `f64::*` aceite até `libm` autorizado |
| ADR-0024 | `EcoString` para mensagens de erro |
| ADR-0033 | Paridade funcional observable |
| ADR-0036 | Atomização — `&Args` explícito |
| ADR-0037 | Coesão por domínio — tudo em `stdlib/calc.rs` |
| ADR-0054 | Perfil observacional graded — IEEE 754 padrão |
| ADR-0059 | `Args` como input vehicle |

---

## 5. Protocolo de Nucleação — sequência prevista

Conforme `CLAUDE.md` §"Protocolo de Nucleação":

| Fase | Acção | Quem | Estado P306 |
|---|---|---|---|
| 1 | Passo planeia tarefas | Humano + IA (este doc) | a redigir |
| 2 | IA redige L0 (`stdlib.md` actualizado) | IA | aguarda Fase 1 |
| 3 | Humano grava L0 + `crystalline-lint --fix-hashes` | Humano | aguarda Fase 2 |
| 4 | IA escreve testes (devem falhar) | IA | aguarda Fase 3 |
| 5 | IA escreve implementação | IA | aguarda Fase 4 |
| 6 | Linhagem `@prompt-hash` + `crystalline-lint .` | IA | aguarda Fase 5 |

**Trava arquitectural**: IA não pode prosseguir para Fase 3 sem
confirmação humana de que L0 está gravado e hash propagado.

---

## 6. Granularidade — fixada

P306 é **passo único agregado**. Justificação:

- 15 funções puramente aditivas; zero decisões arquitecturais
  novas.
- Reusam helpers comuns (existentes em `stdlib/calc.rs`).
- Precedente P156C agregou 5 features Layout (M+).
- P304/P305 também agregam objectivos múltiplos.

ADR-0065 critério #2 aplicável (escolha isolada concreta).

---

## 7. Critérios de fecho do passo

P306 está fechado quando todas as fases do Protocolo de
Nucleação estiverem cumpridas:

- [ ] **Fase 2**: L0 `stdlib.md` actualizado com as 15 funções
- [ ] **Fase 3**: Hash propagado; `crystalline-lint .` zero
      violations no L0
- [ ] **Fase 4**: Testes escritos e a falhar antes da
      implementação (confirmar com `cargo test`)
- [ ] **Fase 5**: Implementação em `stdlib/calc.rs`; todos os
      testes verdes
- [ ] **Fase 6**: Headers `@prompt-hash` correctos;
      `crystalline-lint .` zero violations no workspace
- [ ] Relatório de passo escrito após o fecho

Invariantes a preservar:
- [ ] `entities/content.rs` hash inalterado
- [ ] `export.rs` bit-exact (23º consecutivo)
- [ ] Tests pré-existentes inalterados (sem regressão)

---

## 8. Pendências e adiamentos explícitos

- **`erf`** — adiada para P307+. Requer decisão sobre
  aproximação polinomial inline (paridade vanilla literal) vs
  outras vias. Não bloqueia P306.
- **Sincronização documental P284-P305** — independente; pode
  ser passo XS separado antes/depois de P306.
- **ADRs meta DeferredX + §8.4 subcat A** — adiamento consciente
  per anti-padrão P273.17 §0 (14ª vez consecutiva pós-P306 se
  mantido).

---

## 9. Próxima acção concreta

**Aguardar confirmação humana** para a IA prosseguir à Fase 2 do
Protocolo de Nucleação (redacção do L0 actualizado em
`00_nucleo/prompts/engine/stdlib.md`).

A IA não escreve código, testes, nem implementação até o L0
estar gravado e o hash propagado.
