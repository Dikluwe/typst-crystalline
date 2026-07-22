# Prompt — typst-passo-842: `layout` (define) — 8 achados (#32-#39)

**Origem**: achados #32 a #39 de P831 (lote 5)
**Estado**: aguardando execução

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para cada achado. Contagem de testes de `typst-core` a bater com os testes novos declarados. Tratar cada achado com sonda própria — são pontos distintos dentro do mesmo módulo, sem relação de causa comum aparente (exceto onde indicado).

---

## #32 (L1) — `type(50%)`/`type(30% + 1em)` devolvem `length` em vez de `ratio`/`relative`

Cristalino: `entities/value.rs:341` (`Relative → Type::Length`, com um comentário que **afirma paridade e está errado** — a medição de P831 o refuta). Vanilla: `ratio.rs:62`, `rel.rs:76`. **Sonda**: `type(50%)` e `type(30% + 1em)` nos dois binários. **Implementação**: corrigir o mapeamento de `Type` para diferenciar `Ratio` puro, `Relative` puro (que na verdade é `Ratio` no vanilla quando a parte absoluta é zero) e `Relative` misto — conferir exatamente os três casos no vanilla antes de mexer, e corrigir o comentário errado junto com o código.

## #33 (L2) — `measure` rejeita `width:`/`height:` — **scope-out já documentado, ADR-0054**

Não precisa de sonda nova — já está registrado como decisão consciente. **Ação deste passo**: só confirmar que a entrada ADR-0054 ainda reflete a medição de P831 corretamente; não implementar sem nova decisão do dono.

## #34 (L3) — `measure` devolve métricas diferentes para o mesmo conteúdo

Cristalino `(width: 33pt, height: 14.85pt)` vs vanilla `(22.19pt, 7.24pt)` para o mesmo conteúdo medido. P831 registrou como "provável colateral do engine de texto, a investigar". **Sonda**: reproduzir o caso exato de P831, e testar `measure` em conteúdos mais simples (uma letra, uma palavra) para isolar se a divergência vem de métrica de fonte, de layout de parágrafo, ou de outra coisa. Esta é a sonda mais aberta do lote — não assumir a causa antes de medir.

## #35 (L4) — `#context (10pt)` não renderiza

Cristalino: página vazia; vanilla: `10pt`. Causa: `value_to_content` cai em `Content::Empty` para `Value::Length` (`stdlib/state.rs:137-161`, via `03_infra/src/pipeline.rs:156`). **Nota**: esta é a mesma função que P821 já corrigiu para `Value::Type` (braço `Value::Type(t) => Content::text(...)`) — este achado pede o braço equivalente para `Value::Length` (e provavelmente outros tipos numéricos/geométricos que ainda faltem — conferir quais). **Implementação**: adicionar o(s) braço(s) faltante(s) em `value_to_content`, com o display já usado em `repr`/markup para cada tipo.

## #36 (L5) — `Sub` de `Angle` ausente

`90deg - 45deg` — cristalino erro; vanilla `45deg`. Mesma família do achado #31 (P841, `Sub` de `Length`) e #37 abaixo — **conferir no relatório de P841 se ele já tratou isso junto**; se sim, não duplicar aqui, só confirmar e testar.

## #37 (L6) — `Add` de `Fraction` (`fr`) ausente

`1fr + 2fr` — cristalino erro; vanilla `3fr`. Mesma observação do #36 quanto a possível tratamento conjunto com #31/#36 em `operators.rs`.

## #38 (L7) — `#h(1fr)` rejeitado

Cristalino `error: h() espera amount como length, recebeu fraction`; vanilla compila (`fractional spacing`, distribuição de espaço restante proporcional). Cristalino: `stdlib/layout.rs:879`. Vanilla: `spacing.rs:37,130-135`. **Sonda**: testar `#h(1fr)` e `#h(2fr)` lado a lado com espaço restante conhecido, medir a distribuição no vanilla. **Implementação**: aceitar `Fraction` em `h()`, propagando para o mecanismo de distribuição fracionária do layout (se já existir no cristalino para outro contexto — conferir).

## #39 (L8, menor) — mensagem de `2 * ltr` diverge

Ambos rejeitam (exit 1); cristalino `cannot apply Mul to int and direction` vs vanilla `cannot multiply integer with direction`. Só texto de mensagem — corrigir para bater, sem mudança de comportamento.

---

## Validação (comum aos 8)
1. Recompilar após cada correção. Repetir os casos de sonda respectivos, batendo com o vanilla (ou decisão formal registrada, para #33).
2. Suíte completa, comando + contagem antes/depois, consolidada no fim.

## Relatório

`00_nucleo/diagnosticos/typst-passo-842-relatorio.md`, uma seção por achado (#32-#39), testes nomeados `p842_l1_...` a `p842_l8_...`. Nota explícita de coordenação com P841 (#31) sobre #36/#37, para não haver trabalho duplicado nem lacuna entre os dois passos.
