# Sonda em lote — os "ausentes" da Lista A: estado real e ordem (Passo 389)

**Tipo:** Sonda de viabilidade (read-only; ADR-0108). **Não materializa código.**
**Data:** 2026-06-22. **HEAD:** `08fed76d3` (pós-P387).
**Lição aplicada (P388):** rótulo do Inventário ≠ código; conferir adiamento deliberado antes de
tratar "ausente" como dívida. Esta sonda fá-lo para os 16 de uma vez.

> **Método.** Grep de existência (`file:line`) do símbolo / variant `Content` / função stdlib /
> tipo `Value`; classificação do estado; varredura de ADR/DEBT por adiamento com gatilho; mapa de
> dependência de tipo (ADR-0017); custo do substrato. A contagem é grep; a natureza da ausência é
> julgamento (confirmável pelo dono).

---

## 1. Resultado em uma frente — dos 16 "ausentes", **só ~6 são dívida genuína**

| Balde | Nº | Features |
|-------|---:|----------|
| **A — mal-rotulado, já-feito** (sai da fila; ganho) | 4 | `cmyk`, `oklab`, `gradient` (parcial), `pad/corners/sides` |
| **B — adiamento deliberado** (DEBT-53 shaping; fora da fila de dívida) | 5 | `smallcaps`, `text.dir`, `text.region`, `text.script`, soft-hyphen |
| **C — bloqueado por tipo** (ADR-0017; tipo antes da feature) | 1 | `tiling` (precisa `Value::Tiling`) |
| **D — dívida genuína acidental** (a fila real) | 6 | `square`, `lorem`, `panic`, `#show` selector, `eval`, Model (`document`/`title`/`asset`) |

O mesmo padrão do P388, multiplicado: a maioria dos "ausentes" ou já existe, ou é scope-out
deliberado. A fila de dívida real é **~6 itens**, não 16.

---

## 2. Tabela por feature (estado · substrato · natureza · custo · `file:line`)

### A — mal-rotulado / já-feito (remover da Lista A)

| Feature | Estado real | `file:line` |
|---------|-------------|-------------|
| `cmyk(...)` | **implementado** (P257/ADR-0083) | `foundations.rs:175` (`native_cmyk`) + registado `eval/mod.rs` |
| `oklab(...)` | **implementado** (P257) | `foundations.rs:94` (`native_oklab`); `oklch` `:120` |
| `gradient(...)` | **parcial** (Linear ativo; conic/radial ausentes) | `eval/mod.rs:899` (`make_gradient_module`); `value.rs:84` (`Value::Gradient`, P262/ADR-0087) |
| `pad/corners/sides` (inset modeling) | **mal-rotulado** — `pad()` é `implementado⁺`; a linha era refino/duplicado de `PageConfig` (Fase 3 ADR-0061), não feature nova | `layout.rs:239` (`inset` via `Sides`); nota da Lista A "duplica pad() linha" |

### B — adiamento deliberado (DEBT-53 shaping/rustybuzz — **não é dívida**)

| Feature | Gatilho de adiamento | `file:line` |
|---------|----------------------|-------------|
| `smallcaps` | OpenType features → rustybuzz | DEBT-53 (`DEBT.md:2103`) |
| `text.dir` (LTR/RTL) | bidi shaping (unicode-bidi + rustybuzz) | DEBT-53 |
| `text.region` | regional variants (shaping) | DEBT-53 |
| `text.script` | super/sub script standalone (shaping) | DEBT-53 |
| soft hyphen (`\u{00AD}`) | hyphenation espera literal `-` (P144); refino shaping | DEBT.md / Tabela C |

> Estes 5 são o **paralelo do hayagriva**: ausência por adiamento com gatilho nomeado
> (rustybuzz, XL — `DEBT.md:2142`). Saem da fila de dívida; materializam quando o shaping entrar.

### C — bloqueado por tipo (ADR-0017 — o tipo vem antes)

| Feature | Tipo em falta | `file:line` |
|---------|---------------|-------------|
| `tiling(...)` | `Value::Tiling` (comentado) | `value.rs:89` (`// Tiling(Tiling)`) |

### D — dívida genuína acidental (a fila real)

| Feature | Custo | Substrato (o que falta) | `file:line` |
|---------|:-----:|--------------------------|-------------|
| `square(...)` | **XS** | `ShapeKind::Square` ou derivar de `Rect` com `width==height` | `shapes.rs:48` (`native_rect` existe; sem `square`) |
| `lorem(n)` | **XS** | helper stdlib gerador de texto; **sem shaping** (independente de DEBT-53) | ausente em `make_stdlib` |
| `panic(msg)` | **XS** | helper stdlib que aborta eval com mensagem | ausente |
| `#show <selector>` (regex) | **S** | `Selector::Regex` **já existe**; falta o wiring da aplicação show-rule com regex | `selector.rs:48` (`Regex(Regex)`, ADR-0077); `rules.rs:96` (`apply_show_rules`) |
| `eval(string)` | **M** | runtime de re-parse + re-eval de `String`→`Value` | ausente (sem `native_eval`) |
| Model: `document`/`title`/`asset` | **M** | variants/metadata de documento (`DocumentElem`+title+asset) | ausente (zero traços em L1) |

> **Nuance do `#show`:** a parte **regex** está quase desbloqueada (o tipo `Selector::Regex` foi
> fechado em P209D/ADR-0077; falta só ligar à aplicação de show-rules). A parte **`.where(field:)`**
> (predicado por campo) precisa de `Selector::Where`, **ausente** — mais pesada. Tratar como duas
> meias-features: regex (S) já habilitada, where (M) ainda não.

---

## 3. Tipos habilitadores (Tabela C) — dependências mapeadas (critério 3)

| Tipo `Value` | Estado | Feature dos 16 que o exige |
|--------------|--------|----------------------------|
| `Value::Tiling` | comentado (`value.rs:89`) | **`tiling()`** — bloqueia diretamente |
| `Value::Decimal` | comentado (`value.rs:93`) | nenhuma dos 16 (habilita literais decimais; debt standalone) |
| `Value::Duration` | comentado (`value.rs:94`) | nenhuma dos 16 (habilita aritmética de duração) |
| `Value::Version` | comentado (`value.rs:91`) | nenhuma dos 16 (habilita comparação semver) |
| `Value::Bytes` | comentado (`value.rs:92`) | já **DEBT-62** (P387); habilita `read` binário + cbor byte-strings |

Regra ADR-0017: onde uma feature exige um tipo comentado, **o tipo é um passo anterior**. Só
`tiling` (dos 16) tem essa dependência dura.

---

## 4. Ordem aberta de materialização (revisável — critério 4)

Ranqueada por **custo crescente × dependências resolvidas primeiro × risco de substrato**. Só o
balde D (dívida genuína); B e C ficam fora (adiados/bloqueados).

1. **`square`** — XS, zero deps. Derivar de `Rect` (`width==height`) ou `ShapeKind::Square`.
2. **`lorem`** — XS, zero deps, independente de shaping. Gerador de texto stdlib.
3. **`panic`** — XS, zero deps. Helper que aborta eval com mensagem (paralelo a `assert`).
4. **`#show regex(...)`** — S. `Selector::Regex` pronto; só falta o wiring na aplicação de
   show-rules. (`.where()` fica para depois — precisa `Selector::Where`.)
5. **`eval(string)`** — M. Runtime de re-eval; maior superfície, sem tipo novo.
6. **Model `document`/`title`/`asset`** — M. Cluster de metadados de documento; `document` primeiro
   (habilita `title`/`asset` como campos/derivados).

**Fora da fila (não dívida):**
- **Adiados (DEBT-53 shaping):** `smallcaps`, `text.dir`, `text.region`, `text.script`, soft-hyphen
  — materializam com o shaping (rustybuzz, XL).
- **Bloqueado por tipo:** `tiling` — precede-o `Value::Tiling` (passo de tipo, ADR-0017).
- **Já-feito (atualizar Inventário):** `cmyk`, `oklab`, `gradient` (parcial), `pad` — corrigir o
  rótulo na Lista A / Inventário 148.

> A ordem é **aberta**: o dono pode repriorizar. Recomendação de arranque: os 3 XS
> (`square`/`lorem`/`panic`) são materializações pequenas e limpas, sem tipo nem shaping — o
> piso óbvio antes de `#show`/`eval`/Model.

---

## 5. Achados que corrigem o Inventário (ganho, não erro — §8 do passo)

- **`cmyk`/`oklab`** estão `implementado` (P257/ADR-0083) — a Lista A marcou `ausente`. **Errado.**
- **`gradient`** está `parcial` (Linear) — a Lista A marcou `ausente`. **Errado** (é parcial).
- **`pad/corners/sides`** não é feature ausente — é refino/duplicado de uma linha já implementada.

Recomendação: corrigir estas 4 entradas no Inventário 148 num passo de manutenção (S), à parte da
materialização. (A Lista B do P386 já tinha sinalizado `cmyk`/`oklab` como "migrado, mecanicamente
divergente" — consistente.)

---

## 6. Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | Cada ausente com `file:line` + 4 campos | ✓ §2 |
| 2 | Adiamentos deliberados separados da fila | ✓ balde B (DEBT-53) + tiling (tipo) |
| 3 | Dependências `Value::*` mapeadas; tipo antes da feature | ✓ §3 |
| 4 | Ordem aberta ranqueada, revisável | ✓ §4 |
| 5 | Zero código/L0/ADR; sonda é o único artefacto | ✓ |

---

## Referências
- `typst-falta-migrar-lista-A-passo-386.md` (os 16) · `-lista-B-` (cmyk/oklab já sinalizados).
- `typst-sonda-bibliography-passo-388.md` — a lição (rótulo ≠ código; conferir adiamento).
- ADR-0108 (medir antes de decidir), ADR-0017 (sem variant sem tipo), ADR-0054 (graded),
  ADR-0077 (`Selector::Regex` em L1), ADR-0083 (cmyk/oklab), ADR-0087 (gradient Linear).
- DEBT-53 (shaping/rustybuzz — cobre o balde B), DEBT-62 (`Value::Bytes`).
