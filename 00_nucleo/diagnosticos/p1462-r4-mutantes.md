# P1462 R4 — revisão adversarial final

**Regime:** protocolo completo, quarta e última revisão local.
**Papel:** adversário pré-candidato; não autoriza implementação, teste RED ou selo.
**Execução:** checkout compartilhado; **executado sem atestação de isolamento**.
**Baseline:** commit `61796fa6e183c08207e3828cec6d2e43da1af846`, tree
`4890fb690d222c093e858130da00d2062ae514dd`.
**Registro:** `2026-09-19T15:01:17-03:00`.
**Bundle:** `/tmp/p1462-r4-mutations`.

Foram lidos somente os dois vereditos públicos R3 autorizados. Nenhum oráculo
privado foi lido ou executado e nenhuma campanha, kill ou corpus completo foi
executado por este papel. Manifesto, contrato, L0, código/testes produtivos e
todos os arquivos e bundles R2/R3 permaneceram somente leitura.

## 1. Duas causas públicas e hipótese R4

O R3 terminou `NOT_SEALED` por duas causas independentes:

1. P00 e variantes produziam `expected boolean, found int` em
   `#set format.svg(pretty: 1)`, enquanto o contrato e o vanilla congelados
   exigem `expected boolean, found integer`;
2. `direct`, `direct_repeat` e `reverse` foram iniciadas concorrentemente. Sob
   essa carga, M03/direct excedeu o timeout externo de `180 s` e ficou
   `Unknown`; prefixos parciais não recebem crédito.

A hipótese final é corrigir o vocabulário no ponto tipado comum
`eval/rules.rs::type_mismatch` e retirar a concorrência entre ordens. O helper
agora normaliza `int → integer`, `str → string` e `bool → boolean` antes de
formatar o diagnóstico. O runner R4 não oferece uma ordem isolada: quando há
verificador externo, executa obrigatoriamente `direct`, depois
`direct_repeat`, depois `reverse`, em série, com timeout por variante de no
mínimo `360 s`.

## 2. Delta exato R3 → R4

P00 e M01–M18 foram copiados do bundle R3 com
`cp -a --reflink=always`; não existem hardlinks em fontes editáveis nem em
binários congelados. Em cada uma das 19 árvores, o único arquivo alterado
contra R3 é `01_core/src/compiler/eval/rules.rs`.

| Árvores | SHA R3 | SHA R4 |
|---|---|---|
| P00, M01–M03, M07–M18 | `76bfd52504ec242e1343e5d8a4caec87a4a93358bce8ad598b3db6a3b77c4dee` | `f46a26669ab57b393d22062574dbecdbe5aeb9988e555caf6f656bf0a2be075f` |
| M04 | `db57480db990a871714f665959817712b5e1c17c8c90f466c47a38d918f6f9ec` | `a16870b26b2d15ee0ea1ecb3b14637fc83be9f17324bc6cd9fb329e174965a4c` |
| M05 | `d8f22409b0b88728fae58ca668705efd9a8762aed2c6675e236c4ff94afed6e2` | `70fa197c10652801498288ff214a4ac0d945f83b7f3e3bcf41c71fcb1062d14f` |
| M06 | `860c288996f6c6edc19809e1ab68c9e03700a70c719f42c660c29c132e243874` | `6a297d32c5929584fd8c18be9ca6e70273fc7b7674d2fc6ea257b5b69c6cebc8` |

Esse rebase comum preserva o único delta de cada mutante contra P00. Em
particular, M12 continua exatamente a instância R3: somente
`03_infra/src/pipeline.rs` difere de P00, de
`636ff3c02aae5ea7a3027aab53715a995aac3b327c4ade2362bd64a5872b88af`
para
`16ed2805bcc17fbabfae6f8f6993eb78a1ebaff3ec4e5110380d45492cdd8c63`;
`pretty=true` força `PdfTags::Disabled` e `false` preserva o caller.

## 3. Catálogo 18/18 preservado

As receitas R2, com a substituição M12 já congelada em R3, não mudaram
semanticamente. A fixtures R4 registra source tree, binário, tamanho, modo e
delta exato de cada instância.

| ID | Classe/receita preservada | Obrigação pública discriminatória |
|---|---|---|
| M01 | registrar a factory SVG sob `svx` | O1 exige `format.svg` nominal |
| M02 | default público `pretty=true` | O1/O6 exigem default `false` |
| M03 | construção manual retorna sucesso | O2 exige `cannot be constructed manually` |
| M04 | named desconhecido é descartado | O3 exige `unexpected argument: other` |
| M05 | transportar sob chave `format.svg.prettx` | O3/O6 exigem a chave canônica |
| M06 | regra vazia insere `format.svg.pretty=false` | O3/O5: `true → empty` continua pretty |
| M07 | primeira decisão explícita vence | O5 exige a última decisão explícita |
| M08 | decisão explícita nested é aceita | O4 exige decisão somente no root |
| M09 | valor interno não booleano é ignorado/defaultado | O10 exige fail-closed |
| M10 | L4 colapsa `None` cedo para `false` | O6/Núcleo preservam ausência até consultar documento |
| M11 | documento sempre vence caller | O6 exige override explícito do caller |
| M12 | opção SVG vaza para `PdfTags` | O8-PDF exige metadata independente; outros targets são controles |
| M13 | factory publica filho antecipado `format.png` | O9/P1357 exigem ausência |
| M14 | erro nested vira fallback e documento/artefato | O4/invariante 8 exigem erro sem parcial |
| M15 | exporter resolve precedência | ownership exige resolução na pipeline |
| M16 | L3 percorre `Content` e interpreta style key | ownership/invariante 6 reservam isso a L1 |
| M17 | código aponta `@prompt` para Núcleo | V26 proíbe código apontar para Núcleo |
| M18 | pretty injeta `<g>` XML válido | O7 compara morfologia da mesma fonte, inclusive styled-root |

As exclusões permanecem as mesmas: mutação equivalente, mudança apenas
mecânica, receita que não compila, XML inválido e identidade ambígua não entram
como kill. M15/M16 compilam com semântica preservada e violam ownership; M17
compila e é negativo V26; M18 mantém XML válido e a mesma fonte dos dois lados.

## 4. Construção e focos públicos

P00 e M01–M18 foram recompilados em 19 targets isolados, limpando os quatro
crates locais antes de cada build. Todos os `cargo clean` e `cargo build`
terminaram com exit `0`; os binários foram copiados e `strip`ados. O custo
somado dos builds foi `614882 ms` (`10.248 min`), com mínimo `24690 ms`, máximo
`90048 ms`, e custo lógico observado do bundle de `58G` por reter os targets.

O foco P00 mediu antes de decidir:

| Caso | Resultado |
|---|---|
| `#set format.svg(pretty: 1)` | exit `1`, exatamente `expected boolean, found integer`, sem artefato |
| regra vazia | exit `0` |
| `pretty: false` | exit `0` |
| `pretty: true` | exit `0` |

O controle público de M12 também permaneceu: P00 false/true resulta
`Tagged: yes/yes`; M12 false/true resulta `yes/no`. Os quatro PDFs têm duas
páginas e texto extraído SHA-256
`53cf8e98fe620672c07308b494b85433683a91c4d89066be70715116f2816421`.
Isso valida construção e preservação do operador; não atribui kill.

## 5. Denominador, execução e limite final

O denominador continua exatamente `18`, uma instância válida por M01–M18:

```text
mutation_score = mutantes válidos rejeitados / 18
score exigido  = 18/18 = 1.0
```

`Unknown` e `Pending` nunca recebem crédito. Equivalentes e não compiláveis são
excluídos, nunca contados como mortos. Identidade incompleta, timeout, crash,
parser opaco ou observável ambíguo resulta `Unknown`.

O executor independente deve usar o runner R4 uma única vez com o verificador
externo. As três ordens são estritamente seriais, P00 vem primeiro em cada
ordem e `direct_repeat` repete o vetor completo. Execução concorrente das
ordens é proibida; timeout externo menor que `360 s` por variante é rejeitado.
Somente três vetores completos e compatíveis podem sustentar score `1.0`.

O budget congelado é `120 min`, quatro revisões e até três campanhas completas.
R4 consumiu 19 builds e dois conjuntos focais, mas nenhuma campanha completa.
Esta é a última revisão local: qualquer falha, regressão, mutante sobrevivente,
`Unknown`, budget excedido ou vetor incompleto encerra o ensaio como
**inconclusivo**, sem R5 local, sem selo e sem autorização de candidato.

## 6. Atestação

Hashes demonstram identidade, delta e ordem, não isolamento de capacidades.
O trabalho ocorreu em checkout/filesystem compartilhado, portanto a alegação
máxima é **executado sem atestação de isolamento**. Este catálogo atesta apenas
a construção adversarial R4 e os focos públicos descritos.
