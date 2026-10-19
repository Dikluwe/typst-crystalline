# typst-passo-309 — auditoria conformidade IEEE 754

**Tipo**: Passo de Execução (planeamento táctico)
**Sub-tipo**: diagnóstico-primeiro (sem código tocado)
**Data**: 2026-05-20
**Magnitude**: M documental
**Pré-requisitos**: nenhum (sem ADR nova; passo puramente diagnóstico)
**Materialização adiada**: passo separado posterior decidirá política

---

## 1. Motivação

P308 (`calc.erf`) implementou três short-circuits:
- `NaN → Err` (divergência consciente)
- `±∞ → ±1.0` (paridade)
- `±0 → ±0.0` (paridade)

A divergência `NaN → Err` levantou a pergunta: o cristalino segue
IEEE 754? A inspecção mostra que **não é uma divergência isolada
de `erf`** — é política transversal via helper `guard_float`,
documentada explicitamente em `stdlib.md`:

> `guard_float(f)` — NaN → Err "não é um número", Inf → Err
> "infinito"

E em `eval.md` (rules/eval):

> **Float → IEEE 754**: NaN e Inf propagados silenciosamente
> (sem guarda)

**Descoberta crítica**: o cristalino **já tem duas políticas IEEE 754
contraditórias** em sítios diferentes do código. `eval.rs` propaga
IEEE 754 natural; `stdlib.rs` rejeita NaN/Inf via `guard_float`.
Auditoria precisa mapear todos os sítios antes de qualquer decisão
de política.

---

## 2. Objectivo do passo

Produzir diagnóstico factual ancorado no código que responda:

1. **Quais funções/operações usam `guard_float` e rejeitam NaN/Inf?**
2. **Quais propagam IEEE 754 naturalmente?**
3. **Quais usam terceiro caminho (clamp, saturation, default)?**
4. **Que padrões alternativos existem** (`f64::*` directo,
   `checked_*`, validação prévia, etc.)?
5. **Vanilla typst respeita IEEE 754 em cada um destes sítios?**

Output: catálogo posicional (ficheiro:linha) com classificação
por categoria. Sem alterações ao código. Sem ADR nova.

---

## 3. Não-objectivos

P309 **não**:

- Decide política (Opção A/B/C — fica para passo subsequente).
- Altera código.
- Cria ADR nova.
- Toca testes.
- Reverte ou justifica decisões anteriores de `guard_float`.

O passo é deliberadamente diagnóstico-primeiro per ADR-0065 — padrão
estabelecido em P156B (Layout), P154A (Model), P307a (export).

---

## 4. Catálogo a produzir — categorias

O diagnóstico classifica cada sítio L1 que toca `f64` em quatro
categorias (paralelo a P307a §2 estrutura):

### Categoria A — Rejeita NaN/Inf (divergência IEEE 754)

Função/local que usa `guard_float` ou equivalente manual
(`is_nan() → Err`, `is_infinite() → Err`).

Sítios conhecidos preliminares:
- `stdlib::guard_float` (helper central)
- `calc_pow`, `calc_sqrt` (resultado guarded)
- `calc_sin`, `calc_cos`, `calc_tan` (resultado guarded — P283)
- `calc_log`, `calc_exp` (P283)
- `calc_erf` (P308) — NaN no input
- (a confirmar via grep)

### Categoria B — Propaga IEEE 754 naturalmente

Função/local que aceita NaN/Inf como input e/ou output sem
intervenção.

Sítios conhecidos preliminares:
- `eval_binary_op` (Add, Sub, Mul, Div com Float)
- Layout floats em `entities/layout_types.rs`
- Coordenadas em `FrameItem`
- (a confirmar via grep)

### Categoria C — Validação por intervalo (não IEEE 754)

Função/local que valida domínio antes de operação (`x < 0 → Err`,
`a > b → Err`).

Sítios conhecidos preliminares:
- `calc_sqrt` (negativo → Err)
- `calc_pow` (Int negativo → Err)
- `calc_clamp` (min > max → Err)
- `calc_log` (não-positivo → Err)
- `calc_root` (P306 — index 0, raiz par de negativo)
- `native_rgb` (0..=255)
- (a confirmar)

### Categoria D — Conversões/coerções

Função/local que converte entre tipos numéricos.

Sítios conhecidos preliminares:
- `coerce_to_f64` (Int → f64, Float → f64)
- `native_int(Float)` → Err (divergência consciente vanilla)
- Float → Int em `calc_floor/ceil/round`
- (a confirmar)

---

## 5. Ficheiros tocados (previsão — só leitura)

### 5.1 — Sob diagnóstico

- `01_core/src/engine/stdlib/calc.rs` (P306+P308)
- `01_core/src/engine/stdlib/mod.rs` (helpers `guard_float`,
  `coerce_to_f64`)
- `01_core/src/engine/stdlib/*.rs` (outros submódulos pós-P96.5)
- `01_core/src/engine/eval/binary.rs` (operações aritméticas)
- `01_core/src/engine/eval/unary.rs`
- `01_core/src/engine/layout/**` (matemática de coordenadas)
- `01_core/src/engine/math/layout/**` (matemática de equações)
- `01_core/src/entities/layout_types.rs` (`Length`, `Pt`, `Em`)
- `01_core/src/entities/color.rs` (componentes f64)

### 5.2 — Output novo

Um único ficheiro:

- `00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md`

Sem prompts L0 novos. Sem ADRs novas. Sem código tocado.

---

## 6. Estrutura proposta do diagnóstico

```
§1. Contexto e escopo
§2. Política IEEE 754 declarada
    §2.1. Em eval.rs (propaga)
    §2.2. Em stdlib.rs (rejeita via guard_float)
    §2.3. Contradição declarativa
§3. Inventário factual L1 (grep)
    §3.1. Categoria A — Rejeita NaN/Inf (tabela posicional)
    §3.2. Categoria B — Propaga IEEE 754 (tabela posicional)
    §3.3. Categoria C — Validação por intervalo (tabela)
    §3.4. Categoria D — Conversões (tabela)
§4. Comparação vanilla typst
    §4.1. Política vanilla declarada
    §4.2. Sítios equivalentes vanilla (lab/typst-original/)
    §4.3. Divergências sítio-a-sítio
§5. Política DEBT-libm (ADR-0018)
    §5.1. 7 sítios actuais f64::* (pós-P308)
    §5.2. Como libm afecta a decisão NaN
§6. Inventário de mensagens de erro emitidas
    §6.1. "não é um número"
    §6.2. "infinito"
    §6.3. Outros padrões
§7. Opções de política (sem decisão)
    §7.1. Opção A — Conformidade IEEE 754 total
    §7.2. Opção B — Relaxar selectivamente
    §7.3. Opção C — Manter status quo (documentado)
    §7.4. Opção D — Híbrido por categoria
§8. Implicações por opção (cobertura, paridade, testes)
§9. Recomendação operacional para passo seguinte
```

§9 é **recomendação** (per ADR-0065), não decisão. A decisão
política fica para o operador humano.

---

## 7. Protocolo de Nucleação — sequência prevista

| Fase | P309 (este passo diagnóstico) | P310+ (futuro material) |
|---|---|---|
| 1. Plano | este doc | aguarda decisão pós-P309 |
| 2. L0 | n/a (diagnóstico não toca L0) | passo material decide |
| 3. Hash | n/a | id. |
| 4. Testes | n/a | id. |
| 5. Implementação | n/a (só diagnóstico) | id. |
| 6. Validação | `crystalline-lint` continua zero | id. |

**Saída prevista**: 1 ficheiro markdown em `00_nucleo/diagnosticos/`.
Nenhum hash propagado. Nenhum teste alterado.

---

## 8. Granularidade — fixada

P309 é **passo único diagnóstico**. Justificação:

- Catálogo factual fechado por escopo (L1 + comparação vanilla).
- Sem decisão arquitectural intercalada.
- Precedente: P156B, P154A, P307a — todos diagnósticos-primeiro
  num único passo M.
- Sub-padrão "diagnóstico inline L0" N=2 (P306+P308) **não se
  aplica** — diagnóstico de P309 é ficheiro separado, não inline,
  porque inputs vêm de múltiplos prompts L0 dispersos.

---

## 9. Critérios de fecho

P309 está fechado quando:

- [ ] `diagnostico-ieee754-passo-309.md` publicado
- [ ] Tabelas §3.1-3.4 com pelo menos `ficheiro:linha:função` por
      sítio identificado
- [ ] §4 com comparação vanilla para pelo menos os sítios
      Categoria A (mais sensíveis)
- [ ] §7 com 4 opções enunciadas mas sem decisão
- [ ] §9 com recomendação operacional explícita

Invariantes a preservar:
- [ ] `entities/content.rs` hash inalterado (26º consecutivo)
- [ ] 9 snapshot binários export verdes (2º consecutivo pós-P307)
- [ ] Cobertura calc 41/41 inalterada
- [ ] Zero ADRs meta novas (16ª consecutiva)
- [ ] `crystalline-lint .` continua zero violations

---

## 10. Decisões adiadas explicitamente para passo posterior

Estas decisões **não** são objectivo de P309:

1. **Adoptar Opção A/B/C/D**: política IEEE 754 final.
2. **Reverter `erf(NaN) → Err`** para `erf(NaN) → NaN`.
3. **Remover `guard_float`** em sítios específicos.
4. **Migração agregada `libm`** (ADR-0018 DEBT-libm) —
   ortogonal mas pode informar política.
5. **ADR-`erf`-revogação** ou ADR-IEEE754-conformidade.

Cada uma destas é magnitude S-M+ por si só. P309 só identifica
o terreno; passos subsequentes negoceiam.

---

## 11. Sub-padrão emergente — observação meta

**"Auditoria de política transversal pós-divergência local
isolada"** — primeira ocorrência:

- P308 implementou divergência local (`erf(NaN) → Err`).
- Relatório §10.3 reconheceu divergência consciente.
- Pergunta humana subsequente expôs que divergência é
  sistémica (`guard_float`), não local.
- P309 emerge como diagnóstico transversal.

Candidato a observação futura se padrão repetir (N≥3).

---

## 12. Próxima acção concreta

**Aguardar confirmação humana** para a IA prosseguir.

Decisões a tomar pelo humano antes do arranque:

1. **Arrancar P309?** Diagnóstico-primeiro, baixo risco, sem
   código tocado.

2. **Escopo do diagnóstico**:
   - α) L1 completo (proposta acima).
   - β) Só stdlib (mais restrito; perde sítios eval/layout).
   - γ) Só calc (ainda mais restrito; perde paralelos
     `eval_binary_op`).

3. **Comparação vanilla obrigatória ou opcional?**
   - α) Obrigatória para Categoria A (sítios sensíveis).
   - β) Obrigatória para todas as categorias (mais trabalho).
   - γ) Opcional/best-effort.

Recomendações:
- Escopo α (L1 completo) — diagnóstico é mais útil quando
  vê todo o terreno.
- Comparação α (Categoria A obrigatória) — sítios menos
  sensíveis podem ficar com nota "não verificado vs vanilla".

Confirma escolhas e a IA arranca a redacção do diagnóstico
factual. Sem confirmação, P309 fica em standby.
