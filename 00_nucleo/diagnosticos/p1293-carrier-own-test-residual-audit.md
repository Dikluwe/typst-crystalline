# P1293 — auditoria causal independente dos resíduos próprios pós-carrier

## Escopo e conclusão

Auditoria somente leitura do produto e dos testes próprios que ficaram vermelhos
após a introdução de `MathAttachSlot`. Entrada autorizada:
`p1293-implementation-receipt-b.md`, SHA-256
`3c3818245c2704d6dfe3724b23dde0db5de16a6d247709a25d4badfa5529cd8d`
(o sufixo não hexadecimal recebido com a tarefa foi tratado como erro de
transcrição, sem ampliar a leitura).

Conclusão causal:

1. `p1293_omissao_e_none_explicito_continuam_sem_spacing` é **teste obsoleto
   por codificação antiga do input**. Ele ainda passa `Some(Content::Empty)` à
   API interna binária; após o carrier, esse valor é a projeção de
   `Present(Content::Empty)`, não de `ExplicitNone`. O produto já trata
   `ExplicitNone` como omissão na fronteira correta.
2. `p1293_plain_text_e_traversal_preservam_carrier` tem **expectativa textual
   errada**. `ExplicitNone` não produz marcador; `Present(Content::Empty)`
   produz o marcador do quadrante, com payload textual vazio. O traversal
   preserva os três estados e transforma somente o conteúdo de `Present`.

Portanto, os dois resíduos requerem somente correções test-only. Não foi
identificada correção produtiva, alteração de L0, novo carrier, sentinel,
heurística ou gate ADR-0127. Esta auditoria não aprova o lote B nem executa os
gates de fechamento.

## Proveniência da medição

- instante pré-gravação: `2026-09-02T10:44:27,854809099-03:00`;
- HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`;
- branch: `Tekt`;
- working tree compartilhada e não commitada;
- SHA-256 de `git status --short`: `9c599fda46ef70a605f080a77a1bcf7d17b4bec72185b5b00c752ff6ee99549a`;
- SHA-256 de `git diff HEAD --stat`:
  `7eb081946d0abe22b6bde1969b0f4386b5b2a00a428ea10a53c84a6a12b9994b`;
- estatística: `55 files changed, 5238 insertions(+), 687 deletions(-)`;
- único write no repositório por esta auditoria: este recibo;
- temporários próprios: `/tmp/p1293-carrier-own-test-*`.

Hashes relevantes no instante:

| Artefato | SHA-256 |
|---|---|
| Núcleo `math-attach-slot-presence.toml` | `29a7ec096d899850eb602afa0cbe0b44d46147214b06296f2e90c40776e3cb7b` |
| L0 `compiler/math/layout/attach.md` | `16c9ac2baf6bbcdd866dd3ee90c949c51580f549091ec064e4ca8fdd5c80759a` |
| L0 `entities/elements/math_attach.md` | `8be46d4c97b9f63b0b757274610bb257c1e1f19378bcc7cf342039bd00f00ae7` |
| L0 `compiler/stdlib/structural/math.md` | `18b2fb0d9a3e1f6a0ff9cd8fb2e25047b4c9f2185cb6a511ae022e7a9c4e80a2` |
| L0 `compiler/eval/repr.md` | `d9499bbbc889eb42d485c700823cd428950bedefda4496065488a678c6e8b500` |
| `compiler/math/layout/attach.rs` | `73c77f10fcd663fbd07f5a102458a5a226b92fdd99b4c9911ed0173583558be3` |
| `entities/elements/math_attach.rs` | `60e38cb641bada40339788337fc5cd3e70f6adf3ff56d184e2f45d1168aa2bb6` |
| `compiler/stdlib/structural/math.rs` | `657b33efd5a08c5bc845d9190cfdfa5fc62442349894336fc377b19d3aa85bec` |
| `compiler/eval/repr.rs` | `9c5316ecaa9332379bad5afa1c1e8de1e504e77d085c91b475480591b30a3d1b` |
| `compiler/math/layout/mod.rs` | `1c386e48028c85d1a93ab35829b910a9f03d1b8e12155bae4c14efadf75492a4` |
| `entities/content.rs` | `34838354261fa472c5efbc73c39ee6587b8b77be4d4606ca7587537818ed4a98` |
| `entities/math_constants.rs` | `7954ed7045f293ead728c784fb669aa3077303576de35f91dde6e4ef9717f672` |
| binário de teste corrente | `46560d9858a0d288d7493a2324523cc1a09c65f83e0b09b411540638ace40cd0` |
| vanilla ratificado `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| vanilla `math/scripts.rs` | `d3f8a9fc8a4f58eafdc3edbac9cdb67279ff0f023dd9b4967f209e81165f286b` |
| vanilla `math/attach.rs` | `7efdda527260ad0ac91436b35abe5f98b85019ff8491192c1ed5077c17953f5f` |
| vanilla `math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |
| vanilla `foundations/content/mod.rs` | `70f2fe3abe9ed674452ab0d26729e76ac3ebef540b41420fb9feb5a1ab6dd0c9` |

### Ressalva de integridade do Núcleo

Os quatro L0s inspecionados exibem o pin textual
`81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715`,
mas o SHA-256 bruto do arquivo de Núcleo na working tree é `29a7ec09…`.
As cláusulas semânticas lidas são coerentes entre si e bastam para classificar
estes dois resíduos, mas esta auditoria não declara V26 verde nem corrige o pin.
Esse desvio precisa ser resolvido pelo fluxo de selo antes do fechamento.

## Autoridade semântica medida antes da classificação

O Núcleo, linhas 9–19, define exatamente três variantes e estabelece que
`Present(content)` inclui `Content::Empty`; linhas 23–34 projetam conteúdo e
layout somente de `Present`; linhas 42–49 proíbem inferência por
`Content::Empty` e alteração do carrier global.

O L0 de entidade, `entities/elements/math_attach.md:64-78`, repete a
cardinalidade e decide `plain_text` na ordem `^tl _bl base ^t _b ^tr _br`
**somente para `Present`**. `Omitted` e `ExplicitNone` não acrescentam texto.
O mesmo L0, linhas 87–95, exige que traversal preserve os três estados.

O L0 de layout, `compiler/math/layout/attach.md:744-782`, é posterior e revoga
a codificação antiga de `none` como `Some(Content::Empty)`: somente
`Present(Content)` cria caixa e conta para `SpaceAfterScript`; `Omitted` e
`ExplicitNone` não criam caixa. A mudança pública do carrier já foi classificada
como ADR-0127 categoria 1 e confirmada em `2026-09-02T08:06:37-03:00`.

Classificação ADR-0107: omissão, `none`, markup vazio e `repr` são morfologia de
linguagem; largura do frame é geometria observável; enum Rust, helper de
projeção, `Option<&Content>` interno e forma do teste são mecânica. A decisão
abaixo sucede essas medições e registra refutadores, conforme ADR-0108.

## Resíduo 1 — input antigo confunde `Present(empty)` com `ExplicitNone`

### Trace causal `file:line`

1. `compiler/stdlib/structural/math.rs:638-647` mapeia `Value::None` para
   `MathAttachSlot::ExplicitNone` e todo `Value::Content`, inclusive
   `Content::Empty`, para `MathAttachSlot::Present(content)`.
2. `compiler/math/layout/mod.rs:668-677` envia os seis carriers a
   `layout_attach_slots`.
3. `compiler/math/layout/attach.rs:22-49` é a fronteira de projeção:
   `Present(content) -> Some(content)` e
   `Omitted | ExplicitNone -> None`.
4. A API interna inferior `layout_attach`, linhas 53–63, recebe só
   `Option<&Content>` e não pode distinguir a origem do `Some`.
5. O teste vermelho, `attach.rs:740-756`, não usa a fronteira triestatal. Cria
   `none = Content::Empty` e passa `Some(&none)` diretamente à posição `tr`.
   Isso codifica `Present(Content::Empty)`.
6. Linhas 146–153 criam uma `MathBox` para todo `Some`; linhas 311–314 leem
   `SpaceAfterScript`; linhas 421–424 adicionam essa constante a `tr_post`.
   `FixedMetrics` usa `56du` (`entities/math_constants.rs:273`). Em `12pt`:

```text
56du × 12pt / 1000du = 0,672pt
7,872pt − 7,200pt = 0,672pt
```

Reprodução direta do teste corrente:

```text
target/debug/deps/typst_core-d51858d7cd5964e0 \
  p1293_omissao_e_none_explicito_continuam_sem_spacing --nocapture
=> FAIL; left 7.871999999999999, right 7.199999999999999
```

Dois controles produtivos correntes fecham a distinção:

```text
... p1293_b_attach_none_explicito_tem_layout_equivalente_ao_omitido
=> PASS 1/1

... p1293_markup_vazio_presente_recebe_um_space_after_script
=> PASS 1/1
```

O primeiro controle, `structural/math.rs:1662-1693`, constrói
`ExplicitNone` e `Omitted`, atravessa `Content::MathAttach` e obtém extent e
quantidade de items iguais. O segundo, `attach.rs:710-736`, passa markup vazio
presente aos quatro quadrantes e mede exatamente um `SpaceAfterScript`.

### Contraprova vanilla pública

Fontes próprias qualificadas, a `11pt`, produziram SVGs vanilla:

| slot `tr` | largura × altura vanilla |
|---|---:|
| omitido | `5,808 × 7,513pt` |
| `none` | `5,808 × 7,513pt` |
| `[]` | `6,424 × 7,513pt` |

O delta `6,424 − 5,808 = 0,616pt` é exatamente
`56du × 11pt / 1000du`. Os SVGs omitido e `none` são byte-idênticos
(`2021ac08…`); `[]` tem SHA `4e1cf656…`. Isso coincide com o vanilla em
`math/ir/resolve.rs:554-568`, que mantém `Some` ao resolver conteúdo vazio, e
`math/scripts.rs:149-177`, que adiciona a constante a cada post-script
presente.

### Correção indicada e owner

Correção **somente test-only** no owner já existente
`compiler/math/layout/attach.md` → `compiler/math/layout/attach.rs`:

- importar `MathAttachSlot` no módulo `smoke`;
- substituir, dentro do teste em `:740-759`, as chamadas diretas a
  `layout_attach` por chamadas a `layout_attach_slots`;
- usar `MathAttachSlot::Omitted` nos seis slots do controle e
  `MathAttachSlot::ExplicitNone` apenas no slot sob teste;
- manter os asserts de igualdade de largura e items.

Não se deve trocar a expectativa para `7,872`, pois o nome e a obrigação do
teste são `ExplicitNone`; tampouco se deve ensinar `layout_attach` a tratar
`Content::Empty` como `none`, pois isso recolapsaria `Present(empty)` e violaria
o Núcleo.

## Resíduo 2 — marcador indevido na expectativa de `ExplicitNone`

### Decomposição do vetor exato

O input em `entities/elements/math_attach.rs:179-188` é:

| componente | carrier | contribuição de `plain_text` |
|---|---|---|
| `tl` | `Present("tl")` | `^tl` |
| `bl` | `Omitted` | vazia |
| base | `"x"` | `x` |
| `t` | `ExplicitNone` | vazia |
| `b` | `Present("b")` | `_b` |
| `tr` | `Present(Content::Empty)` | `^` |
| `br` | `Present("br")` | `_br` |

Logo o valor canônico é `^tlx_b^_br`, exatamente o observado. O esperado
`^tlx^_b^_br` contém um `^` extra entre base e `_b`, isto é, atribui marcador
ao `t: ExplicitNone`, em contradição direta com o L0.

`math_attach.rs:23-29` projeta conteúdo somente de `Present`; o `plain_text`
em `:67-87` usa essa projeção. O traversal em `:31-39,91-103` preserva
`Omitted`/`ExplicitNone` e chama `map_content` somente dentro de `Present`.

Um executável temporário próprio ligado ao `typst-core` corrente enumerou:

```text
Omitted="x"
ExplicitNone="x"
PresentEmpty="x^"
PresentText="x^T"
mapped=t:ExplicitNone;bl:Omitted;tr:Present(empty);b:Present(text("B"))
```

- fonte: `/tmp/p1293-carrier-own-test-rust/src/main.rs`, SHA-256
  `7238ea9d4bb6ee4c1e1d115ca90ea6660bb97316c183cd4ecd2c2ff81dc59a58`;
- binário: SHA-256
  `4a2b83e96f55d5b3d4e125dc6d03560fb2485c94ba577e01b0305dad1a8ba885`;
- output: SHA-256
  `2c0840a85b3f5f1c97e74645ecf76109d74a8419fe1f24e2d386ff0bf0cf14bd`;
- comando: `cargo run --offline --quiet --manifest-path
  /tmp/p1293-carrier-own-test-rust/Cargo.toml`.

A execução do teste corrente reproduziu `left: "^tlx_b^_br"`,
`right: "^tlx^_b^_br"`. Os asserts de traversal não são alcançados após essa
falha, mas o probe independente executou a mesma transformação e confirmou os
quatro estados finais.

### Vanilla, `repr` e observabilidade

O vanilla `math/attach.rs:19-49` declara `AttachElem` sem implementar/derivar
`PlainText`; `foundations/content/mod.rs:418-427` extrai texto visitando os
elementos que implementam `PlainText`. Portanto, o vanilla não fornece suporte
para sintetizar um marcador a partir de `none`; `none` não possui conteúdo a
visitar. Os marcadores `^`/`_` do método cristalino são uma representação
textual interna contratada pelo seu L0, não uma exigência de igualdade mecânica
com o trait vanilla.

Na superfície pública morfológica, uma fonte própria com `repr` no vanilla
ratificado produziu:

```text
omitted=attach(base: [x])
none=attach(base: [x], t: none)
empty-markup=attach(base: [x], t: [])
empty-string=attach(base: [x], t: [])
text=attach(base: [x], t: [T])
```

Fonte `/tmp/p1293-carrier-own-test-repr.typ` SHA-256 `00de0b94…`; output
extraído SHA-256 `9e492b4b…`. O controle corrente
`p1293_b_attach_repr_preserva_slots_ordem_e_none_explicito` passou `1/1` e
confirma no candidato que `repr` omite `Omitted` e imprime `none` para
`ExplicitNone`. Assim, `repr` preserva a morfologia enquanto `plain_text`
projeta apenas conteúdo presente; não há contradição entre os consumers.

### Correção indicada e owner

Correção **somente test-only** no owner existente
`entities/elements/math_attach.md` → `entities/elements/math_attach.rs`:

```rust
assert_eq!(elem.plain_text(), "^tlx_b^_br");
```

Nenhuma linha produtiva deve mudar. Em particular, fazer `as_content` devolver
`Some(Content::Empty)` para `ExplicitNone`, ou acrescentar marcador por variant
fora de `Present`, quebraria simultaneamente `sup/sub`, layout e a regra
compartilhada do Núcleo.

## ADR-0127 e refutadores

As duas correções indicadas alteram somente inputs/expectativas de testes para
o contrato público já confirmado; não alteram assinatura, default,
compatibilidade nem fase. Portanto não constituem novo gate ADR-0127.

Refutariam estas conclusões:

- `layout_attach_slots` projetar `ExplicitNone` como `Some`;
- o teste carrier-aware de `ExplicitNone` divergir em extent ou items da
  omissão;
- `Present(Content::Empty)` deixar de reservar exatamente um
  `SpaceAfterScript`;
- o L0/Núcleo vigente passar a exigir marcador para `ExplicitNone`;
- o probe de traversal mudar `Omitted`/`ExplicitNone`, perder
  `Present(Content::Empty)` ou transformar conteúdo fora de `Present`;
- uma superfície pública demonstrar que `none` e omissão deixaram de ser
  layout-equivalentes ou que `repr` deixou de distingui-los.

Nenhum desses refutadores ocorreu. A ressalva de pin do Núcleo permanece
independente e impede usar este recibo como aprovação de integridade ou do lote.

## Comandos reproduzíveis

```text
sha256sum <L0s, Núcleo, fontes produtivas e vanilla listadas acima>
target/debug/deps/typst_core-d51858d7cd5964e0 \
  p1293_omissao_e_none_explicito_continuam_sem_spacing --nocapture
target/debug/deps/typst_core-d51858d7cd5964e0 \
  p1293_plain_text_e_traversal_preservam_carrier --nocapture
target/debug/deps/typst_core-d51858d7cd5964e0 \
  p1293_b_attach_none_explicito_tem_layout_equivalente_ao_omitido --nocapture
target/debug/deps/typst_core-d51858d7cd5964e0 \
  p1293_markup_vazio_presente_recebe_um_space_after_script --nocapture
target/debug/deps/typst_core-d51858d7cd5964e0 \
  p1293_b_attach_preserva_sete_slots_e_none_explicito --nocapture
target/debug/deps/typst_core-d51858d7cd5964e0 \
  p1293_b_attach_repr_preserva_slots_ordem_e_none_explicito --nocapture
cargo run --offline --quiet --manifest-path \
  /tmp/p1293-carrier-own-test-rust/Cargo.toml
/usr/local/bin/typst compile --format svg <fonte-própria> <output.svg>
/usr/local/bin/typst compile /tmp/p1293-carrier-own-test-repr.typ \
  /tmp/p1293-carrier-own-test-repr-vanilla.pdf
pdftotext -layout /tmp/p1293-carrier-own-test-repr-vanilla.pdf \
  /tmp/p1293-carrier-own-test-repr-vanilla.txt
```

O SHA-256 deste recibo é calculado externamente após a gravação final.
