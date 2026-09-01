# P1292 — recibo de implementação do lote C (`math.vec`), reabertura v12

**Papel:** implementador segregado, lote C somente
**Regime:** protocolo completo segregado, sem atestação técnica de isolamento
**Estado:** sem mudança produtiva legítima; RED isolado em helper de teste legado
**Manifest de integração mais recente recebido:** `fcf9c3dff41e869272068c12d65cd0b4c1801cf36b6ce9aec337c6dac86f0552`
**Contrato canônico v12 recebido:** `493f5c58ce956b5a142cbf5016db4bbd7e4dc776aaff86749be7ebd156c2e5ef`
**Seal v12 recebido:** `d1a04bcc9351d8f63e77c35ee8bfddf581691bf44484c35b9c4975173edaae8f`

## Reabertura final — diagnóstico reproduzível sem patch produtivo

O receipt C anterior de SHA-256
`64e736f0614a8be8acbfac4cc5eb25ed7a868f375a058c68764e32b81c9d228d`
fica supersedido por este diagnóstico. A verificação final expôs o RED:

```text
cargo test -q -p typst-core \
  compiler::eval::tests::tests::p1030_set_vec_delim_aplica \
  -- --nocapture

left: None
right: Some(('[', ']'))
```

O comportamento pedido já está legitimado pelos L0 v12 vigentes:
`compiler/eval/math.md` exige que a sintaxe `vec` consuma defaults/set-rules da
style chain e produza `Content::MathVec`; `compiler/stdlib/structural/math.md`
define o cast/cascata de `delim`; `entities/elements/math_vec.md` exige
identidade vetorial distinta, nunca normalizada para matrix; e
`compiler/math/layout/vec.md` mantém a geometria nesse owner próprio.

### Causa isolada

O caminho produtivo está coerente com esses L0:

- `compiler/eval/rules.rs` inclui `("vec", "delim")` em
  `MATH_SET_LIGADOS` e grava `math.vec.delim` na style chain;
- `compiler/eval/math.rs` lê `math.vec.delim`, aplica o cast e chama
  `Content::math_vec_full`;
- o payload conserva `delim=('[', ']')` com identidade `MathVec`.

O RED nasce no helper congelado `find_matrix_delim` de
`01_core/src/compiler/eval/tests.rs`: ele reconhece somente
`Content::MathMatrix`. A própria docstring ainda afirma que `vec` constrói
MathMatrix, comportamento anterior refutado e substituído por P1292. Assim o
helper devolve `None` mesmo quando o set-rule foi aplicado corretamente.

Uma fixture própria pública confirmou os dois observáveis separadamente:

```typst
#set math.vec(delim: "[")
$ vec(1, 2) $
```

O PDF extrai como coluna delimitada por `[` e `]`; uma segunda fixture com
`repr` conserva a identidade `vec(children: ([1], [2]))`. A ausência de
`delim` no `repr` é correta para set-rule: o valor efetivo muda, mas o bit
morfológico `explicit` permanece falso.

Não existe patch no owner produtivo C capaz de tornar o helper
`MathMatrix`-only GREEN sem degradar, duplicar ou embrulhar `MathVec` como
matrix, violando os quatro L0 selados. A correção necessária pertence ao
helper de teste (adicionar braço `Content::MathVec`), arquivo cuja edição foi
explicitamente proibida nesta reabertura. Portanto não houve RED→GREEN deste
teste e nenhum código produtivo foi alterado. O focal C vigente permaneceu
GREEN em 6/6.

### Proveniência

Medições em `2026-09-01T12:25:50-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree compartilhada e
não commitada. `git diff HEAD --stat` registrou
`47 files changed, 3318 insertions(+), 508 deletions(-)`; essas alterações já
existiam na árvore compartilhada e foram preservadas. Nesta reabertura o papel
escreveu somente duas fixtures em `/tmp` e esta seção do receipt. Não leu nem
executou `04_wiring/tests/p1292_contract.rs`, seal, oráculo protegido, ataques
ou veredito.

| Entrada/artefato | SHA-256 |
|---|---|
| `00_nucleo/prompts/entities/elements/math_vec.md` | `72f2ba668f201d0fb0edf7d2d6bd8f5f85aab156b883a8e644b427f7cd2d1afa` |
| `00_nucleo/prompts/compiler/math/layout/vec.md` | `772331ee68ec1069b57e80dd6674ef60d62fcd8db83d7d3fae15a18b7b82b764` |
| `00_nucleo/prompts/compiler/stdlib/structural/math.md` | `087e24d091d9ab8a191374d5a1ac8c47f8e01a7e2283602f73cc0028cb53e236` |
| `00_nucleo/prompts/compiler/eval/math.md` | `f40e487b4462da0b90bb31236f5c4ce3279b7b6cfa01f38bc8f6b4f80d23f8e8` |
| `01_core/src/entities/elements/math_vec.rs` | `d88b24532f3e51e44713b868e4f93fd2cfbe755971ce15d62f50fdfbe18917d7` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `6b58286a0e99ae4acd7e2a32156c4da7ad091fdcec163ff6c14ed7c71060f241` |
| `01_core/src/compiler/eval/math.rs` | `e5dde4dfdeec1a07744129b0f5cd59e883f0a2eadca8c29c34a2d7c2b0904a87` |
| `01_core/src/compiler/eval/tests.rs` congelado | `c275130b92301de5ad0468920b0fca6e214a137487eecfaa95c222abdfc7d1ce` |
| `target/debug/typst` | `7f4fec533229b34676d8ecec4094635b1e2fb638c90ea81392ebc5b0a0c88052` |
| `/tmp/p1292-c-set-vec-delim.typ` | `adbb16e736fa75c6706b04b10479992aa9d0f9eacd76dcc0d276ed37449a534f` |
| `/tmp/p1292-c-set-vec-delim.pdf` | `4bc4dd88329d5b6346caf915ad3ea463be7345cb37a8c359862b8d716d879c52` |
| `/tmp/p1292-c-set-vec-delim-render.typ` | `6202816de7883488ab9df40f73a0a51bc24c7ced348e65407adcad23c06d30b3` |
| `/tmp/p1292-c-set-vec-delim-render.pdf` | `fb08a634430d35de292630b61630c42c72ff5a912aac40a2b0bf0c1ddbaec2c0` |

| Comando | Resultado |
|---|---|
| teste `p1030_set_vec_delim_aplica` sob timeout | RED reproduzido: `None` vs `Some(('[', ']'))` |
| `env RUSTFLAGS=-Awarnings timeout 240s cargo test -q -p typst-core p1292_c_ -- --nocapture` | PASS — 6/6 |
| compilar as duas fixtures próprias sob `timeout 30s` | PASS |
| `pdftotext -layout` do render | PASS — delimitadores `[` e `]` visíveis |

Parada sem patch produtivo e sem alegação de fecho geral. A continuação requer
autorização para corrigir o helper test-only congelado; não requer owner ou L0
produtivo novo.

---

## Histórico — recibo final C v7

**Papel:** implementador segregado, lote C somente
**Regime:** protocolo completo segregado, sem atestação técnica de isolamento
**Estado:** GREEN nos focais, fixtures bilaterais e gates próprios; paragem antes de D
**Manifest v7:** `13dcb1703a52aada4c18fbbbca5b0921581d3412eacba60cdf7b3540db0f2f50`
**Contrato v7:** `415d8abb1cd5df02fcc08fd2ea90c2d423b67751ef0a09818fb8cc511b225244`
**Seal v7:** `dc59cefb3aa50cbf38d38b4452aacdddf3b5eeb5a1484d9281206cdc17c65bcc`
**Oráculo v7 declarado:** `6ca930ea9b03c8389c9b9bf0d749e90251ccf81322411b7625f26e4fc48a0533`
**Recibo RED v7 declarado:** `0c34e9e2058c5d69be6e92b9e357084d0cd18350fc7218fd356afd68f0c55361`

O oráculo protegido não foi lido, executado ou editado por este papel. Este
recibo substitui o refresh C anterior, SHA-256
`95362a4ba920237859cab0b0434c04217e75e6fb6dc1a065d5d91f00116bee2f`.

## Proveniência e capacidades

Medições finais em `2026-09-01T02:27:18-03:00`, HEAD
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada e
compartilhada. `git diff HEAD --stat` registou 35 arquivos, 1663 inserções e
331 remoções; a árvore continha alterações dos lotes/papéis anteriores, todas
preservadas. Este papel escreveu somente produção/testes próprios C
allowlisted e este recibo. Não leu `materialization/context`, contrato privado,
oráculo protegido, ataques ou veredito.

## Implementação C consolidada

- `MathVecElem`/`Content::MathVec` preservam identidade, children variádicos,
  cardinalidade/ordem, delim, align, `Rel<Length>` e bits de presença.
- `math.vec` qualificado e a sintaxe `vec` convergem no mesmo constructor;
  named e erros não são descartados.
- `gap` aceita `Relative`, `Length` e `Ratio`; ratio puro materializa
  `Rel { rel: ratio.get(), abs: Length::ZERO }`, enquanto inteiro mantém o
  erro exato `expected relative length, found integer`.
- Layout usa denominator style e resolve a componente percentual contra a
  altura efetiva finita; em região auto/infinita somente o percentual zera.
- Dispatcher estático, spacing, introspect/locatable e query classifications
  seguem os owners congelados.

### Correção v7 — projeção morfológica estreita

Somente no braço `Content::MathVec` de `compiler/eval/repr.rs`, children
diretos `MathIdent` e `MathText` são formatados como um `Content::Text`
temporário pelo próprio `repr_content`, produzindo `[a]`, `[23]` e `[α]`.
O valor armazenado não muda: não há wrapper, field, provenance bit, mutação de
igualdade/hash/traversal ou branch de renderer. Variantes estruturadas e
markup continuam no formatter próprio sem projeção recursiva.

O header de `repr.rs` usa o hash canônico v7 calculado pelo linter,
`@prompt-hash 643e33d3`. Nenhum `--fix-hashes` foi executado.

## Arquivos C e SHA-256 finais

```text
d88b24532f3e51e44713b868e4f93fd2cfbe755971ce15d62f50fdfbe18917d7  01_core/src/entities/elements/math_vec.rs
00f30e9c3fc49980850da752b16dfc095c1f77f452e810175a7e0d275a5a1f96  01_core/src/entities/elements/mod.rs
dfcd79f4ecde2868141cea47947f7114afcc25da8c6536fd5e7c1ebdd499037f  01_core/src/entities/content.rs
6b58286a0e99ae4acd7e2a32156c4da7ad091fdcec163ff6c14ed7c71060f241  01_core/src/compiler/stdlib/structural/math.rs
e5dde4dfdeec1a07744129b0f5cd59e883f0a2eadca8c29c34a2d7c2b0904a87  01_core/src/compiler/eval/math.rs
e17357770114cb9fb4431890629fd1f26b793023598581c2f8a40247ba5c7170  01_core/src/compiler/eval/repr.rs
d1368de3e887cdd26fd1ee6277054bdf68bac20b8b0e7ee2915b4b8f554ec98e  01_core/src/compiler/math/layout/vec.rs
8724834cafa14bbfac12ca8db2d1fcabcbbc0b067657c2abc5c08f4daa6928c6  01_core/src/compiler/math/layout/mod.rs
6b17c734af6e680a6c46158958fce33f02a08a8029f896cdcc9099941aa2fcf9  01_core/src/compiler/math/layout/spacing.rs
78acae9f1f2cfe6eb3b384e6851e48c63dd76d43a644951504c6e0c0cfd4b5d5  01_core/src/compiler/math/layout/tests.rs
b33746bc2e49cf86dda294a14edda2af59d90d60b974a4a7d7a04a1e2581454a  01_core/src/compiler/layout/mod.rs
0137beee4d65161351331b0c4bcdb9c2049190865dea95d58ba015cbe4a00f72  01_core/src/compiler/introspect/locatable.rs
130314038d5a9048a58da08d1c26f1db858a86e4e78a3f51f90458800baafa5e  01_core/src/compiler/introspect.rs
986d73a111b9884aa7c35e56caf8d47f719236cd52b60a6286d29341463ed1b3  03_infra/src/query_helpers.rs
```

Nota: o hash `3317205e...` é registrado apenas como predecessor causal de
`repr.rs`, não como hash final.

## Fixtures próprias bilaterais

```text
5103cdcdd0aa56390012bbec778e5ec77cfe8f8ab15c9b83508340edaf3b4550  /tmp/p1292-c-v7-repr-edges.typ
3235eff5a5c3d5c73617c25a2d32d2466b8e65df7e9529b161f69960ababbc14  /tmp/p1292-c-syntax-morph-bare.typ
e540960293e4d9f6fc0636ecd45bdb40d1f27741fd307edd9c087d1dbfccb8d2  /tmp/p1292-c-syntax-morph-qualified.typ
```

Resultados candidato versus vanilla ratificado:

- syntax/qualified para `a,b`: ambos
  `vec(children: ([a], [b]))`;
- números `1,23`, quoted `foo,bar` e símbolos resolvidos `α,β`: todas as
  projeções directas coincidem bilateralmente e entre syntax/qualified;
- `strong`/`emph`: conserva `strong(body: [a])` e `emph(body: [b])`, sem
  double-wrap;
- grupo estruturado mantém o formatter genérico candidato e não recebe
  projeção recursiva. A divergência genérica `MathLr` versus vanilla permanece
  `Unknown` explícito do L0 v7; syntax e qualified candidatos convergem entre
  si, e nenhum resultado foi mascarado;
- SVG bare antes/depois da projeção é byte-idêntico, SHA-256
  `7a40d4076aa900461348d198fed2653188ff1ff518d7b4cd078e60ebda59802a`;
- as posições bare continuam bilaterais com vanilla: delimitador esquerdo
  `(0, 7.601)`, `a` `(6.567, 1.8414)`, `b` `(7.0213, 11.7106)` e delimitador
  direito `(11.341, 7.601)`.

A igualdade byte-a-byte acima é usada somente como controle negativo de
regressão pré/pós do mesmo candidato; a aceitação bilateral usa os observáveis
linguísticos e geométricos, não bytes (ADR-0107).

## Comandos e resultados

```text
cargo test -p typst-core p1292_c_v7_ -- --nocapture
GREEN: 2 passed; 0 failed; 5355 filtered out

cargo test -p typst-core p1292_c_ -- --nocapture
GREEN: 6 passed; 0 failed; 5351 filtered out

cargo build -p typst-wiring
GREEN

cargo check -p typst-core -p typst-infra --tests
GREEN

crystalline-lint . --checks v1,v2,v15,v26 --fail-on warning
GREEN: No violations found

crystalline-lint . --checks v5
GREEN para `compiler/eval/repr.rs` e todos os owners C. Permanecem quatro
warnings anteriores e fora de C: compiler/eval/mod.rs,
compiler/math/layout/cancel.rs, compiler/stdlib/layout.rs e
entities/elements/math_cancel.rs.

cargo fmt --check
GREEN

git diff --check
GREEN
```

As queries bilaterais e as compilações de render próprias terminaram com exit
0. O oráculo protegido e o gate de veredito não foram executados.

## Paragem serial

Lote C finalizado no escopo v7. Não foi iniciada implementação de
`Flush`/lote D. A evidência atesta somente o fragmento C coberto; não declara
equivalência funcional geral nem promove o `MathLr` estruturado de `Unknown`.
