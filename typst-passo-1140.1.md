# Passo 1140.1 — Decomposição dos kinds públicos e preparação por atomização

**Data:** 2026-08-23  
**Origem:** P1140, classe `WRONG_KIND`  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Natureza:** arquitetura e preparação; não consome P1141, reservado a SVG  
**Gate:** mudança de contrato/comportamento público — ADR-0127

## 1. Objetivo

Decompor os dez paths `WRONG_KIND` de P1140 em unidades implementáveis e
preparar os módulos donos quando uma atomização behavior-preserving melhorar a
localidade da lógica.

O passo não autoriza converter imediatamente todos os bindings. Primeiro:

1. separa os casos por semântica real;
2. atomiza apenas monólitos onde a unidade dona já é inequívoca;
3. atualiza e ressela os L0 estruturais;
4. redige o contrato L0 da mudança pública;
5. para para confirmação humana antes do RED semântico.

## 2. Proveniência

Medição inicial em `2026-08-23T21:24:47-03:00`:

- HEAD: `fcbc9763f8925d5c27b3670e35597b9adc0412a0`;
- working tree não commitada: `28 files changed, 527 insertions(+), 400
  deletions(-)`, além dos artefatos P1139/P1140 não rastreados;
- catálogo: `00_nucleo/diagnosticos/superficie-linguagem-p1140.json`;
- probe público: `repr(type(path))` nos binários ratificado e cristalino.

Os números descrevem o estado em que este passo foi escrito. Nenhuma contagem
é usada sem essa proveniência para fechar implementação.

## 3. Medição anterior à decisão

### 3.1 Os dez paths não formam uma única unidade

| Família | Paths | Vanilla | Cristalino | Dono observado |
|---|---|---|---|---|
| construtores já materializados | `decimal`, `duration`, `regex`, `selector`, `stroke`, `tiling`, `version` | `type` chamável | `function` | entidades + construtor stdlib |
| semântica incorreta, não só kind | `label` | `type` chamável que produz `Label` | função que produz `Content::Label` | `entities/label` + `stdlib/label` |
| módulo matemático | `math.equation` | `function`/elemento | `none` sentinela | `stdlib/structural/math` + eval/layout |
| colisão símbolo/função | `math.sqrt` | `function` | `symbol` | math eval + módulo math + símbolos |

Logo, corrigir os dez num só commit esconderia três mudanças de natureza
distinta. A unidade de implementação será a família, não a contagem do
relatório.

### 3.2 O débito já está explicitamente contratado

`00_nucleo/prompts/compiler/eval.md:115-126` e
`00_nucleo/prompts/entities/value.md:145-155` registram que estes nomes não
foram convertidos a `Value::Type` porque isso quebraria construtores e
namespaces. Portanto não é esquecimento simples: a correção deve reconciliar
**tipo + chamabilidade + fields**, preservando:

- `decimal(...)`, `duration(...)`, `regex(...)`, `selector(...)`,
  `stroke(...)`, `tiling(...)`, `version(...)`;
- quaisquer fields/namespaces já públicos;
- `type(valor) == nome-do-tipo`;
- o kind público `repr(type(nome)) == "type"`.

O mecanismo vigente é estático: `Type` é enum fechado,
`Type::is_callable()` é match exaustivo e `eval_func_call` escolhe o
construtor. A solução não introduz mapa dinâmico, vtable, `dyn`, PropMap ou
metadado reflexivo.

### 3.3 Medição dos módulos tocados

| Ficheiro | Linhas | Responsabilidades/funções públicas medidas | Juízo de atomização |
|---|---:|---|---|
| `compiler/stdlib/primitives_constructors.rs` | 1.005 | `decimal`, `duration`, `version` + parsers + 47 testes | **sim, antes da semântica** |
| `compiler/stdlib/foundations/str.rs` | 262 | `str`, `str.from-unicode`, `regex` | **sim: regex tem dono próprio** |
| `compiler/stdlib/foundations/query.rs` | 392 | metadata/query/locate/here/target + selector + parser partilhado | **sim, como unidade selector completa** |
| `compiler/stdlib/layout.rs` | 1.957 | 15 construtores; `stroke` reutiliza `extract_length` e `extract_stroke` é usado por grid/block/box | candidato futuro; **não pré-condição** |
| `compiler/stdlib/label.rs` | 144 | somente `native_label` e testes | já atomizado; corrigir contrato, não mover |
| `compiler/stdlib/visualize.rs` | 332 | `tiling` + helpers e testes da própria unidade | localidade suficiente; não mover neste lote |
| `compiler/stdlib/structural/math.rs` | 521 | accent/cancel/class/underover/op + construção do módulo | candidato no passo math, fora deste lote |
| `compiler/eval/mod.rs` | 2.082 | pipeline eval + composição da stdlib; bindings são linhas de wiring | manter hub; não espalhar registro |

Contagens reproduzíveis:

```sh
wc -l 01_core/src/compiler/stdlib/{primitives_constructors.rs,layout.rs,label.rs,visualize.rs}
wc -l 01_core/src/compiler/stdlib/foundations/{str.rs,query.rs}
wc -l 01_core/src/compiler/stdlib/structural/math.rs 01_core/src/compiler/eval/mod.rs
rg -n '^pub fn native_|^fn [a-zA-Z0-9_]+\(' <ficheiros acima>
```

### 3.4 Aplicação correta da ADR-0109

A atomização aqui significa mover cada constructor/parser/testes para o
ficheiro da feature **na mesma camada L1**, deixando os hubs com declaração e
reexport estáticos. Não significa reduzir imports, criar crates, usar despacho
dinâmico ou mover comportamento para os structs de entidade.

A forma B da ADR-0109 continua obrigatória onde há render. Este passo não move
render; para stdlib, a analogia válida é free function no módulo da feature,
chamada estaticamente pelo hub.

## 4. Decomposição aprovada para planeamento

### P1140.1-A — atomização behavior-preserving

Antes de qualquer mudança pública, propor nos L0 e materializar:

```text
compiler/stdlib/primitives_constructors.rs       (hub/reexports)
compiler/stdlib/primitives_constructors/
  decimal.rs
  duration.rs
  version.rs

compiler/stdlib/foundations/str.rs               (str + from-unicode)
compiler/stdlib/foundations/regex.rs             (regex)

compiler/stdlib/foundations/query.rs             (query/locate/here/target/metadata)
compiler/stdlib/foundations/selector.rs          (constructor + parsing comum)
```

Regras:

- mover testes junto com a unidade dona;
- preservar nomes públicos e reexports usados por `stdlib/mod.rs`;
- `query` e `locate` chamam a free function de parsing em `selector.rs`;
- nenhuma visibilidade `pub(crate)` nova se `pub(super)`/descendência bastar;
- nenhum comportamento, mensagem, assinatura ou ordem de binding muda;
- snapshots/probes antes e depois devem ser idênticos.

L0 a atualizar antes desta atomização:

- `compiler/stdlib/primitives-constructors.md` — trocar alvo único pelo hub e
  três unidades donas;
- `compiler/stdlib/foundations/str.md` — retirar regex;
- novo `compiler/stdlib/foundations/regex.md`;
- `compiler/stdlib/foundations/query.md` — retirar selector/parser;
- novo `compiler/stdlib/foundations/selector.md`;
- hubs `compiler/stdlib/foundations.md` e `compiler/stdlib.md`, se suas listas
  de módulos exigirem atualização.

Essa fase é refactor interno behavior-preserving: segue fluxo contínuo após L0
primeiro e resselo, com testes de não regressão. Se a execução revelar mudança
de API pública Rust necessária, parar e reclassificar pelo ADR-0127.

### P1140.1-B — contrato de `Type` chamável

Depois da atomização verde, redigir — sem ainda implementar — o L0 para:

1. registrar sete bindings como `Value::Type`:
   `Decimal`, `Duration`, `Regex`, `Selector`, `Stroke`, `Tiling`, `Version`;
2. fazer `Type::is_callable()` reconhecer exatamente esses sete;
3. fazer o despacho estático de chamada delegar aos mesmos construtores
   nativos atomizados;
4. preservar fields por braços explícitos de `Value::Type`, quando existirem;
5. manter `Value::Type` como enum fechado e o match exaustivo;
6. testar identidade do kind e semântica completa da chamada.

L0 donos mínimos:

- `entities/value.md`;
- `compiler/eval.md` e o L0 atomizado de call/field access;
- `entities/{decimal,duration,regex,selector,tiling,version}.md`;
- `entities/geometry.md` e `compiler/stdlib/layout.md` para `Stroke`;
- L0 dos construtores atomizados.

Esta fase muda comportamento default e contrato público. Após escrever e
resselar os L0, **parar obrigatoriamente** para confirmação humana antes do
primeiro teste RED semântico.

### P1140.1-C — `label` separado

Não incluir `label` no lote B por mera semelhança nominal. Medir e especificar:

- vanilla `label("x")` produz valor de kind `label`;
- sintaxe `<x>` e associação de label a conteúdo continuam caminhos próprios;
- a função cristalina atual `label(name, body) -> Content::Label` não pode ser
  silenciosamente reutilizada como construtor de tipo;
- compatibilidade da extensão `label(name, body)` deve ser decidida: remover,
  preservar por outro path ou documentar quebra.

Como envolve comportamento e possível quebra de compatibilidade, recebe L0 e
gate próprios. `stdlib/label.rs` já é uma unidade legível; atomização adicional
não é justificável.

### P1140.1-D — matemática separada

`math.equation` e `math.sqrt` formam passo posterior próprio:

- `math.equation`: substituir a sentinela `Value::None` por função/elemento
  legítimo sem quebrar set/show/selectors/layout;
- `math.sqrt`: reconciliar o símbolo atual com o callable vanilla, medindo se o
  símbolo permanece acessível por alias/modificador;
- avaliar atomização de `structural/math.rs` somente nesse passo, depois de
  mapear consumidores em eval/layout.

Nenhum dos dois entra no RED do lote de tipos gerais.

## 5. Sequência de execução

1. Congelar probes dos dez paths e chamadas existentes.
2. Atualizar os L0 estruturais da Fase A.
3. Atomizar `primitives_constructors`, `regex` e `selector` sem semântica nova.
4. Rodar testes focados, catálogo P1140 e matriz integral; exigir igualdade
   pré/pós-refactor.
5. Redigir os L0 da Fase B com medição vanilla `file:line` de constructor,
   fields e mensagens.
6. Resselar hashes e **parar no gate humano ADR-0127**.
7. Após aprovação: RED de kind + chamada + fields para os sete tipos.
8. Implementar por match estático, um tipo por commit lógico/teste.
9. Remedir o catálogo: sete `WRONG_KIND` devem virar `MATCH` ou classe mais
   específica comprovada; os três casos `label/math` permanecem abertos.
10. Escrever os passos separados de label e math.

## 6. Testes obrigatórios

### Atomização

- testes existentes movidos, não apagados nem enfraquecidos;
- `cargo test -p typst-core` focado em cada constructor;
- probes de chamada e erro byte-idênticos antes/depois;
- nenhuma alteração no catálogo P1140 causada apenas pela mudança de ficheiro;
- `tekt-cargo-dsm --estrutura` pode medir impacto, mas não é gate de linguagem.

### Sete tipos gerais

Para cada path `t`:

- `repr(type(t)) == "type"`;
- `type(valor-produzido) == t`;
- chamada mínima válida coincide com vanilla;
- argumentos named/positional/default e erros coincidem;
- fields existentes continuam acessíveis;
- `repr(t)` e igualdade de valores-tipo coincidem;
- `std.t` tem o mesmo kind que `t`, quando o path existe no vanilla.

Casos mínimos:

```typst
decimal("1.5")
duration(seconds: 1)
regex("a+")
selector("heading")
stroke(thickness: 1pt)
tiling(size: 10pt)[x]
version(1, 2, 3)
```

## 7. Critérios de aceitação

- [ ] Os dez casos permanecem divididos em 7 + 1 + 2, sem lote monolítico.
- [ ] L0 estrutural precede a atomização.
- [ ] `primitives_constructors`, regex e selector ficam legíveis por unidade.
- [ ] A atomização não muda nenhum observável.
- [ ] `layout.rs`, `visualize.rs`, `label.rs` e math não são movidos por
      conveniência incidental.
- [ ] O hub de stdlib continua estático e explícito.
- [ ] Nenhum `dyn`, vtable, PropMap ou registro global mutável é introduzido.
- [ ] L0 semântico dos sete tipos é escrito e ressellado.
- [ ] O humano confirma o gate antes do RED semântico.
- [ ] Após implementação aprovada, sete `WRONG_KIND` deixam de divergir.
- [ ] `label`, `math.equation` e `math.sqrt` continuam visíveis e recebem
      passos próprios, não scope-out silencioso.
- [ ] `cargo build`, testes relevantes, matriz, `crystalline-lint .` e
      `git diff --check` passam.

## 8. Limites

- Não implementa `ParamInfo` nem reflexão de assinaturas.
- Não corrige globais ou os 1.155 `MISSING_MEMBER` de P1140.
- Não toca SVG/P1141 nem as frentes P1142–P1145.
- Não usa atomização para reduzir a métrica de imports da DSM.
- Não converte `label` até decidir a compatibilidade da função cristalina
  existente.
- Não substitui sentinelas math sem medir set/show/selector/layout.

## 9. Execução de P1140.1-A e gate de P1140.1-B

Executado em `2026-08-23T21:39:31-03:00`, HEAD
`fcbc9763f8925d5c27b3670e35597b9adc0412a0`, com working tree não commitada
(`git diff HEAD --stat`: 36 ficheiros, 595 inserções e 1.520 remoções antes
da redação semântica; inclui trabalho P1139/P1140 já presente).

Materializado:

- hub `primitives_constructors.rs` reduzido de 1.005 para 95 linhas;
- unidades `decimal.rs` (90), `duration.rs` (450) e `version.rs` (421), com
  implementação e testes donos;
- `regex` separado de `str`; `selector` e parser separados de `query`;
- testes de referência: decimal 33, duration 81, version 70, regex 34 e
  selector 53, todos verdes;
- catálogo cristalino após rebuild: 1.020 entradas e `cmp` byte-idêntico a
  `/tmp/p1140-crystalline.json` anterior;
- 22 probes públicos após rebuild byte-idênticos ao baseline estrutural
  (`cmp = 0`; 3 já coincidentes com vanilla, sem mudança nesta fase).

O L0 de P1140.1-B foi redigido para os sete tipos chamáveis. Como a próxima
ação muda o kind público dos bindings e o comportamento por defeito, a
execução para aqui no gate obrigatório da ADR-0127. Nenhum teste RED ou
código semântico de P1140.1-B foi escrito.

## 10. Execução confirmada de P1140.1-B

Gate confirmado pelo dono e implementação executada em
`2026-08-23T21:48:46-03:00`, no mesmo HEAD não commitado.

O RED confirmou as três fronteiras previstas: `Type::is_callable`, kind do
binding global e identidade `type(valor) == binding`. O GREEN:

- registra os sete nomes como `Value::Type`;
- acrescenta os sete arms ao despacho estático de `call_dispatch`;
- delega sem adaptação aos mesmos construtores L1;
- preserva `Type` e `Value` como enums fechados, sem registry/dyn/vtable;
- cobre chamadas reais dos sete tipos, inclusive `tiling(rgb(...))`.

Medição pós-implementação:

- catálogo: 1.020 entradas; exatamente sete paths mudaram, todos de
  `function` para `type`;
- classificação combinada: `MATCH` 791 → 798 e `WRONG_KIND` 10 → 3;
- os três `WRONG_KIND` restantes são exatamente `label`, `math.equation` e
  `math.sqrt`, os lotes separados previstos neste passo;
- probes: 7/22 coincidem agora; os quatro tipos presentes na amostra
  (`decimal`, `duration`, `regex`, `version`) passaram a coincidir, além dos
  três controles positivos anteriores.

Validação final da sessão:

- todas as suítes focadas dos sete tipos e os três testes P1140.1 passaram;
- `cargo build`, `cargo fmt --check`, `git diff --check` e
  `crystalline-lint .` passaram, com zero violações;
- a suíte integral L1 passou 5.116/5.118 testes. As duas falhas restantes,
  `p862_content_tree_splits_plain_text_on_space` e
  `p862_repr_plain_text_splits_on_space`, pertencem à morfologia de texto,
  não aparecem no diff P1140.1 e não atravessam nenhum dos sete bindings.
  Permanecem visíveis; não foram enfraquecidas nem corrigidas incidentalmente.
