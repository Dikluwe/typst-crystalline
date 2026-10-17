# Travessia única: retirada dos pré-exames — 2026-10-10

## Resultado

Mantida a retirada de `may_need_finish_occurrences` e `may_need_page_column_wrap`, inclusive seus helpers privados. `finish_occurrences` e `wrap_page_columns` usam diretamente a travessia que propaga ausência explícita de mudança. Enum, callbacks, localização, transformação de colunas e assinaturas permanecem. O teste local de tabelas/colunas verifica o comportamento da operação, sem depender do helper removido.

O benefício medido é pequeno: **blocks-400 −1,21%** de tempo pareado, com mediana negativa em cada uma das seis rodadas. Os seis casos conservaram a quantidade de pedidos de alocação. Os demais resultados não sustentam uma melhoria geral. Não há redução de memória residente certificada.

## Medição antes da decisão

A decisão veio após as duas coletas e os controles. `content-preflight-removal-2026-10-10/acceptance.json` fixou o critério antes da primeira coleta de tempo: benefício de pelo menos 1% com rodadas negativas, nenhum caso com regressão robusta de pelo menos 1% e nenhum aumento nas alocações. A confirmação repetiu blocos, matemática e os dois controles; o gate conjunto só aceitou `blocks-400`.

| Caso | Tempo pareado conjunto | Rodadas | Pedidos glibc antes/depois |
|---|---:|---:|---:|
| blocks-400 | −1,21% | 6 negativas | 602.007 / 602.007 |
| semantic-1600 | −0,73% | 3, sinais mistos | 1.313.003 / 1.313.003 |
| math-400 | −0,88% | 6, sinais mistos | 921.556 / 921.556 |
| matrix-400 | +0,45% | 3, sinais mistos | 1.250.189 / 1.250.189 |
| local-set-1600 | −0,04% | 6, sinais mistos | 418.092 / 418.092 |
| no-fill-1600 | −0,41% | 6, sinais mistos | 342.821 / 342.821 |

Blocos melhoraram 1,68% na primeira coleta e 0,49% na confirmação. A mediana conjunta usa os 24 pares individuais, não a média dessas duas porcentagens. A diferença constante de 159 bytes solicitados por processo no interpositor não foi atribuída à transformação. O interpositor mede pedidos cumulativos, não heap vivo.

Inferência de engenharia: remover a descida emprestada elimina trabalho redundante; o resultado de tempo é compatível com essa hipótese, mas não isola ciclos por helper nem demonstra que cada retirada beneficia separadamente todos os documentos. Nos caminhos sem mudança, o mapeamento ainda percorre os nós e executa callbacks. Contêineres conservadores continuam existindo na travessia genérica.

## Validação

- Suite core: **6.680 passaram, 0 falharam, 5 ignorados**; log integral em `validation.json.gz`.
- **34 documentos** preservaram texto, palavras, tamanho de página, coordenadas e raster contra o release anterior: 25 tabelas, 6 controles de colunas/origem/contexto/show/espaços, 2 controles de matemática/morfologia e 1 de efeitos. A ocorrência de show foi executada três vezes no controle e a tabela continuou consultável.
- Seis sondas `query(table).len()` retornaram 1 nos três motores.
- Gate quantitativo externo: baseline contra si próprio não satisfaz melhoria; candidato satisfaz benefício repetido e ausência de regressão robusta. Isso é um gate de desempenho, não um teste funcional que falhava antes: os testes funcionais anteriores já estavam verdes.
- `cargo build`, `cargo build --release` e `crystalline-lint .`: resultados finais em `final-validation.json.gz`. Os dois owners consomem o Núcleo de prova de mudança existente, com pins e hashes atualizados. Há avisos anteriores no projeto; não se afirma ausência global de avisos.

A preservação de Arc nos testes é evidência auxiliar de engenharia. Os controles de linguagem usam conteúdo, morfologia, efeitos e geometria; igualdade de bytes de PDF não é gate de paridade.

## Limites contra vanilla

O oracle foi executado nas mesmas janelas e controles, no pin registrado abaixo. Os casos grandes de matemática e blocos já apresentam paginação/conteúdo divergentes do vanilla, documentados no experimento anterior. Seus tempos servem à comparação Tekt anterior × candidato, **não à comparação de trabalho equivalente com vanilla**. Este passo não corrige essas divergências nem certifica paridade global.

## Proveniência e reprodução

Diretório: `00_nucleo/diagnosticos/content-preflight-removal-2026-10-10/`.

- HEAD: `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. `before.json.gz`, as duas coletas e os recibos de qualificação conservam status/diffstat completos; before/candidate/validation preservam as fontes e L0 exatos dos owners. `candidate.patch` preserva a experiência anterior à redação normativa final.
- Antes: SHA-256 `2d7f5dc93f6a7f15c81ff93b74270c80a14077d6753c8f79c73c9575d113d5f1`, cópia em `/tmp/tekt-before-preflights-2026-10-10`.
- Candidato medido: SHA-256 `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`, `target/release/typst`. O rebuild após a redação final dos L0 deve preservar esses bytes; o recibo final verifica isso.
- Vanilla: commit pinado `9f2b6e8715237cb086899a42873660fe744622e8`, árvore/receita do P1489, `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Primeira coleta: **2026-10-10T01:29:20.904845+00:00 → 01:30:51.493383+00:00**.
- Confirmação: **2026-10-10T01:32:23.201981+00:00 → 01:33:09.852014+00:00**.
- Scripts, fontes, fixtures e fontes tipográficas são registrados com SHA-256. CPU fixada no menor ID permitido, um worker, ambiente registrado e stack de 64 MiB para os três motores, conforme a necessidade já identificada no baseline.
- Reproduzir com `python3 .../measure.py`, `python3 .../confirm.py`, `python3 .../allocation_gate.py red` (exit 1 esperado para baseline sem melhoria), `python3 .../allocation_gate.py green` e os scripts `qualify_*`. O baseline precisa estar disponível no caminho registrado. Cada receipt inclui argv por execução, UTC e hashes; arquivos de resultados são sobrescritos numa nova execução.

## Continuação

Esta melhoria consolida a peça de travessia do experimento de contêineres. Para novo ganho de RAM, o próximo alvo precisa reduzir reconstruções restantes ou buffers; a retirada destes pré-exames não as reduziu nos casos medidos.
