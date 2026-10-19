# L0 — Passo 1063: Espaçamento `above`/`below` de Heading + Colapso (Caso 1 do P1059)

**Gate**: `ADR-0127` — mudança de comportamento por defeito (todo documento com
heading é afectado). **Requer confirmação do dono antes de codificar.**

**Base**: `compiler/layout/heading.md` (residual já documentado, não achado novo) +
`compiler/layout/heading.rs` (código real, confirmado por leitura directa) +
mecanismo de colapso do P1061 (`prev_block_below_pending`/`block_chain_active`).

---

## 1. O que está confirmado por leitura directa

`heading.rs::layout` (função actual, citada linha a linha) não toca em nenhum
espaçamento vertical `above`/`below` — só chama `flush_line()` antes (se a linha
estiver a meio) e depois do body. Não há avanço de `cursor_y` por margem em lado
nenhum desta função.

`heading.md` já regista isto como resíduo explícito (não descoberta deste passo):

> `1.8em` (nível 1) / `1.44em` (nível 2+) acima; `0.75em` abaixo — vanilla,
> `heading.rs:288-289` — **não medido nem portado**.

Isto é consistente com o Caso 1 do P1059 (`Parágrafo → Heading`, Δy = −9.2950 pt):
não é só ausência de colapso — é ausência do espaçamento inteiro.

## 2. Mecanismo autorizado

### 2.1 Medir antes de codificar (disciplina do projecto — nunca aceitar valor sem número real)

Os valores `1.8em`/`1.44em`/`0.75em` estão citados de `heading.md`, mas o próprio
`heading.md` diz que não foram medidos neste projecto — só citados da localização no
vanilla. Confirmar por `pdftotext -bbox-layout` antes de portar, mesmo padrão já
usado em P1059/P1061:

- `= Heading\n\nParágrafo` (nível 1, below) e `Parágrafo\n\n= Heading` (nível 1,
  above) — comparar contra os números já existentes no Caso 1 do P1059
  (Δy = −9.2950 pt) para confirmar que `1.8em` fecha exactamente essa divergência,
  não uma aproximação.
- Repetir para nível 2 (`1.44em`) — sem caso equivalente já medido nesta conversa;
  medir de raiz.

### 2.2 Adicionar `above`/`below` a `heading.rs`, ligados ao colapso do P1061

Antes de `flush_line()` inicial (entrada do heading) — aplicar o mesmo padrão de
`block.rs` (P1061): `above_pt` resolvido em pt (a partir do tamanho de fonte base,
**não** do tamanho já escalado do heading — ver §3, a divisão por escala no residual
sugere que o `em` é relativo ao corpo, não ao heading), colapsado via
`block_chain_active`/`prev_block_below_pending` do mesmo `Layouter`.

Depois do body, antes de restaurar o estilo — aplicar `below_pt` da mesma forma que
`block.rs` regista `prev_block_below_pending` no fim.

**Não duplicar a lógica de colapso** — reutilizar exactamente o mesmo par de campos
e a mesma fórmula `max()`/`advance` já em produção desde o P1061, não reimplementar
uma variante paralela.

### 2.3 `Content::Sequence` (`sequence.rs`) — incluir `Content::Heading` na preservação de cadeia

O P1061 mudou o filtro de reset para incluir `Content::Parbreak` junto de
`Content::Block`/`Content::Shape`. `Content::Heading` continua fora dessa lista —
precisa de entrar, senão o `below` do heading nunca sobrevive até ao próximo
elemento:

```rust
if !matches!(
    part,
    Content::Block { .. } | Content::Shape(_) | Content::Parbreak | Content::Heading { .. }
) {
    layouter.block_chain_active = false;
    layouter.prev_block_below_pending = 0.0;
}
```

(Sintaxe exacta do padrão de `Content::Heading` a confirmar contra o enum real —
não vi a definição do variant nesta conversa, só o uso em `heading.rs`.)

## 3. Pergunta em aberto (não decidir sozinho)

O residual diz "`1.8em` ... ÷ escala acima" — a divisão por escala sugere que o valor
em pt final é `1.8 * tamanho_base_do_corpo`, não `1.8 * tamanho_do_heading_já_escalado`
(isso bateria com o número do P1059: `1.8em = 19.80pt` para nível 1, que é
`1.8 × 11pt`, não `1.8 × 15.4pt`). Confirmar isto contra o vanilla directamente
antes de codificar — não assumir a partir da frase do resíduo, que é uma nota, não
uma fórmula verificada.

## 4. Critérios de verificação

1. Caso 1 do P1059 (`Parágrafo → Heading`, nível 1) — Δy alvo: 0.0000 pt.
2. Novo caso — `Heading → Parágrafo` (não estava nos 4 casos originais do P1059,
   mas é o mesmo tipo de transição, na direcção oposta) — medir e confirmar
   colapso correcto.
3. Nível 2+ — pelo menos um caso adicional, já que `1.44em` nunca foi medido nesta
   conversa.
4. Re-rodar os 4 casos do P1059 + os 4 isolados bloco↔bloco do P1061 — zero
   regressão.
5. Decalque do corpus canónico completo (7 documentos) — comparar **pré-P1063 vs
   pós-P1063** (mesmo binário HEAD anterior, mesma disciplina usada no P1061 para
   `04-math.typ`/`05-tables.typ`/`07-context.typ`), não vanilla-vs-cristalino
   directamente — já demonstrado nesta conversa que é o método que isola
   correctamente regressão de divergência pré-existente.
6. `crystalline-lint .` — 0 erros.
7. `cargo test --workspace` — 100% pass.

## 5. Scope-out explícito

- `outline` continua a não usar o pattern de numeração (achado do P1036, já
  registado como gate `ADR-0127` ponto 1, passo próprio) — não tocado aqui.
- O separador `0.3em` fraco entre número e título (residual do P1036, geometria) —
  não tocado aqui.
