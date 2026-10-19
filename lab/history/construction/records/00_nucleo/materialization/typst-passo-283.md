# Passo 283 — `calc` trig + hiperbólicas + log/exp

**Frente**: `P-stdlib-calc-trig` (P282 §3 #1; recomendada em §9 estratégia C).
**Data prevista**: próxima sessão (pós-P282).
**Pré-requisitos**: nenhum — frente sem bloqueador, primeira da lista de
quick wins ROI alto.

---

## §1 — Objectivo

Expandir `make_calc_module` em `01_core/src/engine/stdlib.rs` (ou
submódulo equivalente pós-ADR-0037) com as funções matemáticas
vanilla actualmente em falta — trigonométricas, hiperbólicas,
logarítmicas e exponenciais.

Estado actual (P282 §A2): `calc` tem 9 funções (`abs`, `pow`,
`sqrt`, `floor`, `ceil`, `round`, `min`, `max`, `clamp`).
Cobertura estimada ~22%. Vanilla typst expõe ~25 funções no
módulo `calc`.

Objectivo numérico: subir cobertura `calc` de ~22% para ~70%
adicionando o bloco trig+hyperbolic+log+exp (~16 funções
faltantes, todas XS empíricas — uma chamada `f64::*` + guards
NaN/Inf + 1-3 testes cada).

Objectivo arquitectural: nenhum tipo novo. Nenhum ADR novo
**condicional** — só se Fase A revelar decisão não-trivial
(ver §3).

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Há duas ambiguidades factuais que requerem inventário antes
de materializar:

### A.1 — Scope exacto vs vanilla

Listar literalmente todas as funções do módulo `calc` em
vanilla typst (consultar `lab/typst-original/` ou docs.typst.app)
e separar em três buckets:

1. **Materializar neste passo** — trig/hyperbolic/log/exp puras
   sobre `f64`, semântica clara, zero dependências de tipos
   tipográficos cristalinos.
2. **Adiar com justificação** — funções que dependem de tipos
   ainda não materializados (`Angle` com conversão automática
   deg/rad; `fract` se interagir com `Length`; etc.).
3. **Já existentes** — confirmar que `abs`/`pow`/`sqrt`/etc.
   estão completas vs vanilla (paridade de assinatura, não só
   nome).

Output: tabela em diagnóstico
`00_nucleo/diagnosticos/diagnostico-calc-passo-283.md` com 3
colunas (nome, bucket, justificação).

### A.2 — `libm` vs `f64::*` (decisão pendente desde ADR-0018)

ADR-0018 §"DEBT" registou: *"calc_pow usa f64::powf directamente
em vez de libm::pow. Quando libm for adicionado como dependência
do workspace, migrar."*

Este passo é a oportunidade natural para resolver a pendência —
não para escrever ~16 chamadas `f64::*` agora e migrar todas
depois. Decisão a tomar na Fase A:

| Opção | Prós | Contras |
|---|---|---|
| **(a)** Materializar com `f64::*` agora; migrar a `libm` em passo dedicado | Sem dependência nova nesta sessão | Garante 2× trabalho futuro (~17 sítios) |
| **(b)** Adicionar `libm` a `[l1_allowed_external]` e usar `libm::*` desde já | Resolve ADR-0018 §DEBT; uma única materialização | Decisão arquitectural (whitelist L1) que pode merecer ADR |
| **(c)** Materializar com `f64::*` mas extrair helper `fn trig_op(f: fn(f64) -> f64, ...)` que centraliza guards; migração futura toca 1 sítio | Compromisso; preserva ADR-0018 §DEBT mas reduz custo de migração | Helper extra que pode ser ruído |

Decisão é da Fase A. Default sugerido: **(c)** se o helper
emergir natural durante materialização; **(a)** se o helper for
forçado. **(b)** apenas se decisão humana explicita preferir.

### A.3 — Tratamento de domínio

Funções com domínio restrito (`ln(x≤0)`, `log(x≤0)`,
`asin(|x|>1)`, `acos(|x|>1)`, `sqrt` já existente serve de
precedente) precisam decidir entre:

- **Err explícito** (precedente `calc_sqrt` em negativo).
- **NaN propagado via guard_float** (helper já existente
  em stdlib.rs).

Vanilla typst: confirmar qual comportamento na Fase A.

---

## §3 — Materialização

Após Fase A produzir scope literal + decisão libm + tratamento
de domínio:

1. Adicionar funções `calc_<nome>` no módulo stdlib.
2. Registar cada uma no `make_calc_module` (Dict).
3. Reusar helpers existentes (`coerce_to_f64`, `guard_float`).
4. Testes unitários L1 por função: pelo menos
   - 1 caso típico (`sin(0) = 0`, `ln(e) ≈ 1`).
   - 1 caso de fronteira (`sin(π) ≈ 0` com tolerância;
     `acos(1) = 0`).
   - 1 caso de erro/domínio se aplicável (`ln(-1)` → Err ou NaN
     conforme A.3).
5. Actualizar L0 `00_nucleo/prompts/engine/stdlib.md` com tabela
   das funções novas + propagar hash via `crystalline-lint
   --fix-hashes`.

**Sem caps** (per P282 §7). Range numérico esperado de testes:
~30-50 (16 funções × 2-3 testes). Reformula-se a estimativa se
Fase A revelar scope diferente.

---

## §4 — Critério de fecho

- `cargo test --workspace` verde (baseline P282: 2624 testes;
  esperado ~2654-2674 pós-P283).
- `crystalline-lint` zero violations.
- Hash L0 stdlib.md propagado.
- Tabela cobertura `typst-cobertura-vanilla-vs-cristalino.md`
  actualizada na linha `calc`: ~22% → ~70%.
- Diagnóstico A produzido (mesmo se decisões forem triviais —
  documenta o scope materializado).
- Sem regressões em hash L0 `export.rs` (`bc7b8b95` preserved —
  passo não toca em export).

---

## §5 — Não-objectivos

Explicitar para evitar deriva de scope:

- **Não** materializar `Angle` com conversão automática deg/rad.
  Funções trig recebem `f64` em radianos directamente (igual ao
  `f64::sin`). Se vanilla typst usa `Angle` automático,
  registar em A.1 bucket 2 (adiar) com referência cruzada a
  ADR-0028 / ADR-0029 (sistema tipográfico simplificado).
- **Não** adicionar funções estatísticas (`gcd`, `lcm`,
  `quo`, `rem`) se aparecerem em A.1 — são scope diferente.
  Listar em A.1 bucket 2 com justificação "scope distinto;
  passo dedicado".
- **Não** mexer em `calc_pow` existente. ADR-0018 §DEBT migra
  para libm **se A.2 escolher (b)**; caso contrário fica
  inalterado.
- **Não** alterar `make_calc_module` para `Value::Module` em
  vez de `Value::Dict`. Divergência registada em stdlib.md está
  estabilizada.

---

## §6 — Pendência aberta resolvível em paralelo

`P-line-color-rg-emit` (P282 §1.5) — emit `RG` em Line.
**Não incluir neste passo**. Listada como frente #6 separada;
toca export.rs (este passo não); justifica passo próprio para
manter o critério de fecho de P283 limpo (hash export.rs
preserved).

---

## §7 — Risco residual

Único risco identificado: A.2 pode revelar que decidir libm vs
f64 merece ADR formal (precedente ADR-0007 → ADR-0018 — o ciclo
de remover/reintroduzir `rustc_hash` mostra que decisões sobre
dependências L1 não são triviais).

Mitigação: se A.2 → opção (b), abrir sub-passo P283.1 dedicado
à promoção da decisão (ADR nova revogando ADR-0018 §DEBT ou
estendendo-a). Materialização das funções **só depois** do
sub-passo .1. Per ADR-0065, decisão arquitectural não-trivial
exige inventário-primeiro — A.2 é exactamente o caso.

Se A.2 → (a) ou (c), sem ADR nova; passo procede directo.

---

## §8 — Ponteiros

- L0 actual: `00_nucleo/prompts/engine/stdlib.md` §"Módulo calc".
- Código actual: `01_core/src/engine/stdlib.rs` (`make_calc_module`,
  `calc_abs`, ..., `calc_clamp`).
- ADR relevante: ADR-0018 (`rustc_hash` reintroduzido — usar
  como precedente metodológico para A.2; **não** confundir com
  decisão sobre libm que é deste passo).
- ADR processual: ADR-0065 (inventariar-primeiro — justifica
  Fase A obrigatória).
- Helpers reusáveis: `coerce_to_f64`, `guard_float`,
  `format_float`.

---

*Spec P283 produzida 2026-05-18. Frente `P-stdlib-calc-trig` —
quick win ROI alto, zero bloqueadores. Fase A obrigatória
(scope vanilla + decisão libm + tratamento domínio). Sem caps
LOC ou magnitude (P282 §7). Materialização condicional ao
resultado de A.2.*
