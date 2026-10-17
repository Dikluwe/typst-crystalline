# Projeção do contador Page sem cópia e custo no watch

Data: 2026-10-13.

## Decisão

Tentativa rejeitada. Projetar o primeiro nível de Page sem clonar CounterRegistry reduziu pedidos do alocador, mas não melhorou a CPU do watch em duas janelas. O consumer e o L0 foram restaurados exatamente ao snapshot inicial; nenhum ganho novo produtivo foi mantido. A confirmação de paginação, expansões contextuais, layouts, callbacks e budgets continuam integrais.

## Hipótese e fronteira

A fonte inicial 01_core/src/compiler/introspect/from_tags/counter_updates.rs:288–292 copia o registry para toda ocorrência posicionada e aplica delta físico por Step. A leitura em :362–365 projeta somente o primeiro nível. 01_core/src/entities/counter_registry.rs:116–131 mostra que Step de nível 1 trunca outros níveis, preenche vazio com 1 e incrementa o primeiro. A variante projetava first-or-zero + delta nas ocorrências sem ação; ações mantinham o fold completo anterior. Era fórmula privada de L1, sem contrato público/default/fase nova, em fluxo contínuo ADR-0127.

O owner 00_nucleo/prompts/compiler/introspect/from_tags/counter_updates.md foi atualizado antes da implementação e ressellado isoladamente. O Núcleo Page permaneceu inalterado. Não houve reuso de Content, hash de closure, vtable ou memoização entre Worlds. Esta variante não reduziu a frequência de execuções nem saltou confirmação por igualdade de Page.

## Testes e alocações

O teste da projeção foi escrito antes do helper: RED compilatório por helper ausente em red-test.log e GREEN em green-test.log. O gate efetivo de custo, definido antes da mudança, exigia pelo menos 1.000 pedidos a menos no caso de 300 contextos; baseline falhou e candidato passou. O teste privado compara deltas 0/1/8/1600 sobre estados vazio, zero, simples e multinível, mantendo o estado causal que callbacks posteriores recebem. É uma sentinela de fórmula, não certificado de paridade por igualdade Rust.

| Contextos | Pedidos baseline | Pedidos candidato | Diferença |
|---|---:|---:|---:|
| 300 | 713.905 | 712.705 | -1.200 |
| 1.600 | 11.718.854 | 11.712.454 | -6.400 |

Três compilações por condição, CPU fixa, interpositor separado dos tempos nativos. Pedidos acumulados não são heap vivo nem RSS. Receipts alloc-red.json.gz e alloc-green.json.gz identificam binários, fixtures, probe, estado inicial e UTC.

## Watch nativo

Processos antes/depois separados, três rodadas com ordem alternada, quatro eventos de aquecimento excluídos e doze eventos medidos por processo: mesmos bytes, palavra e estrutura. São 36 amostras por tamanho/motor em cada janela disponível. Mesma CPU, Rayon 1, epoch/fontes/stack 64 MiB. Guardas de processos recusam builds/testes/debuggers concorrentes; nenhum esteve ativo durante os tempos. CPU vem da soma schedstat das threads; latência inclui espera/publicação e não é custo exclusivo do compilador.

| Janela | Contextos base | CPU baseline ms | CPU candidato ms | RSS baseline/candidato KiB |
|---|---:|---:|---:|---:|
| Primeira | 300 | 80,98 | 83,43 | 49.866/50.272 |
| Primeira | 1.600 | 899,77 | 905,04 | 138.324/138.120 |
| Repetição | 300 | 81,07 | 84,04 | 50.212/50.030 |

O caso de 300 ficou mais lento nas três rodadas de cada janela. Em 1.600, a primeira janela alternou o sinal por rodada e terminou sem ganho. Estes resultados bastam para rejeitar a variante; a causa microarquitetural da piora não foi medida.

UTC da primeira janela: 2026-10-13T16:36:30.534974+00:00 a 2026-10-13T16:39:30.987329+00:00. Repetição: 2026-10-13T16:39:55.346133+00:00 a 2026-10-13T16:40:42.972419+00:00. watch-first.json.gz e watch-repeat.json.gz guardam argv, CPU, ambiente, fixtures, amostras e hashes dos scripts/binários.

## Linguagem e render

qualification.json.gz contém 18 fontes PDF distintas em 20 entradas e dois controles de erro. Entre baseline e candidato, texto, palavras, páginas, dimensões, caixas e raster 36 dpi permaneceram iguais; diagnósticos dos erros também. Incluem estados vazios/multinível, updates tardios, callback Page, here/posição, queries, numbering variável unário/binário, referências, estilos, margens, contextos aninhados e edições 300/320 e 1600/1620 com Alpha/Bravo. Os testes infra existentes revalidam snapshots, callbacks, retenção e HTML. Core: 6.684 passaram/5 ignorados; infra: 1.285 passaram/2 ignorados, nos logs core.log e infra.log.

Vanilla pinado conserva texto/páginas/raster nos casos contextuais usados para tempo; as caixas mantêm diferenças numéricas pequenas registradas no receipt. Não houve comparação de CPU dos motores nesta etapa. Posição e margem têm diferenças anteriores de texto com vanilla; numbering conserva texto mas difere em render. Não fechar essas diferenças por invariância antes/depois.

exploratory.json.gz registra uma divergência prévia adicional: Page.update((a,b)=>...) após update((7,2)) na mesma página compila nos dois Tekt, mas vanilla acusa argumento b ausente. O numbering estritamente binário falha nos três motores quando uma referência demanda a vista unária; a fixture qualificada usa função variádica. Essas observações não foram convertidas em alterações de comportamento neste ensaio.

## Proveniência e restauração

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. Os receipts before/candidate/final.json.gz guardam UTC, status com todos os untracked, diff HEAD --stat, hashes completos e texto dos dois owners. O baseline foi comparado ao final de watch-context-current-2026-10-10: todos os 966 Rust e 1.039 L0/núcleos coincidiam, conforme provenance.json. Estado inicial UTC 2026-10-13T16:30:59.689674+00:00, candidato UTC 2026-10-13T16:35:09.831981+00:00 e final UTC 2026-10-13T16:42:56.723355+00:00.

Release baseline e final SHA-256 fd758cf69301c297f6123f042ab19cc2de41e38a7f4af7fc351987728fa18655. Candidato SHA-256 4259e9fd55a73b8c617d370449b1479ce466721ac7aa3b48936648f190f645bc. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, árvore/receita ratificadas P1489. A receita candidata foi cargo build --release --locked --offline -p typst-wiring; versões rustc/cargo e hashes de manifests estão em provenance.json. Fontes exatas em candidate.json.gz permitem remontar o ensaio.

Somente o consumer/L0 desta tentativa e o release foram restaurados a partir do snapshot próprio, após conferir o candidato. verification.json comprova todos os hashes iniciais produtivos/L0 e ambos os binários iguais ao final e 1.268 fontes/configurações sem mudança. A árvore preexistente não foi resetada, commitada nem ressellada globalmente. Build final passou; crystalline-lint terminou com exit 0, sem erros e com avisos/informações históricos em lint-final.log. Não declarar zero avisos.

Receipts e scripts ficam em 00_nucleo/diagnosticos/page-observation-fold-2026-10-13/. watch-first.py preserva o script exato da primeira janela; watch.py com argumento repeat --small executa a repetição usando binários congelados em /tmp/tekt-page-fold/before e after. alloc.py red/green e qualify.py reproduzem coletas e sobrescrevem seus recibos; preservá-los antes de repetir. Os scripts auxiliares referenciados têm hashes nos receipts.

A próxima investigação precisa localizar o custo restante das execuções e clones de introspector na rota ordinária. As provas desta tentativa autorizam rejeitar a projeção como otimização produtiva; não autorizam reutilizar contribuições ou reduzir a validação causal.
