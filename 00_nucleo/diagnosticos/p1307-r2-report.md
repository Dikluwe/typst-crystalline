# P1307-R2 — medição destravada; argumentos precisam de informação persistente

A reabertura autorizada foi executada. **A estratégia de medição foi corrigida
sem mudar a CLI. A auditoria não sustenta implementar o contrato completo
apenas em `repr` e `call_dispatch`.** O problema restante não é só apontar
erros: valores de argumentos anteriores também são apagados antes da validação.

## O que mudou na decisão

| Frente | Resultado desta reabertura |
|---|---|
| Valores contextuais | `Location` e `LocatedContent` agora são observáveis nos dois binários e quatro perfis, sem `eval --in`. |
| CBOR | Os controles passaram de `bytes(N)` para a lista pública de todos os octetos. As dívidas anteriores de Symbol/Content continuam separadas. |
| `repr` | State/Counter já possuem os dados necessários. Location foi confirmado bilateralmente. Essas correções cabem no formatter, sem mudar suas entidades. |
| Argumentos | `.with`, spread e sinks precisam conservar ocorrências, valores e origens. Capturar só o span da chamada final não basta. |

### A medição contextual deixou de ser um bloqueio

Usamos `compile` sobre arquivos `.typ` temporários, com as flags existentes.
Dentro de `context`, o valor é convertido em string pública e transportado
integralmente como octetos UTF-8 por um `panic` intencional. O adaptador
reconhece somente o diagnóstico real desse transporte, não o texto da fixture.
Controles de Unicode, newline e aspas validam a reconstrução; sentinelas
`html.elem` e `pdf.table-summary` comprovam o efeito real de cada perfil.

A matriz revisada mediu **48 casos × 4 perfis × 2 binários × 3 ordens =
1.152 execuções**, com zero Unknown e zero instabilidade. Isso significa
observabilidade, **não igualdade dos produtos**: os três encoders continuam
ausentes no cristalino, e suas ausências são diagnósticos explícitos. As
tentativas focais anteriores permanecem registradas; não foram apagadas para
produzir esse resultado.

### Por que mudar só o dispatch não resolve

Este exemplo vanilla ainda falha no `"bad"` original:

```typst
json.encode.with(pretty: "bad")((a: 1), pretty: true)
```

O vanilla valida as ocorrências nomeadas antes de conservar a última válida.
O cristalino usa `IndexMap::extend`, que apaga o valor anterior. Acrescentar
somente um mapa de spans não recuperaria o valor que precisa ser validado.

Outra contraprova faz uma factory escolher entre dois `arguments(nope: 1)`
iguais, criados em linhas diferentes, e devolve `yaml.encode.with(..args)`.
O erro vanilla aponta a origem escolhida. O caminho cristalino atual guarda
os mesmos valores e o mesmo span da linha `.with` em ambos os casos. O teste
foi repetido com argumentos devolvidos por sink de closure, com a mesma
distinção de origens. Não se afirma que o encoder cristalino foi executado:
ele ainda não existe; a perda foi localizada na fonte que prepara seus Args.

Fontes causais: `call_dispatch.rs:403-430,1016-1023`,
`entities/args.rs:18-28`, `closures.rs:72-131`; comportamento ratificado em
`typst-library/src/foundations/args.rs:218-235` e
`typst-eval/src/call.rs:412-460`. Expressões, fontes numerizadas e saídas
integrais estão no recibo R2, casos `with-*-invalid-first-occurrence` e
`with-spread-{arguments,closure}-{f,g}`.

## Retificação explícita do diagnóstico anterior

`repr(json.encode.with())`, e os paralelos TOML/YAML, é **`(..) => ..`**.
Somente o membro original tem repr `encode`. O contrato documental anterior
dizia incorretamente que a função parcialmente aplicada também tinha repr
`encode`. Os dados originais já continham o resultado correto: foi erro de
transcrição/interpretação, não mudança do vanilla entre medições. O predecessor
foi preservado; a expectativa futura precisa incorporar esta retificação.

## Desenho recomendado e nova fronteira de autorização

Recomenda-se associar a `Args` um carrier persistente, interno à linguagem,
que preserve as ocorrências necessárias, seus valores e as duas âncoras:
argumento completo e expressão-valor. A estrutura Rust exata ainda é decisão
de L0. É necessário definir coerência com as projeções atuais `items`/`named`
antes de escrever código; metadata desatualizada não é proveniência válida.

O transporte atravessa `eval_args`, spread, `.with` e sinks de closures.
Transformações de `arguments` em `collections` e a concatenação em `join`
também precisam conservar ou transformar corretamente essa informação.
Uma nuance impede uma regra simplista: no vanilla `arguments.map` pode
preservar o span do argumento completo e deixar detached o valor novo;
`filter` preserva ambos. O contrato deve reproduzir os diagnósticos observados,
não inventar precisão.

Os owners semânticos a incluir na próxima especificação são:

- `entities/args.md`;
- `compiler/eval/call_dispatch.md` e `compiler/eval/closures.md`;
- `compiler/stdlib/collections.md` e `compiler/eval/operators/join.md`;
- `compiler/eval/repr.md`, além dos quatro owners originais dos encoders.

`entities/func.md` precisa de revisão documental das premissas antigas; não
está demonstrada necessidade de mudar a entidade Func se Args transportar os
dados. Literais/reconstruções em math, numbering e int e seus testes exigem
auditoria de migração antes de ampliar o allowlist de implementação. O
inventário detalhado está em `p1307-r2-span-audit.md`; contagens lexicais não
foram apresentadas como estimativa de esforço.

A necessidade demonstrada é **informação causal persistente**, não copiar a
estrutura do vanilla. Um sidecar teria de provar identidade e vida útil por
clones, retornos e imports; não elimina a necessidade de um owner e de um
contrato novo. Estado global ou recuperação por igualdade de valores não são
alternativas aceitáveis.

Adicionar campo ao `Args` público muda o contrato de construção e exige
ADR-0127. A autorização recebida foi para esta auditoria e correção da
medição, não para essa ampliação. Próximo ato recomendado: autorizar a
redação dos L0 de Args e dos pontos de transporte, com escopo de migração
explícito; depois apresentar o gate do contrato público antes de RED/código.

## Evidência, integridade e limites

O estado é HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree
P1306 não commitado. `p1307-r2-baseline.json` registra status, diff integral,
`git diff HEAD --stat`, inventário de fontes e hashes de todos os artefatos
predecessores. Permanecem alterados apenas os quatro tracked anteriores:

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/tests.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/tests.rs
```

O baseline R2 tem SHA-256
`0c0e92fa4c84bb23a1e2adefc95e93ad75ffaf0f12f3edce58ab8e9393b2f611`.
O binário foi reutilizado somente após confirmar os mesmos bytes de fontes
e executável da compilação fresca P1307; não houve rebuild desnecessário.
Vanilla upstream `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
cristalino SHA-256
`945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`.

A matriz R2 ocorreu de `2026-09-07T16:44:21.062280+00:00` a
`16:44:45.243137+00:00`; recibo `p1307-r2-measurement.json`, SHA-256
`847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`.
O recorte adicional de repr tem 136 invocações, entre `16:38:58` e
`16:39:04 UTC`, em `p1307-r2-gates.json`; separa dívidas dos constructors
de State/Counter e de Selector, sem corrigi-las silenciosamente.

Lint estrito V3/V4/V5/V13/V14/V15/V26 e `git diff --check` passaram;
comandos, horários e saídas estão em `p1307-r2-gates.json`. Não houve L0,
Rust, Cargo, headers, RED, stage, commit ou exclusões nesta reabertura.
`p1307-r2-verification.json` registra a conferência documental dos hashes
e da preservação do predecessor, não um certificado de implementação.

A skill `tekt-materializacao-segregada` manteve as autorias de auditoria de
spans e de medição separadas e impediu converter a reabertura em autorização
de código. Regime: **executado sem atestação de isolamento técnico**. Não
foi repetido o corpus completo P1307 nem a matriz histórica de 627 probes;
o sucesso deste recorte não é paridade geral de Content, dos encoders ou de
todos os caminhos de argumentos. Não há mutation score nem selo pronto.
