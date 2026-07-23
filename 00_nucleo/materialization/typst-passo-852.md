# Prompt — typst-passo-852: `layout::corners` — `rect()` não aceita o argumento `radius` (achado #62 de P848)

**Origem**: achado #62 da tabela de P848 (lote 6)
**Estado**: aguardando execução

---

## Achado (medição de P848)

`#rect(radius: 10pt)` e `#rect(radius: (top-left: 15pt, ...))` — cristalino: `error: argumento nomeado inesperado em rect(): 'radius'` (exit 1); vanilla: compila e renderiza cantos arredondados.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Testar `radius:` com um `Length` único (todos os cantos iguais), e com um dicionário de cantos individuais (`top-left`, `top-right`, `bottom-left`, `bottom-right`, e possivelmente atalhos como `left`/`right`/`top`/`bottom` — confirmar a lista completa de chaves aceitas no vanilla) nos dois binários.
2. Localizar no vanilla (`lab/typst-original/`) a assinatura completa de `radius:` em `rect()` — tipo aceito (`Length` puro, `Ratio`, ou `Rel<Length>`?), estrutura do dicionário de cantos, e como o raio é limitado quando maior que metade do lado do retângulo (o vanilla provavelmente faz clamp — confirmar).
3. Localizar no cristalino onde `rect()` é implementado e confirmar a ausência total de `radius:` na whitelist de argumentos.
4. Confirmar se o mecanismo de desenho de retângulo com cantos arredondados já existe no cristalino em algum outro lugar (ex.: reaproveitável de `box()`, que às vezes também tem `radius` no vanilla — testar se `box(radius:)` tem o mesmo problema, para saber se a correção deve ser feita num ponto comum).

## Passo 2 — Implementação

Adicionar `radius:` a `rect()` (e a `box()`, se a sonda do Passo 1.4 confirmar que compartilham a mesma lacuna), com o tipo e a estrutura de dicionário medidos, propagando para o desenho do retângulo (curvas/arcos nos quatro cantos, usando o mecanismo de path/curve já existente no projeto — o handoff menciona `ShapeKind::Path` e curvas em achados anteriores de imagem).

## Passo 3 — Validação

1. Recompilar. `radius:` único e por-canto, batendo com o vanilla — confirmar tanto a aceitação quanto a geometria real (medir a curva do canto, não só ausência de erro).
2. Testar o caso de clamp (raio maior que metade do lado) se o vanilla fizer isso.
3. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-852-relatorio.md` com medição antes, código identificado, diff, medição depois (incluindo verificação geométrica dos cantos, não só ausência de erro), contagem de testes.
