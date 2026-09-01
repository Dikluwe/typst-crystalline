# P1288 — recibo de autoria dos oráculos congelados

## Regime, papel e limite de independência

- Regime: protocolo completo da skill `tekt-materializacao-segregada`.
- Papel deste turno: **autor de oráculos P1288**, anterior a candidato,
  implementação, campanha adversarial, selo e veredito.
- Executor: `/root/p1288_contract2`.
- O mesmo executor acumulou autoria do contrato e dos oráculos por limite de
  slots do orquestrador. Isto reduz a independência nominal, mas não funde
  obrigação/solução/veredito: este executor não recebeu, leu nem executou
  candidato, target, harness existente, código L1–L4, mutantes, ataques ou
  veredito.
- Checkout compartilhado e capacidade física de leitura mais ampla impedem
  alegação de isolamento ambiental forte. Linguagem proporcional:
  **oráculos executados com segregação por entradas, capacidade, ordem e
  hashes, sem atestação de isolamento ambiental forte**.

## Entradas permitidas e congeladas

Este papel leu somente:

- manifesto de contrato P1288, SHA-256
  `0e7282b6b5b445d0532bb5456abfbd765e7878fc44d318f96a0b8b960351ed3a`;
- receipt de contrato P1288, SHA-256
  `69d8ea63d2eb6f0267e135cdcd37a479c1b0a0b72814a1170a01be28cf48d34a`;
- baseline vanilla canônico, SHA-256
  `057bab7534c4855b0b47cc7cd05e243058d8b39407e30c2b3ee478f04d24c377`;
- receipt vanilla final, SHA-256
  `06a32ad3a77cc710a4ba1c54330325d0cc5a788ba2378cf0e6fda32269c5df80`;
- receipt final de Núcleo + L0, SHA-256
  `b259334c262683a2e62979b67cbf3b09f57f65ac9e07c4e5c504b6dda5436c6b`;
- os 12 fixtures vanilla P1288 enumerados e pinados no baseline oracle.

Vanilla reproduzido: upstream/main `a51e02804`, binário
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## Saídas e capacidades

Escrita limitada aos quatro artefatos autorizados:

| Artefato | Papel | SHA-256 |
|---|---|---|
| `lab/parity/matrix/p1288_oracles.py` | runner data-driven e classificador | `98bb462601c255708a646fdc7dc27e2f59455ade5198b2d26105299934723e48` |
| `lab/parity/matrix/test_p1288_oracles.py` | testes do runner e do congelamento | `59f3c8ea71df9fee0221c4daf50c5fb53c4fde6dd0992eb1fd0aa3e849c77f99` |
| `lab/parity/matrix/p1288-oracle-baseline.json` | expectativas e hashes imutáveis | `ac0d97379820eb09f89c69e52858b231794bb4db6455d16527be2d04f1fd7a3d` |
| `00_nucleo/diagnosticos/p1288-oracle-receipt.md` | este recibo | hash calculado após a escrita |

O runner aceita `--binary` futuro somente como identidade de execução. Ele não
mede o binário para regenerar nem adaptar expectativas: todas vêm do baseline
oracle congelado. Se o path não corresponder ao SHA do vanilla, a identidade é
registrada como `future-binary`; as mesmas expectativas permanecem.

## Arquitetura do runner

Antes de executar qualquer caso, o runner verifica SHA-256 do manifesto,
contrato, baseline/receipts e dos 12 fixtures. Drift produz resultado global
`Unknown` e impede execução com crédito.

Casos congelados:

- positivos: features ativas, trio indivisível, composição de flags e
  estruturas PDF de tabela simples, header automático, linha mista, levels,
  spans, multipágina, summary, casts e outros targets;
- negativos: gates desligados, feature inválida, casts/assinaturas inválidos,
  `summary: none` explícito, tags disabled e diagnósticos públicos;
- opacos: AT real, screen reader, reflow, PDF/UA/certificação, bundle e objetos
  PDF não decodificados.

Chamadas inválidas são casos negativos do contrato, não mutantes. Este papel
não criou nem executou mutação adversarial. Um erro esperado corretamente
observado é `Preserved`; uma divergência decisiva é `Violated`; falta de
identidade, ferramenta ou interpretação é `Unknown`. `Unknown` nunca é
promovido a sucesso.

Para PDF, o runner separa estrutura, texto, boxes e raster; verifica
StructTreeRoot/MarkInfo, THead/TBody/TR/TH/TD, scopes, spans, summaries,
StructParents e ParentTree. Tags disabled devem omitir estrutura. Os pares
tagged/untagged comparam texto, bbox, boxes e raster. HTML/SVG/PNG comparam
somente o par de fixtures congelado.

`--order both` executa a suíte em ordem forward e reverse e compara resultados
canonizados. Repetição/composição de features usa grupos de equivalência de
payload. O runner termina sem emitir `REFINED`, `NOT REFINED` ou
`INCONCLUSIVE`; produz apenas evidência classificada.

## Comandos e resultados

Proveniência: `2026-08-31T11:36:30-03:00`, HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, árvore não commitada e
compartilhada.

```text
python3 -m json.tool lab/parity/matrix/p1288-oracle-baseline.json
python3 -m py_compile lab/parity/matrix/p1288_oracles.py \
  lab/parity/matrix/test_p1288_oracles.py
python3 lab/parity/matrix/p1288_oracles.py --self-test
python3 -m unittest lab/parity/matrix/test_p1288_oracles.py
python3 lab/parity/matrix/p1288_oracles.py \
  --binary /usr/local/bin/typst --order both \
  --output /tmp/p1288-oracle-vanilla-run.json
```

Resultados:

- JSON e bytecode Python válidos;
- `SELF_TEST_OK`;
- 10 testes unitários, todos verdes;
- reprodução vanilla: 46 `Preserved`, 0 `Violated`, 4 `Unknown` deliberados;
- forward/reverse: concordância exata;
- output efêmero vanilla SHA-256
  `04f2c408b9f923624825f6d6909a1deeb4a84c13f6ff8e5b6e6d7f659ae884fd`.

Durante a primeira reprodução, a inspeção multipágina usou `mutool show
pages`, que lista referências mas não os dicionários `/Page`; o próprio
oráculo marcou ausência de `StructParents`. A inspeção foi corrigida para
pedir `pages.1`, `pages.2` e `pages.3`, sem alterar qualquer expectativa. A
segunda reprodução produziu os resultados acima.

## Estado causal

Os oráculos estão congelados para futura verificação discriminatória. Qualquer
drift de entrada protegida invalida esta fase. Este recibo não é campanha de
mutação, selo, confirmação do gate humano, implementação, certificado nem
veredito funcional.
