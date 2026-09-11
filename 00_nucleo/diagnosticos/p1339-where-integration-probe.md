# P1339 — sonda de integração de function.where no vanilla

Regime: **executado sem atestação de isolamento**. Executor:
`/root/p1339_where_integration_probe`. Medição delimitada, anterior à
leitura das fontes vanilla pertinentes. Não constitui contrato, proposta
de código, selo ou fechamento de P1339.

## Proveniência e reprodução

Baseline ratificado upstream `a51e02804`; binário `/usr/local/bin/typst`,
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
HEAD local `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não
commitado. Os recibos preservam `git status --short` integral e
`git diff HEAD --stat` antes/depois de cada rodada; o diff inicial era:

```text
 .../compiler/eval/bindings/value_methods.md        | 88 ++++++++++++++++++++++
 .../prompts/compiler/eval/operators/equality.md    | 43 +++++++++++
 .../prompts/compiler/eval/selector_matching.md     | 58 ++++++++++++++
 00_nucleo/prompts/entities/selector.md             | 80 ++++++++++++++++++++
 00_nucleo/prompts/entities/show.md                 | 46 +++++++++++
 5 files changed, 315 insertions(+)
```

Esses nomes são proveniência de estado, não leitura dos L0. Entradas
documentais permitidas: passo explicitamente autorizado
`00_nucleo/materialization/typst-passo-1339.md`, diagnósticos
`p1339-where-l0-gate.md`, `p1339-where-l0-review.md` e
`p1339-full-a2.md`. Seus hashes estão no recibo inicial. Os documentos
incluem descrições do desenho; não foi lido arquivo L0 nem código produtivo
cristalino. Li a skill de materialização segregada e ambas as referências;
a busca `rg -l -i 'segregad' 00_nucleo/adr --glob '*.md'` não retornou ADR.

A rodada inicial executou 24 casos entre `2026-09-10T00:18:55.842594+00:00` e
`2026-09-10T00:18:59.537177+00:00`; complemento focal com 5 casos entre
`2026-09-10T00:20:17.650961+00:00` e `2026-09-10T00:20:19.215281+00:00`.
O complemento foi motivado pela aceitação inesperada de strong/emph e pela
distinção pública da chave vazia; não repetiu o corpus inicial.
Comandos de reprodução:

```sh
python3 00_nucleo/diagnosticos/p1339-where-integration-probe.py
python3 00_nucleo/diagnosticos/p1339-where-integration-probe-supplement.py
```

Os runners imprimem JSON e não sobrescrevem os recibos. Toda entrada Typst,
argv, comando shell cotado, UTC individual, exit code, stdout e stderr
integral consta de `p1339-where-integration-probe-runs.json` e
`p1339-where-integration-probe-supplement-runs.json`. O segundo contém
hash do primeiro e hashes das fontes vanilla consultadas. Executado somente
no perfil default, target paginado. Não houve comparação cristalina.

## Medições públicas

| Fronteira | Observação no baseline pinado |
|---|---|
| `selector(heading.where())` | Tipo selector, repr `heading.where(:)`, igual ao argumento e diferente de `selector(heading)`. |
| `selector(heading.where(level: 1))` | Tipo selector, repr preserva campo; igual ao argumento. |
| `selector(strong.where())`, `selector(text.where(text: "Hi"))` | Ambos aceitos; identidade e campos aparecem no repr. |
| `counter(heading.where())` | Tipo counter, repr `counter(heading.where(:))`; chave diferente de `counter(heading)`. |
| `counter(heading.where(level: 1))` versus level 2 | Reprs diferentes e igualdade falsa. |
| `query(heading)` versus `query(heading.where())` | Ambos retornam First nível 1, Child nível 2, Last nível 1, nessa ordem. |
| Query de heading com level 1 / level 2 | Respectivamente First+Last / Child; filtros são aplicados. |
| Query de heading com level 9 / level string "bad" | Ambos aceitos e vazios; o tipo do valor de filtro não é convertido para o tipo do argumento do construtor. |
| `counter(...).get()` ao final da fixture numerada | Bare `(2,)`, vazio `(2,)`, level 1 `(2,)`, level 2 `(0, 1)`, level 9 `(0,)`. |
| `counter(...).display()` | Bare, vazio e level 1 retornam string "2"; realização em show text produz metadata `["2"]`. |
| `strong.where()` e `emph.where()` em query/counter | **Aceitos**; a fixture encontra e conta um de cada. |
| Strong/emph com body matching / miss | Query retorna body correspondente / vazio; counter get retorna `(1,)` / `(0,)`; display do vazio retorna "1". |
| `text.where()` em query/counter | Rejeitado com `text is not locatable`; spans integrais nos recibos. |
| Atualização manual da chave bare | Após um heading numerado e `counter(heading).update(42)`, get bare = `(42,)`, get vazio = `(1,)`. |

O último caso demonstra uma consequência observável da distinção bare/vazio:
não se trata apenas de repr. Os seletores contam inicialmente os mesmos
headings, mas a atualização manual pertence à chave usada.

Os IDs iniciais `counter_reject_strong/emph` e
`query_reject_strong/emph` preservam o nome da hipótese original. **Não são
resultados de rejeição:** todos terminaram com exit code zero. A hipótese
de strong/emph não localizáveis foi refutada pela medição e pela fonte.

## Fontes após medição e classificação

Paths abaixo relativos a
`lab/typst-original/crates/typst-library/src/`.

- `foundations/selector.rs:167`–174 declara constructor que retorna o
  selector recebido; `:306`–320 distingue no repr o elemento bare do
  filtro presente, inclusive vazio. Isso explica os resultados de transporte
  e morfologia, sem prescrever o enum interno cristalino.
- `foundations/selector.rs:132`–138 verifica identidade do elemento e todos
  os valores de campos. É explicação da filtragem medida; estrutura Rust e
  sequência interna de operações não são obrigações arquiteturais.
- `introspection/query.rs:160`–175 declara query contextual, aceita
  `LocatableSelector` e documenta where como alvo; `:190` delega a consulta.
  Retorno, ordem observada e rejeição são linguagem.
- `foundations/selector.rs:421`–460 valida locatability e unqueriable sem
  descartar o selector. `:426`–427 produz o diagnóstico do elemento;
  `:434`–447 valida também componentes de combinadores. Combinadores não
  foram sondados aqui; essa leitura não equivale a comprovação pública deles.
- `model/strong.rs:21` e `model/emph.rs:26` declaram **Locatable**.
  Junto às sondas, refutam o enquadramento que os agrupava com text como
  não localizáveis no vanilla ratificado.
- `introspection/counter.rs:338`–357 documenta e recebe chave que pode ser
  where, para contar elementos com campos específicos; `:551`–554
  transporta a chave validada; `:557`–562 usa o repr do selector.
- `introspection/counter.rs:243` e `:256`–257 incluem atualizações da
  chave exata e elementos do selector. Isso explica a diferença pública
  entre update da chave bare e get da chave vazia.
- `introspection/counter.rs:360`–372 documenta get contextual;
  `:375`–396 documenta display e numbering; `:934`–955 começa em zero
  e aplica atualizações aos elementos selecionados.
- `model/heading.rs:311`–316 associa contagem de heading numerado ao nível.
  Isso explica level 2 isolado retornar `(0, 1)`, sem inferir que todo
  counter filtrado seja uma mera cardinalidade de query.

A documentação normativa encontrada autoriza queries de elementos localizáveis
e contadores filtrados. A identidade vazia, o resultado de update e os
valores específicos acima são comportamentos medidos; não se presume
intenção adicional a partir deles. Inferência delimitada: eliminar filtros
ou colapsar a chave vazia na chave bare perderia pelo menos um observável
deste conjunto. Refutação: reprodução equivalente no binário pinado em que
os valores registrados deixem de divergir. Não se propõe solução interna.

## Limites

A warning de depreciação do subcomando query foi registrada em todos os casos
de fixture; não impediu compilação/introspecção. Não houve erro de parser nem
falha de transporte nesta sonda. A busca inicial de fontes tentou
`text/strong.rs` e `text/emph.rs`, inexistentes; foi corrigida para
`model/` antes da leitura. Esse erro de inspeção não é erro da linguagem
e não afetou execução de probes.

Resultados não são comparações `Preserved/Violated` contra candidato.
Não há teste de todos os elementos, combinações, targets ou perfis; não há
selo, mutation score, atestação de isolamento nem conclusão de paridade.

