# Estudo das falhas de texto e das correções

As falhas formam dependências: paginação e medição precisam ficar corretas antes da troca de algoritmo. A atomização deu owners próprios a cada causa, mas os contratos ainda conservam limitações antigas. Este estudo confirmou os caminhos atuais e acrescentou uma falha de continuidade: texto pode ser emitido com baseline herdada da página anterior, fora da página nova.

Este é um diagnóstico com candidatos de correção e refutadores. Não altera código produtivo, L0 ou o pin; não declara as correções implementadas.

## Medições antes das propostas

Working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`. `estudo-correcao-falhas-texto-2026-09-28/initial.json`, `proveniencia.json` e `final-receipt.json` registram UTC, lista exata de alterações, `git diff HEAD --stat`, fontes/L0 e SHA-256 dos binários. O código e os prompts permaneceram iguais durante este estudo. O build release atual passou; a receita está em `build.log`.

Oráculo: vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, árvore ratificada `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Fonte Noto Sans versionada, 12pt, margens e dimensões explícitas. Não se reivindica isolamento universal de fontes.

Foram executadas **30 novas entradas nas duas implementações**: finalização/continuação/pagebreak, combinações de hyphenate e justify, palavra longa, argumentos inválidos, fronteira de parágrafo e tracking fracionário. `sondas.py`, `resultados.json` e os PDFs/XMLs/stderr conservam tudo; `summary.json` facilita a leitura. São controles de investigação, não testes de paridade aprovados.

A sonda externa eval→layout→shaping examinou **seis entradas** sem alterar consumers. `layout-probe.rs.txt`, `layout-probe-recipe.json` e `layout-probe-results.txt` registram bibliotecas, comando e resultado. Ela não substitui toda a pipeline de callbacks/introspecção; sua concordância com os PDFs nestes casos não certifica o restante do compilador.

Os **15 owners examinados** têm header `@prompt-hash` igual ao cálculo canônico do L0 (`L0-inventory.json`); o linter oficial não encontrou V5/V6/V15/V26 nesses owners. O campo histórico Hash do Código de par não coincide com um cálculo bruto auxiliar; esse cálculo não é o gate oficial nem prova, sozinho, de deriva normativa. O estudo não ressela esse registro. O linter terminou sem erros, com 497 avisos (`lint-summary.json`).

## 1. Paginação: uma próxima baseline não é conteúdo

### Prova

| Entrada, página 140×100pt | Páginas Tekt | Páginas vanilla | Controle adicional |
| --- | ---: | ---: | --- |
| `terminal` | 2 | 1 | Antes do PDF, Page 2 tem zero items |
| `terminal-pagebreak` | 3 | 2 | Antes do PDF, Pages 2 e 3 têm zero items |
| `terminal-two-pagebreaks` | 4 | 3 | Cada quebra explícita continua observável |
| `empty` | 1 | 1 | A página inicial vazia é legítima |
| `empty-explicit-pagebreak` | 2 | 2 | Uma página vazia solicitada não deve ser eliminada |
| `terminal-continuation` | 3 | 2 | Todas as páginas Tekt contêm items; não é só página branca |

O controle de continuação emite `Four` e `additional` na Page 2 com baseline **101,84pt**, numa página de **100pt** (`outside-viewport.json`). A sonda mostra esse conteúdo no layout; a extração PDF deixa de recuperá-lo integralmente. Esse critério mede baseline fora do viewport, não toda a caixa de tinta.

### Caminho causal e L0

`01_core/src/compiler/layout/line_commit.rs:657–696` drena a linha, soma avanço para a próxima baseline e chama `region_progress::post_line`. `region_progress.rs:113–128` usa o cursor já avançado para criar região. `mod.rs:1928–1943` chama flush no finish; `mod.rs:1980–1984` permite compor a página corrente vazia. O L0 `00_nucleo/prompts/compiler/layout/region_progress.md` preserva explicitamente esse comportamento para a refatoração anterior.

A continuação passa por `parbreak.rs:41–80`, depois `cursor.rs:80–135`. O candidato causal mais forte é a reutilização de `prev_line_baseline` da página fechada: a baseline medida da última linha é 78,872pt; spacing padrão é 14,4pt; top edge é 8,568pt. A soma produz 101,84pt, exatamente a baseline observada. `next_region` não substitui `prev_line_baseline`; o ramo block_chain reconstrói a próxima baseline e sua proteção exclui uma página corrente vazia. Esta ligação é **inferência sustentada por fonte e medidas**, não uma captura instrumentada de todos os campos. Refutador: uma sonda de estado mostrar outra origem desses valores ou a correção dessa causalidade não eliminar a emissão fora da região.

### Correção proposta

Separar **fechamento da linha** de **necessidade de admitir conteúdo na próxima região**. Comitar a linha que cabe pela sua tinta; conservar a previsão da próxima baseline como demanda pendente; efetuar o avanço quando chegar a próxima unidade material, uma quebra explícita ou uma drenagem realmente necessária. Ao avançar, não transportar espaçamento fraco/baseline causal da página anterior como coordenada da página nova. Estado pendente deve participar do checkpoint e rollback.

Uma alteração apenas em finish pode corrigir o caso simples, mas não o espaço/parbreak que fecha antes dele, a duplicação anterior ao pagebreak nem a continuação fora do viewport. Eliminar páginas vazias do documento final perde páginas legítimas. Retirar post_line sem preparar a entrada da próxima unidade também quebra continuidade: ensure_initial_baseline não possui hoje uma proteção geral em todos os ramos.

Owners a atualizar antes do código: `00_nucleo/prompts/compiler/layout/region_progress.md`, `00_nucleo/prompts/compiler/layout/line_commit.md`, `00_nucleo/prompts/compiler/layout/cursor.md`, `00_nucleo/prompts/compiler/layout/parbreak.md`, `00_nucleo/prompts/compiler/layout/pagebreak.md` e `00_nucleo/prompts/compiler/layout/flow_suffix.md` conforme o escopo final e `00_nucleo/prompts/compiler/layout.md` para estado/checkpoints/finish. Não usar page_run_boundary_empty como substituto genérico: já expressa outra causa.

Refutadores obrigatórios: terminal com e sem parbreak; continuação com conteúdo dentro do viewport; uma/duas quebras explícitas e paridade odd/even; página inicial vazia; altura auto/infinita; inline alto; colunas; floats/notas pendentes; rejeição/replay sem duplicar texto, posições ou numeração. Os controles de colunas/floats/notas ainda precisam ser medidos para fechar o contrato; a proposta não declara cobertura desses eixos.

## 2. Medição: uma autoridade de extensão para decidir e emitir

### Prova

Na mesma Noto Sans, `office`, 12pt, quatro glifos shaped:

| Tracking | FontMetrics.text_width | Advances emitidos | Diferença |
| ---: | ---: | ---: | ---: |
| −0,333pt | 29,475pt | 30,132pt | −0,657pt |
| +0,333pt | 32,805pt | 32,148pt | +0,657pt |
| +3pt | 46,14pt | 40,14pt | +6pt |

A diferença não desaparece ao permitir tracking negativo. O arredondamento para unidades da fonte explica parte dos controles fracionários; o número de fronteiras de cluster explica o restante. Não se atribui toda divergência de linha dos PDFs a esse termo isolado.

### Fonte/L0 e correção

`text_measure.rs:17–30` soma tracking por pares de codepoints, após advance_shaped. `03_infra/src/shaper/shaping.rs:70–108` mede advances com um caminho de shaping próprio; `:224–290` configura tracking/features para emissão; `:301–334` aplica tracking entre clusters e arredonda por fonte. Seus L0 conservam essa diferença. O caminho produtivo injeta FallbackFontMetrics (`pipeline/paged.rs:518–521`); o owner `00_nucleo/prompts/infra/font_metrics/fallback.md` já governa essa implementação.

Não há dados de clusters num Pt devolvido pelo trait. Portanto, alterar só a contagem em L1 não pode reconstruir o shaping real. Candidato preferido: uma preparação privada de sub-run no shaper, consumida por medição e emissão, e override do **método existente** FontMetrics.text_width no provider real para devolver extensão completa. Preservar advance_shaped como base onde seu contrato a exige; evitar somar tracking duas vezes. O fallback sem shaping continua explícito e FixedMetrics conserva sua aproximação. Não é necessário acrescentar método público ao trait para iniciar essa correção.

Essa preparação deve compartilhar idioma, direção, variações, script/features, unidades da fonte e quantização. O estudo encontrou diferenças estáticas de configuração math/ssty entre os caminhos; não mediu um novo defeito matemático causado por elas. Antes de harmonizar, usar controles próprios que distingam GlyphFragment/TextItem e suas regras vigentes.

Também é preciso medir as fronteiras entre fontes/sub-runs e palavras: o vanilla aplica tracking no conjunto de glifos em `inline/shaping.rs:1269–1296`, enquanto a emissão Tekt encerra tracking no fim de cada sub-run. Igualar estimate ao emissor atual é uma obrigação interna; não prova, sozinho, paridade completa com o vanilla.

Refutadores: ligadura office; combining marks; texto sem ligadura; tracking zero/positivo/negativo/fracionário; idioma/direção; fallback entre fontes; fronteira de estilos e espaços; multiline; métricas com override; fonte não resolvida; cache frio/quente. Testar coerência geométrica e texto antes/depois do layout, sem impor IDs de glifo ou bytes de PDF como paridade.

## 3. Configuração: validar, transportar e só então decidir

### Prova

As combinações de hyphenate absent/auto/false/true produziram a mesma hifenização Tekt, com ou sem justify. O vanilla resolve **auto segundo a justificação**; absent segue o mesmo default. Configurar false desativa e true ativa a possibilidade. Justificar também altera a seleção de linhas; estas duas variáveis não devem ser confundidas.

Os quatro controles inválidos falham no vanilla e compilam no Tekt:

- `#set text(hyphenate: 1)`;
- `#par(linebreaks: "nonsense")[...]`;
- `#par(justify: 1)[...]`;
- `#set par(justify: 1)`.

A chamada nativa ignora silenciosamente os argumentos reconhecidos; os set rules avisam scope-out. `native-par-boundary` mostra uma diferença adicional: vanilla coloca BEFORE/MIDDLE/AFTER em parágrafos distintos; Tekt mistura o trecho e hifeniza AFTER.

### Fonte/L0 e correção

`eval/rules.rs:2972–3010,3407–3420` descarta essas propriedades. `stdlib/structural/par.rs:119–126` aceita e ignora. Seu L0 declara isso deliberadamente; não é apenas implementação divergente de um prompt correto. `Content::Par` **já existe** (`entities/content.rs:225`), mas `layout/mod.rs:1469` o trata como transparente. A função de justificação já existe em LineCommit, porém é invocada pelo linebreak explícito justificado (`layout/linebreak.rs:40–43`), não por uma política de parágrafo completa.

Proposta em etapas: validação equivalente para set e constructor; transporte lexical tipado internamente; resolução de auto com contexto do parágrafo; consumo no seletor/compositor; fronteiras explícitas da chamada par. O canal Styles custom existente pode transportar valores sem acrescentar campos públicos a TextStyle. Isso não dispensa validar precedência, body styled, show-set e captura lexical. Não aceitar silenciosamente uma estratégia ainda sem consumer.

Não aplicar uma justificação genérica a toda última linha nem fazer native_par executar o layout. O owner nativo constrói conteúdo/estilo; a fase de layout consome a configuração. Usar o Content::Par existente evita inventar uma nova variante pública, mas muda uma restrição expressa do L0 nativo: atualizar esse contrato primeiro, inclusive a atual proibição de helpers privados.

## 4. Palavra longa: origem da linha e progresso são decisões distintas

`origin-long-true` produz uma linha Tekt contra três vanilla; auto/false e idioma sem padrões são controles diferentes, nos quais não se deve exigir hifenização indisponível.

`break_selection.rs:19–38` retorna Keep quando o cursor não está além da margem, mesmo que a palavra exceda a região. `word_layout.rs:39–44` aplica Wrap com flush seguido da palavra inteira, sem repetir a seleção na nova linha. Portanto, remover uma única comparação não basta: isso pode gerar flush vazio ou deixar intacto o caso em que nenhum prefixo cabe no resto da linha, mas um prefixo caberia na linha nova.

Proposta: representar separadamente overflow, linha com prefixo material e oportunidades permitidas. Tentar uma divisão válida também na origem; se nada cabe no resto de uma linha ocupada, fechar e reavaliar na nova; se a linha está vazia e não existe divisão válida, conservar progresso de uma palavra indivisível. Progresso depende de consumir conteúdo ou mudar uma região válida, não apenas de uma coordenada x. Usar origem útil de linha/célula, não margem física como substituto universal.

Refutadores: palavra única; prefixo curto; prefixo sem espaço restante; no idioma/idioma sem padrões; hyphenate false; Unicode com acentos; margem/célula deslocada; largura infinita; palavra mais larga que toda a linha; não perder/duplicar conteúdo nem entrar em loop. A semântica de hyphenate deve chegar ao seletor antes de declarar esse percurso correto.

## 5. Alternativas de seleção e wiring

O seletor privado atual recebe uma palavra e geometria local. Um otimizador precisa de oportunidades e custos do **parágrafo**, estilos/caixas/restrições e fronteiras forçadas. Acrescentar uma opção optimized à função atual sem essa preparação não implementa a capacidade observada no vanilla.

No pin, `inline/mod.rs:194–196` resolve auto para Simple sem justify e Optimized com justify; `inline/linebreak.rs:152–160` seleciona os algoritmos. Isso comprova coexistência de peças, não obriga o Tekt a copiar estruturas de dados ou custos internos do upstream.

Contrato futuro a estudar: preparação produz oportunidades/extensões; seletores simple e optimized recebem esse mesmo problema e devolvem quebras; compositor recebe quebras, não recalcula escolha; wiring fornece implementações estaticamente, e a política de linguagem seleciona o comportamento em L1. Decidir pela configuração do documento é lógica de domínio e não deve ser deslocada para L4. Nenhum benchmark foi executado; ganhos e limites continuam por medir.

## Ordem de trabalho e gates

| Correção | Pré-requisito para fechar | Gate provável |
| --- | --- | --- |
| Demanda regional e geometria causal de continuação | Provar terminal, páginas explícitas e continuação sem texto fora da região; ampliar controles de inserções/colunas | Interna, L0 primeiro e RED→GREEN |
| Extensão coerente entre medição/emissão | Controles de clusters, fronteiras, arredondamento e cache | Interna usando método existente; novo trait público exigiria gate |
| Validação/transporte de configuração | Set/native, precedência, auto e fronteira de parágrafo | Paridade interna com tipos existentes; qualquer contrato público novo exige gate |
| Palavra longa na origem/reavaliação | Política de hyphenate consumida e garantia de progresso | Interna, L0 primeiro e RED→GREEN |
| Preparação de parágrafo + seletores alternativos | Larguras/configuração/fronteiras corretas e custo medido | Classificar contrato/fase/default antes de materializar |

As propostas de correção não foram escritas como L0 nem implementadas. ADR-0127 permite fluxo contínuo para correções internas/paridade; exige confirmação para contrato público, mudança de fase, default deliberado de produto ou incompatibilidade. O uso futuro de um campo público/método de trait não fica autorizado por este diagnóstico.

Os testes RED→GREEN deverão afirmar comportamento da língua: conteúdo conservado, geometria/páginas causais, efeitos das propriedades e erros. Testes de estrutura e suítes verdes da atomização são complementares. O estudo aponta limites explícitos dos prompts e dependências de construção; não permite inferir preguiça ou intenção dos autores.
