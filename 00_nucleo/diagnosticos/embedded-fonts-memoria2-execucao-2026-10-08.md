# Memória — prioridade 2: compartilhar bytes das fontes embutidas

2026-10-08. Autorização: dono pediu “faça o 2”. Correção interna em fluxo contínuo conforme ADR-0127: L0 primeiro, testes antes da implementação, resselo e revalidação. Nenhuma assinatura, campo público, variante Font, flag, default ou fase do pipeline foi alterada.

## Resultado conservado

`FontSlot` retém privadamente `Option<Arc<Vec<u8>>>`. `new_embedded` transfere o Vec ao Arc, sem validar antecipadamente. `source_bytes` empresta o slice. `get` valida primeiro a face solicitada; para fonte simples/não extraída, conserva a validação do índice zero e devolve a variante `Font::Vec` já existente com clone somente do Arc. Para coleção suportada, reconstrói e valida bytes independentes. Sucesso/falha continuam no OnceLock, idempotentes por slot. Fontes de disco/mmap e extração de coleções mantêm seus caminhos. A fonte retornada continua válida após descartar o slot.

O carregador do catálogo obtém metadados diretamente do slice estático de typst_assets e só então copia uma vez para o Vec transferido ao slot. Remove a cópia temporária por iteração. Preserva ordem, classificação e pareamento slot/book. Os bytes estáticos do binário e o único buffer próprio de cada slot ainda existem: esta correção não torna os assets zero-copy nem elimina as cópias exigidas pelo export/subsetting.

## Medição antes da decisão e proveniência

`fonts.rs:32,119,158-163` e `embedded_fonts.rs:92-96` no estado original demonstravam Vec próprio no slot, clone persistente no get e clone temporário no carregamento. O tipo Font já oferecia Arc<Vec<u8>>, em `entities/world_types.rs:40-61`; não foi necessário criar API L1.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. `embedded-fonts-memoria2-2026-10-08/before.json.gz`, `red-state.json.gz` e `candidate.json.gz` incluem textos completos de código/L0/config, UTC, status com untracked e `git diff HEAD --stat`. A extensa árvore herdada foi preservada. Somente estes quatro arquivos produtivos/L0 mudaram:

- `03_infra/src/fonts.rs` ↔ `00_nucleo/prompts/infra/fonts.md`
- `03_infra/src/embedded_fonts.rs` ↔ `00_nucleo/prompts/infra/embedded_fonts.md`

Scripts, estados e relatórios são diagnósticos adicionais. Build anterior (já inclui memória 1): binário congelado `/tmp/typst-before-embedded-fonts-memory2`, SHA-256 `0600523559f45dca81cf8b8c5f41ab01ab878fd412dd7053282ed767fdd91867`, receita no relatório font-binding-memoria1. Final: `target/release/typst`, SHA-256 `4686d15bd91c1b9cd9ca4605fdec16b8c7babd59f4e125276ebcaba87969379c`. Builds Cargo offline dev/release; logs conservados.

Vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore/receita P1489 e binário de `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Não usar apenas string de versão como proveniência.

## Testes, linhagem e controles

Uma primeira redação do teste usou assinatura incorreta de extract_coverage; erro de compilação preservado em `test-compilation-first.log`, não contado como RED. Após corrigir só o teste, `red.log` e `red-state.json.gz` mostram: bytes/metadados/cobertura já iguais, mas ponteiro do buffer carregado diferente do original (testemunha de duplicação); 1 falhou e o controle de coleção passou. Identidade física é prova de custo interno, não paridade de linguagem.

Os dois testes novos cobrem compartilhamento, metadados/cobertura, validação lazy, vida após descarte, coleção válida no índice 1, preservação de source_bytes e rejeição repetida do índice 2 inválido. A alternativa inicial passou esses testes; na representação final, a suite completa passou novamente: 1252 passed, 2 ignored (`infra-full.log`, RUST_MIN_STACK=67108864). Inclui regressões preexistentes de índice inválido, mmap, classificação, binding/variantes/eixos e SVG/PNG. Testes host-condicionais não representam prova universal de presença de fontes CJK.

Ownership/núcleos foram validados antes do trabalho; resselo apenas dos dois pares, hashes em `seals.json`. Builds dev/release passaram. `crystalline-lint .`: 0 errors, 535 warnings, 1640 infos, passivo herdado inalterado; isso não significa zero violations globais.

Sete documentos antes/depois: texto com layout, sequência de palavras, páginas/dimensões, estilos, bounding boxes (delta 0) e raster a 36 dpi idênticos (`qualification.json.gz`). Contra vanilla, diferenças herdadas continuam: context/denso delta vertical ~0,658 pt, alias diferença de whitespace na extração; raster não universalmente igual. Não declarar paridade global, nem usar normalização de whitespace para fechá-la.

## Alternativa avaliada e descartada

O primeiro armazenamento privado usou Option<Font>, binário `a4d93e4410e329ad7dd34fcef61f9114c863b3a4a96aba98f178b63bcc6e7953`, congelado em `/tmp/typst-embedded-fonts-memory2-attempt-font-container`. Estado, duas baterias, controles e testes em `attempt-font-container/`. A repetição confirmou ~−1,54% RSS na tabela local, mas +0,91/+1,00/+2,01% de tempo em context/simples/denso. Foi substituído por Arc<Vec<u8>> direto, atualizando L0 antes do código. Hipótese: retirar o despacho do enum do armazenamento de origem pode diminuir custo; a causa não foi demonstrada por perfil de alocações/CPU. O que se conserva é a nova medição, não a hipótese causal.

Uma leitura antecipada de resultados dessa primeira alternativa falhou porque o JSON ainda não estava publicado; os resultados só foram processados após o fechamento. Essa tentativa fica exploratória e arquivada; a bateria final foi executada e encerrada antes de análise/qualificação, sem sobreposição. Scripts arquivados preservam os caminhos da execução original; comandos e identidades constam no JSON.

## Bateria final

UTC 2026-10-08 22:33:43.068595–22:34:33.635026 (19:33–19:34 em São Paulo). 378 amostras: sete casos x três blocos x seis trios x três motores, seis permutações de ordem e uma rodada de aquecimento por motor/caso/bloco. Processos CLI frescos, filesystem aquecido, CPU 4, Rayon 1, vanilla jobs 1, epoch 0, locale e fontes comuns. `%M` de `/usr/bin/time` mede pico RSS, não volume cumulativo de alocações; wall inclui wrapper idêntico. Fontes, fixtures, config, código, L0 e binários com fingerprints validados antes/depois (`bench.json.gz`, `protocol.md`, `measure.py`). Nada de build/test/linter/profiler concorrente.

| Caso | RSS antes → final (KiB, medianas) | Δ RSS pareado | Δ tempo pareado |
|---|---:|---:|---:|
| Context | 44360 → 44266 | -0.39% | +0.64% |
| Imagem | 19360 → 19348 | -0.08% | -0.55% |
| Texto simples | 42802 → 42904 | -0.01% | +0.63% |
| Shapes | 20614 → 20456 | -0.67% | -0.00% |
| Tabela alias | 45512 → 45234 | -0.71% | +0.56% |
| Tabela local | 46450 → 45796 | -1.62% | -0.02% |
| Texto denso | 45520 → 45018 | -0.99% | +0.52% |

Mediana das razões pareadas não é razão das medianas. `summary.json` contém cada motor e blocos. Quedas RSS nos três blocos: alias −0,74/−0,24/−0,80%; tabela local −1,90/−1,06/−1,27%; denso −0,84/−1,02/−0,61%. Não certificar economia relevante em simples/context/gráficos.

Custo de tempo final: ~+0,52 a +0,64% nos quatro casos simples/context/alias/denso, com sinais positivos nos três blocos; tabela local ~−0,02%, sinais mistos. Portanto não afirmar ausência de regressão ou ganho de velocidade: conservar a mudança por economia de memória, com pequeno custo de tempo medido. A redução de custo em relação à alternativa não é prova de causalidade do enum.

Os picos medianos em texto ainda ficam ~41,9–44,7 MiB, contra ~29,3–31,0 MiB do vanilla nesta bateria. A lacuna geral de memória continua. Não somar esses percentuais à prioridade 1 ou a baterias antigas. Esta rodada não mede watch nem disco frio.

Decisão: conservar Arc<Vec<u8>> e transferência única ao slot; duplicação eliminada em teste causal, resultados de produto preservados e economia pequena/repetida nos três casos acima, com custo de tempo explicitado. Próximo passo útil é medir retenção por fase para localizar o verdadeiro pico, antes de novas alterações de representação.

`completion.json` verifica estado produtivo/L0/config e binários finais após as medições e registra hashes dos artefatos. Nenhum commit foi criado.
