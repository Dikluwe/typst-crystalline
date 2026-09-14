# Passo 1374 — largura relativa e altura fracionária de `block`

## Objetivo

Fechar os dois scope-outs nomeados pelo P1373 sem sentinelas: preservar
`block(width: <relative>)` e transportar `block(height: <fraction>)` por um
carrier público explícito, com distribuição proporcional no flow vertical.

## Medição anterior à decisão

Fonte ratificada `586e1bd43`: `layout/container.rs:257-284` declara width como
`Smart<Rel<Length>>` e height como `Sizing`; `container.rs:476-505` distingue
`Auto`, `Rel` e `Fr`. `flow/collect.rs:244-264` coleta o peso fracionário e
`flow/distribute.rs:589-681` desconta conteúdo fixo, soma os pesos e reparte o
espaço restante antes de posicionar os itens.

Medição black-box limitada, página 200pt × 240pt e margem 10pt:

- width 60% + `align(right)[Right]`: vanila `xMin=93,668`; cristalino cortou o
  texto e expôs apenas `ght`, `xMin=-4,894`;
- heights 1fr e 2fr seguidas de `After`: vanila colocou `One` em
  `yMin=7,404`, `Two` em `82,724670` e `After` em `220,166`; cristalino
  rejeitou `height: 1fr` como `fraction`.

Identidades: vanila `eb60986b…1468f`, cristalino P1373
`0b318c3f…4e15`, `pdftotext` `0fb98ea1…565e7`. As sondas têm SHA-256
`2032e3c1…261` e `adb161d2…603`.

## Contrato proposto

1. Criar em `BlockElem` o enum público fechado `BlockHeight` com variantes
   `Relative(Rel<Length>)` e `Fractional(f64)`; `Option<BlockHeight>` conserva
   `None = auto` sem copiar a mecânica Rust do vanila.
2. `BlockElem::width` passa a `Option<Rel<Length>>`; Fraction continua inválida
   para width.
3. `Content::block` mantém a assinatura pública com `Option<Length>` para width
   e height, elevando valores absolutos para relative e `BlockHeight::Relative`.
4. `native_block` aceita Length/Float/Int/Ratio/Relative em width; height aceita
   esses valores e Fraction. Fração negativa, não finita ou width fracionária
   são rejeitadas.
5. O Block resolve width relativa uma vez contra a largura útil externa. Height
   relativa mantém P1373. Height fracionária nunca é convertida em pt no eval
   nem resolvida isoladamente pelo Block.
6. O flow de uma Sequence em região finita mede a cauda fixa sem contar o body
   das ocorrências fracionárias, conserva seus gaps normais, soma pesos e
   entrega a cada Block sua parcela do espaço restante. Peso zero recebe zero;
   soma zero não divide. A ordem não altera as parcelas.
7. A primeira fatia cobre ocorrências diretas de Block numa Sequence finita e
   uma única região. P1375 completa wrappers transparentes, colunas e
   distribuição multi-região; esses casos ficam `Unknown`, nunca `MATCH`.

## Testes RED após confirmação

- carrier distingue auto, relativo e fracionário sem NaN/sinal/sentinela;
- width 60% resolve para 108pt dos 180pt úteis e preserva `Right`;
- height 1fr/2fr recebe parcelas na razão 1:2 após desconto fixo;
- zero fr não divide nem gira;
- `Content::block` absoluto mantém compatibilidade;
- matriz bilateral das duas sondas fecha somente os observáveis congelados.

## Memória

Testes e builds sempre seriais. Primeiro `cargo check` com 3 GiB; um único
link focal com 4 GiB e teto de arquivo de 512 MiB; release apenas depois do
GREEN. Matrizes usam subprocessos sequenciais com 2 GiB, arquivo 16 MiB,
canal 1 MiB e timeout/CPU 45 s. Nenhuma suíte ampla durante calibração.

## Gate ADR-0127

**CONFIRMADO em 2026-09-13.** O humano reiterou explicitamente a ordem de
implementar após receber o contrato público. RED e código estão autorizados
somente dentro da fatia congelada.
