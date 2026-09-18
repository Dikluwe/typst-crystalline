# Passo 1451 — Materializar `math.undershell` com identidade e geometria de spreader inferior

**Estado:** L0 APROVADO — MATERIALIZAÇÃO AUTORIZADA (ADR-0127)
**Data da medição:** 2026-09-18T20:07:48-03:00
**Commit medido:** `c2813b6f775172a63bf4da2ffe1b41ab54fbe238`
**Árvore medida:** `820b3dfb642560b421f643dcbac8a88205d5cdbc`, limpa (`git status --short` sem saída)
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual P1422 ordena `math.undershell` imediatamente após
`math.underparen`. P1450 fechou `underparen`; portanto a próxima unidade
autocontida é `math.undershell`. As demais rotas P1422 ficam fora do recorte.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs:124-139` declara `UndershellElem` com
`body: Content` obrigatório/posicional e `annotation: Option<Content>`
opcional/posicional. `math/ir/resolve.rs:1380-1395` usa
`resolve_underoverspreader` (`:1418-1454`): U+23E1 (`⏡`) abaixo do body,
classe `Diacritic`, stretch horizontal de 100% com shortfall zero e annotation
como script inferior. O composto conserva a classe matemática do body.

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

O binário medido foi `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
que reporta `typst 0.15.1 (586e1bd4)` e corresponde ao pin ratificado.

### 2.2 Observáveis públicos medidos no vanilla

- `repr(math.undershell)` produz `"undershell"`;
- body-only produz `undershell(body: [x])`;
- com annotation produz `undershell(body: [x], annotation: [n])`;
- `undershell([x], none)` conserva `annotation: none`, enquanto omissão não
  assenta esse campo;
- `fields()` contém sempre `body` e somente contém `annotation` quando ela foi
  fornecida; `func()` devolve `undershell`;
- `$ undershell(x, n) $` preserva a mesma identidade pública;
- nesting conserva cada identidade e casts de `none`, string e símbolo seguem
  o contrato comum de conteúdo.

Fronteira negativa byte-exata medida:

- ausência do body: `missing argument: body`;
- terceiro positional: `unexpected argument`;
- `body:`: `the argument body is positional`;
- `annotation:` e named desconhecido: `unexpected argument: <nome>`;
- body inteiro: `expected content, found integer`;
- annotation inteira: `unexpected argument`.

### 2.3 Estado cristalino medido

`math.undershell` falha com `module math does not contain undershell`. A forma
bare produz `equation(block: true, body: undershell(x","n))`, sem identidade
pública própria; terceiro positional permanece no conteúdo e named é
silenciado. A geometria madura de under/over já existe, logo o defeito é de
contrato, morfologia e fronteira de argumentos.

Hashes dos consumers no estado limpo medido:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `023e36837db5e5dd36f26dddcdfebd512e0cc1ef0c46d3487f59370b108fdf51` |
| `01_core/src/entities/content.rs` | `b002e41a610d51876a69acf0c893c15497ee33dc06804c355b1bd85b679b157e` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `4f7d158b8909d54656f7ca98b5fe7c05200a751a1534a98c802ace8a6df64630` |
| `01_core/src/compiler/eval/math.rs` | `312ccd5c2c09cea6264e72608ef24da549d40f5468b251ef436c8eea9decaaa5` |
| `01_core/src/compiler/eval/repr.rs` | `8dd1bd6443ed9439d08dd107307d5553b59b7760f0825090b9e61ca45e205b34` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `7c48fd7e5bd6ae1e9bf35d3064f5eb70cf9f63ecf226ad506fe61b0750fc6960` |
| `01_core/src/compiler/math/layout/underover.rs` | `24fe059b7a8b03f83238cd090fc9e6aada28e53e7a757e6fb3e28f819cba0d08` |
| `01_core/src/compiler/math/layout/mod.rs` | `5b3a41b84d4f627cf5caed56a296c31782a89407c09a4c71d7bd3c27d18909c1` |
| `01_core/src/compiler/math/layout/spacing.rs` | `000962f0878cd90436e6474e9a610adda9fc76a85098528167707445725b81b8` |
| `01_core/src/compiler/eval/mod.rs` | `d825c25d264b17e6212e920e0b7cd9dc6db0ccfd07159fd5dedb6e032bab7bd6` |
| `01_core/src/compiler/introspect.rs` | `6a7d438a67b88976e649a786f973bc0da35abbcefcab057237e3f16aa826ba2f` |
| `01_core/src/compiler/introspect/locatable.rs` | `8c54134fe8a151451e4d8e982354eb74de7a5e13cb82338a06062ab81c256856` |
| `01_core/src/compiler/layout/mod.rs` | `ff97a3a050bda20d0c846fc66c0d4f9413f39311702bee2b6ed292d4e6af909d` |
| `03_infra/src/query_helpers.rs` | `856f25b214e0538ec18121054eb9a1f7cd3c3251514189e198682f69a38338c0` |

## 3. Classificação anti-deriva

Identidade `undershell`, assinatura/casts, presença explícita, repr, fields,
classe herdada e geometria observável são linguagem (ADR-0107). A árvore IR,
caixas e a redução cristalina para `MathUnderover` são mecânica. Inferência:
stretch de 100% e shortfall zero exigem que U+23E1 cubra o body antes de a
annotation ser centralizada; um witness vanilla contrário refutaria isso.

## 4. Decisão L0 proposta

1. Adicionar `MathUndershellElem { body, annotation, annotation_explicit }` ao
   owner coeso `entities/elements/math_underover.rs`.
2. Adicionar `Content::MathUndershell(Arc<MathUndershellElem>)`, preservando
   identidade e presença causal.
3. Registrar `math.undershell`; a forma bare delega ao mesmo builder e aplica
   exatamente assinatura, casts e diagnósticos medidos.
4. Projetar `undershell(body: ..., annotation?: ...)`; omissão e `none`
   explícito permanecem diferentes em repr e fields.
5. O owner de layout reduz localmente para body com U+23E1 abaixo e annotation
   inferior opcional, mantendo o match central magro (ADR-0109).
6. Classes esquerda/direita herdam recursivamente o body; U+23E1 e annotation
   não substituem essa classe.
7. Reconstruções e walkers preservam variante, filhos e presença;
   `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos helpers introspectivos/L3.

As demais rotas P1422 permanecem fora de escopo.

## 5. Gate ADR-0127

Esta proposta cria elemento e variante públicos e corrige o comportamento por
defeito da rota bare. Este commit contém somente passo e L0 e **PARA**. Testes,
código, ataques e resselo começam apenas após confirmação humana explícita.

## 6. Execução após confirmação

Congelar manifesto/contrato/oráculos/mutantes; obter RED atribuível; implementar
carrier, builder, dispatch, reflexão, walkers e layout; exigir 100% dos mutantes
rejeitados e zero `Unknown`; executar suítes focal/completa, build, fmt,
`git diff --check` e `crystalline-lint .`; então criar commits separados de
produto e certificado.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 em 2026-09-18 após o commit
de proposta `89544dadd`. A cadeia RED→GREEN, ataques, resselo e veredito está
autorizada sem ampliar o escopo acima.

## 8. Execução e veredito

O RED focal produziu 9 erros atribuíveis aos símbolos P1451 ausentes. Após a
materialização, 5/5 oráculos P1451 ficaram GREEN. A campanha adversarial
rejeitou 39/39 execuções (13 mutantes, ordem forward, repetição forward e
reverse), score `1.0` e zero `Unknown`.

Em 2026-09-18T20:32:13-03:00, sobre a working tree não commitada baseada em
`89544daddf4da93ca16d33dbcbdc6970a1f46a96`, passaram `cargo fmt --check`,
`git diff --check`, `cargo build --workspace` e `crystalline-lint .`. A suíte
completa mediu 5923 testes: 5915 passaram e as mesmas 8 falhas históricas
permaneceram, nenhuma atribuível ao P1451. Recibos e selo ficam em
`00_nucleo/diagnosticos/p1451-*`.
