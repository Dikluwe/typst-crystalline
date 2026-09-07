# P1307-R5 — Heading: dados observáveis para redação do contrato

Estado: **medição documental concluída; não é selo de implementação**. Autoria independente das expectativas, executada sem atestação de isolamento técnico. Nenhum candidato foi lido; nenhum Rust, teste, L0 ou header foi editado. A skill `tekt-materializacao-segregada` determinou preservar a tentativa contextual incompleta e separar referência vanilla, dívida preexistente e obrigação ainda sujeita à decisão do owner.

## Proveniência e orçamento

Baseline autorizado `p1307-r5-baseline.json`, SHA-256 `32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a`. HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não commitado: a lista exata `git diff HEAD --stat` e os horários constam em cada `provenance` da medição. O baseline contém o estado produtivo P1306, sem candidato P1307. Binários conferidos antes de cada execução/consolidação:

- Vanilla `/usr/local/bin/typst`: `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`, upstream ratificado `a51e02804`.
- Baseline `/dev/shm/p1307-baseline-target.hqbv0ooo/release/typst`: `945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`.

Medições de `2026-09-07T18:22:30.680597+00:00` a `2026-09-07T18:23:45.617347+00:00`: 24 execuções na tentativa focal inicial, 24 na revisão focal e 134 na extensão limitada; total 182, soma de tempo dos processos 36,108192855 s. A suíte corrente tem 64 casos, todos no perfil default; cinco controles também nos três outros perfis (`html`, `a11y`, `html+a11y`), sempre target PDF. São 158 observações correntes, zero `Unknown` e zero falhas de pré-condição vanilla. Os cinco controles preservaram o mesmo envelope em todos os perfis. Não houve matriz global, repeat/reverse nem inferência de paridade do target HTML.

Artefatos congelados:

| Arquivo em `00_nucleo/diagnosticos/` | SHA-256 |
|---|---|
| `p1307-r5-content-measure.py` | `3fa2d0680e7aff934100ec47d18ad3976d252cc15414154e5dc4889ad3564ec5` |
| `p1307-r5-content-measurement.json` | `82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8` |
| `p1307-r5-content-oracle.json` | `ccfd62a66676561d58562123dfe91e039ef8a50d12ff707fe0d03c6a66204520` |

O runner R4 foi reutilizado byte a byte. `finalization.predecessor_hashes` pina os predecessores; nenhuma expectativa antiga foi sobrescrita. Cada caso guarda source integral e SHA; cada execução guarda argv, cwd, ambiente efetivo declarado, horário, duração, returncode e stdout/stderr integrais. O transporte decimal de UTF-8 observa a string pública inteira, não roundtrip de serialização nem normalização de payload. Apenas identidades exatas dos arquivos temporários são canonicalizadas, pelo adapter R4 congelado.

## Fonte antes da classificação

Vanilla `lab/typst-original/crates/typst-library/src/model/heading.rs:40–245` declara a ordem dos campos e `level = explicit level` ou `offset + depth`; `:248–286` sintetiza `level` e `supplement`. `numbers` é interno/sintetizado e não é campo público de `fields`. `Content::fields`, `foundations/content/mod.rs:580–603`, reúne campos conhecidos e acrescenta label quando presente; Serialize `:709–719` põe `func` antes dos campos. Os hashes destas fontes estão na medição e no baseline.

`foundations/content/raw.rs:300–310,354–361` compara o elemento e seus campos, sem comparar diretamente os metadados de label/location. Isso apoia, mas não substitui, os testes de igualdade da linguagem abaixo. O baseline `01_core/src/compiler/eval/operators/equality.rs:112–130` compara `morph_canon` e exclui Location; sua projeção atual não conserva os padrões de numbering observados.

## Resultados que delimitam o contrato

Os IDs abaixo apontam diretamente para `cases[].id` no oracle e `focal.rows`/`bounded.rows` na medição. A fonte/observação precede qualquer decisão arquitetural do owner.

| Casos | Observação vanilla medida |
|---|---|
| `direct-default` / `query-default` | `heading[Probe].fields()` contém só `body`; query de markup contém, nesta ordem, `level, depth, offset, numbering, supplement, outlined, bookmarked, hanging-indent, body, label`. JSON acrescenta `func: "heading"` à frente. |
| `set-level` | level 4, depth 1, offset 0: level e depth não são o mesmo campo. |
| `set-depth-markup` / `set-depth-constructor` | `set depth:3` não sobrescreve o depth 1 explicitado por `=`, mas constructor sem depth produz depth 3 e level 3. |
| `set-offset` / `explicit-depth-offset` | offset 2 + depth 1 → level 3; depth explícito 2 + offset 1 → level 3. |
| `numbering-1`, `numbering-I`, `numbering-none` | O campo preserva a string exata ou `none`, não um booleano. |
| `numbering-callback` | Query preserva Func; `fields` mostra `(..) => ..` e JSON usa essa string opaca. Não é o número renderizado. |
| `supplement-auto`, `language-en`, `language-pt` | Auto é realizado em Content: `[Section]` em en, `[Seção]` em pt. |
| `supplement-none` | Após síntese é Content vazio `[]`; JSON é `{"func":"sequence","children":[]}` na formatação integral congelada, não `null`. |
| `supplement-Alpha`, `supplement-Beta`, `supplement-callback` | Query contém respectivamente `[Alpha]`, `[Beta]`, `[Callback]`; o callback foi resolvido, não preservado como Func neste campo. |
| `outlined-false`, `bookmarked-true`, `bookmarked-false`, `hanging-indent` | Preserva false, true/false, 10pt; os outros campos mantêm seus defaults. `bookmarked:auto` continua `auto`, não o booleano efetivo para PDF. |
| `frozen-array-before`, `frozen-array-after` | Value atravessa array e closure. Mudar depois o bloco para numbering I, supplement Beta, offset 3 e idioma pt não altera o snapshot antigo: level 2, depth 1, offset 1, numbering "1", supplement Alpha e label probe. |
| `two-styles` | Dois headings com mesmo body conservam separadamente `(level2,offset1,"1",Alpha,left)` e `(level3,offset2,"I",Beta,right)`. Não se pode reconstruir pelo estilo de quem consulta. |

`direct-explicit` é controle adicional de constructor, não prova de que todos os seus campos já sejam suportados no baseline: vanilla conserva os named args explícitos; baseline `.fields()` dá `(:)`. A fonte baseline `stdlib/structural/heading.rs:127–138,181–194` encaminha numbering por uma rota de conteúdo numerado. Não atribuí esse resultado vazio apenas ao serializer ou assumi que a representação transportada fosse uma Heading nua. `direct-markup-label` constrói uma **sequence** contendo Heading e space, como o JSON vanilla revela; não foi rebatizado como Heading simples para forçar cobertura.

## Igualdade e acesso — não usar igualdade do Rust como oracle

| ID `equality.*` | Vanilla | Baseline |
|---|---|---|
| `inline-vs-query` | false | false |
| `clone-through-array` | true | true |
| `different-labels` | true | true |
| `same-labels` | true | true |
| `no-labels` | true | true |
| `different-numbering` | false | true |

Assim, **label e Location não podem, por si, tornar desiguais os pares medidos**; numbering realizado faz diferença. `inline-vs-query` não prova que apenas a nova variante de Value determine desigualdade: também diferem campos conhecidos. Não há licença para implementar igualdade como `derive` de um container que inclua label/location, nem para generalizar estes seis pares a toda Content.

`access.unlabelled`: `has("label")` false e `at("label", default:"absent")` retorna o default. Com label (`access.labelled`), vanilla retorna true e `<probe>`; baseline perde essa informação. Campo desconhecido `missing` dá false/default nos dois. Vanilla conhece numbering (`none`), supplement (`[Section]`) e body (`[Probe]`), e `repr(x.func())` é `heading`; baseline conhece body/func mas não numbering/supplement.

`access.error-label` e `access.error-missing` preservam nos dois binários os diagnósticos integrais, inclusive origem e range: `heading does not have field "label" and no default was specified` e a variante `"missing"`. Não confundir inexistência de label com um label sintetizado, `none`, nem o erro de um campo declaradamente conhecido mas ainda não assentado no conteúdo direto.

## Limites, dívida e tentativa incompleta preservada

1. Tentativa focal 1: o valor capturado atravessou array/closure e um **novo nó `context`**. Vanilla observou campos/JSON antigos; baseline saiu 0 sem executar o marcador. São dois `Unknown` históricos genuínos, em `focal_history[0]`, não dois tipos inexistentes. A revisão 2, única, trocou esse veículo por um novo bloco de style no mesmo context e ficou observável bilateralmente. Ela não demonstra que nested context foi consertado. Não se promove a observação vanilla histórica a aprovação bilateral.
2. Baseline `json.encode` está ausente. Seu erro medido não demonstra falta de Content. As observações públicas `.fields()` independentes mostram a perda: markup consultado dá somente `(depth:1, body:[Probe])`, constructor consultado somente body. Os pares Alpha/Beta e "1"/"I" repetem a lacuna R4 sem adulterar aqueles artefatos.
3. Set-rule de supplement tem dívida anterior, indicada pelo coordenador e fonte baseline `eval/rules.rs:970–1003`; os padrões de numbering e outros estilos percorrem canais distintos. Não atribuir toda perda à introspecção, nem converter a correção do serializer em reparo implícito de todos os set-rules.
4. Callback de supplement é resolvido na síntese vanilla; callback de numbering continua Func nos campos públicos. O recorte usa callbacks constantes bem formados. **Não mediu** callbacks contextuais, recursivos, que falham ou com efeitos em state/counter; não autoriza nova fase de execução. Baseline aceitou os set-rules do recorte, mas não expôs seus resultados. O constructor baseline rejeita numbering Func na fonte indicada; isso não foi confundido com o caso set-rule medido.
5. O oracle R5 guarda pares `observations[profile].vanilla/baseline`, como **entrada para contrato**, sem `future_expected` automaticamente escolhido. O owner deve classificar explicitamente comportamento requerido, dívida preservada e expansão sujeita a ADR-0127. O comparador integral executável continua `p1307-r4-oracle.py::classify`. Esta redação não decide a API nova nem certifica paridade geral de query/where, Content, render, callbacks, HTML ou igualdade.

Próximo gate: owner incorpora estes limites no L0, humano aprova as mudanças públicas/de fase aplicáveis, e só então são possíveis selo do oracle de aceitação, RED e candidato independente. Nenhum desses gates é declarado concluído aqui.
