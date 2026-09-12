# Matriz de paridade P1137

Este diretório contém casos versionados, não uma alegação de paridade. `manifest.yaml` é JSON válido e YAML 1.2 válido; essa forma permite validação reproduzível apenas com a biblioteca padrão do Python. `schema.json` documenta o contrato completo em JSON Schema.

O alvo vanilla é exclusivamente `upstream/main 586e1bd43`, materializado em `lab/typst-original/target/release/typst`. A string impressa por `--version` não prova sozinha a revisão. A proveniência ratificada vem do pin e da árvore do upstream, da receita com `TYPST_COMMIT_SHA` explícito e do SHA-256 do binário; o relatório registra também caminho, versão observada, HEAD e estado integral da árvore.

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

`UNKNOWN` nunca fecha um item. Uma inferência só pode ser promovida após registrar a evidência que a sustenta e o resultado que a refutaria. Resultados gerados devem ir para `/tmp` ou para diagnósticos, nunca substituir o manifesto.
