# Passo 1450 — Materializar `math.underparen` com identidade e geometria de spreader inferior

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
**Data da medição:** 2026-09-18T19:18:31-03:00  
**Commit medido:** `e3e0cced85ea675faa58a9a4c5facd57e5ac903a`  
**Árvore medida:** `206c2a4e03f5379063ccf381dba9bbf4854d4ead`, limpa (`git status --short` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual ordena `math.underparen` após `math.underbracket` e
antes de `math.undershell`. P1449 fechou `underbracket`; portanto a próxima
unidade autocontida é `math.underparen`. `undershell` fica fora deste recorte.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `math/underover.rs:98-111` declara `UnderparenElem` com
`body: Content` obrigatório/posicional e `annotation: Option<Content>`
opcional/posicional. `math/ir/resolve.rs:1347-1361` usa
`resolve_underoverspreader`: U+23DD (`⏝`) abaixo do body, classe `Diacritic`,
stretch horizontal de 100% com shortfall zero e annotation como script
inferior. O composto conserva a classe matemática do body.

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/underover.rs` | `49fe33135766b83c16d27f84c51bd8f867cd5f3f790b075242e8043661a7e298` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |

O binário medido foi `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
que reporta `typst 0.15.1 (586e1bd4)` e corresponde ao pin ratificado.

### 2.2 Observáveis públicos medidos no vanilla

- `repr(math.underparen)` produz `"underparen"`;
- body-only produz `underparen(body: [x])`;
- com annotation produz `underparen(body: [x], annotation: [n])`;
- `underparen([x], none)` conserva `annotation: none`, enquanto omissão não
  assenta esse campo;
- `fields()` contém sempre `body` e somente contém `annotation` quando ela foi
  fornecida; `func()` devolve `underparen`;
- `$ underparen(x, n) $` preserva a mesma identidade pública;
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

`math.underparen` falha com `module math does not contain underparen`. A forma
bare produz `equation(block: true, body: underparen(x","n))`, sem identidade
pública própria; terceiro positional permanece no conteúdo e named é
silenciado. A geometria madura de under/over já existe, logo o defeito é de
contrato, morfologia e fronteira de argumentos.

Hashes dos consumers no estado limpo medido:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_underover.rs` | `c92c5831f29d3625dd5b0dab6d39bd9ad7066f3384d717c1e055ea6c6d9a366e` |
| `01_core/src/entities/content.rs` | `084073ada3402f913b7b0dde88390ed825adc239f90e3702d75c135ee0694c8c` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `61853ecbb2c580954fb0496ebf989054a10e8e1236fe7af51e16ab3079b52522` |
| `01_core/src/compiler/eval/math.rs` | `420367e7ce795f7b4c07521853851ca98192b2c359023d11d85697d6cf53962a` |
| `01_core/src/compiler/eval/repr.rs` | `438050952604f6ef17af146bf490e825a2a308963b49ccac503c38e9e9b9fd00` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `82e0ac47f58101bb452d5ea452ad51099943f4ef7642fe760f813c22e6356d8a` |
| `01_core/src/compiler/math/layout/underover.rs` | `2ba1def13469094097c53c9de27e15f11f902faa69ed66069bad60292783a216` |
| `01_core/src/compiler/math/layout/mod.rs` | `e2a876c3ae145e4dba05b0fc7f5b2abbf2fb2db80a76f41270e0b0493bb22349` |
| `01_core/src/compiler/math/layout/spacing.rs` | `b60f35c05d1e96f766ddd4c01b20d823af5792b74f5b9bb912ae1ca3f0ae1fee` |
| `01_core/src/compiler/eval/mod.rs` | `56cdb57f9b5048aa12e2248be3d838066f5eadd86ffead059d1ec67921228500` |
| `01_core/src/compiler/introspect.rs` | `4feb0329fc5437de7d2791b629fa38fbaef261a9bd96462bb5bd1f835b085961` |
| `01_core/src/compiler/introspect/locatable.rs` | `d234e941233fed7b85c98a6171c04f85e0a8f28d34bddcb56e7c01e4ce74fc35` |
| `01_core/src/compiler/layout/mod.rs` | `4dcb48e3ab81ed2416abd0cae40b2d1a9325f5e55d27918865ce154d806a1bb0` |
| `03_infra/src/query_helpers.rs` | `f60b2b2fa79804e0d4564783da20ed8d8eaf351fe853c3569fa87d56df74b36c` |

## 3. Classificação anti-deriva

Identidade `underparen`, assinatura/casts, presença explícita, repr, fields,
classe herdada e geometria observável são linguagem (ADR-0107). A árvore IR,
caixas e a redução cristalina para `MathUnderover` são mecânica. Inferência:
stretch de 100% e shortfall zero exigem que U+23DD cubra o body antes de a
annotation ser centralizada; um witness vanilla contrário refutaria isso.

## 4. Decisão L0 proposta

1. Adicionar `MathUnderparenElem { body, annotation, annotation_explicit }` ao
   owner coeso `entities/elements/math_underover.rs`.
2. Adicionar `Content::MathUnderparen(Arc<MathUnderparenElem>)`, preservando
   identidade e presença causal.
3. Registrar `math.underparen`; a forma bare delega ao mesmo builder e aplica
   exatamente assinatura, casts e diagnósticos medidos.
4. Projetar `underparen(body: ..., annotation?: ...)`; omissão e `none`
   explícito permanecem diferentes em repr e fields.
5. O owner de layout reduz localmente para body com U+23DD abaixo e annotation
   inferior opcional, mantendo o match central magro (ADR-0109).
6. Classes esquerda/direita herdam recursivamente o body; U+23DD e annotation
   não substituem essa classe.
7. Reconstruções e walkers preservam variante, filhos e presença;
   `ObservationEq` compara os três campos.
8. A variante é não-locatable e terminal nos helpers introspectivos/L3.

`math.undershell` permanece fora de escopo.

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
