# Passo 1437 — Materializar `math.mid` com identidade, classe e stretch do `lr` mais próximo

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
**Data da medição:** 2026-09-17T21:09:30-03:00  
**Commit medido:** `39826a825a11d966be7adb832606fe6c14373e46`  
**Árvore medida:** limpa (`git status --short` e `git diff HEAD --stat` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordenou as rotas ainda abertas como
`abs, accent, cases, frac, limits, lr, mat, mid, norm, ...`. Depois dos
fechos de `accent`, `cases`, `frac`, `limits`, `lr` e `mat`, a próxima rota
nominal não materializada é `math.mid`. O Passo 1436 deixou-a explicitamente
fora do seu escopo.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `crates/typst-library/src/math/lr.rs` declara `MidElem`
com um único campo público, obrigatório e posicional, `body: Content`, e o
documenta como delimitador escalado verticalmente para o grupo `lr` envolvente
mais próximo. Em `math/ir/resolve.rs`, a resolução de `MidElem` resolve o body,
marca cada item real para stretch pelo `lr` exterior e força classe `Relation`.
`resolve_lr` aplica então a mesma meta de stretch do grupo. Um `lr` aninhado
estabelece a fronteira mais próxima; `mid` não calcula uma meta independente.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/lr.rs` | `2a762699e7d3eed9a0b4b2d1afbbf5b58649339ec47ec12a2a82907519c107e0` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado:

- `repr(math.mid)` produz `"mid"`;
- `repr(math.mid([x]))` produz `"mid(body: [x])"`;
- `repr($ mid(|) $)` produz `equation(block: true, body: mid(body: [|]))`;
- `math.mid()` falha com `missing argument: body`;
- `math.mid([x], [y])` falha no segundo argumento com `unexpected argument`;
- `math.mid(body: [x])` falha porque `body` é posicional;
- `math.mid(none)` produz body vazio, string converte para conteúdo e inteiro
  é rejeitado;
- spread canônico é aceito e `math.mid(math.mid([|]))` preserva as identidades;
- dentro de `math.lr(size: 150%, ...)`, `mid` permanece no body e recebe o
  contexto geométrico do owner exterior.

### 2.3 Estado cristalino medido

O módulo `math` não contém `mid`; a forma qualificada falha. A forma bare
`$ mid(|) $` degrada para conteúdo textual `mid(|)` e não conserva identidade,
classe `Relation` nem vínculo de stretch. `compiler/eval/rules.rs` já reconhece
o nome no vocabulário de campos, mas não há carrier, builder, `repr` ou layout.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_delimited.rs` | `fd00c580a6d98cadac9ec79eb38fa5994677ef1756f1fb0cf5b7d8249a6d596a` |
| `01_core/src/entities/content.rs` | `77133735eca113684f6fa6cbeadf0b3b83e84233c45768ed9097cf4ab037fc59` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `d1d872796e1d54a9dcded20f5d34580240ab862a2f9db0d5f23b064a7cf13006` |
| `01_core/src/compiler/eval/math.rs` | `dfaee47456376a48eff77e9ac52d77c69a51ce470c8e381060254adb3f9442b3` |
| `01_core/src/compiler/eval/repr.rs` | `d68c6707c15f9ff459cec2e981c70c03a9dc14f8fd4b9cc338d8fd9d6a94e959` |
| `01_core/src/compiler/math/layout/delimited.rs` | `dbb096078771a4649ce873fee6a29e9a6daae8de8fd310f6424d077ff90df174` |
| `01_core/src/compiler/math/layout/mod.rs` | `86e9f51e9ebddf11270ded454e48f4340f7dda00bcbc1ed45e04f09a795e6ee0` |
| `01_core/src/compiler/math/layout/spacing.rs` | `c12029dc075e80871a461074b3ceb28cb8c93619619e759b2bf497feaee77bca` |
| `01_core/src/compiler/eval/mod.rs` | `330e2781fc624262e9e0bd6722aabe19272c2d3acf20492c3d5bffbce0d61862` |
| `01_core/src/compiler/introspect.rs` | `760b67f2220c76eb0eaca3ae5071cb3ba909c047d39d70adaae3f9e12e383803` |
| `01_core/src/compiler/introspect/locatable.rs` | `269cff41d994020cbb42e181b722eeba907a8ae508360427683f8f216a4f1da3` |
| `01_core/src/compiler/layout/mod.rs` | `4990b9cd303b91597ff334ab8959dca4b9dbd2236f75d51cb056b069a7b341cc` |
| `03_infra/src/query_helpers.rs` | `02168b0bdbf6d3d172a21c94aaa86b06caf0879e14344fc8154c067c5267b6c3` |

## 3. Classificação anti-deriva

- Identidade `mid`, aridade/cast, `repr`, classe `Relation` e escala para o
  `lr` mais próximo são linguagem e exigem paridade (ADR-0107).
- O booleano interno `mid_stretched`, os passes do resolver e a estrutura dos
  itens são mecânica vanilla e não serão copiados como contrato.
- Inferência: o marcador interno materializa a intenção de aplicar ao body a
  mesma meta geométrica do `lr` exterior, respeitando nesting. Refutação: um
  witness vanilla em que `mid` imediato tenha altura diferente das cercas do
  grupo ou em que o exterior atravesse um `lr` interior explícito.
- Tratar `mid` só como `Relation` falharia escala; tratá-lo só como delimitador
  mecânico perderia identidade e a regra do grupo mais próximo.

## 4. Decisão L0 proposta

1. Adicionar `MathMidElem { body: Content }` ao owner coeso
   `entities/elements/math_delimited.rs`; `mid`, `lr` e delimitadores formam a
   mesma unidade semântica vanilla, sem novo Prompt materializável órfão.
2. Adicionar `Content::MathMid(Arc<MathMidElem>)`; `plain_text` e
   `map_content` descem no body, `map_text` mantém a política estrutural math.
3. Registrar `math.mid`; a sintaxe bare delega ao mesmo builder, exige um único
   positional convertível em conteúdo e preserva spread/avaliação única.
4. Projetar sempre `mid(body: <content>)` no `repr`.
5. Classificar fronteiras e itens resolvidos como `Relation`.
6. Fora de `lr`, manter tamanho natural. No `lr` mais próximo, aplicar a cada
   item do body a meta vertical do grupo, incluindo size/cascata/short-fall já
   resolvidos. Não atravessar `lr` interior.
7. Manter dispatch magro para a free function de `delimited.rs` (ADR-0109).
8. Tratar a variante como não-locatable e terminal nos walkers introspectivos
   e helpers aproximados L3; o walker estrutural L1 continua dono do body.
9. `ObservationEq` compara somente o body; reconstruções transformam somente
   o body e preservam a variante.

As outras 14 rotas residuais do inventário P1422 permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento público, variante pública e comportamento de
produto. Portanto este commit contém apenas o passo e L0 e **PARA**. Código,
testes e resselo começam somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Criar oráculos RED para identidade qualificada/bare, argumentos, `repr`,
   `Relation`, stretch dentro/fora de `lr`, nesting e walkers.
2. Confirmar RED cristalino e GREEN vanilla onde aplicável.
3. Implementar carriers, builders, dispatch e layout na forma B.
4. Atacar mutantes: remover `Relation`, neutralizar stretch, atravessar nesting,
   aceitar segundo positional, aceitar `body:` e apagar identidade no `repr`.
   Exigir score `1.0` para os mutantes definidos.
5. Resselar apenas após V15/V26 válidos.
6. Validar `cargo build`, suítes focal/completa e `crystalline-lint .` zero,
   registrando commit, árvore e hora das medições de fecho.

## 7. Confirmação e materialização

O dono confirmou explicitamente o gate ADR-0127 após o commit de proposta
`b0f48017d29f476357279a678aea3ff2e4c9df70`.

P1437 foi materializado em 2026-09-17 sob o regime
`executed_without_isolation_attestation`. O RED atribuível apresentou seis
erros centrados na ausência de `MathMidElem`, `Content::MathMid` e dos
observáveis congelados. O candidato fechou quatro testes focais L1: binding e
casts, diagnósticos, carrier/walker/igualdade/classe/locatable e geometria do
`lr` mais próximo com barreira de nesting.

A primeira campanha física obteve 18/30 porque quatro receitas selecionaram
ocorrências homônimas anteriores, fora de P1437; não houve crédito. Após
recalibração focal M04/M05/M06/M10 (12/12), a campanha integral repetida nas
ordens direta, direta repetida e reversa matou 30/30 mutantes, score `1.0` e
zero `Unknown`.

O binário cristalino reproduziu as formas qualificadas/bare, body vazio,
nesting, `lr(size: 150%)` e cinco diagnósticos negativos medidos no vanilla.
`cargo fmt --check`, `git diff --check`, `cargo build --workspace` e o gate
focal terminaram verdes. A suíte completa executou 5.857 testes: 5.849 verdes
e 8 falhas históricas já presentes no estado HEAD (quatro expectativas P981
anteriores ao carrier P1436, uma de `repr vec`, uma de `cases` e duas de
`html.frame`); nenhuma toca arquivos ou observáveis P1437. Elas ficam
registradas como baseline e não recebem crédito de sucesso.
