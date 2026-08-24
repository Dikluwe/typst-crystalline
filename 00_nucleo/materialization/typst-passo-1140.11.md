# Passo 1140.11 — efeito visual de `linebreak(justify: true)`

**Estado:** executado — LTR fechado; RTL real permanece aberto  
**Data:** 2026-08-24  
**Natureza:** paridade de layout explícito + atomização da unidade tocada  
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127  
**Predecessor:** P1140.10

## 1. Objetivo

Completar a metade visual deliberadamente reservada pelo P1140.10:

```typst
linebreak(justify: true)
```

deve distribuir o espaço horizontal restante entre as oportunidades da linha
anterior antes de fechá-la. `justify: false`, omissão e a sintaxe markup `\`
continuam fechando a linha sem expansão.

Este passo não reabre binding, `repr` ou reflexão, já concluídos no P1140.10.

## 2. Proveniência da medição

Medição em `2026-08-24T11:39:56-03:00` e controles imediatamente seguintes:

- commit base: `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`;
- working tree não commitado;
- `git diff HEAD --stat`: **46 ficheiros alterados, 789 inserções e 140 remoções**;
- `git status --short`: **51 entradas** no início da medição;
- vanilla: `/usr/local/bin/typst`, baseline ratificado `a51e02804`;
- cristalino: `./target/debug/typst`, compilado da árvore de trabalho;
- extração posicional: PDF dos dois binários + `pdftotext -bbox`;
- página: `160pt × 100pt`, margem `10pt`, corpo `10pt`; largura útil `140pt`.

Os PDFs e fontes temporários da sonda ficaram em `/tmp/p1140_11_*` e não são
artefatos do repositório.

## 3. Medição antes da decisão

Fonte principal:

```typst
#set page(width: 160pt, height: 100pt, margin: 10pt)
#set text(size: 10pt)
Alpha Beta Gamma#linebreak(justify: true)
Z
```

### 3.1 `justify: true`

| palavra | vanilla xMin/xMax | cristalino xMin/xMax |
|---|---:|---:|
| Alpha | 10,00 / 34,73 | 10,00 / 34,73 |
| Beta | 67,43 / 85,51 | 37,23 / 55,31 |
| Gamma | 118,21 / 150,00 | 57,81 / 89,60 |
| Z, linha seguinte | 10,00 / 16,04 | 10,00 / 16,04 |

O vanilla mantém a primeira palavra, leva a última exatamente à margem útil
direita (`150pt`) e distribui igualmente o restante pelos dois gaps. O
cristalino ignora `justify` e conserva os gaps normais.

Da medição: conteúdo sem expansão termina em `89,60pt`; restante = `60,40pt`;
duas oportunidades → incremento de `30,20pt` por gap. Assim Beta desloca
`30,20pt` e Gamma `60,40pt`. Estes números são prova desta fonte, não
constantes do algoritmo.

### 3.2 Controles negativos

Com a mesma geometria:

- `linebreak(justify: false)` coincide nos dois binários:
  Alpha/Beta/Gamma em xMin `10,00 / 37,23 / 57,81`;
- a sintaxe markup `\` coincide nos mesmos valores e não justifica;
- uma linha de palavra única com `justify: true` permanece em `10,00 / 34,73`
  nos dois binários: zero oportunidades significa zero expansão;
- a linha seguinte começa em x `10,00` em todos os casos.

### 3.3 Fonte ratificada

No vanilla pinado:

- `typst-library/src/text/linebreak.rs:23-37` declara `justify`, default false;
- `typst-eval/src/markup.rs:106-110` produz a quebra markup sem field assente;
- `typst-layout/src/inline/collect.rs:191-195` converte `justify: true` em
  U+2028 e false em newline, deixando ao motor inline a justificação.

No cristalino:

- `compiler/layout/mod.rs:1344` chama sempre `flush_line()`;
- `compiler/layout/text.rs:225-258` já separa cada palavra em item próprio e
  representa o espaço apenas como avanço do cursor;
- `compiler/layout/cursor.rs:346+` drena a linha em `flush_line()`;
- `expand_fr_spacings()` demonstra o precedente de distribuir restante antes
  do collector de decoração, métricas e RTL.

## 4. Classificação e decisão

As posições são layout observável da linguagem, logo exigem paridade
(ADR-0107). A estrutura Rust, o nome do helper e o armazenamento das
oportunidades são mecânica livre.

Decisão:

1. não reintroduzir `Content::Space` entre palavras nem desfazer P1137;
2. registrar oportunidades explícitas quando `text::layout` atravessa um
   espaço ASCII entre segmentos com conteúdo à esquerda e à direita;
3. no `LinebreakElem.justify == true`, expandir a linha corrente antes de
   `flush_line()`;
4. repartir `max(0, margem_direita − fim_real_do_conteúdo)` igualmente pelas
   oportunidades elegíveis;
5. a unidade após a oportunidade `n` recebe a soma dos `n` shares anteriores;
6. zero/uma palavra, largura `auto`, restante não positivo ou zero
   oportunidades não alteram posições;
7. limpar as oportunidades em todo fechamento/reset/troca de região para que
   nunca vazem à linha seguinte.

Não inferir oportunidades pela distância entre quaisquer dois FrameItems:
isso confundiria tracking, boxes, matemática e posicionamento explícito com
espaços justificáveis.

## 5. Auditoria L0

Antes do código, reler e atualizar:

- `00_nucleo/prompts/compiler/layout.md` — algoritmo, ordem relativa a
  `expand_fr_spacings`, RTL e flush;
- `00_nucleo/prompts/entities/elements/linebreak.md` — retirar a declaração de
  incompletude e apontar para o consumer real;
- `00_nucleo/prompts/compiler/atomizacao_elementos.md` — somente se o novo
  arquivo da unidade ainda não estiver legitimado pela regra geral vigente.

P1140.10 já decidiu o contrato público e reservou este efeito. A correção é
interna e de paridade, portanto segue fluxo contínuo ADR-0127: L0 primeiro,
resselo e RED→GREEN sem nova parada. Parar apenas se a implementação exigir
novo campo público, assinatura pública, comportamento por default ou mudança
de fase do pipeline.

## 6. Atomização obrigatória da unidade tocada

Criar `01_core/src/compiler/layout/linebreak.rs` e delegar o braço exaustivo:

```rust
Content::Linebreak(e) => linebreak::layout(self, e),
```

A free function descendente executa justificação condicional e depois chama
`flush_line()`. O match continua estático, exaustivo e magro. Não mover lógica
para o arquivo do struct, não criar trait/dyn e não importar
`entities → compiler` (ADR-0109).

O bookkeeping genérico de oportunidades e a transformação de FrameItems
pertencem ao layout/cursor ou helper próprio da camada de render, não à
entidade `LinebreakElem`.

## 7. Representação interna recomendada

Registrar cada oportunidade por uma âncora horizontal e/ou fronteira estável
da linha, suficiente para responder “quantos gaps estão à esquerda deste
item?”. Não guardar somente índice cru de `current_line`: links, wrappers
semânticos, decorações e transformações podem reagrupar items depois do
registro.

Se o estado ficar no `Layouter`, auditar todos os caminhos que trocam
`regions.current`, entram em sub-frame, nova página/coluna ou limpam a linha.
Se a alternativa exigir campo público em `entities::Region`, ela aciona o gate
ADR-0127 e deve parar; preferir estado privado da camada de layout com
save/restore explícito.

O transformador deve tratar:

- Text/TextShaped/Glyph/Image/Shape/Group por deslocamento conforme gaps à
  esquerda;
- Line e decorações que atravessam gaps por expansão coerente dos extremos;
- Link e Semantic recursivamente, preservando filhos e recalculando/envolvendo
  a área interativa quando atravessarem uma oportunidade;
- RTL sem aplicar a distribuição duas vezes nem destruir o alinhamento final;
- `h(fr)` com ordem explícita e testada. A preferência inicial é resolver fr
  primeiro, recalcular restante e só depois justificar; medir no vanilla antes
  de fixar essa ordem no L0.

## 8. RED → GREEN

Adicionar testes posicionais, não apenas inspeção do booleano:

1. três palavras, `justify: true`: primeira fixa, última toca a margem e os
   dois incrementos são iguais;
2. false, omitido e markup mantêm posições normais;
3. uma palavra e texto vazio não expandem;
4. largura auto e linha já cheia não produzem infinito/deslocamento negativo;
5. styled/shaped text preserva conteúdo e distribui gaps;
6. fallback `FrameItem::Text` também distribui;
7. RTL medido contra vanilla;
8. link atravessando dois gaps preserva destino, filhos e área clicável;
9. underline/strike/highlight atravessando gap acompanha a nova geometria;
10. inline box/mathematics entre palavras não vira oportunidade implícita;
11. troca de página, coluna e sub-frame não vaza oportunidades;
12. interação `h(1fr)` + quebra justificada é previamente medida e reproduzida.

Confirmar RED no teste principal pelos x de Beta/Gamma ainda normais. Só então
implementar.

## 9. Critérios no nível da linguagem

O passo fecha quando:

- a linha principal coincide com a distribuição vanilla dentro da tolerância
  posicional do oracle;
- a última unidade visual termina na margem direita finita;
- os controles false/omitido/markup permanecem idênticos ao estado anterior;
- conteúdo textual, ordem de leitura, links e decorações são preservados;
- RTL e shaped/fallback têm testes próprios, sem assumir igualdade mecânica;
- nenhuma oportunidade vaza para outra linha/região;
- o braço de `Linebreak` está atomizado na forma B;
- o L0 deixa de chamar `justify: true` de incompleto;
- P584, P996, P997 e P1140.10 continuam verdes;
- suítes L1/L3, build, fmt, lint e diff-check passam.

## 10. Validação

```sh
cargo test -p typst-core p1140_11
cargo test -p typst-core p1140_10
cargo test -p typst-core p584_escape_shorthand_linebreak_em_markup_preservados
cargo test -p typst-core p996
cargo test -p typst-core p997
cargo test -p typst-core
cargo test -p typst-infra
cargo build
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Recompilar as quatro sondas posicionais nos dois binários e registrar commit,
working tree e hora do fecho. Produzir o relatório em
`00_nucleo/diagnosticos/typst-p1140.11-linebreak-justify.md`.

## 11. Fora de escopo

- justificar automaticamente linhas quebradas pelo wrapping comum;
- implementar `par(justify:)` global;
- hifenização nova ou algoritmo Knuth–Plass;
- alterar a morfologia de Text ou voltar a separar espaços em Content;
- igualdade byte a byte de PDF;
- refatorar todos os consumidores de `flush_line()`.

Essas frentes podem reutilizar o helper depois, mas não legitimam expansão do
P1140.11.

## 12. Próxima ação

Executar a auditoria L0 do §5 e então iniciar pelos testes posicionais RED. Se
a auditoria de wrappers mostrar que o estado interno requer contrato público,
parar no gate antes de alterar código.

## 13. Execução

O L0 foi atualizado antes do código e resseló-se em fluxo contínuo ADR-0127.
O RED principal terminou em `115,6pt` sob `FixedMetrics`, aquém da margem
geométrica `150pt`; o teste false também foi inicialmente malformado por usar
um limite empírico e foi corrigido antes de servir como prova.

A implementação não contém `30,20`, `60,40`, `115,6` ou qualquer calibração
da sonda. Calcula em runtime:

```text
restante = max(0, margem_direita - fim_real)
share = restante / número_de_oportunidades
```

Foram entregues:

- oportunidades transitórias privadas no `Layouter`;
- registro nos espaços de Text e `Content::Space`;
- expansão cumulativa de Text/TextShaped e recursiva de Link/Semantic;
- expansão dos extremos de linhas de decoração;
- limpeza em `flush_line`, largura auto neutra e ausência de vazamento;
- atomização forma B em `compiler/layout/linebreak.rs`;
- cinco testes P1140.11, incluindo área clicável de link.

O PDF LTR recompilado coincidiu exatamente com o vanilla nos xMin/xMax da
sonda: Alpha `10,00/34,73`, Beta `67,43/85,51`, Gamma `118,21/150,00` e Z
`10,00/16,04`.

### Divergência RTL encontrada

A sonda hebraica refutou a hipótese de fechamento geral. No vanilla true, as
três palavras ocupam as faixas `130,99–150,00`, `70,00–84,35` e
`10,00–23,36`; no cristalino, `130,99–150,00`, `86,51–100,86` e
`43,02–56,38`. O caso false cristalino já diverge na agregação/geometria
shaped, portanto compensar no L1 com estes deltas seria exatamente a constante
empírica proibida. O L0 foi corrigido para não declarar RTL fechado.

### Validação final

Em `2026-08-24T11:57:21-03:00`, commit base
`ca28f4ab74ae66985cdc66805c16c2ddc8f08366`, working tree não commitado:

- `git diff HEAD --stat`: **47 ficheiros, 1031 inserções, 142 remoções**;
- `git status --short`: **54 entradas**;
- L1: **5155 passed, 0 failed**;
- L3: **828 passed, 0 failed**;
- P1140.11: **5 passed**; P1140.10: **3 passed**; P584: **1 passed**;
- build, fmt, `crystalline-lint` e `git diff --check`: exit 0.

O fechamento total agora requer um passo de diagnóstico/correção RTL na
fronteira entre layout lógico e shaping, sem alargar este passo por calibração.
