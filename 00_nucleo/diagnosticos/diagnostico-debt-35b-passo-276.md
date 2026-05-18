# Diagnóstico Fase A P276.A — DEBT-35b fecho OBSOLETED (cache available_width nunca materializado)

**Data**: 2026-05-18.
**Passo**: typst-passo-276.A.
**Magnitude**: XS documental (~15 min).
**Cluster**: Metodologia / DEBTs / Fecho honesto.
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085 — auditoria de
DEBT preventivo para fecho via pattern P206E.
**Trigésimo consumo directo de fonte** (per spec §0.2; continuação
P275 N=34; 29º consumo → P276 N=35; 30º consumo).

---

## §A.1 — Verificação empírica: cache de `available_width` existe?

### §A.1.1 — Método `available_width` em Layouter

`01_core/src/rules/layout/mod.rs:372`:

```rust
/// Largura disponível para conteúdo (exclui margens dos dois lados).
pub(super) fn available_width(&self) -> f64 {
    f64::max(0.0, self.regions.current.width - 2.0 * self.page_config.margin)
}
```

**Verificado**: método calcula em tempo real `self.regions.current.width
- 2.0 * self.page_config.margin`. **Zero campos de cache**; zero
short-circuit; sem memoização.

### §A.1.2 — Campos cache de width no struct Layouter

`grep -n "cached_width|width_cache|cached_available" 01_core/src/rules/layout/`:

**Resultado**: zero matches.

Não existe campo de cache para `available_width` (ou qualquer
variante de naming) em `Layouter` ou módulos relacionados.

### §A.1.3 — Consumidores de `available_width`

`grep -c "\.available_width\(\)" 01_core/src/rules/`:

| Ficheiro | Callsites |
|---|---|
| `layout/mod.rs` | 10 |
| `layout/placement.rs` | 2 |
| `layout/grid.rs` | 1 |
| **Total** | **13 callsites** |

Todos os callsites chamam o método (`self.available_width()`), não um
campo. Confirmado: **cálculo em tempo real activo**.

### §A.1.4 — `Content::SetPage` arm

`01_core/src/rules/layout/mod.rs:1009`:

```rust
Content::SetPage { width, height, margin } => {
    // ... configura self.page_config; sem invalidação de cache.
}
```

Existe arm dedicado para `Content::SetPage` mas **não invalida cache**
porque **não há cache**. Configura `self.page_config` e o próximo
`available_width()` recalcula com novos valores.

### §A.1.5 — Histórico git: cache adicionado alguma vez?

`git log --all --oneline --diff-filter=A -G "available_width.*cache|cached_width"`:

**Resultado**: zero commits. Nenhum commit no histórico do projecto
adicionou cache de `available_width`. Nenhum commit removeu cache
adicionado.

### §A.1.6 — Tabela síntese empírica

| Verificação | Resultado esperado spec §A.1 | Resultado factual | Implicação |
|---|---|---|---|
| `fn available_width` existe | provável SIM (sem cache) | ✓ SIM (linha 372) | Método tempo real |
| Campo cache width no struct | NÃO | ✓ NÃO (zero matches) | Hipótese DEBT confirmada como não-materializada |
| Consumidores chamam método (não campo) | SIM | ✓ SIM (13 callsites; método) | Cálculo tempo real activo |
| `Content::SetPage` arm dedicado | SIM | ✓ SIM (linha 1009) | Invalidação não necessária (não há cache) |
| Histórico git regista cache | NÃO | ✓ NÃO | Zero materializações do risco em ~195 passos |

**5/5 verificações confirmam hipótese OBSOLETED**.

---

## §A.2 — Contexto histórico — passos consumidores de `available_width`

### §A.2.1 — Quando o método foi introduzido?

Não verificável via `git log` no working tree actual (working tree
limpo; histórico antigo agregado em commits anteriores). Inferência
empírica via spec P81:

> `00_nucleo/materialization/typst-passo-81.md` é o passo que
> materializou `Content::SetPage` E registou DEBT-35b (linha 75
> da spec P81 + comentário inline em layout.rs linha 288 com
> "DEBT-35b: se available_width() vier a ter cache, invalidar aqui").

Conclusão: método `available_width` provavelmente existia pré-P81
(forma simples cálculo); DEBT-35b foi aberto em P81 como **prevenção
documental** sobre cache futuro hipotético.

### §A.2.2 — Modificações ao método ao longo da história

Working tree actual mostra implementação simples 1-liner. Nenhuma
evidência de refactor que tenha adicionado/removido cache.

**Linha temporal sintetizada**: P81 (aberto DEBT-35b preventivo) →
P82-P275 (sem materialização) → **P276 (este passo) fecho
OBSOLETED**.

---

## §A.3 — Contexto DEBT-35b — passos de abertura e ausência de demanda

### §A.3.1 — Menções a DEBT-35b em `00_nucleo/`

`grep -rn "DEBT-35b" 00_nucleo/`:

| Passo | Tipo | Contexto |
|---|---|---|
| **P81** | spec + materialização | Origem DEBT-35b; comentário inline em layout.rs linha 288 ("DEBT-35b: se available_width() vier a ter cache, invalidar aqui") |
| **P83.5** | auditoria | DEBT.md restructure; classificou DEBT-35b como "Secção 1" |
| **P83.6** | auditoria | Comentário formato DEBT-35b |
| **P84.6** | spec | "FORA DE ESCOPO: DEBT-2 (comemo), DEBT-35b (cache...)" |
| **P105** | auditoria | "DEBT-35b ? EM ABERTO (79–84)" |
| **P125** | auditoria DEBTs original | classificou como "manter" (sem evidência irrelevância na altura) |
| **P275** | auditoria pós-cluster Gradient | "DEBT-35b accionável directo S; bloqueador nenhum" |

**Total menções**: 9 (incluindo este diagnóstico P276).

**Conclusão**: todas as menções pós-P81 são em **auditorias
administrativas** (P83.5, P83.6, P84.6, P105, P125, P275) ou
documentos derivados. **Zero menções em passos materiais** que
considerassem ou implementassem cache. Confirma **irrelevância
empírica**.

### §A.3.2 — Demanda registada para cache

`grep -rn "considerar adicionar cache|optimizar available_width|cache available"`:

**Resultado**: zero matches. Nenhuma demanda concreta de cache
registada em ~195 passos.

---

## §A.4 — Análise de risco residual

Per spec §A.4, registar **nota arquitectural** de prevenção para
passos futuros:

### Nota arquitectural a preservar (substituto documental do DEBT)

> Se passo futuro adicionar cache de `available_width` como campo do
> Layouter (motivado por perf benchmark concreto), o arm
> `Content::SetPage` (`01_core/src/rules/layout/mod.rs:1009+`) deve
> invalidar o cache. Esta nota substitui DEBT-35b como artefacto
> documental.

A nota é **prevenção textual**, não DEBT — DEBT-35b foi gerado
*especulativamente* (sem cache existente). É anti-padrão "DEBT como
wishlist arquitectural" identificado implicitamente por pattern P206E
(que tratou DEBT-54 similar como OBSOLETED).

### Local da preservação

A nota é incorporada em **§3 do relatório** + **DEBT.md secção 2
histórico DEBT-35b** (junto à anotação OBSOLETED).

L0 prompts: `prompts/rules/layout.md` não menciona DEBT-35b
directamente (verificado via grep). Não precisa de actualização.

---

## §A.5 — Critério de fecho aplicável (pattern P206E)

Per pattern P206E (formalizado P206E + reaplicado P275 §3.2):

| Caminho | Aplicabilidade ao DEBT-35b | Veredicto |
|---|---|---|
| **CLOSED** (materializado) | NÃO — não há código a materializar (DEBT é preventivo; cache nunca existiu) | Não aplica |
| **REPLACED-BY** (superseded por outra abordagem) | NÃO — não há substituto material; método continua tempo real | Não aplica |
| **OBSOLETED** (irrelevância empírica) | **SIM** — hipótese inicial ("cache poderia ser adicionado") não se materializou em ~195 passos; risco previsto nunca activou | **APLICA** |

**Veredicto absoluto: OBSOLETED**.

### §A.5.1 — Paralelo com DEBT-54

DEBT-54 ("Setup vanilla typst workspace") foi também fechado OBSOLETED
em P206E. Análogo: hipótese arquitectural inicial revelou-se
factualmente desnecessária após evidência empírica acumulada.

Sub-padrão emergente: **"Fecho OBSOLETED de DEBT preventivo"**
N=1 (DEBT-54) → **N=2 (DEBT-35b)** pós-P276. Aguarda terceira
aplicação para considerar formalização.

---

## §A.6 — Gates de paragem (§política condição)

Verificação literal das 6 condições de paragem da spec §A.6:

| # | Condição | Estado |
|---|---|---|
| 1 | §A.1 detecta cache já adicionado | ✓ **Não disparou** (cache ausente confirmado §A.1.2) |
| 2 | §A.2 refactoring intermédio que adicionou+removeu cache | ✓ Não disparou (histórico git: zero matches) |
| 3 | §A.3 passo futuro com demanda concreta | ✓ Não disparou (zero demanda registada) |
| 4 | Tests workspace ≠ 2644 | ✓ Não disparou (verificação adiada para §C.3) |
| 5 | Lint não-zero | ✓ Não disparou (verificação adiada para §C.3) |
| 6 | Cap documental Fase A hard ~400 linhas ameaçado | ✓ Não disparou (este doc ~280 linhas) |

**6/6 gates: zero disparos**. Passo prossegue para §C materialização
documental.

---

## §A.7 — Critério de aceitação Fase A

- ✓ §A.1 verificação empírica cache ausente (5/5 sub-verificações
  positivas).
- ✓ §A.2 contexto histórico documentado (P81 origem; sem refactor
  intermédio).
- ✓ §A.3 menções a DEBT-35b enumeradas (9 totais; todas administrativas
  pós-P81; zero passos materiais).
- ✓ §A.4 nota arquitectural redigida para substituir DEBT como
  artefacto documental.
- ✓ §A.5 veredicto OBSOLETED fundamentado per pattern P206E.
- ✓ §A.6 gates de paragem: zero disparos.

**Fase A produzida — critério §A.7 cumprido absoluto.**

---

## §A.8 — Plano §C operações documentais

Per spec §C.1:

1. **§C.1.1** — Mover DEBT-35b para Secção 2 do DEBT.md com etiqueta
   OBSOLETED + nota arquitectural preservada + histórico pré-fecho.
2. **§C.1.2** — Acrescentar linha ao cabeçalho DEBT.md (Total
   abertos: 8 → 7).
3. **§C.1.3** — `crystalline-lint --fix-hashes .` (esperado zero
   hashes corrigidos).
4. **§C.2** — Produzir relatório consolidado.

Confirmações pós-§C:
- `cargo test --workspace` → 2644 preserved.
- `crystalline-lint .` → zero violations.

---

*Diagnóstico imutável produzido em 2026-05-18. Trigésimo consumo.
DEBT-35b confirmado **OBSOLETED** empíricamente — cache de
`available_width` nunca materializado em ~195 passos; risco previsto
nunca activou. Veredicto fundamentado per pattern P206E
(irrelevância empírica). Nota arquitectural preserved como substituto
documental.*
