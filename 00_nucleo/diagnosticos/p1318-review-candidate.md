# P1318 — revisão independente do candidato

Veredito: candidato conforme ao recorte L0; fechamento global ainda pendente
dos gates e do A/B contra o executável liberado.

Auditoria `p1318-review-audit.cjs`, executada somente leitura em
`2026-09-08T15:41:02.209Z`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado:
fonte SHA-256 `870f861651c35f44afd5f3f68548bb0364463b2ca3d4d2087b5460bdbc986f5a`,
L0 raw `8cfa574ea121eb053598a5a45b7d54d4c1546ed920f519a3e2341b833f3e8246`.
Diff/stat observado: loading.md 261 linhas no diff, loading.rs 551;
789 inserções e 23 remoções acumuladas nos dois arquivos desde HEAD.
Não confundir esse acumulado P1315–P1318 com tamanho do patch P1318.

A reconstrução independente do source RED usando o diff arquivado contra
seu HEAD resultou exatamente em `fece2c4d...433ffd0`. Todo o módulo de testes
é literalmente idêntico entre RED e candidato. As regiões anteriores ao
decode CSV (salvo metadata) e os demais helpers/nativas são idênticos ao
baseline P1318. Todos os artefatos P1315/P1316/P1317 e inputs congelados A/B
conservam seus hashes. L0 normativo segue `2b23a5ad...49312ee6`.

Revisão de fonte: decode_csv conserva assinatura e chama a delegação privada
com posição desligada; Path/Str continuam chegando nessa API. Somente Bytes
liga posição. Há um Reader/uma passagem de registros; nenhuma validação
UTF-8 antecede o erro do parser. A causa continua selecionada por ErrorKind,
ordinal continua vindo da enumeração incluindo header, e o span causal Bytes
continua vindo da primeira ocorrência posicional. Não há reparse ou extração
de posição da mensagem formatada.

O helper limita offset a u32, rejeita offset fora do buffer ou fora da
fronteira de char sem panic/sufixo inventado, e seleciona texto/binário pela
validade do buffer inteiro. Texto conta chars e todas as quebras normativas;
CR seguido de LF só avança a linha ao atravessar LF, preservando `at 1:5`
entre CR/LF. Binário conta LF no prefixo e chars desde o último LF; ausência
de LF produz coluna 1. `String::from_utf8_lossy` é compatível com a
substituição do iterador usado pelo vanilla: a documentação local da crate
utf8_iter 1.0.4 declara expressamente a mesma substituição. Sem Position,
o mapper usa o ordinal e coluna 1. Nenhum owner/API/crate/fase/I/O novo.

GREEN `p1318-unit-green.json`, SHA-256
`0c8e81b7213ffc5d88e744552f84005956aa471244fc02728c8ac63a75a2682c`:
mesmo comando do RED, 67 testes passam, zero falhas, exit 0; source e L0
estáveis entre `2026-09-08T15:35:55.515580+00:00` e
`2026-09-08T15:37:56.819934+00:00`. Não foram relaxados testes após o RED.

Regime A/B sem atestação de isolamento; não constitui selo de refinamento,
nem paridade geral CSV. Fallback e offsets impossíveis foram revisados na
fonte; a suíte pública A/B declara corretamente que não os atesta pela CLI.
