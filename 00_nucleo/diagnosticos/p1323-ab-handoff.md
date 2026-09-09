# P1323 — entrega da autoria A/B

Regime: A/B executado sem atestação técnica de isolamento. Autor dos testes:
`/root/p1323_tests`; ambiente compartilhado, restrições de leitura seguidas
operacionalmente. Não é protocolo completo nem certificado de produto.

Entradas lidas: skill `tekt-materializacao-segregada/SKILL.md`, suas duas
referências operacionais, `CLAUDE.md`, `prompts/wiring.md` integral, seus três
Núcleos Tekt, fixture `p1323-fixtures/hello.typ`, ajuda e execução do vanilla
ratificado, manifesto `p1323-manifest.json`. A busca em nomes/conteúdo das ADRs
não encontrou ADR de segregação. Não foram lidos `04_wiring/src/main.rs`,
diffs de código, binário candidato, código baseline, recibos com código ou
ficheiros de materialization/context. Metadados `git diff HEAD --stat` e
status foram registrados sem ler os diffs.

Saídas escritas: somente diagnósticos `p1323-ab-*` e fixture
`p1323-fixtures/ab-error.typ`. O snippet não altera produção e foi entregue
ao integrador antes da implementação. O teste compara todos os bytes do
literal obrigatório, incluindo cada hint e as duas quebras finais.

O frozen suite registra a proveniência UTC/HEAD/working tree, manifesto
SHA-256 `0ffe55374fa98a5b6ddc40284a52fef616069b38517f582cf79fa52e577cd260`,
hashes do runner, snippet, fixtures, Núcleos e corpo normativo do L0. A única
exclusão do hash normativo é a linha recíproca `Hash do Código:`. O hash raw
observado também permanece registrado. Não foi fixado target de build.

Hashes entregues antes de observar baseline/candidato:

- `p1323-ab-unit.rs`: `376719d97b8aa1648dcaf3ad40dfe5bc9c2d1fcda82a05fc9056650cb53934bb`.
- `p1323-ab-cli.py`: `b9528afb9fe9723e0a9ccb859424d58c160db833bf775b25b509957be295ebbc`.
- `p1323-ab-frozen-suite.json`: `4bcc3816d12c35f072e289dd1918e5184d7a3226016583591ed9d01f375fe284`.

O oráculo foi executado exclusivamente contra `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`, antes de
qualquer observação do produto. Uma primeira tentativa de congelamento falhou
porque `--color` após `compile` não é aceito pelo vanilla. A revisão anterior
ao freeze moveu essa opção global para antes dos subcomandos; não alterou
expectativas do warning. O recibo registra a falha, UTC e custo de um processo.

Calibração no frozen suite: um controle positivo, seis cópias negativas do
observável e um opaco. Negativas: hint removido, hints reordenados, envelope
duplicado, newline removida, newline extra, canal stdout. Os seis foram
`Violated`; o positivo foi `Preserved`; o opaco foi `Unknown`. O predicado foi
reaplicado em ordem reversa. Não são mutantes produtivos e não demonstram
adequação de mutation testing sobre o programa.

O runner fixa 15 casos: HTML legado/canônico/combinado, duas serializações,
dois gates sem html, erro HTML para ordem do warning, PDF/PNG/SVG, query por
ficheiro/stdin, eval e help. Executa ordem normal, repetida e reversa. Registra
stdin, argv, stdout/stderr em base64, exit, UTC, hashes de binários e artefatos.
Os artefatos temporários são removidos pelo contexto temporário; seus hashes
e tamanhos permanecem no recibo. `--document-id` fixa o documento PDF entre
builds cristalinos. Nenhum artefato PDF é comparado byte a byte com vanilla.

Comando para o integrador, após build do candidato:

```sh
python3 00_nucleo/diagnosticos/p1323-ab-cli.py run \
  --manifest 00_nucleo/diagnosticos/p1323-manifest.json \
  --suite 00_nucleo/diagnosticos/p1323-ab-frozen-suite.json \
  --baseline /tmp/p1322-target.Ir19xI/release/typst \
  --candidate /tmp/p1323-target.9NrOxY/release/typst \
  --receipt 00_nucleo/diagnosticos/p1323-ab-process-receipt.json
```

O processo do runner retorna sucesso quando conseguiu produzir o recibo;
isso **não** significa que o candidato passou. O verificador deve examinar
`checks.warning`, `checks.preservation` e `order_checks` integralmente.
`Unknown` em observação obrigatória impede fechamento. Nos gates sem `html`,
`Preserved` significa apenas que ausência/rejeição foram preservadas; não
concede crédito de paridade HTML exercitada. A paridade exata observada é
somente o warning sem cores. Nenhum veredito sobre o produto foi emitido pelo
autor A/B.
