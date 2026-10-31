# Matriz de paridade P1137

Este diretório contém casos versionados, não uma alegação de paridade. `manifest.yaml` é JSON válido e YAML 1.2 válido; essa forma permite validação reproduzível apenas com a biblioteca padrão do Python. `schema.json` documenta o contrato completo em JSON Schema.

O alvo vanilla é exclusivamente `upstream/main 64c5e8c02fe0394d913ead36e42ef8c1334d2107`, materializado em `lab/typst-original/target/release/typst`. A string impressa por `--version` não prova sozinha a revisão. A proveniência ratificada vem do pin e da árvore do upstream, da receita com `TYPST_COMMIT_SHA` explícito e do SHA-256 do binário; o relatório registra também caminho, versão observada, HEAD e estado integral da árvore. O recibo vigente está em `lab/results/construction/diagnostics/vanilla-resync-2026-10-26/`. As medições anteriores com `9f2b6e871` ou `586e1bd43` permanecem históricas; estados esperados só são confirmados para o alvo novo após executar novamente a matriz.

## Reprodução

```sh
python3 -m unittest lab/parity/matrix/test_runner.py
python3 lab/parity/matrix/runner.py --validate-only
python3 lab/parity/matrix/runner.py --output /tmp/p1137-results.json
python3 lab/parity/matrix/runner.py --case P1137-E-001
```

O runner captura comando, exit code, stdout, stderr e presença de artefato. Paths não são normalizados nesta primeira versão; nenhuma divergência é promovida a `MATCH` por tolerância. Os comparadores são deliberadamente separados:

- `exit_code`: igualdade exata;
- `exact_output`: exit code, stdout e stderr exatos;
- `semantic_version`: extrai apenas o triplo público `major.minor.patch`; o hash impresso não é usado como prova de revisão;
- `capability`: sucesso vanilla seguido de falha cristalina é `ABSENT`;
- `artifact_presence`: ambos precisam terminar com sucesso e produzir artefato.
- `diagnostic`: exit code, mensagem e span após remover ANSI e substituir
  somente o prefixo absoluto do repositório;
- `typed_value`: JSON convertido para DTO com tags de tipo (bool não colapsa
  em int; dict é ordenado por chave);
- `semantic_tree`: árvore SVG/HTML, preservando ordem, texto e atributos;
  somente `id` SVG é descartado como identificador volátil;
- `geometry`: páginas e caixa em pt, arredondadas a 0,001 pt por campo;
- `raster`: dimensões e pixels normalizados para RGBA, com contagem de pixels
  diferentes, delta máximo/médio e PNG de diferença;
- `pdf_observables`: páginas/caixa (`pdfinfo`), texto (`pdftotext`) e famílias
  de fontes (`pdffonts`), sem bytes nem ordem interna de objetos.

Quando `--output` é fornecido, os artefactos reproduzíveis ficam ao lado do
JSON em `<stem>-artifacts/`. Sem `--output`, o runner usa diretório temporário.

`MATCH` em `artifact_presence` comprova somente a capacidade explicitamente observada, não igualdade de geometria, raster ou bytes. Comparações futuras de valor tipado, estrutura, geometria, raster e árvores SVG/HTML devem ganhar comparadores próprios e tolerâncias documentadas por unidade, razão e fonte.

Uma sentinela divergente está verde quando o estado medido coincide com `expected_state=DIFF` ou `ABSENT`. O processo do runner sai com código 1 quando uma expectativa não coincide; diferenças reais esperadas permanecem dados válidos.

## Recalibração P1487

Os perfis fechados são `default`, `html`, `a11y-extras`, `html+a11y`,
`bundle` e `html+bundle`. O perfil sem `bundle` mede o gate negativo;
os casos funcionais de bundle recebem `DISABLED_BY_PROFILE` fora dos
perfis ativos, nunca `MATCH` por falha bilateral.

`DIFFERENCE` registra um desvio observável cuja semântica não foi
demonstrada: diferenças de pixels com as mesmas dimensões e de nomes
internos de fontes PDF preservam métricas e artefatos. `VIOLATED`
indica divergência em valor, diagnóstico, capacidade, geometria, texto,
estrutura PDF/UA, rótulo/margem ou bytes exportados de `asset`.
Nenhuma tolerância de pixels foi acrescentada. `UNVERIFIED_METADATA`
no inventário de superfície permanece distinto de ausência real e
não recebe crédito de execução. O manifesto P1487 executa `spot.tint`,
`asset`, bundle, PDF/UA e page labels, com controles de rejeição.
`pdf_ua` é um smoke estrutural; não certifica conformidade integral.

Exemplo: `python3 lab/parity/matrix/runner.py --vanilla
lab/typst-original/target/release/typst --crystalline target/debug/typst
--profile bundle --output /tmp/p1489-bundle.json`.
Os seis perfis devem registrar o hash dos binários utilizados; o release
cristalino não deve ser presumido atualizado por causa de `--version`.

`UNKNOWN` nunca fecha um item. Uma inferência só pode ser promovida após registrar a evidência que a sustenta e o resultado que a refutaria. Resultados gerados devem ir para `/tmp` ou para diagnósticos, nunca substituir o manifesto.

## Ressincronização P1489

O manifesto incorpora sete sondas RED do novo pin: `measure().baseline`,
localização em HTML, espaço CJK de quebra de linha, `repr(stroke())`,
`sym.chess`, `sym.tack.r.double` e `sym.not.inv`. `VIOLATED` esperado
significa lacuna conhecida e reproduzível, não sucesso de paridade.
Para símbolos, o inventário separa `VERIFIED_METADATA_DIFFERENCE` de
`UNVERIFIED_METADATA`; diferença de metadados só vira violação da
linguagem após sonda executável. `join` persiste no cristalino como extra
de compatibilidade e não é removido automaticamente.
