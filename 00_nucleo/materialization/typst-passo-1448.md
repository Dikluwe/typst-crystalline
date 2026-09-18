# Passo 1448 — Materializar `math.underbrace` com identidade e geometria de spreader inferior

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)
**Data da medição:** 2026-09-18T17:45:17-03:00
**Commit medido:** `661c9b021be4b543d4a5f5770709c94e8445b14e`
**Árvore medida:** `c30281033c134100b141682b05e64afe004cb5ba`, limpa (`git status --short` e `git diff HEAD --stat` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário P1422 identifica `math.underbrace` como `P1420-MATH-019`, após
`math.round` (`017`) e `math.stretch` (`018`). Esses dois itens e a correção
fora de ordem de `math.abs` (`001`) já foram materializados; portanto a próxima
unidade residual é `math.underbrace`. `underbracket`, `underparen` e
`undershell` permanecem fora deste recorte.

O Passo 1447 chamou `math.abs` de `P1420-MATH-017`; a correção documental para
`P1420-MATH-001` integra este commit de proposta e não altera produto.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs` declara `UnderbraceElem` com
`body: Content` obrigatório e `annotation: Option<Content>` opcional e
posicional. `math/ir/resolve.rs` resolve o body, cria U+23DF (`⏟`) com classe
`Diacritic`, stretch horizontal de 100% e shortfall zero, e o combina abaixo
do body com largura exata. O composto herda a classe matemática do body;
quando presente, a annotation é adicionada abaixo em estilo de subscript.

Hashes SHA-256 das fontes medidas:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |
| `lab/typst-original/crates/typst-library/src/math/mod.rs` | `a8a64e505979b8a6fce3bb5fef978677a40200199e9dc36738a3343ae1104957` |

O binário medido foi `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
igual ao build vanilla ratificado.

### 2.2 Observáveis públicos medidos no vanilla

| Witness | Resultado |
|---|---|
| `repr(math.underbrace)` | `"underbrace"` |
| `repr(math.underbrace([x]))` | `underbrace(body: [x])` |
| `repr(math.underbrace([x], [n]))` | `underbrace(body: [x], annotation: [n])` |
| `repr(math.underbrace([x], none))` | conserva `annotation: none` |
| body `none`, string ou símbolo | converte para conteúdo vazio, texto ou símbolo |
| nesting | conserva cada identidade `underbrace` |
| `.body`, `.fields()`, `.has(...)`, `.at(...)`, `.func()` | expõem campos, presença causal e função pública |
| `$ underbrace(x, n) $` | `equation(block: true, body: underbrace(body: [x], annotation: [n]))` |

Omissão e `none` explícito diferem em `fields`/`has`: annotation omitida não
existe; annotation fornecida como `none` existe e seu valor é `none`.

Fronteira negativa medida:

- ausência do body: `missing argument: body`;
- terceiro positional: `unexpected argument`;
- `body:`: `the argument body is positional`;
- `annotation:` e named desconhecido: `unexpected argument: <nome>`;
- body inteiro: `expected content, found integer`;
- annotation inteira: `unexpected argument`;
- as formas bare com positional excedente ou named desconhecido reproduzem a
  mesma rejeição.

### 2.3 Estado cristalino medido

A forma qualificada falha com `module math does not contain underbrace`. A
forma bare baixa diretamente para `MathUnderover`: o witness aparece como
`equation(block: true, body: xunder("⏟")under(n))`, apagando identidade
pública e a distinção entre omissão e `none` explícito. O braço agregado ainda
ignora named e positionals excedentes. A geometria física madura já existe em
`layout/underover.rs`; o defeito é de contrato, morfologia e fronteira de
argumentos, não ausência da mecânica de spreader.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `d53e726bd0d8749c19fa6e9ef6faedda8a95712385961404c025188f948bfb69` |
| `01_core/src/entities/content.rs` | `e2097377f367593c8d348094c73d30db0ca67de58c75972013ad668faa3a9423` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `0d9f5ed4673ff7b34d53b563cc55a6e563942c11f8b9b67c8a3cf11732672289` |
| `01_core/src/compiler/eval/math.rs` | `f535ce04b5a3be8a392ce34ab9db96a7c74aa213c7effd087cdb86ac4aa4d8fa` |
| `01_core/src/compiler/eval/repr.rs` | `cebd017e43fcfc6d2752450a736963ab4f01734a06af6530697fe814b3191199` |
| `01_core/src/compiler/math/layout/underover.rs` | `1cba8a367819437592aeeca9992e1d9c2870a909f7bfa66169b618d11c725089` |
| `01_core/src/compiler/math/layout/mod.rs` | `8695c454d506807303b06f9940aec0c94486eb7740e47fc680a386a756518e91` |
| `01_core/src/compiler/math/layout/spacing.rs` | `a860d9add9f461d41234fa6dec0f53e60fdbc9f09a755d861977167639084baf` |
| `01_core/src/compiler/eval/mod.rs` | `aff22826fb5c2b957201351136157586173da1b2104fc1a24255b3e70dddcd1e` |
| `01_core/src/compiler/introspect.rs` | `d69d59b394214fe67790609405dac2318640b70c78cc5e5493deb675e63e08b9` |
| `01_core/src/compiler/introspect/locatable.rs` | `4c4fd5106a7acb555fcbf45b6bbe63fbc655ad9b4726fbfa4efe1b62148d5578` |
| `01_core/src/compiler/layout/mod.rs` | `62e14334c2de2d3ab0a0c9906f6fec63a6e62f7dc2c3cd0d07e810d41e837c7c` |
| `03_infra/src/query_helpers.rs` | `2051ef58ad1753b57120a07e4e4231696bd4a86e375c4a3a2412b82662068ffe` |

## 3. Classificação anti-deriva

- Identidade `underbrace`, assinatura/casts, presença explícita, `repr`,
  fields, classe herdada e geometria observável são linguagem (ADR-0107).
- `AccentItem`, `ScriptsItem`, árvore IR e a redução cristalina para
  `MathUnderover` são mecânica e podem divergir.
- Inferência: shortfall zero + largura exata exigem que a chave cubra o body
  antes de centralizar a annotation. Refutação: witness vanilla em que a chave
  fique deliberadamente aquém do body ou a annotation determine seu alvo.
- Reutilizar somente o carrier genérico não basta: ele não preserva função,
  fields nem presença causal da annotation.

## 4. Decisão L0 proposta

1. Adicionar `MathUnderbraceElem { body, annotation, annotation_explicit }` ao
   owner coeso `entities/elements/math_underover.rs`.
2. Adicionar `Content::MathUnderbrace(Arc<MathUnderbraceElem>)`; presença
   explícita é morfologia e nunca é inferida do valor convertido.
3. Registrar `math.underbrace`; a forma bare delega ao mesmo builder. Exigir
   body posicional e no máximo uma annotation posicional, com casts,
   diagnósticos, spread, ordem e avaliação única medidos.
4. Projetar `underbrace(body: ..., annotation?: ...)`, emitindo annotation
   somente quando fornecida e conservando `none` explícito.
5. O layout owner reduz localmente a variante à composição madura: body com
   U+23DF abaixo e, se presente, annotation inferior. A free function fica em
   `layout/underover.rs`; o match central permanece magro (ADR-0109).
6. A classe esquerda/direita herda recursivamente o body. A chave permanece
   diacrítica, estica horizontalmente com shortfall zero e não troca a classe.
7. Reconstruções transformam body e annotation, preservam presença e
   identidade; `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos walkers introspectivos e helpers
   aproximados L3; o walker estrutural L1 continua dono dos filhos.

As rotas `underbracket`, `underparen` e `undershell` permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare. Portanto este commit contém apenas passo e L0 e **PARA**.
Testes, código e resselo começam somente após confirmação humana explícita.

## 6. Execução após confirmação

1. Criar oráculos RED para binding qualificado/bare, aridade, casts, presença,
   fields, `repr`, nesting, classe herdada, geometria e walkers.
2. Confirmar RED cristalino e GREEN vanilla onde aplicável.
3. Implementar carrier, builders, dispatch e layout na forma B.
4. Atacar mutantes: apagar identidade/presença, aceitar named/extra, trocar
   U+23DF, inverter posição, perder classe herdada ou deixar de esticar.
5. Exigir score `1.0` para os mutantes definidos; resselo após V15/V26.
6. Validar `cargo fmt --check`, `cargo build --workspace`, suítes focal e
   completa, `git diff --check` e `crystalline-lint .`, com proveniência.
