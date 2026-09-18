# Passo 1446 — Materializar `math.stretch` com tamanho causal e cascata

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)
**Data da medição:** 2026-09-18T15:50:44-03:00
**Commit medido:** `6f24506eaab9fcf2775238d9e5c55885827b7f86`
**Árvore medida:** `85bcaa9b1d87a5fc80f2ece3aa43a5c5d9d29c29`, limpa
(`git status --short` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual do Passo 1422 ordena `math.stretch` imediatamente após
`math.round`. P1445 fechou `math.round`; portanto P1446 toma somente a rota
`P1420-MATH-018` (`math.stretch`). `math.abs` e as rotas `under*` seguintes
permanecem residuais e não recebem crédito neste passo.

## 2. Medição antes da decisão

### 2.1 Fonte e binário vanilla ratificados

No pin `586e1bd43fae6c9a973218163d3165c53ab8d16d`, `math/attach.rs:115-144`
declara `StretchElem` público com `body: Content` obrigatório e posicional e
`size: Rel<Length>` settable com default `100%`. `math/mod.rs` registra o
elemento. `math/ir/resolve.rs:637-647` resolve primeiro o body, lê o tamanho
da cascata e chama `update_stretch` com short-fall zero. A atualização marca
os eixos disponíveis do glifo; o alvo percentual é relativo à dimensão
natural do eixo e o termo absoluto é resolvido no tamanho de fonte causal.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/attach.rs` | `7efdda527260ad0ac91436b35abe5f98b85019ff8491192c1ed5077c17953f5f` |
| `lab/typst-original/crates/typst-library/src/math/mod.rs` | `a8a64e505979b8a6fce3bb5fef978677a40200199e9dc36738a3343ae1104957` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

O binário `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
é byte a byte igual ao build ratificado em
`lab/typst-original/target/release/typst`.

### 2.2 Observáveis públicos medidos

| Sonda vanilla | Resultado |
|---|---|
| `repr(math.stretch)` | `stretch` |
| `repr(math.stretch([x]))` | `stretch(body: [x])` |
| `repr(math.stretch(size: 150%, [x]))` | `stretch(body: [x], size: 150% + 0pt)` |
| `repr(math.stretch(size: 100%, [x]))` | conserva `size: 100% + 0pt` explícito |
| `repr(math.stretch(size: 1em, [x]))` | `stretch(body: [x], size: 0% + 1em)` |
| `repr(math.stretch(none))` | `stretch(body: [])` |
| `repr(math.stretch("x"))` | `stretch(body: [x])` |
| `repr(math.stretch(sym.arrow.r))` | `stretch(body: [→])` |
| `repr(math.stretch(math.stretch([x])))` | conserva os dois carriers aninhados |
| `.body`, `.fields()`, `.has("size")`, `.func()` no default | `[x]`, `(body: [x])`, `false`, `stretch` |
| `.size`/`.at("size")` no default omitido | campo não conhecido neste ponto |
| `.size`, `.fields()`, `.has/.at("size")` com `150%` explícito | valor presente e causal |
| bare `$ stretch(arrow.r) $` | `equation(block: true, body: stretch(body: [→]))` |
| bare `$ stretch(arrow.r, size: #150%) $` | mesmo carrier com `size: 150% + 0pt` |

Ausência de body, segundo posicional, `body:` named, named desconhecido,
`size` string/inteiro, `size` duplicado e body inteiro falham. As mensagens
primárias medidas são, respectivamente, `missing argument: body`,
`unexpected argument`, ``the argument `body` is positional``,
`unexpected argument: foo`, `expected relative length, found ...`,
`duplicate argument: size` e `expected content, found integer`.

`#set math.stretch(size: 150%)` produz um wrapper estilizado e a fonte
ratificada lê esse valor somente na resolução do elemento; o `repr` do filho
continua sem fabricar presença explícita. Portanto “omitido com cascata” não
é o mesmo estado morfológico que “argumento explícito”.

### 2.3 Estado cristalino medido

`repr(math.stretch)` e a chamada qualificada falham com
`module math does not contain stretch`. A sintaxe bare existe apenas como
aplicação genérica e produz `stretch("→")`, apagando identidade de elemento,
campos, tamanho causal, introspecção e cascata. O validador de `#set` conhece
o nome `stretch.size`, mas o par não integra `MATH_SET_LIGADOS` e não chega ao
layout.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `entities/elements/mod.rs` | `fd8b7f2a1ddc18ae88e5272f8775e138044f36b097c28a679f6c23af63ce534f` |
| `entities/content.rs` | `160869242f41ae886d130fb84a384296fa2e49bfbcbddd32ff408baf1c7c372b` |
| `compiler/stdlib/structural/math.rs` | `aeba757253893b15ccc14c9f3a2b432981b69f9a1e24fcaa0fd4630e2af25018` |
| `compiler/eval/math.rs` | `751925bb44ea29c89e7b3ecf6c2b2c8dd380d58501e1f7176824da2476062a66` |
| `compiler/eval/repr.rs` | `676b659e13a3426db0d95af0e0b5328888255521873339ba810d9ab48025a8b0` |
| `compiler/eval/mod.rs` | `3ddd0f44d126f31063f2b9004f7b9706f345433c92f488ec1e11da161ff6c4e4` |
| `compiler/eval/bindings/field_access.rs` | `f2676ec3f8d3e44f378c41a8393cfafac65bd8baed259f772b60a6a11b4ab269` |
| `compiler/eval/rules.rs` | `69b2b34e14d504cf028fa7a727a9b6b987e9f763b241531d23c775969da93f46` |
| `compiler/math/layout/mod.rs` | `cdc3f8c6dd9340bf33aa01c08cdc1a18a8a1f7f14970ee4469a64d4111b19267` |
| `compiler/math/layout/stretchy.rs` | `5aac1f86fc8c6e19fd47eed1f6727581e7d82a214dab322dc109cdc29ff62751` |
| `compiler/math/layout/spacing.rs` | `585f67a5da7d3c411c991e586e498e3725fe9b704945c924dcb9a2338e5f5f6f` |
| `compiler/introspect.rs` | `12bdb42aab66edf5fc100d7b49a76608c656919af313a73415d2e2d3d5e91126` |
| `compiler/introspect/locatable.rs` | `1c66808eae8a693f461cd5293fd3c503c24dc7c29af04883ee8a90a4b19b20c4` |
| `compiler/layout/mod.rs` | `5e1d7f405f8c9622472f95f5608ed32e8be36f0fb0bf90e0ab504608cae64763` |
| `infra/query_helpers.rs` | `21a3685ef4d81fb4622be4e77ab391a95b6f59194ac9ff8f7ff9fc3948d2fed3` |

## 3. Classificação anti-deriva

- Identidade `stretch`, body, presença causal de `size`, assinatura, cascata,
  `repr` e resultado geométrico são semântica/morfologia da língua.
- O IR `StretchInfo`, interior mutability, enum Rust e escolha concreta entre
  variante/assembly são mecânica e podem divergir.
- Mensagens primárias de erro são observáveis; span/hint continuam sujeitos à
  limitação diagnóstica global e não recebem equivalência integral sem prova.
- Medido: o vanilla atualiza ambos os eixos disponíveis e o render escolhe o
  único eixo extensível; glifo já suficiente permanece natural. Inferência: o
  carrier cristalino pode delegar aos seletores horizontal/vertical existentes
  sem reproduzir o IR. Refutam isso witnesses ratificados em que a direção,
  dimensão de referência, composição de attachments ou resultado visível
  divirjam.
- A checagem extra refutou o enquadramento cômodo “o bare já existe”: ele
  preserva tinta textual parcial, mas não o objeto público nem seu estilo.

## 4. Decisão L0 proposta

1. Criar owner `entities/elements/math_stretch.md` e consumer único
   `math_stretch.rs`, com `MathStretchElem { body, size, size_explicit }`.
   `size_explicit` é memória causal interna necessária à morfologia; o campo
   público de linguagem continua sendo `size` apenas quando fornecido.
2. Adicionar `Content::MathStretch(Arc<_>)`, constructor canônico, igualdade,
   hash, `ObservationEq`, `plain_text`, `map_content` recursivo no body e
   `map_text` terminal, preservando tamanho/presença.
3. Registrar `math.stretch` e fazer a sintaxe bare convergir no mesmo helper:
   exatamente um body posicional castável a content; `size` named opcional
   `Rel<Length>`; default `100%`; named/posicionais extras rejeitados.
4. Expor `body` sempre e `size` somente quando explícito por field access,
   `fields/has/at`; `repr` ordena `body` antes de `size`, como medido, e
   `func()` devolve `stretch`.
5. Ligar `#set math.stretch(size:)` à StyleChain como relative length validado.
   Valor explícito vence cascata; cascata vence `100%`; cascata não liga
   `size_explicit` nem altera a `repr` do carrier.
6. Layout resolve o body primeiro e aplica o alvo efetivo ao glifo
   extensível no seu eixo nativo. Percentual referencia a dimensão natural
   máxima do glifo e attachments; absoluto resolve no font-size causal;
   short-fall é zero. Glifo não extensível mantém o fallback público medido;
   não inventar escala geométrica contínua.
7. O dispatch exaustivo continua magro e delega a free function do arquivo da
   feature de layout, reutilizando primitivas de `stretchy.rs` (ADR-0109).
   A classe/bordas vêm do body resolvido; o wrapper não fabrica classe nova.
8. A variante é não-locatable e terminal nos walkers introspectivos/L3, como
   os carriers matemáticos estruturais vigentes. Parser, AST, fase
   eval→layout e outras rotas P1422 não mudam.

## 5. Gate ADR-0127

A proposta acrescenta struct/campos públicos, variante pública, binding de
stdlib, comportamento bare por defeito e propriedade settable. Incide nos
itens 1 e 2 do ADR-0127. Este commit contém somente passo e Prompts L0 e
**PARA**. Nenhum L1/L2/L3, teste, header ou hash de consumer pode mudar antes
de confirmação humana explícita.

## 6. Execução após confirmação

1. Congelar contrato e oráculos de binding, casts, presença causal, repr,
   igualdade, bare/qualified, cascata e geometria horizontal/vertical.
2. Confirmar RED atribuível e GREEN vanilla dos observáveis aplicáveis.
3. Implementar carrier, binding, introspecção, style transport e layout em
   forma B.
4. Atacar remoção de identidade/presença, inversão de precedência, perda de
   eixo, uso de short-fall, scaling contínuo falso e apagamento do body.
5. Exigir score 1,0 nas ordens direta, repetida e inversa; `Unknown` bloqueia.
6. Executar fmt, diff-check, build, suítes focal/integral e
   `crystalline-lint .`, com proveniência completa.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 em 2026-09-18, após o
commit de proposta `503e6416c78563b0d3949dfb81a58412da79dccb`. A cadeia
contrato → RED → implementação → ataques → veredito fica autorizada somente
para o recorte desta proposta.

## 8. Materialização e pré-veredito

Medição final em 2026-09-18, sobre working tree não commitado derivado do
commit de proposta `503e6416c78563b0d3949dfb81a58412da79dccb`; o estado
exato é enumerado por `git status --short` e `git diff HEAD --stat`.

- RED atribuível: quatro testes estruturais executaram e falharam somente pela
  ausência do carrier `MathStretch` e do binding público.
- GREEN focal: 4 passados, 0 falhados; o teste unitário local do elemento
  também passou na suíte integral.
- Ataques: 30/30 execuções eficazes mortas nas ordens direta, repetida e
  inversa; score 1,0, zero `Unknown`, todos M01–M10 mortos 3/3.
- Paridade pública positiva: identidade, casts, presença causal de `size`,
  campos, nesting, `repr`, convergência bare/qualificada e transporte da
  cascata foram preservados. A quebra multilinha do pretty-printer global
  permanece fora deste recorte.
- Os oito negativos de assinatura e mensagem primária são cobertos pelo
  oráculo estrutural congelado.
- Gates finais: `cargo fmt --all -- --check`, `git diff --check`,
  `cargo build --workspace` e `crystalline-lint .` passaram;
  V5/V15/V26 = 0.
- Suíte integral: 5.889 passados e os mesmos oito falhados históricos do
  baseline, nenhuma falha atribuível a P1446.

Pré-veredito limitado: `Preserved` para o fragmento `math.stretch` descrito;
não declara equivalência Typst geral, paridade integral do pretty-printer nem
fecha as rotas residuais de P1422. O commit produtivo e o certificado são
fixados somente depois dos gates finais reproduzíveis.
