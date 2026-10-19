# Reuso de payloads Unicode cmap — 2026-10-10

## Decisão

**Correção incorporada.** O extrator privado reconhece payloads nativos Unicode já extraídos com sucesso dentro da mesma Face/chamada. Ganho repetido de CPU e alocações na maioria dos casos; **não há redução comprovada de RSS**. A decisão aceita o custo observado em texto simples: cerca de420KiB a mais entre as medianas de pico, ou +1,472% na mediana das razões pareadas da repetição, para tempo −2,542%. Esta troca não fecha o alvo de custo fixo/residente das fontes.

O caso variants permanece aberto: medianas pareadas de tempo +0,059%, +0,830% e +0,361% nas três janelas, com sinais mistos na última. Não alegamos melhora desse caso. A rodada instável da tabela local é preservada, não excluída; a verificação dirigida confirmou ganho mediano −1,907% com três rodadas negativas. Não certificamos a magnitude exata da janela instável.

## Medição antes da decisão

Fonte recebida `03_infra/src/fonts/coverage.rs:76–105`: cada registro Unicode era extraído mesmo apontando ao mesmo offset/formato bruto de outra subtabela da mesma cmap. Auditoria/protótipo em `prototype-probe.json.gz`: **1.140 faces**, **3.673 registros nativos**, **1.819 repetições de payload**, **1.111 faces com duplicação**; máximo atual de dois payloads nativos únicos por face. Intervalos intermediários **1.533.618→767.936**. Cobertura igual ao extrator recebido e ao callback legado para todas as faces.

Protótipo isolado:17,7808→11,8123ms por corpus; razões por rodada −33,37%, −34,18%, −33,92%. Consumer materializado, `current-probe.json.gz`:18,1597→12,1293ms. Esses timers são da extração/união com faces já parseadas em um executable Rust diagnóstico, módulos compilados com-O e dependências existentes de `target/debug/deps` com hashes/receita preservados. Não incluem abrir fontes/construir catálogo e não estimam ganho da CLI release.

## Correção

- Lista local de até oito chaves `(offset relativo à cmap, formato4/12/13)` em array na pilha. Sem heap para a lista e sem retenção entre Faces/tentativas/watch.
- Continuar `get(i)` tipado em ordem, terminar no primeiro inválido e filtrar não Unicode antes da consulta.
- Verificar acesso/formato bruto, depois dispensar apenas a extração nativa que já terminou com sucesso para aquela chave.
- Entradas diferentes com bytes iguais, formatos callback-only e tentativas parciais/falhas continuam processadas. Quando a lista enche, os demais payloads são processados normalmente, sem memorização.
- União/codificação permanecem em Coverage L1. Facade público, descoberta, catálogo, ordenação de slots, fontes finais, flags e fases permanecem os mesmos contratos. Apenas o consumer L3 `03_infra/src/fonts/coverage.rs` foi alterado; o L0 proprietário foi atualizado antes do código e resselo restrito executado. Nenhuma mudança produtiva em `fontdb/selective.rs`.

Cobertura como conjunto/membership/seleção é língua; eliminar extrações repetidas da mesma origem imutável é mecânica. Não inferimos intenção do vanilla nem copiamos sua execução. Fluxo contínuo ADR0127, sem alteração pública/default/fase/compatibilidade.

## Testes e controles

- **RED de execução**: dois novos testes falharam no baseline, mostrando intervalos duplicados e a ausência da regra de limite/misses; outros onze passaram. Recibo `red-validation.json.gz` contém a fonte exata naquele momento, L0, HEAD, dirty tree, UTC, comando/exit e log.
- **GREEN**:13 testes do extrator,1.283 de infra (2 ignorados),6.682 de core (5 ignorados),90 de shell; zero falhas.
- Oráculo independente BTreeSet e callback legado: aliases consecutivos/separados por outro formato, payloads iguais em offsets distintos, formatos callback-only, registro inválido entre aliases, alias não Unicode, mais de oito únicos com duplicatas posteriores, truncamento/rollback/overflow e cmap ausente.
- Corpus materializado:1.140 faces instaladas/embutidas/fixtures, cobertura igual ao recebido e ao legado. Hashes de todas as fontes conferidos novamente após benchmarks.
- **34 documentos** antes/depois: textos, palavras, páginas, tamanho, coordenadas e raster36dpi iguais. Inclui controle multiescrita, matemática e tabela; stderr vazio nos três engines nesse controle. Isso preserva a saída recebida, não certifica nova cobertura de idiomas ou resolve divergências prévias com vanilla.
- **Seis consultas** `query(table).len()` nos três engines, todas1. Fonte/normalização e seleção/fallback existentes preservadas.
- `cargo build`, `cargo build --release`, `crystalline-lint .`: exit0. Nenhum V5/V15/V26 do owner afetado. V16/V19 preexistentes desse adapter e avisos em arquivos históricos continuam; não declarar zero avisos global.

## Alocações e tempo/RSS

Interposer separado, três execuções por caso: **−2.157 chamadas de alocação** e **−24.299.904 bytes solicitados acumulados (~23,17MiB)** em cada um dos seis casos. Valores exatos iguais nas três repetições. Bytes acumulados não são RSS nem memória simultaneamente viva. Tempo instrumentado não foi usado como prova de desempenho.

CLI sem interposer: primeira janela três rodadas×quatro pares para seis casos com oracle e quatro controles; repetição três rodadas×oito pares para todos os dez; terceira verificação dirigida três rodadas×oito pares somente em local-set-1600/variants-1600. Warmups excluídos, ordem embaralhada, CPU fixada, RAYON1, stack64MiB, binários antes/depois congelados com basename/caminhos de comprimentos iguais.

Tabela: mediana das razões **pareadas** candidato/baseline da repetição; não a razão entre medianas agregadas.

| Caso | Tempo | Pico RSS |
|---|---:|---:|
| plain | -2.542% | +1.472% |
| unused-font | -2.384% | +0.215% |
| unused-style | -5.650% | +0.946% |
| default-scopes | -3.199% | -0.022% |
| local-set-400 | -1.882% | +0.315% |
| local-set-1600 | -1.226% | -0.070% |
| no-fill-1600 | -1.704% | +0.169% |
| inline-set-1600 | -2.255% | -0.100% |
| families-1600 | -2.465% | +0.089% |
| variants-1600 | +0.830% | -0.002% |

Verificação dirigida: local-set-1600 **−1,907% tempo / +0,257% RSS**; variantes **+0,361% tempo / +0,015% RSS**, com tempo por rodada +0,683%, −0,418%, +0,386%. A rodada da repetição geral local-set-1600 teve +7,007%; preservamos a amostra e classificamos aquela estimativa como instável, em vez de removê-la ou concluir a partir dela.

Conclusões por caso: redução de tempo repetida em plain, local400, no-fill, inline, famílias e estilos não usados; memória continua sem ganho. Variantes não melhoraram. A comparação com vanilla é controle separado: os workloads de variants têm divergências anteriores e seus tempos não provam trabalho equivalente.

## Proveniência e entrega

- HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitada**. Lista completa de arquivos modificados/não rastreados e `git diff HEAD --stat` em cada recibo; fonte integral/L0 e hashes dos966 consumers em `before.json.gz` e `final-validation.json.gz`.
- Repetição geral UTC: 2026-10-10T11:16:20.617624+00:00 → 2026-10-10T11:18:09.934820+00:00.
- Verificação dirigida UTC: 2026-10-10T11:19:25.119246+00:00 → 2026-10-10T11:20:16.020162+00:00.
- Baseline SHA-256 `90ab169f7d583f948613f6b1f6694d9b144f7402f44406b91fdd4687981fb335`.
- Release entregue SHA-256 `584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba`. Idêntico ao candidato congelado usado nos testes/benchmarks.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; árvore/receita ratificadas em P1489. String de versão não usada como proveniência.
- Recipes/argv/ambiente/fontes/fixtures/scripts/binários/CPU/UTC/todas as amostras nos recibos. `summary.json` e `summarize.py` reproduzem as razões por rodada. `coverage.rs.patch`, `coverage.md.patch`, `seal.json` e `decision.json` descrevem o delta e a decisão.
- Todos os demais consumers têm hashes exatamente iguais ao snapshot recebido. Sem commit, reset, alteração do oráculo ou mutação do pacote anterior de post.

Próximo alvo de memória: medir retenção do alocador durante/depois da criação do catálogo e separar payload vivo, capacidade de buffers e páginas residentes. Menos pedidos de alocação por si só não explica os420KiB adicionais medidos em texto simples; a causa não foi isolada neste pacote.
