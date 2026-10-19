# L0 — Passo 1092: Corrigir Composição de Escala `MathSize` em `attach.rs`

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer script
aninhado dentro de script é afectado, não só `x^*` dentro de sub-attach).

**Base**: P1091 (causa raiz confirmada com precisão de 6 casas decimais —
`attach.rs` compõe escalas multiplicativamente, `0.70×0.70=0.49`, em vez da
transição discreta por nível absoluto do vanilla, `0.50` directo).

---

## 1. Ler antes de codificar

Não presumir a estrutura de `MathSize`/`TextStyle` nem o padrão de
`numerator_style`/`denominator_style` citado como referência — pedir ou
localizar:

- `01_core/src/compiler/math/layout/attach.rs` (arquivo já parcialmente
  citado nesta investigação, linhas 40-52 — ver o resto da função).
- `00_nucleo/prompts/compiler/math/layout/_comum.md` (citado como já tendo o
  padrão de transição discreta para frações — confirmar que é mesmo o mesmo
  modelo antes de replicar).
- Definição real do enum/estrutura `MathSize` (Display/Text/Script/
  ScriptScript ou nomes equivalentes) — confirmar quantos níveis existem
  antes de assumir só 3.

## 2. Mecanismo — transição discreta por nível, não multiplicação composta

Substituir o cálculo actual (`style.size * script_percent_scale_down`,
aplicado cegamente independente do nível de partida) por transição baseada no
nível absoluto de destino, replicando o padrão já usado em
`numerator_style`/`denominator_style`:

- `Display`/`Text` → `Script`: factor `0.70` sobre o tamanho da base original
  (nível 0), não sobre o tamanho corrente se este já não for nível 0.
- `Script` → `ScriptScript`: o tamanho final deve ser `0.50` do tamanho da
  base **original** (nível 0), não `0.70` do tamanho já reduzido do nível
  Script. Se a implementação continuar a calcular incrementalmente a partir
  do tamanho corrente (não do nível 0 directo), o factor incremental correcto
  é `0.50/0.70 ≈ 0.714286` — mas confirmar qual das duas abordagens
  (recalcular do nível 0 sempre, ou factor incremental) é mais robusta e mais
  consistente com o resto do código antes de escolher, não assumir a
  sugestão inicial sem comparar as duas.
- `ScriptScript` → `ScriptScript` (terceiro nível ou mais fundo, se a
  sintaxe permitir): manter em `0.50` (piso), não continuar a reduzir.
  Confirmar contra o vanilla se existe mesmo um piso ou se a redução
  continua além do segundo nível — não presumir.

## 3. Critérios de verificação — isolado, aninhado, e em linha completa

### 3.1 Casos isolados de 2º nível (tamanho exacto, não só posição)

- `$ x^(y^z) $`, `$ x_(i_k) $`, `$ x_(i^*)^j $` — confirmar que o tamanho de
  2º nível é exactamente `5.50pt` (base 11pt), não `5.39pt`.

### 3.2 Linha completa — o teste que realmente importa (lição do P1089/1090)

Mesma disciplina já estabelecida: um caso isolado não teria apanhado o bug
original de `italics_correction` (P1089), por isso não basta testar §3.1
sozinho.

- `$ nabla g_i(x^*) $` e `$ nabla h_j(x^*) $`, dentro da Equação 3 completa
  da Secção 30 (não só a expressão isolada) — confirmar ausência do degrau de
  `0.0555pt` após `)`.

### 3.3 Convergência de página, limiar estrito

- `compare.py --limiar 0.001` na Secção 30 completa — 0 glifos acima do
  limiar (não o limiar de 0.5pt que mascarou o problema antes).
- Largura de página convergindo para `262.649pt` exactamente.

### 3.4 Não-regressão

- Re-rodar P1057-1091 (toda a cadeia desta investigação de espaçamento
  vertical e horizontal) — zero regressão.
- `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Estrutura real de `MathSize`/`attach.rs` lida antes de escolher entre
  recalcular-do-nível-0 vs factor incremental (§2).
- §3.1, §3.2, §3.3 todos confirmados — não só um dos três.
- Resíduo de página e degrau de `)` eliminados, não reduzidos.
