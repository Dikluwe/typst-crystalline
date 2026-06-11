# Relatório P319 — Lote 4 (decorações de texto) + 3 caronas de registro

**Pré-condição**: Lote 3 (P318) fechado — lint 0, suíte verde. ✅ Verificado.
**Dois commits isoláveis**: `Passo 319 — caronas de registro` (1º, tree limpo)
e `Passo 319 — lote 4`.

---

## Caronas de registro (commit próprio)

- **C1 — Caveat de resolução no M3** (`medicao-pre-f-passo-318.md`): as 5
  execuções deram exatamente 0.07 s — granularidade 0.01 s do `/usr/bin/time`
  ⇒ quantização **~15%**; baseline só deteta regressões grosseiras. O "depois"
  do F deve refazer antes+depois no par de commits com `hyperfine`/corpus 10×.
- **C2 — `Space` na triagem do DEBT-58**: largura 13, **cola de texto**
  (parente de `Empty`, não de `Divider`). Observação correlata registada:
  `Text` (40) é folha — candidato natural à mesma triagem.
- **C3 — "Contabilidade de variantes" no modelo**: roteiro dos lotes
  (22 migradas até Lote 4 · 4 `Set*`→F · 11 DEBT-58 · ~40 element-shaped
  restantes por largura · estimativa **5–8 lotes**). **Atualizar essa seção
  passou a ser item obrigatório do relatório de todo lote** (regra adicionada
  ao modelo) — o roteiro mora no repo, não em conversa.

---

## Lote 4 — decorações de texto, instância do modelo

**Composição (confirmada no checkpoint): 3 variantes** element-shaped, ordem por
largura crescente: `Overline`(10) · `Strike`(10) · `Underline`(31) = **51 sites**.

**Locatabilidade**: confirmado **nenhuma é locatável** (lista exaustiva
não-locatável de `introspect/locatable.rs`) → `element_kind`/`to_payload` default.

**Família** (contentores de prosa com corpo + cosméticos `stroke`/`offset`/
`extent`): `map_text`/`map_content` **recursam** no body (precedente Lote 3/
Heading); **`is_empty` delega ao body** (override, content-preserving).

**Novidade — `Hash` manual** (aprovada no checkpoint, 3 condições cumpridas):
`Length` carrega `f64` e **não** implementa `Hash`; o trait `Element` exige-o,
logo os 3 `…Elem` derivam só `Debug, Clone, PartialEq` e implementam `Hash`
**à mão via `Debug`** (`format!("{self:?}").hash(state)`, paridade
`content_hash::hash_content`). **(1)** Causa + ressalva (`-0.0` vs `0.0`; `…Elem`
não são chaves de mapa; revisitar se vierem a sê-lo) documentadas no L0 **e** em
comentário no código de cada `impl`. **(2)** Nota no modelo (Fase B, junto ao
derive) para os lotes 5+. **(3)** Testes relacionais de hash
(`campo_diferente_produz_hash_diferente`) presentes nos 3 módulos.

### Medições (métrica ADR-0104) — vs preditor

| medição | valor | comando |
|---|---|---|
| `content.rs` antes/depois | **5700 → 5639 (−61)** | `wc -l`; diff `+61 / −122` |
| parte atómica (3 módulos novos, c/ testes) | **373 linhas** | `wc -l elements/{overline,strike,underline}.rs` |
| suíte typst-core | **2521 → 2533 (+12)** | só os 12 testes unitários novos (4×3) |
| `crystalline-lint .` | **0 violations** | gate binário |

**Trajetória `content.rs`**: 5782 (baseline P313) → 5735 (Lote 2) → 5700
(Lote 3) → **5639** (Lote 4). O hub encolhe lote a lote; este lote cortou 61
(arms de decoração eram verbosos: 4 campos × 6 matches). Custo-por-módulo
homogéneo (~122–128 linhas, inclui o `impl Hash` manual + teste de hash).
**Preditor validado**: custo ∝ largura (51 sites: bulk de Underline em
`layout/tests.rs`).

**Plano de toque (realidade)**: `introspect.rs` (materialize_time + walk —
o `|` combinado teve de **separar** em 3 arms: tipos `Arc` distintos),
`layout/mod.rs` (o arm `|`-combinado com factor-por-kind foi refeito num único
`match` que extrai campos + `kind_em`), `introspect/locatable.rs` (match
exaustivo), `stdlib/text.rs` (construção via construtor, body sem `Box`),
`stdlib/mod.rs`/`layout/tests.rs`/`export/tests.rs` (construções e matches de
teste). **Nenhuma asserção alterada.**

### Proposta do Lote 5 (derivada do mapa C3 — decisão humana)

Do mapa de variantes (modelo §Contabilidade), por largura crescente, duas
famílias coerentes:

- **Quebras/espaços**: `Linebreak`(11)† · `Colbreak`(12)† · `VSpace`(14) ·
  `HSpace`(18) · `Pagebreak`(22)† — os três `†` são **comandos unit**
  (precedente `Divider`, elegíveis); `VSpace`/`HSpace` têm campo `amount`.
  (`Space` fica de fora — está na triagem do DEBT-58, C2.)
- **Grid family**: `GridFooter`(7) · `GridHeader`(7) · `TableFooter`(10) ·
  `TableHeader`(11) · … (contentores com `body`/`repeat`).

---

## Encerramento

- `git log --oneline` (dois commits isoláveis):
  - `Passo 319 — lote 4`
  - `Passo 319 — caronas de registro`
- `crystalline-lint .` → **0 violations**.
- `git status` limpo (só resta cruft pré-existente de `lab/` quarentena).
- Caveat conhecida: stack default estoura em `recursao_infinita_*` — **não é
  regressão** (`RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Lotes 5+ (instância do modelo, a partir do mapa C3); DEBT-58 (gatilho: fim dos
element-shaped); F / `Set*` / 99.E (diagnóstico consome a medição P318 com o
caveat C1); otimizações sugeridas por medição (medir ≠ mexer).
