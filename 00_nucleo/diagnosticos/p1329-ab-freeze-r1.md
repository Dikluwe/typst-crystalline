# P1329 — freeze A/B r1, anterior a C

UTC: 2026-09-09T12:27:44.861Z. Regime A/B executado sem atestação técnica de isolamento; sem selo de refinamento. Autor /root/p1329_tests, contexto novo sem histórico herdado. Nenhum candidato foi lido ou executado pelo autor.

## Entradas e capacidades

Manifesto SHA-256 d1e9d154733644b2e170c9872bd2cb1aeb165ed95fe34945960ccbc1550f9080; L0 normativo 3756c7998c5a81cfdc54119b6fc152ca1f186f0f74df654e01877b73bdadbe0d, descartando somente a linha Hash do Código. Baseline público e7af5fca1897431c6060ded8a8072df03e8946aa7be95ab71aa3ebd366d40063. HEAD medido d31047d7b8af7837c84adae4ded3d2ff50c62093, working tree não commitado; diff/stat de proveniência já identificado no baseline público recebido, sem nova leitura de diffs produtivos pelo autor. Binários e argumentos completos estão no recibo CLI, UTC inicial 2026-09-09T12:25:26.395976+00:00.

Leituras: skill e suas duas referências; L0 calc completo; manifesto e baseline público; APIs públicas Args, Value, layout_types e fixtures/interfaces test-only P1328. Históricos P1328 lidos somente nos arquivos autorizados. Proibidos: calc.rs de produto, patch candidato, recibos root protegidos, baseline integral P1329, materialization e context. Escritas somente 00_nucleo/diagnosticos/p1329-ab-*. O ambiente compartilhado não impõe tecnicamente essas capacidades; o registro é declarativo.

## Artefatos congelados

- p1329-ab-tests-r1.rs: 567f8d4eda2fa5313af8449323248d814621cb47e5122a4ab19842dbf93fa838
- p1329-ab-p1328-successor.rs: 1fb3d1c0b6ddfd486aa1cfd4cbdaffbdf2ecd774021b5512d217b769e92c3a7c
- p1329-ab-cli.py: 392144197866dc2c9539256de1a657b8fa14a1ab61d17e3516ccb6f01a37b70a
- p1329-ab-cli-baseline-r1.json: b89049333f76a7f49b2828a6ced0fc0c7ba747f6b7ca9ea019fba5a27e0bd91d
- p1329-ab-cli-expected-r1.json: 699ab2fe050a1207a7ad9a98d8b4d78e7366fa421324f5d64aa06826c2289c51

BASE: /tmp/p1328-target.T7Tg57/release/typst, SHA-256 94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9. VANILLA: /usr/local/bin/typst, SHA-256 7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8, ratificado a51e02804.

Sucessor P1328 mantém módulo completo e todas as outras asserções. Único delta: import Angle/Length/Ratio e migração de quatro expressões (-2pt/-2deg/-2%/-2fr) da tabela de rejeições à tabela de sucessos. Diff literal e hashes em p1329-ab-delta-r1.json. Nos históricos CLI, 96 expectativas são idênticas e somente 16 mudam (quatro dimensionais × quatro perfis).

Nova suíte nativa verifica espécies e magnitudes, incluindo NaN como classe, infinitos positivos, Length puro e misto com não finitos já construídos; nenhuma sonda anterior à construção é usada como prova de abs. Testa sinais/zeros/componente zero negativo/ângulos além de uma volta, Ratio além de 100%, cm/mm/in/rad/em, guards, âncoras value_span, origens agregadas/occurrence/detached/ausentes, sequência sintética com ocorrência named anterior à posicional, aliases, With encadeado, arrays/arguments spread, UTF-8 e warnings. Math qualificada e bare usa -2deg como conteúdo. O caso -2pt em math falha antes em variável pt desconhecida: a medição refutou usá-lo como prova de abs; permaneceu somente sentinela CLI de construção, sem enfraquecer os casos math P1328 ou -2deg.

## Medição, dívidas e política

CLI tem 63 casos × quatro perfis = 252 observações por execução, 504 processos na medição BASE/VANILLA pré-C. Preserva integralmente os 28 casos P1328 (nomes, expressões e perfis). Novos casos são adicionados depois. Ordem normal = catálogo; reverse inverte casos e mantém ordem dos perfis. Comparador confronta literalmente exit/stdout/stderr completos; não normaliza candidato.

A expectativa de mixed-with, mixed-with-nested e mixed-spread-args é transcrição integral do vanilla com apenas o nome externo de trace calc.abs preservado do baseline. As três transcrições completas, por perfil, estão em p1329-ab-cli-expected-r1.json, produzidas e congeladas antes de C. Isso é dívida do dispatcher, não paridade. As três dívidas equivalentes de conteúdo P1328 permanecem byte a byte iguais ao histórico.

Dívidas preservadas: guards named/aridade, str/symbol, saturação i64::MIN, calc.sqrt, nome externo de dispatcher e Float×Fraction. prior-float-fraction falha em cannot multiply float with fraction antes de abs; prior-zero-division falha em cannot divide by zero e é controle anterior à chamada, não prova NaN. math-dimension-content, apesar do nome do catálogo, mede variável pt desconhecida antes de abs; math-alias-content mede conteúdo sem coerção. Essas fronteiras estão classificadas antes de C. Unknown em obrigação do escopo bloqueia fechamento; as falhas anteriores conhecidas não contam como sucesso dimensional.

## Ordem e gates

1. L0/manifesto/baseline público recebidos antes da autoria; nenhum patch C existia.
2. Sucessor e novos testes escritos; baseline CLI medido com --color never antes de eval; expectativas materializadas antes de C.
3. rustfmt --edition 2021 --config-path rustfmt.toml --check nos dois snippets: exit 0. Delta histórico conferido. Revisão de fixture math guiada por baseline pré-C, sem patch.
4. Root integra os snippets cegamente; executa RED real com filtro compiler::stdlib::calc::p132; depois C e GREEN dos mesmos testes. Só root executa CLI candidato normal/repetido/reverse e gates build/workspace/fmt/lint/V5/V15/V26.

Comando runner: python3 -B 00_nucleo/diagnosticos/p1329-ab-cli.py --candidate CAMINHO --output 00_nucleo/diagnosticos/p1329-ab-cli-NOME.json --order normal|reverse. A gravação usa apply_patch via stdin; neste ambiente deve executar no host (require_escalated), como em P1328. Nunca sobrescreve recibos. Nenhum commit produzido.

Budget congelado: uma revisão inicial; máximo duas revisões consecutivas sem ganho na mesma causa antes de reabrir método/escopo. Uma medição pré-C do catálogo; pós-C normal/repetida/reverse pelo root. Hashes identificam artefatos, não provam isolamento ou equivalência geral de abs/calc.
