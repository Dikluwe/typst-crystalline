# L0 — Passo 1082: Reclassificação N16[α/β/γ] — Lote 1 (`entities/`)

**Gate**: `ADR-0127` — toca 28 pontos de código de produção (anotação, não
lógica). Mesma cautela de mudanças multi-arquivo já usada nesta conversa.

**Base**: P1070 (amostragem, previu Lote 1 como quase 100% mecânico β) + P1081
(inventário real, 28 casos confirmados, 0% tageado, lista completa já
levantada linha a linha).

---

## 1. Não presumir β só porque a amostra sugeriu isso

O P1070 classificou a amostra de `entities/` como quase 100% β — mas isso foi
amostra, não os 28 completos. **Classificar os 28 individualmente**, mesmo
que a maioria confirme β, para não deixar passar um γ escondido (mesmo cuidado
que o P1080 teve no Lote 3, onde 2 casos da amostra já eram ambíguos).

Casos com maior probabilidade de merecer atenção extra, por não serem projeção
simples óbvia (a confirmar, não presumir):
- `entities/font_variations.rs:98` — "filtro de tags inválidas em dict" (`_ =>
  continue`) — comportamento de filtro, não projeção pura; confirmar se é
  mesmo β.
- `entities/content.rs:3958` — callback de `map_content` (`_ => Ok(None)`) —
  callback genérico, verificar se cobre caso de evolução futura (candidato a
  γ) ou é mesmo uniforme.

## 2. Anotação

Formato já homologado: `// neutro: N16[α/β/γ] — <justificação específica>.`

Usar a lista completa já levantada no P1081 (§3.1, 28 itens com arquivo+linha)
como checklist — não regerar o inventário do zero.

## 3. Critérios de verificação

1. 28/28 casos anotados (não amostra).
2. `crystalline-lint --checks v16` aceita todas as tags.
3. `cargo test --workspace` — 100% pass.
4. `crystalline-lint .` — 0 erros.

## Critério de conclusão

- 28 casos classificados individualmente, com atenção aos 2 casos do §1.
- Nenhum caso rebaixado de γ para β só para reduzir contagem.
