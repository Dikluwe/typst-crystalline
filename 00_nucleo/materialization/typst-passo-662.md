---
# P662 — Reconsiderar `variant: (eixo: valor)`, sintaxe não confirmada contra o vanilla

> **Passo:** 662
> **Data:** 2026-07-09
> **Foco:** P660 implementou `variant: (eixo: valor)` a pedido de um passo anterior que assumiu, sem confirmar primeiro, que esta era sintaxe real do vanilla. A sonda do próprio P660 confirmou o contrário: `"error: unexpected key 'variant', in dict"` no binário vanilla de referência. Isto é uma extensão de linguagem — um documento que use esta sintaxe compila no cristalino mas falha no Typst real — não uma diferença de implementação aceitável. Este passo decide o que fazer com ela.
> **Tipo:** Decisão + Implementação, conforme a decisão.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P660 (onde a extensão foi criada por engano), a distinção agora explícita entre diferença de implementação (aceitável) e diferença de linguagem (inaceitável).

---

## Contexto

O projecto aceita que o cristalino implemente internamente de forma diferente do vanilla (caches diferentes, algoritmos diferentes), desde que um documento `.typ` válido produza o mesmo resultado nos dois lados. Não aceita que a linguagem em si — a sintaxe e semântica que um documento pode usar — divirja, porque isso quebra a portabilidade: um documento escrito para o cristalino deixaria de ser um documento Typst válido.

`variant: (eixo: valor)` viola isto directamente: é sintaxe que o vanilla rejeita.

---

## Opções

1. **Reverter por completo.** Remover a sintaxe, voltar ao estado anterior a P660 (só nomes de variante e `weight`/`style`/`stretch`). Mais simples, sem risco de confundir utilizadores com uma funcionalidade que não é portável.

2. **Manter, mas atrás de uma bandeira explícita de "capacidade não-padrão".** Seguindo o precedente de P617 (`--document-id`, já registado como capacidade nova, não paridade) — mas isto é sintaxe de documento, não uma bandeira de CLI; um utilizador que escreva `variant: (...)` no seu `.typ` não vai ver nenhuma bandeira, o documento simplesmente vai deixar de compilar se for para o Typst real. O precedente de P617 não se aplica bem aqui, porque uma bandeira de CLI não entra no documento; sintaxe de linguagem entra sempre.

3. **Manter como está, documentado como extensão.** O mesmo tratamento dado a `table.numbering` — mas isso normaliza exactamente o tipo de coisa que o utilizador acabou de identificar como inaceitável.

### Recomendação

Reverter (opção 1). A diferença entre isto e `table.numbering` é que `table.numbering` pelo menos oferece uma conveniência genuína dentro da filosofia do Typst (numerar sem `figure`), ainda que desviando da arquitectura. `variant: (eixo: valor)` não foi um desvio deliberado de design — foi um erro de partida, uma sintaxe que nunca existiu em lado nenhum, introduzida só porque foi pedida sem confirmação.

---

## Implementação, se a decisão for reverter

- Reverter as alterações de P660 em `01_core/src/entities/font_list.rs`, `01_core/src/entities/layout_types.rs`, `01_core/src/entities/style_chain.rs`, `01_core/src/rules/eval/rules.rs`, `01_core/src/rules/layout/text.rs`, `03_infra/src/font_variant.rs`, `03_infra/src/shaper.rs`, `03_infra/src/font_metrics.rs`, `03_infra/src/export/builder.rs`, `03_infra/src/pipeline.rs`, `03_infra/src/export/mod.rs`, `03_infra/src/export/stream.rs`.
- **Manter** a correcção de merge de `font` em `layout/text.rs` (`ns_font.or(...)` a vencer o default da chain) — isso é uma correcção de bug independente da sintaxe de `variant`, não deve ser revertida junto.
- **Manter** a correcção de P659 (chave da cache incluir eixos de variação) — continua correcta e necessária mesmo sem a sintaxe explícita, porque `weight`/`style`/`stretch` já produzem eixos internamente, e a colisão de cache era real para esses casos também, não só para `variant` explícito.
- Reverter os quatro testes específicos de P660 relacionados com a sintaxe `variant: (eixo: valor)`, mantendo os testes de P659 (agora de volta a teste unitário, já que a sintaxe real deixa de existir).

### Critério de fecho

- [ ] Sintaxe `variant: (eixo: valor)` removida.
- [ ] Correcção de merge de `font` (bug real) mantida.
- [ ] Correcção de P659 (chave de cache) mantida.
- [ ] Testes ajustados de acordo.
- [ ] `cargo test --workspace` sem regressão.
- [ ] `crystalline-lint .` limpo.

---

## Critério de fecho do passo

- [ ] Decisão tomada e justificada por escrito (reverter, recomendado, ou razão para manter se a decisão for diferente).
- [ ] Implementação conforme a decisão.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p662.md`, com hash do commit.
- [ ] Nota adicionada ao histórico de P660, deixando claro que a sintaxe foi criada por erro de verificação, não por decisão de design.
