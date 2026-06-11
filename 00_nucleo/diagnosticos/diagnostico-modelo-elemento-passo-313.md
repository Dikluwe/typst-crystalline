# Diagnóstico — modelo de elemento do cristalino (enum atual vs alternativas)

**Tipo**: diagnóstico-primeiro (ADR-0065), estilo P311a. **Zero código de
produto. Zero decisão executada.** Termina em recomendação; a decisão é humana
e trava qualquer implementação.
**Produzido por**: passo de materialização `typst-passo-313.md`.
**Data**: 2026-06-10.

### Nota sobre o número de passo (registro da escolha)

O prompt sugeriu o nome `diagnostico-modelo-elemento-passo-312a.md` mas pediu
para confirmar contra o estado do repositório. **P312 já está consumido** pela
materialização do Mapa de Migração (`typst-passo-312.md` →
`00_nucleo/mapa-migracao-vanilla-cristalino.md`, tópico distinto). O passo que
executa **este** diagnóstico é o 313 (`typst-passo-313.md`). Pela convenção de
nomenclatura (o diagnóstico leva o número do passo que o produz — cf.
`diagnostico-math-style-passo-311a.md` ← P311a), escolheu-se
**`passo-313`**. Nenhum ficheiro `*-passo-313*` ou `*-passo-312a*` existia
antes deste (`ls 00_nucleo/diagnosticos/ | grep -E '31[23]'` → vazio).

---

## §0 — Motivação (transcrição do requisito do dono do projeto)

> **Atomicidade para agentes.** O custo real de um passo de materialização é
> proporcional ao que a sessão de IA precisa **ler e editar** (tokens de
> contexto). O desenho atual (`Content` enum fechado, ADR-0026) concentra cada
> elemento novo num hub (`content.rs` + um match por backend), fazendo esse
> custo crescer com a cobertura. O vanilla era atómico (um módulo por elemento)
> mas pagava com macro-mágica (`#[elem]`), opaca para agentes porque o código
> gerado não está no texto. **A pergunta**: existe desenho com a atomicidade do
> vanilla, sem a opacidade das macros, mantendo a verificação mecânica do enum
> — e qual o custo de chegar lá a partir das 59+ variantes existentes?

Dois débitos implícitos da ADR-0026, levantados em análise externa
(2026-06-10), entram como contexto:

- **(a)** a StyleChain (DEBT sucessor aberto, registado em 99.E; magnitude L)
  pressupõe um **sistema de propriedades reificadas** (elemento+campo → chave
  resolvível) que o enum não fornece;
- **(b)** o churn do `content.rs` é custo recorrente por elemento (o hash
  estável por 27 passos quebrou no P311).

**Método deste documento**: toda contagem traz o comando que a produziu. Onde o
histórico git não permite medir, o documento declara *"não medível com as
fontes autorizadas"* — não estima de memória.

---

## §1 — Inventário: o pipeline de um elemento hoje

### §1.0 — Medições mecânicas de apoio

| Medição | Valor | Comando |
|---|---|---|
| Variantes do `Content` | **77** | parse do bloco `pub enum Content` em `content.rs` (L43–1231) |
| (auditoria F2 anterior contou 59) | +18 desde então | — |
| Linhas de `content.rs` | **5782** | `wc -l 01_core/src/entities/content.rs` |
| — só a definição do enum | **1189** (L43–1231) | parse do bloco |
| Refs `Content::` em L1 | **3994** | `grep -rh 'Content::' 01_core/src \| wc -l` |
| Refs `Content::` em L3 | **110** | idem em `03_infra/src` |
| Refs `Content::` em L2/L4 | **0** | idem |
| Backends que tocam `Content` | **layout (L1+L3) + PDF export (L3)** | `grep -l 'Content::' 03_infra/src` → `layout.rs`, `export/images.rs`, `export/gradients/mod.rs` |

**Constatação estrutural 1 — não há os 5 backends do vanilla.** O cristalino
tem **dois** consumidores de `Content`: o Layouter (`rules/layout` em L1; wiring
em `03_infra/src/layout.rs`) e o export PDF (L3). **Não existem backends
html / svg / render** (a varredura confirma: nenhum ficheiro fora de
layout/export referencia `Content::`). O custo "um match por backend" do
enunciado é, hoje, **um match de layout** (+ o export, que consome `FrameItem`,
não `Content` diretamente). Isto reduz materialmente o argumento "N backends"
— mas **não** elimina o problema, porque o hub real é o próprio `content.rs`
(827 refs `Content::` internas) e o sistema de introspeção (ver §1.2).

### §1.1 — Os matches por-variante dentro de `content.rs` (o hub real)

Cada variante nova obriga a editar os matches exaustivos de `content.rs`. Os
três maiores (o compilador força arm novo em todos):

| match (fn) | linhas | papel |
|---|---|---|
| `map_content` | **416** (L2035–2451) | recursão estrutural sobre filhos |
| `map_text` | **316** (L2451–2767) | transformação de texto terminal |
| `map_content`+`map_text` juntos | **732** | (os dois maiores; 13 % do ficheiro) |
| `eq` (`PartialEq`) | **228** (L1780–2008) | igualdade estrutural |
| `plain_text` | 222 (L1558–1780) | extração de texto |
| `get_field` | 17 (L2018–2035) | acesso a campo por nome |
| `hash` | 10 (L2008–2018) | delega a `content_hash` |

O relatório do P311 declara que `Content::MathStyled` exigiu **"5 sítios
exhaustive cobertos"** dentro de `content.rs`. Esse é o piso de edição do hub
por elemento, **independente** da complexidade do elemento.

### §1.2 — O caso `Heading` de ponta a ponta

`Heading` é uma variante **fina** — `Heading { level: u8, body: Box<Content> }`
(`content.rs:66`). Mesmo assim, entendê-la/editá-la exige ler:

| arquivo | refs `Heading` | o que vive aqui |
|---|--:|---|
| `entities/content.rs` | 26 | variante (L66); ctor `heading()` (L1266); arms em `plain_text`/`eq`/`get_field`/`map_content`/`map_text` |
| `rules/introspect.rs` | 119 | indexação, counter "heading", show/realize-equivalente |
| `entities/introspector.rs` | 22 | índice por kind |
| `rules/stdlib/mod.rs` | 15 | registo da função `#heading` no scope |
| `entities/element_payload.rs` | 12 | **2ª definição** do Heading (payload de introspeção) |
| `entities/element_kind.rs` | 12 | **3ª definição** (`ElementKind::Heading`) |
| `rules/introspect/locatable.rs` | 11 | predicado "é locatable" |
| `rules/introspect/extract_payload.rs` | 10 | match `Content::Heading → ElementPayload::Heading` |
| `entities/selector.rs` | 9 | selector por kind para `#show`/`query` |
| `rules/introspect/fixpoint.rs` | 8 | convergência da numeração |
| `entities/style_chain.rs` | 5 | `HeadingLevel` como propriedade de estilo |
| `ast/markup.rs` + `rules/parse/markup.rs` + `rules/lexer/markup.rs` | 5+4+1 | origem sintática `= ` |
| `rules/layout/mod.rs` + `outline.rs` + `cursor.rs` | 4+3+1 | arm do Layouter |
| `entities/{content_hash,syntax_kind,tag,source,show,resolved_label_store,counter_registry}.rs` | ~15 | hash, tags, índice, labels |

**Constatação estrutural 2 — a multiplicação de enums.** Porque `Heading` é
**locatable**, ele é declarado **três vezes** em enums fechados paralelos:
`Content::Heading`, `ElementKind::Heading`, `ElementPayload::Heading` — mais o
match-de-tradução `extract_payload` entre eles. `ElementPayload` (541 linhas,
`@prompt entities/element_payload.md`) **não é** uma reificação genérica: é um
**segundo hub fechado**, espelhando um subconjunto do `Content` só para o índice
de introspeção. Cada elemento locatável paga o custo do enum **duas vezes**.

### §1.3 — O piso: um elemento simples (`Divider`)

`Divider` (singleton, `Content::Divider`, P154B) é o chão do custo: sem campos,
não-locatável. Toca `content.rs` (variante + arms triviais em `eq`/`plain_text`/
`map_content` — retorno `self`/vazio), o ctor stdlib, e o arm do Layouter (emite
um `FrameItem::Shape::Line`). **Não** toca `ElementKind`/`ElementPayload`/
introspeção. Ordem de grandeza: ~5 sítios em `content.rs` + 1 ctor + 1 arm de
layout. **Mesmo o piso paga os 5 sítios do hub** — é o custo fixo do enum.

---

## §2 — Baseline: atomicidade nos passos reais (medição git)

**Método**: `git show <commit> --numstat`. A granularidade dos commits do
cristalino é **grossa** (vários passos por commit: `git log --oneline | grep
Passo` mostra `Passo 308 - 310`, `Passo 290 -301`, etc.). Consequências:

- **P311b (MathStyled)** está num commit limpo de um passo só
  (`515e46e76 "Passo 311"`) → **medível com precisão**.
- **P298 (MathOp)** está dentro do bundle `c823fca5b "Passo 290 -301"`
  (12 passos) → **não isolável por commit**. Declarado; medido por proxy
  estático (§2.2), não estimado de memória.
- **Terceiro passo limpo**: os commits de um-passo-só recentes (P305, P302–304,
  P280, P271, P262, P258, P255, P251) **não adicionam variante** ao `Content`
  (`git show <c> -- content.rs` → 0 variantes novas; são refinamentos). Os
  passos que adicionaram elementos foram todos **agrupados** (P284–286,
  P287–289, P290–301). **Conclusão honesta: não há terceiro elemento isolável
  com confiança** nas fontes autorizadas.

### §2.1 — P311b (MathStyled) — touch set preciso

`git show 515e46e76 --numstat -- '*.rs'` (29 ficheiros `.rs`; ~1038 inserções):

| categoria | ficheiros | linhas | atómico? |
|---|--:|--:|---|
| **Módulos próprios do elemento** (novos) | 2 | **420** | ✅ `entities/math_style.rs` (296) + `rules/stdlib/math_style.rs` (124) |
| **Hub** `content.rs` | 1 | 46 (+45/−1) | ❌ 5 sítios exhaustive |
| **Layouter math** `rules/math/layout/mod.rs` | 1 | 235 | ❌ algoritmo num ficheiro partilhado |
| **Wiring de registo** `rules/stdlib/mod.rs` | 1 | 160 | ❌ |
| **Wiring eval/introspect/layout** | 4 | ~27 | ❌ `eval/mod`(17)+`introspect`(4)+`locatable`(3)+`layout/mod`(3) |
| **Testes** `math/layout/tests.rs` | 1 | 131 | — |
| **Imposto de hash L0** (`@prompt-hash` +1/−1) | **18** | ~18 | ❌❌ ver abaixo |

**Constatação estrutural 3 — o imposto de hash de prompt partilhado.** Os 18
ficheiros com `+1/−1` **não** mudaram lógica: mudaram **uma linha**, o
`@prompt-hash`. P311 editou os prompts L0 partilhados `rules/stdlib.md` e
`rules/math/layout.md`; pela trava V5 (PromptDrift), **todo** ficheiro `.rs`
cuja linhagem aponta para esses prompts teve de re-espelhar o hash (10 ficheiros
`stdlib/*.rs` + 8 ficheiros `math/layout/*.rs`). Exemplo: `stdlib/text.rs`
mudou só `@prompt-hash aa4ca50f → 292ed749`. **Este é um segundo eixo de
não-atomicidade, distinto do `content.rs`**: prompts L0 de granularidade grossa
(`stdlib.md` cobre *todas* as funções stdlib) acoplam por hash dezenas de
ficheiros não relacionados ao elemento.

**Resumo P311b**: 29 ficheiros `.rs` tocados, dos quais só **2** são o elemento
em si (420 linhas atómicas, ~40 %); **6** são hub/wiring real; **18** são puro
imposto de hash; mais ~4 prompts L0 (`content.md` +112, `math_style.md` +287
novo, `stdlib.md` +107, `math/layout.md` +63).

### §2.2 — P298 (MathOp) — proxy estático (não isolável por commit)

Footprint atual de `MathOp` (`grep -rc 'MathOp' 01_core/src`), como proxy do
que uma sessão toca:

| arquivo | refs |
|---|--:|
| `rules/eval/tests.rs` | 24 |
| `rules/stdlib/mod.rs` | 20 |
| `entities/content.rs` | 8 |
| `rules/stdlib/structural.rs` | 7 |
| `rules/math/layout/mod.rs` | 7 |
| `rules/eval/math.rs` | 6 |
| + `math/layout/attach`, `introspect`, `lexer/math`, `layout/mod`, `locatable`, `eval/mod` | 1–2 cada |

O `diagnostico-math-op-passo-298.md` enumera os sítios `content.rs` que MathOp
tocou: `plain_text()`, `PartialEq`, `map_content()` (recursão no `text`),
`map_text()` (terminal). **Mesmo padrão do §1.2**: hub + layouter math + stdlib
+ eval. Coerente com P311b; nº de ficheiros da mesma ordem.

### §2.3 — Tabela "passo × toque"

| passo | elemento | ficheiros `.rs` | linhas (≈) | **dos quais hub+wiring+imposto** | método |
|---|---|--:|--:|---|---|
| **P311b** | MathStyled | 29 | ~1038 | hub 46 + wiring 187 + **18-file hash tax** + layouter-partilhado 235 | git numstat (preciso) |
| **P298** | MathOp | ~12 (footprint) | não medível por commit | hub 8 sítios + stdlib 20 + layouter 7 | proxy estático + diagnóstico P298 |
| (3.º) | — | — | — | — | **não isolável com confiança** (commits agrupados) |

**O número que os candidatos prometem reduzir**: por elemento, ~6 ficheiros de
hub/wiring + um imposto de hash de até ~18 ficheiros, **antes** de o elemento
ter qualquer lógica própria.

---

## §3 — Os candidatos, com números do repositório

Legenda de custo: **tokens/contexto** = ficheiros+linhas a ler/editar por
elemento novo (proxy de §1–§2).

### B — enum atual (ADR-0026): baseline

- **Custo/elemento**: §2 — ~6 ficheiros hub/wiring + 5 sítios em `content.rs` +
  imposto de hash L0 (até ~18 ficheiros se o prompt stdlib for tocado) + o
  elemento. Locatável dobra (ElementKind+ElementPayload+extract_payload).
- **Verificação mecânica**: máxima — o compilador força arm em cada match
  exaustivo.
- **Débito**: `content.rs` 5782 linhas; cresce monotonicamente; hash quebra
  (P311) por edição fundacional.

### D — enum fino com delegação por módulo

Cada variante vira `Nome(Arc<nome::Nome>)`; campos+regras+layout vivem no módulo
`entities/elements/nome.rs`; os matches de backend viram dispatchers de 1 linha
(`Content::Nome(e) => e.layout(ctx)` via trait).

- **Migração das 77 variantes (mecânico, por variante)**: extrair os campos de
  cada arm dos 6 matches de `content.rs` (`plain_text`/`eq`/`hash`/`get_field`/
  `map_content`/`map_text`) para um `impl Element for Nome`. Custo dominado por
  `map_content` (416) + `map_text` (321): cada variante tem 1 arm em cada;
  mover 77×~6 arms. **Estimativa: L** (mecânico mas amplo; ~77 módulos novos;
  os 6 matches gigantes encolhem para dispatchers).
- **Custo/elemento novo depois**: **1 módulo** (`nome.rs`, toda a lógica) + **1
  linha** no enum (`Nome(Arc<Nome>)`) + **1 linha por dispatcher** (layout,
  eval se sintático, introspect se locatável). O hub deixa de ter os 5 sítios.
- **Exaustividade**: **intacta** — o `match Content` continua exaustivo; o
  compilador força a linha-dispatcher nova. Verificar: nenhum match perde
  exaustividade (os dispatchers são `Content::Nome(e) => …`, exaustivos por
  construção).
- **Não resolve**: o imposto de hash L0 (continua se os prompts forem grossos);
  a dupla-definição locatável (ElementPayload) só sai se o trait incluir o
  payload; **não** entrega o sistema de propriedades da StyleChain (§4).

### F — propriedades reificadas

Nó genérico `{ kind: ElementKind, props: PropMap }` + descritor por elemento em
módulo próprio + **uma tabela const** (elemento → campos/tipos/defaults).

- **Redesenho (tipos centrais que mudam)**: `Content` deixa de ser enum-de-77
  e vira (parcialmente) dados; os ~7 matches de `content.rs` viram **lookup na
  tabela**; `eq`/`hash`/`get_field` tornam-se genéricos sobre `PropMap`. O
  `Style`/`StyleDelta` (hoje propriedades **hardcoded**: 1 variante + 1 campo
  `Option<T>` por propriedade — ver §4) tornam-se chaves na **mesma** PropMap.
- **Para onde migra a verificação mecânica** (o que o compilador deixa de
  dar): opção concreta — **um teste que varre a tabela const × os backends**
  (para cada `(kind, field)` declarado, assertar que existe handler) **ou** uma
  trava nova do `crystalline-lint` (V-novo: "todo descritor na tabela tem
  handler de layout"). Sem isto, F troca erro-de-compilação por erro-de-runtime.
- **Custo/elemento novo depois**: **1 descritor** (módulo) + **1 entrada** na
  tabela const. Sem arms, sem dispatchers. O mais barato dos três.
- **O que entrega de graça à StyleChain**: **tudo** — é a mesma PropMap (§4).
- **Custo de chegar lá a partir de B**: **L+** (o maior); redesenha o núcleo.
- **Já existe meio caminho?** **Não.** `ElementPayload`/`ElementKind` são
  enums fechados (segundo hub), **não** uma PropMap genérica. F começa do zero
  no mecanismo de propriedades.

### E — geração por `macro_rules` (registrar e descartar)

`#[elem]`-equivalente em `macro_rules`. **Descartado pela razão medida**: código
gerado é o ruído que a lente quantificou (resíduo `__ComemoCall`/`__ComemoSurface`
visível no mapa de migração; medição 0077) e a indireção custa tokens de
**leitura** — o agente não vê o código gerado no texto. **Contradiz o requisito
de origem (§0)**: atomicidade-para-agentes exige texto legível, não expansão.

### §3.x — Matriz final

| candidato | tokens/contexto por elemento novo | verificação mecânica | custo migração desde B | efeito na StyleChain (§4) | riscos |
|---|---|---|---|---|---|
| **B** enum atual | ~6 ficheiros hub/wiring + 5 sítios `content.rs` + imposto hash; locatável dobra | **máxima** (compilador) | — (baseline) | nenhum (propriedades continuam hardcoded) | `content.rs` cresce sem teto; hash frágil |
| **D** enum fino + delegação | **1 módulo + 1 linha enum + 1 linha/dispatcher** | **intacta** (enum exaustivo) | **L** (77 módulos; mecânico) | nenhum direto (mas compatível como etapa, §4) | trabalho amplo; não toca imposto hash nem ElementPayload |
| **F** propriedades reificadas | **1 descritor + 1 entrada de tabela** (o mais barato) | **migra p/ teste-varre-tabela ou trava lint** (perde compilador) | **L+** (redesenha núcleo) | **resolve-a** (mesma PropMap) | maior risco; verificação mecânica passa a depender de teste/lint |
| (E macro) | baixo na escrita, **alto na leitura** | macro | M | — | **descartado**: viola §0 |
| ref. vanilla | 1 módulo/elemento (atómico) | proc-macro `#[elem]` | — | tem sistema de props | opacidade (código gerado) |

---

## §4 — Interação com o DEBT da StyleChain

**Constatação verificada no código**: o sistema de estilo atual reifica
propriedades **à mão**. `entities/style.rs` (`enum Style`) tem **10** variantes
hardcoded (`Bold`, `Italic`, `Size`, `Fill`, `HeadingLevel`, `Lang`, `Weight`,
`Tracking`, `Leading`, `Font`); `entities/style_chain.rs` (`struct
StyleDelta`) tem **um campo `Option<T>` por propriedade** (os mesmos 10: bold,
italic, size, fill, heading_level, weight, tracking, leading, lang, font). **Cada
propriedade de estilo nova = 1 variante de `Style` + 1 campo de `StyleDelta` +
accessor + arm em `eval_set_rule`.** Não há mapa genérico `(elemento, campo) →
chave resolvível`.

O comentário em `style_chain.rs` é explícito: *"a coexistência [StyleDelta vs
StyleChain] é intencional até que o pipeline #set/#show e o Layouter migrem
para StyleChain directamente (DEBT sucessor registado em 99.E)"*. Esse DEBT
sucessor — materializar a StyleChain real — **exige** o sistema de propriedades
genérico.

**Logo: o caminho A do DEBT StyleChain e o candidato F são a mesma obra.** Ambos
constroem a PropMap `(elemento+campo → valor resolvível)`. Consequências para a
sequência:

- **Fazer D agora e A depois constrói propriedades duas vezes?** Não
  necessariamente. D move a **lógica do elemento** para módulos; não toca o
  mecanismo de propriedades. A obra de propriedades (A/F) fica intocada por D.
  Mas D **não** adianta A — são eixos ortogonais (lógica-por-elemento vs
  propriedades-genéricas).
- **D é etapa intermediária de F?** **Sim, de forma natural.** Os módulos de
  elemento do D (`entities/elements/nome.rs` com `impl Element`) são exatamente
  os **descritores** que F precisa: um módulo por elemento já existe; F
  acrescenta a tabela const + a PropMap, e o `impl Element` de D ganha
  `fn descriptor() -> &'static ElementDescriptor`. D **não** constrói
  propriedades que F deite fora; constrói o **continente** (módulo por
  elemento) que F preenche.

**Resposta pela forma do código**: D primeiro reduz o custo-por-elemento já
(§3) e prepara o terreno de F sem desperdício; F (= StyleChain real) entra
quando o DEBT 99.E for atacado, reusando os módulos de D como descritores.

---

## §5 — Recomendação (primária + secundária) e trava

### Primária — **D incremental agora; F como destino declarado**

Os números sustentam D como primeiro movimento: corta o custo-por-elemento de
"~6 ficheiros hub/wiring + 5 sítios `content.rs`" para "1 módulo + 1 linha +
dispatchers de 1 linha", **mantendo a exaustividade do compilador** (verificação
mecânica intacta — o risco que F introduz). A migração das 77 variantes é **L**
mas **mecânica** (mover arms dos 6 matches gigantes para `impl Element`), e pode
ser **incremental** (variante a variante; o enum tolera mistura
`Nome(Arc<Nome>)` e `Nome { … }` durante a transição). Sequência proposta:

1. **D-incremental** (L, fatiável em vários S/M): trait `Element` + migração
   por lotes; `map_content`/`map_text`/`eq` encolhem para dispatchers.
   Absorver `ElementPayload` no trait fecha a dupla-definição locatável (§1.2).
2. **ADR declarando F como destino** (S): registra que os módulos de D são
   descritores-em-construção de F; congela a forma do `impl Element` para ser
   compatível.
3. **F = StyleChain real** (L+), **quando** o DEBT 99.E for atacado: a PropMap
   serve elemento e estilo; **antes** disso, adicionar a trava de verificação
   (teste-varre-tabela **ou** regra `crystalline-lint` nova) para não perder o
   compilador.

**Trava adicional sugerida com D**: prompt L0 **por elemento** (não o
`stdlib.md` monolítico), para matar o imposto de hash de §2.1 — hoje editar uma
função stdlib re-hasheia ~18 ficheiros.

### Secundária — **F direto**

Se o DEBT StyleChain (99.E) for prioridade imediata e a equipa aceitar trocar a
verificação-por-compilador por teste/lint, ir direto a F evita a migração-dupla
conceptual. Custo **L+** e risco maior (a verificação mecânica deixa de ser o
compilador). Só se justifica se A/StyleChain já está agendado.

### Trava (decisão humana)

**Nenhuma implementação começa sem decisão humana.** Este documento recomenda;
não executa. O arranque de D (ou F) depende de o dono escolher a sequência e o
tamanho do primeiro lote.

**Tamanho estimado**: D-incremental = **L** (fatiável em S/M por lotes de
variantes); ADR-destino = **S**; F = **L+** (adiável até 99.E).

---

## §6 — O princípio a registrar (texto proposto; adoção é decisão humana)

Proposta de entrada de ADR **ou** LESSONS — **"Atomicidade para agentes"** como
força arquitetural de primeira classe do Tekt:

> **Atomicidade para agentes.** O custo de um passo de materialização é
> proporcional ao contexto (tokens) que uma sessão de IA precisa ler e editar
> para o completar. Logo:
> 1. **Hubs concentradores são anti-padrão de manutenção por IA.** Um ficheiro
>    que cresce com a cobertura (ex.: `content.rs`, 5782 linhas; `enum` de 77
>    variantes; matches de 400+ linhas) faz o custo-por-feature crescer com a
>    cobertura já entregue. Preferir um módulo por unidade de feature.
> 2. **Acoplamento por linhagem deve ser fino.** Prompts L0 de granularidade
>    grossa (um prompt para toda a stdlib) propagam o `@prompt-hash` a dezenas
>    de ficheiros a cada edição (medido: ~18 ficheiros por P311). Um prompt por
>    elemento/função mantém o toque local.
> 3. **A verificação mecânica não pode depender de o agente lembrar.** Se um
>    desenho troca erro-de-compilação (exaustividade do enum) por convenção,
>    tem de repor a garantia com um **teste que varre** ou uma **trava de
>    linter** — caso contrário a regressão fica silenciosa entre sessões (a
>    classe da falha F4 do `diagnostico-bloqueio-processo-2026-06-09.md`).
>
> Corolário de sequência: redesenhos que aumentam a atomicidade (D, F) são
> investimento contra o custo marginal crescente; medir o custo-por-elemento
> (ficheiros tocados, dos quais hub/imposto) é a métrica de saúde.

---

## Fontes consultadas

Código `01_core/`–`03_infra/`; `git log/show` (commits `515e46e76`,
`c823fca5b`); `00_nucleo/adr/` (0026, 0038, 0065, 0098, 0102/0103);
`diagnostico-math-op-passo-298.md`; `diagnostico-bloqueio-processo-2026-06-09.md`;
`mapa-migracao-vanilla-cristalino.md`; `CLAUDE.md`. Pastas restritas
(`materialization/`, `context/`) não varridas; `typst-passo-313.md` lido por
path explícito.
