# P1339 — ocorrências strong/emph no vanilla ratificado

Regime: **executado sem atestação de isolamento**. Papel delimitado de medidor,
executor `/root/p1339_strong_emph_occurrence_probe`. Entradas permitidas:
normas do repositório, skill Tekt e referências, sonda anterior
`p1339-where-integration-probe.py/.md`, fontes vanilla. Não li L0 nem código
cristalino; nomes de L0 no estado Git abaixo são proveniência, não leitura.
Acesso tecnicamente compartilhado ao workspace impede atestação de isolamento.
Não houve autoria de implementação, contrato ou veredito bilateral.

## Proveniência e reprodução

Baseline upstream ratificado `a51e02804`; binário `/usr/local/bin/typst`,
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
Rodada de 11 casos entre `2026-09-10T01:06:52.262421+00:00` e `2026-09-10T01:06:55.221414+00:00`,
todos exit code zero. Perfil default e target paginado.
Estado inicial exato registrado por `git diff HEAD --stat`:

```text
 .../prompts/compiler/eval/bindings/field_access.md | 43 +++++++++++
 .../compiler/eval/bindings/value_methods.md        | 89 ++++++++++++++++++++++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   | 53 +++++++++++++
 .../prompts/compiler/eval/operators/equality.md    | 44 +++++++++++
 00_nucleo/prompts/compiler/eval/repr.md            | 38 +++++++++
 00_nucleo/prompts/compiler/eval/rules.md           | 34 +++++++++
 .../prompts/compiler/eval/selector_matching.md     | 59 ++++++++++++++
 .../compiler/stdlib/foundations/selector.md        | 35 +++++++++
 00_nucleo/prompts/entities/element_payload.md      | 88 +++++++++++++++++++++
 00_nucleo/prompts/entities/selector.md             | 82 ++++++++++++++++++++
 00_nucleo/prompts/entities/show.md                 | 48 ++++++++++++
 11 files changed, 613 insertions(+)
```

O recibo `p1339-where-occurrence-probe-runs.json` contém antes/depois:
HEAD, diffstat, status integral incluindo untracked, horários individuais,
argv, comando shell cotado, fonte Typst completa via stdin, stdout e stderr.
Os hashes do runner e documentos de entrada estão no recibo. O estado
Git pode mudar concorrentemente no trabalho principal; a execução depende do
binário fixo, não dos L0 modificados.

Reprodução sem sobrescrever recibos:

```sh
python3 00_nucleo/diagnosticos/p1339-where-occurrence-probe.py
```

O runner emite JSON em stdout. A rodada focal não precisou de reparos nem
de repetição. Antes de congelá-la, um eval exploratório executou
`{ let e = strong[outer #strong[inner]]; (repr(e), repr(e.fields()), e.has("delta"), e.at("delta", default: "missing")) }`
com `/usr/local/bin/typst eval ... --format json`, exit zero. Esse preflight
só validou a API de projeção; a evidência decisória de construtor ausente
está no caso reproduzível `constructor_projection`.

## Medição pública

| Caso do recibo | Cardinalidade, ordem e campos observados |
|---|---|
| call_vs_markup | Calls e markup geram 2 strong e 2 emph, na ordem escrita. Cada um expõe a própria função e body. |
| same_function_nested | 2 strong e 2 emph; em cada par, outer antecede inner. O body outer mantém a construção semântica inner. Não há colapso de ocorrências por igual função. |
| mixed_function_nested | Ordem combinada strong-outer, emph-inner, emph-outer, strong-inner. Cada outer conserva no body a função interna, não apenas o texto. |
| markup_nested | `*outer *inner* tail*` produz dois strong irmãos, bodies "outer " e " tail"; o mesmo vale para underscores/emph. Este texto não expressa o aninhamento do caso com calls. |
| text_style_controls | text com weight bold e style italic sozinho não produz strong/emph. O strong explícito dentro de text produz exatamente 1 strong. |
| set_strong_delta | Sob set strong(delta: 100), markup e call sem delta expõem 100 na query. Delta explícito 0 e 300 prevalecem. São 4 ocorrências; os filtros where(delta: 100/0/300) retornam respectivamente as duas primeiras, a terceira e a quarta. |
| explicit_delta | Os três strong retornam delta 100, 0 e 300 na ordem escrita. A ocorrência de delta 0 permanece consultável. |
| label_boundaries | 2 strong e 1 emph. Outer recebe outer-s, inner recebe inner-s. query(inner-s) e strong.where(label: inner-s) retornam o inner strong; filtro strong com label do emph retorna vazio. |
| reused_content | Reutilizar o mesmo valor strong duas vezes gera 2 ocorrências; emph idem. Não há deduplicação por valor da expressão. |
| body_morphology | 5 strong e 3 emph devido aos elementos internos. Filtro body: [inner] seleciona 3 strong e 2 emph de body textual; exclui os outers cujo body é strong/emph([inner]). |
| constructor_projection | Antes da realização, strong[inner].fields() contém somente body; delta está ausente. emph contém somente body. Na query, os strong sem delta explícito apresentam o default 300 (ou o valor vindo de set). |

Nos casos consultados, `where()` vazio tem a mesma cardinalidade que o
seletor bare. `query(selector(strong).or(emph))` preserva a ordem
observada de inserção, incluindo outer antes de inner.
A "identidade" aqui é a função semântica, a fronteira da ocorrência e seu
label/body observável. Não houve comparação de endereços, hashes de
location ou identidade interna Rust. No reuso, a evidência é de duas
ocorrências, não de uma investigação geral do algoritmo de identidade.

## Fonte vanilla, lida depois da rodada

- `lab/typst-original/crates/typst-library/src/model/strong.rs:16`–21
  documenta stars como sintaxe da função e declara Locatable; :23–34
  define delta default 300 e body obrigatório.
- `lab/typst-original/crates/typst-library/src/model/emph.rs:22`–30
  documenta underscores, declara Locatable e body. :3–8 documenta
  alternância de itálico; a sonda não usa render para inferir ocorrências.
- `lab/typst-original/crates/typst-eval/src/markup.rs:145`–159
  materializa as duas formas de markup como StrongElem/EmphElem com seu
  body. Confirma a identidade semântica de markup e call.
- `lab/typst-original/crates/typst-realize/src/lib.rs:540`–553
  estabelece locatability/label como razão de identidade documental;
  :571–582 copia campos da cadeia de estilo antes de capturar o elemento
  para introspecção. Explica delta ausente no construtor e resolvido na
  query, sem obrigar o cristalino a copiar fases internas, tags ou enums.
- `lab/typst-original/crates/typst-library/src/foundations/selector.rs:132`–140
  exige identidade do elemento e valores dos campos; label é consultado no
  alvo. Explica os filtros observados, não prescreve igualdade Rust.
- `lab/typst-original/crates/typst-library/src/introspection/introspector.rs:200`–222
  filtra elementos e conserva índices de ocorrências na união de
  seletores. A ordem da sonda é observável; BTreeSet e cache são mecânica.
- `lab/typst-original/crates/typst-library/src/introspection/query.rs:160`–175
  recebe seletor localizável e retorna os conteúdos consultados.

Hashes das fontes e instruções inspecionadas após medição (as normas
ADR-0107/0108/0121 já estão pinadas no JSON):

```text
d90f66ad854a88b59250ad569b60e2ad2f86f6ce103cdb4290c35ac4628e1221  lab/typst-original/crates/typst-library/src/model/strong.rs
01bc7f4ac66a0c8cc07e1bd1ccc4b0d2957aae383916db0d91870b10e0ed6ba6  lab/typst-original/crates/typst-library/src/model/emph.rs
84615405b19d4869d261029936d3d5d9077f06e7198e3f60f28273c1ecdfcbdc  lab/typst-original/crates/typst-library/src/foundations/selector.rs
fd4223ac9de5604b9807a326cd6174293803e4b1cc173caa1820f9a61a258f09  lab/typst-original/crates/typst-realize/src/lib.rs
229620686521b742189734297ceeb04575caf14e3378cba607fa03fd726d581d  lab/typst-original/crates/typst-library/src/introspection/query.rs
d6aad2ffe38f2daa78d685372d03c50f9525b71ea0d8986c68907048fc363709  lab/typst-original/crates/typst-library/src/introspection/introspector.rs
d37c3e17d8654c4ee62fcf2c51fea751344d95ce05eedcbd2d5b4dc7dba0c00e  lab/typst-original/crates/typst-eval/src/markup.rs
66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48  /home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md
f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417  /home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/papeis-e-capacidades.md
16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d  /home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md
```

A busca por ADR contendo "segregad" não encontrou arquivo. A procura
inicial de `foundations/content.rs` retornou caminho inexistente; não foi
usada como evidência nem houve leitura de outro conteúdo por adivinhação.

## Classificação e limites

Os fatos acima são sintaxe, semântica e morfologia observáveis da linguagem:
multiplicidade/ordem da consulta, identidade de função, labels, body,
projeção de campos e filtragem. Defaults de strong resolvidos por set são
campos públicos da ocorrência consultada; isso é diferente de interpretar
text(weight: bold) como strong. Bytes, aparência, geometrias, índices Rust,
tags, cache e mecanismo de passes não são critérios desta sonda.

Intenção documentada: equivalência de sintaxe e função, body obrigatório,
delta default e participação em locatability. Multiplicidades e valores
concretos da tabela são comportamento medido. Inferência delimitada:
uma representação que colapse strong aninhados, transforme estilo text em
ocorrência strong ou elimine a distinção entre campos de construtor e
campos consultados perde pelo menos um observável desta rodada.
Refutação: reprodução no binário pinado em que essas projeções,
cardinalidades ou ordens não sejam as registradas.

Não é prova de paridade cristalina, selo, mutation score ou contrato
geral. Não se exercitaram show rules que substituem/reemitem elementos,
subdocumentos, HTML, layout geométrico ou todo o corpus. O texto ambíguo
de stars no caso markup_nested é um controle de sintaxe, não proposta de
gramática. A warning de depreciação de query permanece integral nos
recibos e não invalidou as consultas. Nenhum número desta sonda fecha o
P1339 ou uma comparação bilateral.
