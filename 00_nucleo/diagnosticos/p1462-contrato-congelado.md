# P1462 — contrato observável congelado de `format.svg.pretty`

**Estado:** CONGELADO PRÉ-CANDIDATO; ainda não selado pelo gate discriminatório

**Manifesto causal:** `00_nucleo/diagnosticos/p1462-manifesto.json`

**SHA-256 do manifesto:**

`de5d15c28576da4436fdaf05ee1c92416e001577ca07d4d5761f7a6bfb7885b4`

**Baseline imutável:**

`514b2ae8f35b661127e38620dcbdaefa9e158624`

**Tree do baseline:**

`c12c3dac7ba85e0bf9f89b21c5e88203ff84b48e`

**Vanilla ratificado:**

`586e1bd43fae6c9a973218163d3165c53ab8d16d`

**Regime:** protocolo completo de materialização Tekt segregada

**Atestação proporcional:** executado sem atestação de isolamento

Este contrato foi escrito antes de existir ou ser lido qualquer patch
candidato. O checkout e o filesystem são compartilhados; a separação é de
papel, entrada, capacidade, ordem causal e artefato, não de isolamento técnico.
Hashes identificam as entradas e a ordem, mas não provam isolamento.

## 1. Autoridade, gate e significado de “congelado”

As fontes normativas são os dez Prompts L0 e o Núcleo Tekt enumerados no
manifesto. O Passo 1462 coordena a execução, mas não substitui o L0. As ADRs
0107, 0108, 0127 e 0129 governam respectivamente a fronteira língua/mecânica,
a disciplina medir-antes-de-decidir, o gate humano e a cardinalidade dos
owners.

“Congelado” significa que este conjunto de observáveis, matriz, owners,
invariantes, controles e discriminadores não pode ser ajustado a uma
implementação futura. Não significa “selado”: o selo só pode nascer depois de
oráculos independentes, mutantes válidos, repetição/reordenação e mutation
score `1.0`.

O gate ADR-0127 foi confirmado em 2026-09-19. O coordenador informou ao humano
o commit documental
`514b2ae8f35b661127e38620dcbdaefa9e158624`, perguntou explicitamente se
autorizava testes RED/implementação P1462 e recebeu `confirmo` no turno
imediatamente seguinte. O transcript é externo ao repositório e a evidência é
atestada pelo coordenador. A confirmação satisfaz o gate humano, mas não
antecipa o selo discriminatório: teste RED e código continuam proibidos até o
selo.

Qualquer mudança no baseline, Passo, Prompts, Núcleo, manifesto ou neste
contrato invalida a cadeia desde a primeira entrada alterada. Um hash novo não
autoriza reaproveitar silenciosamente o selo antigo.

## 2. Fronteira congelada

P1462 cobre somente:

1. o filho público `format.svg` da raiz `format` já existente;
2. a propriedade documental booleana `pretty`, com default `false`;
3. a set rule `#set format.svg(pretty: ...)` e seus diagnósticos;
4. a extração da decisão documental no conteúdo root e a rejeição em
   containers;
5. o transporte da opção até o documento paginado;
6. a precedência `CLI explícito > documento > false` no export SVG completo;
7. a preservação das fronteiras de target e da cardinalidade L0↔consumer.

Ficam fora: `format.png`, `format.pdf`, `format.html`, geometria, texto,
fontes, destinations, SVG interno de `html.frame`, export PDF/PNG/HTML,
`--compact`, PDF tags, feature gates e equivalência global de SVG. Nenhum
resultado P1462 pode fechar esses eixos por inferência.

## 3. Classificação ADR-0107/0108

São observáveis de língua ou interface pública entre camadas:

- existência, tipo, nome, representação, namespace e default de `format.svg`;
- validação de argumentos e os diagnósticos públicos medidos;
- impossibilidade de construção manual;
- efeito, ordem e fronteira top-level da set rule;
- mensagem de erro de container;
- precedência entre CLI, documento e default;
- modo compacto ou indentado solicitado e morfologia SVG normalizada;
- ausência de efeito nos demais targets e filhos ainda reservados.

São mecânica livre: macro/trait upstream, tipo de coleção, endereços, downcast,
número de passes, representação Rust interna, escolha de contador, layout de
struct, bytes exatos, tamanho e SHA-256 do SVG. Bytes e contagens da fixture
medida identificam aquela medição; não são expected normativo.

A inferência aceita pelo L0 é que a comparação entre travessia exaustiva e
travessia dos envelopes transparentes de raiz distingue ocorrência root de
ocorrência em container sem enumerar nominalmente todos os containers. Ela é
refutada por variante não percorrida, perda da ordem lexical, set rule sem o
carrier esperado ou falso positivo em conteúdo root legítimo. Um refutador
reabre o L0; não autoriza exceção ad hoc neste contrato.

## 4. Observáveis obrigatórios

### O1 — identidade pública

Nos perfis `default`, `a11y-extras`, `html` e `html+a11y`:

```typ
#repr((type(format.svg), format.svg, format.svg.pretty))
```

deve observar exatamente `(function, svg, false)`. `std.format.svg` e alias
lexical preservam identidade nominal e representação. `format` continua um
módulo independente de target/feature.

### O2 — construção manual proibida

`format.svg()` falha e o diagnóstico contém exatamente:

```text
cannot be constructed manually
```

Nenhum `Content` construível nasce dessa chamada.

### O3 — domínio e validação da set rule

- `#set format.svg(pretty: true)` e `pretty: false` são aceitos em root;
- uma set rule sem named materializa a decisão explícita `false`;
- named desconhecido `other` falha com
  `unexpected argument: other`;
- `pretty: 1` falha com `expected boolean, found integer`;
- argumento posicional, named duplicado ou tipo incompatível falha pelo canal
  normal de erro, nunca como warning de target ignorado;
- nenhum campo além de `pretty` é aceito.

### O4 — fronteira top-level/container

Uma regra root é válida. A mesma classe de regra dentro de container falha
com a mensagem exata:

```text
format set rules are not allowed inside of containers
```

O caso de falha não publica artefato SVG de sucesso nem `PagedDocument`
parcial. A prova deve usar ao menos dois containers estruturalmente distintos
e também um controle root legítimo.

### O5 — ordem lexical/cascaded

Entre decisões root repetidas, vence a decisão lexical/cascaded mais interna
ou posterior. Os testemunhos mínimos são:

| Sequência root | Decisão documental efetiva | Forma sem CLI |
|---|---:|---|
| `true → false` | `false` | compacta |
| `false → true` | `true` | indentada |
| `true → set rule sem named` | `false` | compacta |

`Sequence` e `Styled` transparentes de raiz preservam essa ordem. Um `Styled`
sob container continua nested e deve falhar.

### O6 — matriz de precedência SVG

`None` representa ausência de opção explícita do caller. `Some(true)` e
`Some(false)` representam escolhas explícitas e devem sobreviver até o ponto
em que a opção documental está disponível.

| CLI / override | Documento | Booleano final | Serialização |
|---|---:|---:|---|
| ausente / `None` | sem regra (`false`) | `false` | compacta |
| ausente / `None` | `true` | `true` | indentada |
| ausente / `None` | `false` | `false` | compacta |
| explícito `false` / `Some(false)` | `true` | `false` | compacta |
| explícito `true` / `Some(true)` | `false` | `true` | indentada |
| explícito `false` / `Some(false)` | sem regra | `false` | compacta |
| explícito `true` / `Some(true)` | sem regra | `true` | indentada |

Esta matriz aplica integralmente as quatro claims do Núcleo
`format-option-precedence`: override explícito vence, ausência delega ao
documento, default vem depois e colapso antecipado é proibido.

### O7 — forma de saída, não identidade de bytes

As duas formas precisam ser XML válido. A forma compacta não exibe a
indentação pedida; a forma pretty exibe indentação e quebras de linha. Depois
de normalização XML que remove somente diferenças de serialização, a
morfologia deve ser igual. Tamanho, número fixo de LF e SHA-256 não podem ser
usados como aceite.

### O8 — isolamento de targets

`format.svg.pretty` não altera PDF, PNG, HTML nem o SVG interno produzido para
`html.frame`. Esses caminhos mantêm seus defaults, flags e modos próprios.
Falha de um target não coberto não pode ser convertida em evidência positiva
de neutralidade.

### O9 — escopo público estrito

P1462 acrescenta exatamente `format.svg`. `format.png`, `format.pdf` e
`format.html` continuam ausentes, sem placeholders, aliases globais ou filhos
antecipados.

No inventário público P1357, o delta é exatamente `BOTH +1`,
`VANILLA_ONLY -1`, `CRYSTALLINE_ONLY` invariável:

| Perfil | BOTH | VANILLA_ONLY | CRYSTALLINE_ONLY |
|---|---:|---:|---:|
| default | 2000 | 21 | 42 |
| a11y-extras | 2003 | 24 | 42 |
| html | 2115 | 136 | 42 |
| html+a11y | 2118 | 139 | 42 |

### O10 — carriers e fechamento fail-closed

Os carriers documentais públicos são clonáveis, comparáveis e têm default
`pretty == false`. O documento paginado conserva uma decisão booleana, não o
tri-state da CLI. Valor não booleano sob a chave canônica é estado interno
inválido e falha fechado. Relayouts calculam a opção do próprio candidato sem
acumular a iterações anteriores.

## 5. Ownership congelado

| Prompt L0 proprietário | Consumer exclusivo | Responsabilidade P1462 |
|---|---|---|
| `compiler/stdlib/format.md` | `01_core/src/compiler/stdlib/format.rs` | identidade e descoberta de `format.svg` |
| `compiler/eval/rules.md` | `01_core/src/compiler/eval/rules.rs` | validação e transporte da set rule |
| `compiler/layout.md` | `01_core/src/compiler/layout/mod.rs` | fachada pública do coletor puro |
| `compiler/layout/format_options.md` | `01_core/src/compiler/layout/format_options.rs` | extração root, ordem e rejeição nested |
| `entities/document_format_options.md` | `01_core/src/entities/document_format_options.rs` | carriers documentais e defaults |
| `entities/mod.md` | `01_core/src/entities/mod.rs` | exposição do módulo de carriers |
| `entities/layout_types.md` | `01_core/src/entities/layout_types.rs` | transporte no `PagedDocument` |
| `infra/pipeline.md` | `03_infra/src/pipeline.rs` | orquestração, associação e resolução do override |
| `wiring.md` | `04_wiring/src/main.rs` | preservar `Option<bool>` no braço SVG |
| `wiring/tests/p1462-format-svg.md` | `04_wiring/tests/p1462_format_svg.rs` | prova externa integrada do binário |

O Núcleo `format-option-precedence` compartilha obrigação entre `pipeline.md`
e `wiring.md`, mas não legitima código e nunca pode aparecer em `@prompt`.
Cada consumer continua com exatamente um Prompt proprietário. V15 ou V26
inválido bloqueia resselo e fechamento.

Não são owners P1462, salvo refutador e L0 anterior novo:
`02_shell/src/cli.rs`, `03_infra/src/export/svg.rs`,
`01_core/src/entities/style_chain.rs`, `entities/func.rs`,
`entities/module.rs` e fachadas genéricas de export. Mudança meramente para
fazer compilar também precisa respeitar o owner existente; não ganha semântica
P1462 por proximidade.

## 6. Invariantes arquiteturais

1. L1 permanece puro: sem filesystem, rede, relógio, ambiente ou estado global
   mutável.
2. A unidade pública `format.svg` só descobre identidade/default; não conhece
   CLI, documento, target nem bytes.
3. Eval reconhece somente o target estrutural exato e valida antes de
   transportar. Não amplia o fallback genérico.
4. A decisão trafega sob a chave canônica `format.svg.pretty`; nenhuma chave
   textual alternativa é equivalente.
5. A classificação root/container e a mensagem pública pertencem ao coletor
   puro L1. L3 não reimplementa traversal nem interpreta chaves de style.
6. `DocumentFormatOptions`/`SvgFormatOptions` contêm somente o subconjunto
   P1462 e registram sua incompletude para P1463–P1465.
7. A opção é extraída do conteúdo realizado final do candidato antes de um
   `PagedDocument` completo ser devolvido. Erro não deixa documento provisório.
8. O `PagedDocument` transporta o valor documental; não transporta flags CLI,
   bytes ou handles de exporter.
9. O `Option<bool>` da CLI é preservado no braço SVG. PDF e HTML continuam a
   receber a decisão P1360 vigente; PNG continua neutro.
10. O exporter SVG recebe apenas o booleano final e não aprende precedência.
11. O SVG interno de `html.frame` não consulta a opção do documento externo.
12. Nenhum código aponta diretamente para Núcleo Tekt.
13. Ausência, erro, crash, timeout ou opacidade nunca viram sucesso por default.

Essas invariantes congelam responsabilidades já decididas pelo L0; não
escolhem estruturas, algoritmos ou passos internos adicionais.

## 7. Controles e protocolo dos oráculos

### 7.1 Positivos (`Preserved`)

- O1 nos quatro perfis;
- as sete linhas de O6;
- as três ordens de O5;
- XML válido e morfologia normalizada igual;
- defaults/clone/igualdade dos carriers;
- inventário P1357 com o delta exato.

### 7.2 Negativos (`Violated` com testemunha)

- construção manual;
- named desconhecido, tipo inválido, posicional e duplicado;
- nested em dois containers estruturalmente distintos;
- valor interno não booleano sob a chave canônica;
- qualquer filho futuro antecipado;
- qualquer vazamento para outro target;
- qualquer ownership, pin V26 ou consumer fora da fronteira congelada.

### 7.3 Opacos (`Unknown` deliberado)

Controles de harness devem provar que baseline divergente, binário sem pin,
timeout/crash, parser XML sem suporte, XML inválido ou extração ambígua são
classificados como `Unknown`. Esses casos não satisfazem observável do produto
e não matam mutante. Só o caso criado explicitamente para testar opacidade
pode esperar `Unknown`.

### 7.4 Determinismo e ordem

O mesmo corpus roda em `forward`, `reverse` e `forward-repeat`. A
classificação por id deve permanecer idêntica; não se exige identidade de
timestamps ou paths temporários. Cada caso usa diretório temporário isolado e
verifica exit, canais, presença/ausência do artefato, validade XML e o
observável próprio.

Casos afetados e controles de fronteira rodam antes do corpus completo.
Aplica-se o budget do manifesto: 120 minutos, até quatro revisões, no máximo
três execuções completas antes do selo e até três execuções focais por revisão.
Duas revisões consecutivas sem delta discriminatório na mesma causa obrigam
reabrir o desenho; não autorizam afrouxar expected.

## 8. Catálogo discriminatório mínimo

Todo mutante semanticamente válido deve ser rejeitado. Mutante equivalente ou
não compilável recebe evidência e sai do numerador e denominador. O score é:

```text
mutation_score = mutantes válidos rejeitados / mutantes válidos
```

O único valor selável é `1.0`.

| ID | Mutação negativa | Testemunha que deve rejeitá-la |
|---|---|---|
| M01 | omitir `format.svg` ou registrá-lo sob outro nome | O1 nos quatro perfis |
| M02 | default público `pretty == true` | O1 e ausência/ausência de O6 |
| M03 | permitir construção manual | O2 |
| M04 | aceitar/ignorar named desconhecido, tipo inválido, posicional ou duplicado | O3 e canais de diagnóstico |
| M05 | não transportar a set rule ou usar chave diferente | documento `true` sem CLI em O6 |
| M06 | set rule sem named não materializar `false` | terceira sequência de O5 |
| M07 | primeira regra vencer, ou ordem ser invariavelmente invertida | as duas inversões de O5 |
| M08 | aceitar nested, rejeitar root ou não detectar o segundo container | O4 com root e dois containers |
| M09 | aceitar/fazer fallback de valor interno não booleano | O10 fail-closed |
| M10 | colapsar `None` em `false` antes de consultar o documento | documento `true` + CLI ausente em O6 |
| M11 | documento vencer `Some(false)` ou `Some(true)` | os dois conflitos explícitos de O6 |
| M12 | vazar opção para PDF, PNG, HTML ou `html.frame` | controles de isolamento O8 |
| M13 | adicionar `format.png`, `format.pdf` ou `format.html` | O9 e inventário P1357 |
| M14 | publicar artefato/documento após erro nested | O4, ausência de artefato |
| M15 | resolver precedência dentro do exporter SVG | ownership estático e invariantes 9–10 |
| M16 | L3 percorrer `Content` ou interpretar a chave de style | ownership estático e invariante 5 |
| M17 | código apontar para o Núcleo ou quebrar cardinalidade 1:1 | V15/V26 e tabela de ownership |
| M18 | alterar bytes, geometria ou morfologia entre compact/pretty | XML, normalização morfológica e controles semânticos O7 |

Um assert que só confirma que um processo falhou sem identificar o observável
é opaco e não mata mutante. Um mutante que sobreviva a qualquer testemunha
relevante bloqueia o selo, mesmo que todos os testes ordinários estejam verdes.

## 9. Gates de pré-selo e de materialização

Antes do selo discriminatório:

1. conferir novamente hashes de manifesto, Prompts, Núcleo e baseline;
2. conferir o registro já atestado da confirmação humana ADR-0127 posterior
   ao commit causal;
3. materializar oráculos positivos, negativos e opacos sem acesso ao patch;
4. executar casos focais, controles, corpus completo e as três ordens;
5. rejeitar todos os mutantes válidos e obter score `1.0`;
6. emitir preseal por autoridade que não edite os artefatos julgados.

Somente depois podem nascer implementação e testes A/B a partir das entradas
seladas. O implementador recebe Prompts e contrato público selado, não a
redação privada dos ataques. Gates finais devem incluir testes focais L1/L3/L4,
workspace, build, `crystalline-lint .`, inventário P1357, probes bilaterais,
`git diff --check`, repetição determinística e conferência de que nenhuma
entrada protegida mudou.

As oito falhas históricas P1450/P1451 registradas no baseline não recebem
crédito verde nem podem esconder regressão P1462. Resultado novo fora do
recorte é separado e classificado, nunca absorvido por contagem agregada.

## 10. Política final de `Unknown` e limite do veredito

`Unknown` nunca é `Preserved`, nunca mata mutante, nunca aumenta score e nunca
autoriza selo. Budget esgotado, invalidação do gate ADR-0127, baseline
divergente ou instrumento opaco produzem diagnóstico inconclusivo. A saída
correta é refinar observabilidade/protocolo com nova cadeia, reduzir escopo com
autorização humana ou encerrar inconclusivo.

Mesmo um futuro veredito `Conformant` ou `Verified` cobrirá somente o fragmento
registrado: identidade e set rule de `format.svg`, extração documental de
`pretty`, precedência do export SVG completo e neutralidade dos eixos listados.
Não provará paridade global de `format.*`, SVG, layout, exporters, Typst ou
isolamento integral da execução.
