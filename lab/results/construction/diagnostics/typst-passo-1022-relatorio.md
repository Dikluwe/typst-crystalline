# Passo 1022 — Fatiamento de `stdlib/text` em hub + 8 nós

**Data**: 2026-08-13

> **Nota de identificação**: o número do passo é o do plano em
> `00_nucleo/materialization/typst-passo-1022.md` (pasta restrita, não lida). O trabalho
> aqui relatado é o que estava em curso na árvore de trabalho: três L0s de nó
> (`case`, `constructor`, `deco`) já redigidos e não selados, sem código
> correspondente. Se o número estiver errado, o conteúdo não muda — só o nome do ficheiro.

## Proveniência das medições

Todas as medições foram feitas na **árvore de trabalho, não commitada**, às
2026-08-13T00:41-03:00. `HEAD` era `541ca4cc7` no início da sessão e avançou para
`fffcd9060` a meio (commit do relatório de auditoria de prompts, feito fora deste
trabalho); nenhum dos dois toca `stdlib/text.rs`, e o corte foi verificado contra a versão
de `HEAD`:

```
git show HEAD:01_core/src/compiler/stdlib/text.rs == ficheiro cortado   → idêntico
```

Ficheiros alterados no momento das medições: ver `git diff HEAD --stat` abaixo. As saídas
da ferramenta de co-mudança estão em `temp/p1022/cochange_text*.txt` — `cochange_text.txt`
é a leitura **com** o artefacto (antes da correcção) e `cochange_text_tool_fixed.txt` a
leitura corrigida, para que a diferença seja reproduzível.

```
00_nucleo/prompts/auditar-fatiamento.md   | +48 −8
00_nucleo/prompts/compiler/stdlib/text.md | reescrito (420 linhas alteradas)
01_core/src/compiler/stdlib/text.rs       | −1212 (removido)
crystalline.toml                          | +2
tools/analysis/cochange_metrics.py        | +42 −13
+ não rastreados: 01_core/src/compiler/stdlib/text/ (9 ficheiros),
                  00_nucleo/prompts/compiler/stdlib/text/ (8 ficheiros)
```

## Sumário

`01_core/src/compiler/stdlib/text.rs` (1212 linhas, 23 funções livres + 1 enum privado)
passou a **hub + 8 nós**, conforme ADR-0109. Oito L0s de nó e um L0 de hub reescrito; zero
alteração de comportamento; `cargo test --workspace` com **5842 testes a passar**, `#[test]`
contados antes e depois: **6138 = 6138**; `crystalline-lint .` com **zero erros**.

| Nó | Ficheiro | Itens | Nativas exportadas |
|---|---|---:|---|
| `constructor` | `text/constructor.rs` | 7 | `native_text` |
| `case` | `text/case.rs` | 3 | `native_upper`, `native_lower`, `native_replace` |
| `deco` | `text/deco.rs` | 7 | `native_underline`, `native_strike`, `native_overline`, `native_highlight` |
| `smallcaps` | `text/smallcaps.rs` | 1 | `native_smallcaps` |
| `shift` | `text/shift.rs` | 2 | `native_subscript`, `native_superscript` |
| `smartquote` | `text/smartquote.rs` | 1 | `native_smartquote` |
| `lorem` | `text/lorem.rs` | 2 | `native_lorem` |
| `regex` | `text/regex.rs` | 1 | `native_regex` |
| **hub** | `text/mod.rs` | 0 | reexporta as 14 |

## Ordem do método (`auditar-fatiamento.md`)

### Passo 0 — critério-zero: agregado ou interface?

```
grep -n '^pub trait \|^trait ' 01_core/src/compiler/stdlib/text.rs   → 0 resultados
```

Agregado de funções livres. Prossegue.

### Passo 1 — inventário com regex genérico de visibilidade

```
grep -nE '^(pub(\([a-z:) ]+\))? )?fn ' → 23 funções
+ enum privado DecoKind (contado por ser eixo de variação do nó deco)
```

24 itens. Nenhum `pub(crate)`/`pub(super)` neste ficheiro — todas as nativas eram `pub fn`
num módulo privado (`mod text;`), pelo que a lacuna de visibilidade que apanhou P1000 e
P1013 não se aplicou aqui. Registado por medição, não por ausência de menção.

### Passo 2 — os quatro critérios

**Critério 3 (co-mudança) — decidiu, mas só depois de corrigir a ferramenta.**

30 commits tocaram o ficheiro; 9 só linhagem; 21 com mudança de corpo. A primeira leitura
deu quatro clusters aparentes de fronteira. **Três eram artefacto da ferramenta** (ver
secção seguinte). Depois da correcção, os clusters reais internos aos nós são:

| Commit | Data | Cluster real | Nó |
|---|---|---|---|
| `d2faea55d`, `fdd39b892`, `1aea00338` | 2026-04-23 | `upper`+`lower`+`replace`, três vezes, sempre sozinhos | `case` |
| `147605058` | 2026-05-19 | `DecoKind`+`build_decoration`+`underline`+`strike`+`overline` | `deco` |
| `fa5bda1d4` | 2026-06-24 | `default_highlight_color`+`highlight` | `deco` |
| `f28fba77d` | 2026-06-24 | `sub`+`super`, e só os dois | `shift` |
| `c98ffc8ac` | 2026-07-21 | `lorem`+`lorem_impl` | `lorem` |
| `3c8839e72` | 2026-07-23 | `text`+os 6 validadores, e só eles | `constructor` |

Descontados como ruído: `0661aef91` (`cargo fmt` global, 15 funções), `6636c5ea6` e
`0f5575cd0` (renames de módulo, todas as funções), `87bc1c64d` e `04eda8179` (lotes
transversais: `StyleDelta`, span de `Args`), `e1f09cc24` (de-bake de fonte única).

Para `smallcaps`, `smartquote` e `regex` o critério ficou em **vácuo**: nenhum commit os
liga a outra nativa fora de ruído. Nos três, decidiu o critério 4.

**Critério 4 (vanilla) — testado, e correcto onde a co-mudança estava calada.**

`ls lab/typst-original/crates/typst-library/src/text/` → `case.rs`, `deco.rs`, `lorem.rs`,
`shift.rs`, `smallcaps.rs`, `smartquote.rs` (+ `item.rs`, `lang.rs`, `linebreak.rs`,
`raw.rs`, `space.rs`, `font/`, `mod.rs`). Localizações medidas:
`text/deco.rs:214` (`HighlightElem`), `text/smallcaps.rs:44`, `text/shift.rs:16` e `:75`,
`foundations/str.rs:1017` (`Regex`).

Duas correcções que este critério produziu:

1. O comentário de código cristalino cita `text/sub.rs`, `text/superscript.rs` e
   `text/highlight.rs` como homólogos vanilla. **Nenhum dos três existe** — são
   `shift.rs` (os dois primeiros) e `deco.rs` (o terceiro). Corrigido nos L0s dos nós.
2. `regex` não tem homólogo em `text/`: o tipo vive em `foundations/str.rs`. O nó fica em
   `text/` por continuidade de história, com a divergência registada e com dono (o
   fatiamento de `foundations`).

Onde a co-mudança estava em vácuo, o vanilla não foi aceite sozinho — foi aceite **na
ausência de evidência contra**, e isso está escrito assim nos L0s dos nós, para que uma
medição futura que contradiga possa juntar `smallcaps` a `shift` sem estar a contrariar
uma decisão que se apresentasse como medida.

**Critério 2 (pureza vs estado) — vácuo, medido.**

As 14 nativas recebem `_ctx: &mut EvalContext`, `_world: &dyn World`, `_current_file:
FileId` pela assinatura uniforme e **nenhuma os usa** — classe 2 (pass-through) em todas,
sem excepção. O vácuo é informação: a fronteira de estado não é o eixo relevante neste
módulo. O caso digno de nota é `smartquote`, cujo glifo final depende da língua: a
dependência existe, mas é resolvida no Layouter, não aqui — o que o L0 do nó registra como
invariante de fase (mudá-lo seria gate ADR-0127).

**Critério 1 (isolamento de teste) — vácuo total, 5/5 aplicações.**

`grep -c '#\[test\]' text.rs` → **0**. A suite vive em `stdlib/mod.rs`, com harness único
partilhado com o resto do stdlib. Não havia nada para isolar. A proposta de P1014
(substituir por fan-in) **não foi aplicada** — declarado explicitamente: nenhuma fronteira
ficou em dúvida a ponto de precisar de desempate.

### Passo 3 — órfãos

Nenhum prompt órfão candidato a fronteira de `text`:
`crystalline-lint` no início listava 11 avisos V7, dos quais 8 eram os L0s de `text/`
(3 pré-existentes + 5 escritos neste passo, órfãos até o código existir) e 3
pré-existentes noutras áreas. `crystalline.toml` não tinha excepção morta a apontar para
ficheiros de `text`.

### Passo 4 — materialização

Corte e cola, com prova item a item (não "os testes passam"):

```
itens antes: 24        itens depois: 24
linhas de corpo distribuídas: 1181 (+ 31 de cabeçalho/imports = 1212 do original)
linhas não cobertas: nenhuma      linhas duplicadas: nenhuma
24/24 itens byte-idênticos ao original (as 8 diferenças detectadas eram uma linha em
  branco a mais no primeiro item de cada ficheiro; conteúdo idêntico após normalizar
  o espaço nas pontas)
```

Visibilidade: as nativas continuam `pub fn`, reexportadas pelo hub; os 10 helpers privados
(6 validadores, `build_decoration`, `DecoKind`, `default_highlight_color`, `lorem_impl`)
ficaram privados aos nós e **não** subiram para a fronteira. Nenhum `pub(in …)` foi
necessário — o caso E0364 que apanhou P1013 não se reproduziu aqui, porque a origem já era
`pub fn`.

Imports: derivados por nó a partir dos símbolos realmente usados. Três ajustes depois do
primeiro build (`FromStr` em falta em `constructor`, `Content` a mais em `case`, `Style` a
mais em `deco` e `shift`) — zero avisos do compilador nos nove ficheiros novos.

Testes com harness partilhado ficaram no hub (`stdlib/mod.rs`), conforme a regra de P1014.
O hub é **só fronteira de reexportação**, sem `fn` e sem tabela de despacho, conforme a
regra de P1013 — e isso está declarado no L0 para não ser lido como lacuna.

### Passo 5 — validação

```
cargo build                → Finished, zero avisos nos ficheiros novos
cargo test --workspace     → 5842 passed; 0 failed
#[test] em HEAD vs árvore  → 6138 = 6138
crystalline-lint .         → 0 erros; 3 avisos V7 pré-existentes
                             (auditar-fatiamento.md, auditar-spec.md — workflows, não
                              materializáveis; infra/package_version_resolution.md)
```

## Achado de método — artefacto de atribuição de fronteira

`tools/analysis/cochange_metrics.py` atribuía as linhas adicionadas ao item que as contém,
procurando para trás a última declaração `fn`. Quando um passo acrescenta uma função nova,
o banner de comentário que a precede cai **depois** do `}` da função anterior — e era
atribuído a essa função anterior, que não mudou.

Efeito medido em `stdlib/text.rs`: **três dos quatro** clusters aparentes eram falsos.

| Cluster aparente | Commit | Verificação (`git show <c> -U1`) |
|---|---|---|
| `overline`+`smallcaps` | `0b97d2d78` | corpo de `overline` intacto — só o banner de `smallcaps` |
| `smallcaps`+`sub`/`super` | `f28fba77d` | corpo de `smallcaps` intacto — só o banner de `sub` |
| `highlight`+`super` | `fa5bda1d4` | corpo de `super` intacto — só o banner de `highlight` |
| `smartquote`+`overline` | `a1fe997d2`, `077792dfa` | idem, nas duas direcções |

Sem a correcção, o fatiamento teria juntado `smallcaps` a `shift` e discutido `highlight`
com o par errado — duas fronteiras decididas por ruído.

A correcção tem duas partes, e a segunda só apareceu porque a primeira falhou: (1) a
fronteira de um item vai até ao **banner do item seguinte**; (2) só as linhas **a partir da
declaração** contam como corpo — a linha em branco que sobra do item anterior cai sempre no
banner do seguinte e, contada, reinventa o artefacto. As duas estão na ferramenta e a
regra está em `auditar-fatiamento.md`, com o sintoma para a reconhecer à mão.

## Derivas fechadas por este passo

| Deriva | Estado antes | Agora |
|---|---|---|
| `upper`/`lower`/`replace` materializados sem L0 (nota "deriva (F4)" reconhecia a lacuna sem a fechar) | aberta desde 2026-04-23 | fechada em `text/case.md` |
| `size` em `sub`/`super` declarado scope-out, implementado desde `87bc1c64d` | L0 desactualizado | corrigido em `text/shift.md` |
| `radius`/`extent` em `highlight` declarados scope-out, implementados desde `87bc1c64d` | L0 desactualizado | corrigido em `text/deco.md` |
| consumer de `smallcaps` descrito como "stub transparente", real desde `e7ae938f6` | comentário desactualizado | corrigido em `text/smallcaps.md` |
| homólogos vanilla inexistentes (`text/sub.rs`, `text/superscript.rs`, `text/highlight.rs`) | citados no código | corrigidos nos L0s, com `file:line` medido |
| `temp/p1021` sem excepção em `crystalline.toml` (3 erros V1 de scripts de auditoria) | 3 erros | excepção adicionada, a par de `temp_p776`/`temp_p935` |

## Aberto, com dono

1. **`regex` está no domínio errado.** O lugar é `foundations`, junto do tipo
   (`foundations/str.rs:1017`). Movê-lo agora arrastaria `stdlib/foundations.rs` (89 KB,
   monolítico) para dentro deste fatiamento. Dono: o passo que fatiar `foundations` — deve
   absorver o nó ou declarar por que não.
2. **Duas mensagens de erro citam número de passo** (`P284 §A.1`, `P287 §A.2`), visíveis a
   quem escreve o documento. É defeito de produto, não de documentação; corrigi-lo muda
   texto visível → gate ADR-0127. Dono: o passo que revisitar os scope-outs graded de
   `text`.
3. **`pub use self::regex::native_regex;`** — o nome do nó sombreia a crate `regex` e o
   classificador de imports do linter lê a reexportação sem prefixo como tipo externo não
   declarado (V14). Registado no L0 do hub como excepção por colisão de nome, não como
   padrão geral.
4. **Cláusula de guarda de P1014** (`table_counter`): **não se activou** neste fatiamento —
   `stdlib/text` não toca contadores. Declarado explicitamente, conforme a regra.

## Avaliação do método (obrigatória)

- **Que critério decidiu**: o 3 (co-mudança), em cinco das oito fronteiras — mas só depois
  de a ferramenta ser corrigida. Nas outras três decidiu o 4 (vanilla), sobre vácuo do 3.
- **Que critério foi vácuo**: o 1 (isolamento de teste), pela quinta vez em cinco, e desta
  vez de forma terminal — zero `#[test]` no ficheiro. O 2 (pureza) também: pass-through em
  14 de 14.
- **Hipótese inicial corrigida pela evidência**: sim, duas vezes. A leitura por inspecção
  juntaria `smallcaps` a `sub`/`super` (parecem os três "variantes de glifo por escala"), e
  a primeira leitura da co-mudança confirmava-o — era artefacto. E o vanilla citado no
  código apontava para três ficheiros que não existem.
- **Enquadramento cómodo desconfiado**: o cluster `smallcaps`+`sub`+`super` era exactamente
  a fronteira que dava mais jeito (três funções pequenas num nó só) e vinha com aparência
  de medição. Foi a checagem extra da fonte — `git show` no commit, corpo a corpo — que o
  desfez.
