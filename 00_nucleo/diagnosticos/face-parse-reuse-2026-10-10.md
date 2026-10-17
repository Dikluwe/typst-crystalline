# Reaproveitamento de Face na aquisição do catálogo — 2026-10-10

## Estado e proveniência

Item 2 da lista de candidatos: parsing duplicado de Face. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. `face-parse-reuse-2026-10-10/before.json.gz` guarda UTC, status completo, diffstat, hashes dos 966 consumers e textos integrais dos três pares código/L0. Recibos de cada medição guardam HEAD/dirty/UTC, argv, scripts, corpus, fontes/libs/binários e hashes. Não houve commit, reset ou resselo global.

Baseline com o cmap aceito: SHA-256 `584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba`, congelado em `/tmp/tekt-face-parse-benchmark/old/typst`. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, árvore/receita ratificadas em P1489.

## Medição anterior à decisão

Fontes recebidas: `03_infra/src/fonts.rs:371-373` parseia Face para metadata sem coverage. `03_infra/src/fontdb.rs:26-28` chama esse wrapper e parseia novamente os mesmos bytes/índice para coverage. `03_infra/src/fontdb/selective.rs:21-23` repete o padrão na projeção sfnt ao índice zero. Uma aquisição bem-sucedida tem duas chamadas completas de Face::parse. O `fontdb` 0.23.0 faz discovery através de RawFace (`src/lib.rs:1034-1040`), uma etapa distinta: não substituir seus DTOs por FontInfo nem inferir que desaparece esse custo.

Vanilla pinado separa FontInfo::new(bytes,index) e from_ttf(face) (`lab/typst-original/crates/typst-library/src/text/font/info.rs:49-64`). Isso é evidência de que metadata pode ser obtida da face já parseada, não obrigação de copiar sua mecânica. Paridade continua na linguagem; residuais anteriores (ex.: primeiro nome decodificável sem ID1) ficam fora desta correção.

`prototype-probe.json.gz`: 1.140 faces instaladas/embutidas/fixtures, metadados completos e coverage iguais ao extrator recebido; wrapper lazy também igual. Bytes inválidos e índices fora de alcance preservam None. Medianas isoladas de aquisição em memória: 9,1221232 → 8,6135831 ms por corpus. São tempos de parsing/metadata/coverage sobre bytes já carregados, sem discovery/I/O/layout/export; não alegar ganho de CLI a partir deles.

`attempt-separate-helper/current-probe.json.gz`: primeira materialização, mesmas 1.140 faces e igualdade, medianas 9,26702865 → 8,75477315 ms; medianas de ratios pareados por rodada −6,440%, −5,133%, −4,218%. Compilado com rustc -O e dependências release registradas. Scripts, fontes copiadas, receita e todos os samples estão nos recibos.

## Classificação e implementação candidata

Reaproveitar uma Face emprestada entre metadata e coverage da mesma aquisição é correção interna mecânica em L3. Não muda assinatura pública, contrato L1, padrão do produto, fase de pipeline nem política lazy/eager. Helper crate-private é interno ao crate. L0 dos três owners foi editado primeiro; linhagem foi ressellada somente nesses owners.

- `fonts.rs`: corpo recebido de metadata passa a helper privado font_info_from_face; wrapper público font_info_from_bytes mantém parse/None e coverage vazia. Helper crate-private font_info_with_coverage parseia uma vez, extrai metadata e chama o extrator de coverage na mesma face.
- `fontdb.rs`: callback existente mantém assinatura e delega ao helper eager. File seletivo, fallback, SharedFile com bytes retidos e filtro/pairing/ordem permanecem.
- `fontdb/selective.rs`: usa helper no índice zero da projeção. Projeção/limites/identidade/I/O/fallback ficam iguais. Testes de referência conservam o callback legado com duas parses, independente do helper novo.

A Face e o buffer não escapam nem são cacheados; retorno continua FontInfo owned. Não mudou o adapter cmap aceito, FontBook, composição, slots, embedded, descoberta ou carregamento final para shaping/export.

## Testes e gates

Dois testes inline foram escritos antes do código: controles de metadata lazy/eager/cobertura independente, família/variante/flags e bytes/índice inválidos. Baseline sem helper usa callback recebido; essas propriedades já passavam. Após implementação, testes passam a chamar o helper sob teste, conservando referência e expectativas independentes. Não apresentar esses testes como RED de parsing.

O RED de custo foi runtime separado: instrumentação somente no executável diagnóstico, com cópia dos corpos de metadata e aquisição. Uma fonte válida auditada executa duas parses no baseline, limite uma; `parse-gate-red.json` falhou em execução. `current-counted-probe.json.gz` e `parse-gate-green.json` registram uma parse no materializado. Timings dos executáveis instrumentados não são usados como ganho. Contagem refere-se à aquisição metadata/coverage bem-sucedida, não a discovery, final loading ou uma tentativa seletiva seguida de fallback após mudança de identidade.

Suite infra: 1.285 passed, dois ignored, zero failed. Builds dev/release e crystalline-lint . passaram (exit0). Warnings históricos continuam; nenhum V5/V15/V26 dos owners afetados. Testes existentes exercitam File/SharedFile/fallback, projeções TTF/OTF/TTC e bounds, lazy/session e pairing.

34 documentos (25 tabelas/controles + nove escopos, incluindo multiscript) conservaram texto, páginas, tamanho, bbox (delta0pt) e raster a36dpi entre baseline/candidato. Seis consultas table.len deram1 nos três engines. Comparação com vanilla é controle de regressão; casos com diferenças preexistentes de trabalho/linguagem não são prova de performance relativa ao oráculo.

## CLI nativa

`attempt-separate-helper/measurements-repeat.json.gz`: primeira materialização, três rodadas, oito pares por caso, ordem alternada/embaralhada, aquecimento excluído, CPU fixa e um thread. Sem interposer nem instrumentação. Delta de tempo é mediana dos ratios pareados; RSS são medianas absolutas em KiB.

| Caso | Delta tempo | Rodadas de tempo | RSS antes → candidato (KiB) |
|---|---:|---|---:|
| plain | +0.176% | -0.210%, +0.941%, +0.300% | 27322 → 27298 |
| unused-font | +0.295% | -2.702%, +1.613%, +0.141% | 29076 → 29050 |
| unused-style | +0.175% | -0.257%, +0.482%, -0.283% | 27874 → 27894 |
| default-scopes | -0.426% | -0.415%, -0.683%, -0.335% | 53696 → 53782 |
| local-set-400 | -0.123% | -0.379%, -0.712%, +0.087% | 31200 → 31116 |
| local-set-1600 | +0.032% | -0.427%, +0.715%, +0.356% | 42274 → 42324 |
| no-fill-1600 | -0.601% | -1.057%, -0.611%, -0.088% | 38056 → 38006 |
| inline-set-1600 | +0.908% | +0.757%, +1.616%, +0.109% | 47772 → 47662 |
| families-1600 | +0.552% | +0.776%, +0.699%, -0.351% | 38706 → 38822 |
| variants-1600 | -0.078% | -0.053%, -0.554%, +0.132% | 293076 → 292902 |

Repetição independente motivada por inline-set-1600 positivo nas três rodadas e families-1600 positivo em duas; plain/default-scopes/no-fill acompanham como controles. A comparação só estabelece custo contra o baseline Tekt com trabalho equivalente; não extrapolar para vanilla ou RSS total de fontes.

### Repetição da primeira materialização

`attempt-separate-helper/measurements-risk-repeat.json.gz`:

| Caso | Tempo pareado | Rodadas | RSS antes → candidato (KiB) |
|---|---:|---|---:|
| plain | +0.091% | -0.509%, +0.048%, +0.314% | 27224 → 27324 |
| default-scopes | +0.505% | +1.164%, -0.901%, +0.593% | 53584 → 53826 |
| inline-set-1600 | +1.678% | +3.034%, -0.028%, +1.880% | 47708 → 47708 |
| families-1600 | -0.453% | -1.843%, -0.378%, -0.196% | 38790 → 38736 |
| no-fill-1600 | +0.899% | +1.697%, +1.098%, -0.210% | 38048 → 38108 |

A primeira materialização não foi aceita: ganho dos controles não se repetiu, inline permaneceu regressivo. Candidato SHA7247b66d4a89660715f8c588ee746079b31df4e1fd7641351e6e98c674ec9c42 arquivado com fontes/L0/hashes/samples.

## Refinamento: expansão do corpo privado

Disassembly/nm do primeiro candidato mantém font_info_from_face separado, chamado também pelo eager e embedded. Isso não estabelece causalidade dos deltas nativos. Hipótese registrada primeiro no L0: inline(always) do corpo privado pode remover a chamada e manter uma única parse. Nenhuma fórmula, assinatura pública ou política é alterada. O novo binário não conserva o símbolo de corpo separado (a closure auxiliar ainda pode ter símbolo). Duas unit tests focadas passaram, builds/lint passaram; corpus/gates/documents repetidos e novo benchmark nativo são necessários antes da decisão.

## Resultado da variante com expansão estática

O corpus materializado preservou 1.140 faces/metadados/cobertura; runtime GREEN uma parse, os mesmos 34 documentos e seis queries passaram. Recibos atuais correspondem à segunda variante; arquivos da primeira ficam exclusivamente em attempt-separate-helper. A primeira medição usou bytes completos em memória, enquanto o caminho File seletivo da CLI usa projeção menor: não extrapolar o percentual isolado.

### Janela principal

`measurements-repeat.json.gz`; três rodadas, oito pares por caso.

| Caso | Delta tempo pareado | Rodadas | RSS antes → candidato (KiB) |
|---|---:|---|---:|
| plain | +0.006% | +0.250%, -0.607%, +0.103% | 27306 → 27286 |
| unused-font | +0.082% | +0.033%, +0.082%, -0.026% | 29126 → 29096 |
| unused-style | -0.157% | -0.107%, -0.169%, -0.157% | 27782 → 27886 |
| default-scopes | -0.441% | -0.628%, -0.373%, -0.494% | 53780 → 53818 |
| local-set-400 | -0.558% | +0.067%, -1.034%, -0.626% | 31182 → 31138 |
| local-set-1600 | +0.482% | +0.799%, -0.661%, +0.482% | 42342 → 42104 |
| no-fill-1600 | -0.253% | -0.132%, -0.376%, -0.344% | 38066 → 38010 |
| inline-set-1600 | -0.162% | -0.392%, -0.347%, +0.346% | 47680 → 47818 |
| families-1600 | +0.691% | +1.221%, +0.525%, -0.157% | 38760 → 38664 |
| variants-1600 | -0.329% | -0.434%, +0.307%, -0.281% | 293002 → 293006 |

### Repetição dos riscos e controles

`measurements-risk-repeat.json.gz`; três rodadas, oito pares por caso.

| Caso | Delta tempo pareado | Rodadas | RSS antes → candidato (KiB) |
|---|---:|---|---:|
| plain | -0.544% | -0.544%, -0.603%, -0.423% | 27332 → 27254 |
| unused-style | +0.005% | -0.153%, +0.532%, -0.088% | 27876 → 27856 |
| default-scopes | -0.244% | -0.148%, -0.290%, +0.439% | 53762 → 53758 |
| inline-set-1600 | -1.180% | -2.282%, +0.164%, -2.253% | 47808 → 47732 |
| families-1600 | +0.616% | +0.651%, +0.608%, -0.605% | 38634 → 38718 |
| no-fill-1600 | +0.002% | +0.519%, -0.195%, -0.288% | 38062 → 38202 |
| local-set-1600 | -0.511% | +0.690%, -1.695%, +0.262% | 42284 → 42218 |

## Decisão: duas variantes rejeitadas

A segunda variante elimina a regressão inline da primeira, mas permanece um custo em families-1600: +0,691% na janela principal e +0,616% na repetição (quatro de seis medianas de rodada positivas). Plain foi neutro na janela principal e −0,544% na repetição; default-scopes −0,441%/−0,244%; unused-style −0,157%/+0,005%; no-fill −0,253%/+0,002%. Não são uma melhoria homogênea nem autorização para alegar ganho universal; as pequenas diferenças de RSS são mistas e não estabelecem redução persistente do custo das fontes. Também não demonstram causalidade específica por tamanho/chamada/cache de código.

Rejeitada incorporação de ambas as variantes. A redução mecânica de duas parses para uma e a igualdade semântica foram demonstradas, mas o critério de aceitação no processo completo não se sustentou. Não mudar retrospectivamente o critério para aceitar somente o resultado isolado. Restaurar byte a byte os três consumers e seus três L0 do início deste item; cmap aceito e demais consumers permanecem intocados. Candidato final SHA e fontes/L0/logs/patches ficam em candidate.json.gz; a primeira variante em attempt-separate-helper. Sem commit.

Próxima investigação orientada por estas medições: perfil do processo completo e custo real da aquisição sobre bytes projetados, separando discovery RawFace, aquisição typed Face, cmap e layout. Não repetir outra divisão em helpers ou anotação de inlining sem atribuir o custo antes. Validação/carregamento final de FontSlot é outro lifetime e permaneceu fora deste experimento; não alegar que se mediu ou retirou todo parsing repetido do compilador.

Restauração final verificada: nenhum dos 966 consumers produtivos difere do estado inicial deste item; os seis arquivos código/L0 guardados no snapshot estão byte a byte iguais. Builds dev/release e lint após restauração passaram (exit0), warnings históricos permanecem. SHA-256 release recuperado exatamente584d5c24c243d52c3f80999fe96241c01c98cf69c33eb4f3cb00144d5dc5a1ba. Candidato com expansão estática:a121ace7a251468c801dd60db98646c49c88a48081fd34d649d786946269a5c8. final.json.gz guarda estado completo, hashes e logs; os patches do candidato foram preservados.
