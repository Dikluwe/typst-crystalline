# Recuo francês — parágrafos e bibliografia

Etapa geométrica concluída. O objetivo integral **Faça todas as correções** continua aberto; as pendências estão em [proximos.md](proximos.md).

## Resultado

`par(hanging-indent: ...)` e `set par(hanging-indent: ...)` agora transportam o comprimento e usam a largura correta da primeira linha e das continuações. Valores zero, negativos e abs+em, LTR/RTL latino, linhas forçadas, regiões seguintes e restauração de escopo foram exercitados. O em vem do tronco lexical do parágrafo, sem tomar a fonte de um run parcial.

A presença da declaração interrompe o parágrafo mesmo com valor zero, repetido ou corpo vazio. Essa fronteira usa o protocolo existente; não cria texto, tinta, altura ou Location fictícia. Um novo parágrafo completo recupera seu planejamento próprio; o teste discriminante comprova que `simple` escolhe duas palavras na primeira linha onde `optimized` escolhe três, sem impor greedy.

Na bibliografia Hanging, o default MLA de 1.5em é preservado e o show-set efetivamente aplicado ao alvo pode sobrescrevê-lo (incluindo zero/negativo/abs+em). Um set par exterior isolado não substitui esse default. O corpo conserva um inset estrutural e uma compensação privada só da primeira linha, sem HSpace artificial. Os planos Plain e Columns permanecem distintos; seletores, callbacks e aliases de função usam a realização existente. O alias de Content continua como pendência própria.

Fragmentos inline simples de Block/Box e corpos de Heading/Raw/List/Quote não recebem recuo genérico indevido; Par explícito e corpos que exigem parágrafos têm contexto próprio. Os testes puros aprovados não fecham os defeitos de tamanho/recorte de containers observados no CLI.

## Escopo e arquitetura

69 arquivos existentes alterados relativamente ao snapshot inicial desta etapa, mais 3 novos. São 34 consumers existentes e um novo, com 32 existentes funcionalmente ajustados e dois apenas repinados; 35 L0s existentes (incluindo núcleo) e dois novos. A lista exata e os hashes estão em final-state.json.gz e nos inventários prescription.json/decoration-prescription.json, sem misturar o diff herdado das etapas anteriores.

O novo owner `00_nucleo/prompts/compiler/layout/paragraph_indent.md` materializa somente `01_core/src/compiler/layout/paragraph_indent.rs`. Seu estado privado conserva contexto, forma inicial/continuada, direção, admissão/commit e restauração. Os invariantes compartilhados estão no núcleo paragraph-hanging-indent; o núcleo paragraph-selection foi atualizado e os consumers repinados. O match permanece nominal, fechado e estático. Nenhum contrato Rust público, modo, flag, pin de CI ou fase/ordem de realização foi alterado. A aprovação R2-S anterior é preservada; este lote não antecipa R3-T1.

L0 primeiro, grafo V15/V26 válido antes do resselo e da implementação, RED funcional antes dos corpos produtivos. O histórico de tentativas e limites está em [iterations.md](iterations.md).

## Evidência de paridade

O corpus novo tem 132 pares Tekt/vanilla, todos compilados. O critério completo exige palavras, páginas, dimensões e caixas com tolerância numérica 0.001 pt. Passou de 20 para 83 casos completos; nenhum dos 20 anteriores regrediu. Os 63 ganhos não são extrapolados para a linguagem inteira.

| Classificação após o reparo | Casos | Estado |
| --- | ---: | --- |
| Geometria completa | 83 | Critério completo aprovado |
| Recuo horizontal, palavras, linhas, páginas e dimensões iguais | 31 | Resíduo vertical de métricas mistas ~0.156 pt permanece |
| Plain + optimized, CSL custom | 7 | Escolha anterior de linha do corpo ainda diverge |
| Containers Box/Raw/List/Quote | 10 | Resíduos de recorte/tamanho, espaçamento ou quebra |
| Alias de Content armazenado | 1 | Realização textual R3-T1 pendente |

O comparador completo não foi relaxado para contar os 31 como fechamento. A auditoria separada exige x por palavra, atribuição de linha/página e dimensões idênticas; o resíduo vertical permanece mensurado. Em Plain, o corpo normalizado pela sua própria origem é idêntico ao baseline; o set par 3em agora afeta corretamente a citação anterior, podendo transladar o corpo. A primeira auditoria mais ampla que falhou foi preservada, não ocultada.

Foram repetidos 94 controles da etapa de margens: 84 com geometria completa e 89 resultados exatamente iguais ao baseline. Os cinco MLA alterados agora têm o recuo e as quebras corrigidos; só restam ~0.156 pt de métricas mistas. Os 72 controles genéricos de margens continuam integralmente iguais ao oracle. Nos dez casos atuais de containers, nove mantêm os frames extraídos do baseline; Par-em-Box com h10 aplica o recuo, mas o defeito de recorte/tamanho segue aberto.

Valores extremos têm prova de texto lógico nos frames puros. `pdftotext` recorta tinta fora da página; a extração recortada não é prova de perda de Content. Matrizes e critérios: before/after*.json.gz, audit.json.gz, causal-audit.json, controls-audit.json, fontes .typ e arquivos de artefatos iniciais.

## Refinamento complementar de decorações

A revisão final encontrou sobrelinha/riscado/sublinhado iniciando em x8 mesmo com texto recuado a x18. O preview privado agora prevê a origem sem consumir compensação nem admitir material; coletores de flush/subframe e o segmento final usam a borda efetiva. Um preview negativo só produz segmento final quando houve admissão real, evitando traço fantasma em body vazio/metadata/Par vazio. L0s atualizados antes dos testes/código, com novos REDs funcionais.

Doze pares SVG complementares verificam o início correto de cada linha LTR nas três decorações, com e sem quebra anterior, h0/h10. Fora do x inicial de cada traço, todos os atributos das linhas e todo o restante do SVG são exatamente preservados relativamente ao candidato anterior. Nove pares com h negativo e corpos vazios exigem ausência de traço no Tekt e no vanilla. A primeira coleta omitiu primitivas line por procurar apenas path; o registro inicial e a inspeção corrigida estão preservados. Isso não é evidência de falta de decoração no produto.

As diferenças verticais, de espessura e de cauda das decorações já visíveis com h0 não foram alteradas nem fechadas; geometria completa de RTL nesta família também permanece a medir. Os 132 pares e 94 controles principais foram repetidos com o binário final, além desses 21 complementares.

## Testes e gates

**9029 testes passaram, zero falhas, 7 ignorados; 5 doctests.**

Os 14 testes iniciais deram 11 falhas funcionais e três controles aprovados antes da implementação. A rodada final focada aprovou 18, incluindo o discriminante de estratégias, as origens de decorações e a proteção de corpos vazios. Os testes complementares deram dois REDs de origem e um RED de traço fantasma antes dos respectivos reparos. As falhas intermediárias de direção e as correções de fixture estão registradas com logs; não foram usadas falhas de compilação como RED.

Build workspace, Clippy all-targets (`-D warnings`, `-D unfulfilled_lint_expectations`), Rustdoc (`-D warnings`), fmt, V1/V5/V15/V26, linhagem reversa 989 e diff-check passaram. TMPDIR explícito e oracle verificado foram fornecidos desde o início da suíte ampla deste lote. As 2102 fontes/L0s congeladas permaneceram byte a byte iguais durante toda a validação final. O conjunto anterior de 9026 testes, anterior ao refinamento de decorações, está preservado em initial-validation e não foi somado ao total final.

| Tekt pinado no CI | Antes | Depois |
| --- | ---: | ---: |
| V16 erro | 105 | 105 |
| V19 nota | 1122 | 1123 |
| V20 nota | 876 | 879 |

Nenhum erro/warning novo e nenhuma supressão. O gate global continua bloqueado pelos 105 V16 anteriores. As notas novas e removidas estão em lint-diff.json.gz: os grupos nominais de contextos exigem testes pertinentes por alternativa; não se presume cobertura total de um grupo. Padrões Option<Value::Length/Dir> decodificam o transporte tipado de estilos prescrito nos L0s. O executável global não pinado e seus nove achados adicionais continuam pendência do relatório predecessor.

## Correção de configuração da validação

A primeira rodada ampla sobre o código final teve 9028 aprovados e uma falha no alvo typst-infra: `p270_4_pdf_bytes_default_oklab_preserved_p268` exportou um PDF em 10:09:55Z e o seguinte em 10:09:56Z. Os dois têm 13392 bytes; diferem somente em quatro bytes das datas Info/XMP. Substituir apenas esses valores de data torna os arquivos inteiros iguais (timestamp-difference.json e os dois PDFs preservados). Não houve diferença de gradiente, fonte, stream ou geometria.

Faltou fixar o relógio de snapshots na configuração desta validação. O exportador e seu L0 já prescrevem CRYSTALLINE_PDF_FIXED_EPOCH antes de SOURCE_DATE_EPOCH. Foi repetido o alvo inteiro de infraestrutura com o override existente e TMPDIR/oracle explícitos: 1349 testes aprovados, sem mudar código nem expectativas. Os demais 7680 aprovados da mesma rodada final foram reaproveitados com prova de freeze idêntico. O total 9029 substitui o alvo falho, sem somar rodadas ou descartar o log original. finish-results.json registra a conclusão dos gates; gates.log/gates-resources.json conservam a interrupção original. A rodada anterior de 9026 também não entra na soma final.

## Checkpoint: alcance e suspeita preservada

O teste aprovado conserva o prefixo confirmado, elimina a tentativa rejeitada e restaura o recuo da continuação. A tentativa inicial direta, com current_line ainda não drenado e um flush entre capture/restore, perdeu DDDD e conservou FFFF indevidamente. Os buffers por comprimento antecedem este lote; a hipótese de defeito preexistente é inferência a verificar no caminho produtivo de Flush/float. Mudar a fixture para fronteira confirmada não resolve essa suspeita e não autoriza alegar rollback universal. Ver debug.log, iterations.md e proximos.md.

## RAM e proveniência

Todos os comandos pesados foram sequenciais no cgroup com MemoryMax 12 GiB, MemoryHigh 8 GiB, swap 0 e cargo jobs 1, debug/incremental desativados. Pico máximo de todas as tentativas deste lote: **5.247 GiB (5633675264 bytes)**. Nenhum OOM/kill. São picos deste fluxo de validação, não benchmark de RAM do documento nem prova de ganho de runtime.

HEAD **b1b486f3d42431f8ea360bc5712637ee1e3d4f6c**, working tree não commitado. A lista exata do diff herdado+atual, status e UTC acompanha cada medição; a diferença específica deste lote está no final-state. O freeze tem 2102 arquivos. O manifesto predecessor ff86014f88670c1de8adba526a85a76696be450ba78823f3ccf87ca54e52516a foi verificado nas 582 entradas contra artefatos imutáveis ou bytes iniciais preservados.

Oracle upstream pin **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore **cac836c07e4dec2a89219370428292393a3216b5**. As sete fontes consultadas coincidem com os objetos Git pinados, conforme oracle-sources.json.

- Vanilla SHA-256: `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
- CLI anterior: `2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`.
- CLI validado: `ce9981c27e72e389cb208884416cb10b1bbdd4c17803247d9570ecf7e1389a80`.
- Linter pin 12797073972e375d9ab078cf6750c8b30b71e2b9: SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`.

As duas fixtures de bibliografia também coincidem com os bytes do arquivo anterior à medição (fixtures-proof.json). A fonte, árvore, receita e hashes dos binários sustentam a proveniência; versão textual isolada não a substitui. [manifest.sha256](manifest.sha256) identifica o manifesto completo. Não houve commit, push ou release nesta etapa.
