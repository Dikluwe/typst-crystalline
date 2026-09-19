# P1462 R2 — contrato observável recongelado de `format.svg.pretty`

**Estado:** CONGELADO R2 PRÉ-CANDIDATO; ainda não selado pelo gate discriminatório

**Manifesto causal:** `00_nucleo/diagnosticos/p1462-r2-manifesto.json`

**SHA-256 do manifesto:**

`9de72872a6113b35464ae17da0f3f7dd59a70acbd5956426108e233e3bd66e8f`

**Baseline R2 imutável:**

`61796fa6e183c08207e3828cec6d2e43da1af846`

**Tree do baseline R2:**

`4890fb690d222c093e858130da00d2062ae514dd`

**Vanilla ratificado:**

`586e1bd43fae6c9a973218163d3165c53ab8d16d`

**Regime:** protocolo completo de materialização Tekt segregada

**Atestação proporcional:** executado sem atestação de isolamento

Este contrato R2 foi escrito antes de existir ou ser lido qualquer patch
candidato. O checkout e o filesystem são compartilhados; a separação é de
papel, entrada, capacidade, ordem causal e artefato, não de isolamento técnico.
Hashes identificam entradas e sucessão, mas não provam isolamento.

## 1. Autoridade, sucessão R1→R2 e gate

As fontes normativas são os dez Prompts L0 R2 e o Núcleo Tekt enumerados no
manifesto R2. O Passo coordena; não substitui os owners L0. As ADRs 0107,
0108, 0127 e 0129 governam língua/mecânica, medir-antes-de-decidir, gate humano
e cardinalidade L0↔consumer.

O gate ADR-0127 permanece satisfeito. Em 2026-09-19, depois de o coordenador
informar ao humano o commit documental
`514b2ae8f35b661127e38620dcbdaefa9e158624` e perguntar explicitamente se
autorizava testes RED/implementação P1462, o humano respondeu `confirmo` no
turno imediatamente seguinte. O transcript é externo ao repositório e
atestado pelo coordenador.

A R2 é correção contínua de paridade, L0-primeiro, provocada por refutador
pré-candidato. Ela não exige nova confirmação humana, mas exige manifesto,
contrato, oráculos, catálogo adversarial, gate e selo R2 novos antes de teste
RED ou código.

Os quatro artefatos originais têm valor exclusivamente histórico:

| Artefato R1 | SHA-256 | Disposição |
|---|---|---|
| `p1462-manifesto.json` | `de5d15c28576da4436fdaf05ee1c92416e001577ca07d4d5761f7a6bfb7885b4` | sucedido pelo manifesto R2 |
| `p1462-contrato-congelado.md` | `96052223adf9ed4ff49332ee72ee2c4096f1a9e04cb653077f4499b42fbd8503` | obrigação M06/O3/O5 refutada |
| `p1462-oraculos-independentes.md` | `f452d0229c4dba6a87b5c3ed316daaaca84c9c066686a197034460555ed624d7` | ensaio inconclusivo; fonte do refutador e dos defeitos de instrumento |
| `p1462-mutantes.md` | `04bdc94b7cfb766867f0a891f5162fa3537cca0e6ebfa7a9b22cc10e136b1b95` | catálogo histórico; M06 invertido |

R1 nunca foi selo, não produziu mutation score e não pode fornecer crédito de
gate. Os únicos fatos causais transportados à R2 são:

1. no vanilla ratificado, `true → #set format.svg()` permaneceu pretty em
   `forward`, `reverse` e `forward-repeat`;
2. O8 perdeu a identidade do subcaso antes do timeout;
3. `styled-root` foi comparado incorretamente com uma fixture vazia distinta.

“Congelado R2” significa que expected, instrumentos e discriminadores abaixo
não podem ser ajustados a um candidato futuro. Não significa “selado”.
Qualquer mudança no baseline, Passo, Prompts, Núcleo, manifesto ou contrato R2
invalida a cadeia desde a primeira entrada afetada.

## 2. Fronteira congelada

P1462 cobre somente:

1. o filho público `format.svg` da raiz `format` existente;
2. a propriedade documental booleana `pretty`, default `false`;
3. a set rule `#set format.svg(pretty: ...)`, sua regra vazia neutra e seus
   diagnósticos;
4. extração da decisão documental em root e rejeição de decisão explícita em
   containers;
5. transporte da opção até o documento paginado;
6. precedência `CLI explícito > documento > false` no export SVG completo;
7. neutralidade de outros targets e integridade dos owners Tekt.

Ficam fora: `format.png`, `format.pdf`, `format.html`, geometria, texto,
fontes, destinations, SVG interno de `html.frame`, comportamento próprio de
PDF/PNG/HTML, `--compact`, PDF tags, feature gates e equivalência global de
SVG ou `format.*`.

## 3. Classificação ADR-0107/0108

São observáveis de língua ou interface pública: identidade/default do filho,
validação de argumentos, construção proibida, neutralidade da regra vazia,
ordem das decisões explícitas, fronteira root/container, mensagem pública,
precedência CLI/documento/default, modo de serialização pedido, morfologia XML
normalizada da mesma fonte e ausência de efeito nos eixos excluídos.

São mecânica livre: macros/traits upstream, coleções, endereços, downcast,
número de passes, contadores, forma Rust interna, tamanho, número de LF, bytes
e SHA-256 do SVG. Mensagem de erro é observável precisamente porque a mecânica
textual é a interface pública naquele ponto.

A inferência L0 sobre travessia exaustiva versus envelopes root continua
condicionada: variante não percorrida, perda de ordem, carrier ausente onde há
named explícito ou falso positivo em root reabre o desenho. Um
`Content::Styled` sem a chave `format.svg.pretty` é envelope transparente e
não é ocorrência.

## 4. Observáveis obrigatórios R2

### O1 — identidade pública

Nos perfis `default`, `a11y-extras`, `html` e `html+a11y`:

```typ
#repr((type(format.svg), format.svg, format.svg.pretty))
```

observa exatamente `(function, svg, false)`. `std.format.svg` e alias lexical
preservam identidade nominal e representação. A raiz existe independente de
target/feature.

### O2 — construção manual proibida

`format.svg()` falha com:

```text
cannot be constructed manually
```

Nenhum `Content` construível nasce da chamada.

### O3 — domínio, validação e neutralidade da regra vazia

- `#set format.svg(pretty: true)` e `pretty: false` são aceitos em root;
- quando `pretty` está presente, seu booleano é validado e transportado sob a
  chave canônica `format.svg.pretty`;
- `#set format.svg()` é aceito, mas não escreve a chave, não cria decisão
  documental e não apaga a cascata anterior;
- regra vazia isolada conserva o default final `false` por ausência de decisão,
  não por escrita artificial de `false`;
- depois de `pretty: true`, a regra vazia preserva `true`;
- named desconhecido `other` falha com `unexpected argument: other`;
- `pretty: 1` falha com `expected boolean, found integer`;
- posicional, duplicado ou tipo incompatível falha pelo canal normal, nunca
  como warning de target ignorado;
- nenhum campo além de `pretty` é aceito.

### O4 — fronteira top-level/container

Uma decisão explícita root é válida. A mesma regra com `pretty` explícito sob
container falha com a mensagem exata:

```text
format set rules are not allowed inside of containers
```

O caso falho não publica SVG de sucesso nem `PagedDocument` parcial. A prova
usa ao menos dois containers estruturalmente distintos e um controle root.
O contrato não inventa semântica nested própria para a regra vazia: sem chave,
ela não é contada como ocorrência documental P1462.

### O5 — ordem lexical/cascaded com vazio neutro

Somente decisões explícitas participam de last-write-wins. Regra vazia é
transparente:

| Sequência root | Decisão efetiva | Forma sem CLI |
|---|---:|---|
| sem regra | `false` por default | compacta |
| regra vazia | `false` por ausência | compacta |
| `true → false` | `false` | compacta |
| `false → true` | `true` | indentada |
| `true → regra vazia` | `true` | indentada |
| `false → regra vazia` | `false` | compacta |

`Sequence` e `Styled` transparentes de raiz conservam ordem e neutralidade.

### O6 — matriz de precedência SVG

`None` é ausência de opção explícita do caller; `Some(bool)` é override que
deve sobreviver até a opção documental estar disponível.

| CLI / override | Documento | Booleano final | Serialização |
|---|---:|---:|---|
| ausente / `None` | sem decisão (`false`) | `false` | compacta |
| ausente / `None` | `true` | `true` | indentada |
| ausente / `None` | `false` | `false` | compacta |
| explícito `false` / `Some(false)` | `true` | `false` | compacta |
| explícito `true` / `Some(true)` | `false` | `true` | indentada |
| explícito `false` / `Some(false)` | sem decisão | `false` | compacta |
| explícito `true` / `Some(true)` | sem decisão | `true` | indentada |

As quatro claims do Núcleo são integrais: override explícito vence, ausência
delega ao documento, default vem depois e colapso antecipado é proibido.

### O7 — XML e comparação morfológica válida

Compacto e pretty precisam ser XML válido, distinguir a forma de serialização
solicitada e ter morfologia normalizada igual quando provêm da mesma fonte.
Tamanho, quantidade de LF e SHA não são expected.

O controle `styled-root` usa um único arquivo `.typ` byte-idêntico, com a mesma
morfologia e conteúdo, em duas execuções: uma produz pretty pela decisão
documental e outra força compacto por override CLI explícito. A comparação
normalizada ocorre exclusivamente dentro desse par. É proibido comparar
`styled-root` com a fixture de página vazia ou com qualquer fonte/conteúdo
distinto.

### O8 — isolamento de targets com instrumento atribuível

`format.svg.pretty` não altera PDF, PNG, HTML nem o SVG interno de
`html.frame`. Cada target é um subcaso independente:

| ID persistente | Recorte | Comparação exigida |
|---|---|---|
| `O8-PDF` | PDF com decisão documental `false` versus `true` | texto e metadata pública selecionada iguais |
| `O8-PNG` | PNG com decisão documental `false` versus `true` | dimensões e pixels iguais |
| `O8-HTML` | HTML com decisão documental `false` versus `true` | DOM normalizado igual |
| `O8-HTML-FRAME` | HTML contendo `html.frame`, `false` versus `true` | DOM e SVG interno normalizados iguais |

Antes de iniciar cada subprocesso, o runner persiste no recibo o id do
subcaso, target, hash da fonte, flags, hash do binário, timeout e estado
`started`. Depois persiste `completed`, `failed` ou `timed_out`, exit, canais e
artefato observável. Um timeout torna somente o subcaso identificado
`Unknown`; O8 agregado não é `Preserved` enquanto qualquer linha estiver
`Unknown`.

R2 executa primeiro esses quatro recortes focais, cada qual com diretório
temporário e timeout próprio, mais controles de fronteira. Não repete o corpus
completo até os subcasos estarem atribuíveis e conclusivos. O PDF conclusivo
de R1 é histórico e não fornece crédito ao selo R2.

### O9 — escopo público estrito

P1462 acrescenta exatamente `format.svg`. `format.png`, `format.pdf` e
`format.html` continuam ausentes. O inventário P1357 deve observar somente
`BOTH +1`, `VANILLA_ONLY -1`, `CRYSTALLINE_ONLY` invariável:

| Perfil | BOTH | VANILLA_ONLY | CRYSTALLINE_ONLY |
|---|---:|---:|---:|
| default | 2000 | 21 | 42 |
| a11y-extras | 2003 | 24 | 42 |
| html | 2115 | 136 | 42 |
| html+a11y | 2118 | 139 | 42 |

### O10 — carriers e fechamento fail-closed

Os carriers documentais são clonáveis, comparáveis e têm default
`pretty == false`. O documento conserva decisão booleana; o tri-state pertence
à CLI. Valor não booleano sob a chave canônica falha fechado. Relayouts
calculam a opção do próprio candidato sem acumular estado anterior.

## 5. Ownership congelado R2

| Prompt L0 proprietário | Consumer exclusivo | Responsabilidade P1462 |
|---|---|---|
| `compiler/stdlib/format.md` | `01_core/src/compiler/stdlib/format.rs` | identidade e descoberta de `format.svg` |
| `compiler/eval/rules.md` | `01_core/src/compiler/eval/rules.rs` | validação, neutralidade vazia e transporte quando named existe |
| `compiler/layout.md` | `01_core/src/compiler/layout/mod.rs` | fachada pública do coletor puro |
| `compiler/layout/format_options.md` | `01_core/src/compiler/layout/format_options.rs` | extração root, ordem e rejeição nested |
| `entities/document_format_options.md` | `01_core/src/entities/document_format_options.rs` | carriers e defaults |
| `entities/mod.md` | `01_core/src/entities/mod.rs` | exposição do módulo de carriers |
| `entities/layout_types.md` | `01_core/src/entities/layout_types.rs` | transporte no `PagedDocument` |
| `infra/pipeline.md` | `03_infra/src/pipeline.rs` | orquestração, associação e resolução do override |
| `wiring.md` | `04_wiring/src/main.rs` | preservar `Option<bool>` no braço SVG |
| `wiring/tests/p1462-format-svg.md` | `04_wiring/tests/p1462_format_svg.rs` | prova externa integrada do binário |

O Núcleo `format-option-precedence` compartilha obrigação entre `pipeline.md`
e `wiring.md`, mas não legitima código e nunca aparece em `@prompt`. V15 ou
V26 inválido bloqueia resselo.

Não são owners P1462, salvo refutador e L0 anterior novo:
`02_shell/src/cli.rs`, `03_infra/src/export/svg.rs`,
`01_core/src/entities/style_chain.rs`, `entities/func.rs`,
`entities/module.rs` e fachadas genéricas de export.

## 6. Invariantes arquiteturais

1. L1 permanece puro e sem estado global mutável.
2. A unidade pública `format.svg` define identidade/default; não conhece CLI,
   documento, target nem bytes.
3. Eval reconhece o target estrutural exato. Named presente é validado antes
   de transportar; named ausente retorna neutralidade sem chave.
4. Nenhum owner posterior fabrica `false` para representar uma regra vazia.
5. Decisão explícita usa somente a chave canônica `format.svg.pretty`.
6. Classificação root/container e mensagem pertencem ao coletor puro L1; L3
   não percorre `Content` nem interpreta style keys.
7. Carriers P1462 não antecipam campos P1463–P1465.
8. A opção é extraída do conteúdo realizado final antes de devolver documento
   completo; erro não deixa documento provisório.
9. `PagedDocument` não transporta flag CLI, bytes ou handle de exporter.
10. O `Option<bool>` é preservado no braço SVG; demais targets conservam seus
    fios vigentes.
11. O exporter SVG recebe somente o booleano final e não aprende precedência.
12. O SVG interno de `html.frame` não consulta a opção documental externa.
13. Código nunca aponta diretamente para Núcleo Tekt.
14. Instrumento sem identidade atribuível, XML válido ou comparação da mesma
    fonte produz `Unknown`, nunca sucesso.

## 7. Controles e execução dos oráculos R2

### 7.1 Positivos (`Preserved`)

- O1 nos quatro perfis;
- O3 com regra vazia isolada e após `true`;
- todas as linhas O5 e O6;
- O7 com pares de fonte idêntica, inclusive `styled-root`;
- O8 com quatro subcasos focais atribuíveis;
- defaults/clone/igualdade dos carriers;
- inventário P1357 com delta exato.

### 7.2 Negativos (`Violated` com testemunha)

- construção manual;
- named desconhecido, tipo inválido, posicional e duplicado;
- decisão explícita nested em dois containers distintos;
- valor interno não booleano;
- filho futuro antecipado, vazamento de target ou owner indevido;
- regra vazia que escreve `false`.

### 7.3 Opacos (`Unknown` deliberado)

Baseline/hash divergente, binário ausente, timeout/crash, subcaso não
persistido, parser sem suporte, XML inválido, comparação entre fontes
distintas ou extração ambígua são `Unknown`. Só um controle criado para testar
opacidade pode esperar esse resultado. Nenhum deles fornece crédito de produto
ou mata mutante.

### 7.4 Ordem, repetição e budget

Depois de todos os recortes R2 afetados passarem, o corpus roda em `forward`,
`reverse` e `forward-repeat`; a classificação por id deve ser idêntica.
Diretórios temporários são novos por caso.

Preserva-se o budget: 120 minutos, até quatro revisões, no máximo três
execuções focais por revisão e três execuções completas antes do selo. Casos
afetados e controles vêm primeiro. Duas revisões consecutivas com mesmo vetor
e mesma causa obrigam reabrir o desenho. Budget esgotado é inconclusivo.

## 8. Catálogo discriminatório mínimo R2

Todo mutante válido deve ser rejeitado. Equivalente ou não compilável é
evidenciado e excluído do numerador e denominador. O único score selável é:

```text
mutation_score = mutantes válidos rejeitados / mutantes válidos = 1.0
```

| ID | Mutação negativa | Testemunha obrigatória |
|---|---|---|
| M01 | omitir `format.svg` ou registrar nome errado | O1 nos quatro perfis |
| M02 | default público `pretty == true` | O1 e ausência/ausência de O6 |
| M03 | permitir construção manual | O2 |
| M04 | aceitar/ignorar named desconhecido, tipo inválido, posicional ou duplicado | O3 e canais de diagnóstico |
| M05 | named explícito não transportar ou usar chave diferente | documento `true` sem CLI em O6 |
| M06 | fazer `#set format.svg()` escrever `format.svg.pretty = false` | `true → regra vazia` permanece pretty em O3/O5 |
| M07 | primeira decisão explícita vencer ou ordem ser invertida | `true → false` e `false → true` em O5 |
| M08 | aceitar nested, rejeitar root ou não detectar segundo container | O4 com root e dois containers |
| M09 | aceitar/fazer fallback de valor interno não booleano | O10 fail-closed |
| M10 | colapsar `None` em `false` antes do documento | documento `true` + CLI ausente em O6 |
| M11 | documento vencer `Some(false)` ou `Some(true)` | conflitos explícitos de O6 |
| M12 | vazar opção para PDF, PNG, HTML ou `html.frame` | quatro subcasos atribuíveis O8 |
| M13 | adicionar `format.png`, `format.pdf` ou `format.html` | O9 e inventário P1357 |
| M14 | publicar artefato/documento após erro nested | O4 e ausência de artefato |
| M15 | resolver precedência dentro do exporter SVG | ownership e invariantes 10–11 |
| M16 | L3 percorrer `Content` ou interpretar style key | ownership e invariante 6 |
| M17 | código apontar para Núcleo ou quebrar cardinalidade 1:1 | V15/V26 e ownership |
| M18 | alterar morfologia entre compacto/pretty mantendo XML válido | O7 em pares da mesma fonte, inclusive `styled-root` |

M06 R2 é o inverso do catálogo histórico R1. A classe válida faz o vazio
escrever `false`; o witness é `true → empty` ainda pretty. O M06 R1 não pode
entrar no denominador R2.

M12 só recebe kill quando o subcaso causal estiver persistido antes da
execução e o desvio for observado naquele target. Um timeout sem id é falha do
instrumento e `Unknown`, não kill. M18 não pode usar comparação
`styled-root`↔página vazia; deve introduzir diferença morfológica em uma das
duas serializações da mesma fonte.

Cada classe M01–M18 exige ao menos uma instância válida. `Unknown`,
`Pending`, `Excluded-Equivalent` e `Excluded-NonCompilable` não satisfazem a
cobertura. `Survived` entra no denominador, reduz score e bloqueia selo.

## 9. Gates R2 de pré-selo e materialização

Antes do selo R2:

1. conferir hashes de manifesto, contrato, Prompts, Núcleo e baseline R2;
2. verificar novamente que R1 permanece histórico, sem score ou selo;
3. construir oráculos R2 positivos, negativos e opacos sem acesso a candidato;
4. executar primeiro `true → empty`, os pares `styled-root` da mesma fonte e
   O8-PDF/PNG/HTML/HTML-FRAME com ids persistidos;
5. só depois executar corpus completo, três ordens e mutantes válidos;
6. exigir cobertura M01–M18, score `1.0` e preseal por autoridade que não
   edite artefatos julgados.

Somente o selo R2 libera teste RED e implementação. Gates finais incluem
testes focais L1/L3/L4, workspace, build, `crystalline-lint .`, inventário
P1357, probes bilaterais, `git diff --check`, repetição determinística e
verificação de que nenhuma entrada protegida mudou.

As oito falhas históricas P1450/P1451 do workspace não recebem crédito verde
nem escondem regressão P1462.

## 10. Política final de `Unknown` e limite do veredito

`Unknown` nunca é `Preserved`, nunca mata mutante, nunca aumenta score e nunca
autoriza selo. Budget esgotado, baseline divergente, instrumento opaco,
subcaso não persistido ou comparação entre fontes distintas produz diagnóstico
inconclusivo e exige correção focal antes de nova execução completa.

Mesmo um futuro veredito positivo cobre somente identidade/set rule de
`format.svg`, neutralidade da regra vazia, extração documental de `pretty`,
precedência do export SVG completo, neutralidade dos targets listados e
ownership Tekt correspondente. Não prova paridade global de `format.*`, SVG,
layout, exporters, Typst nem isolamento integral.
