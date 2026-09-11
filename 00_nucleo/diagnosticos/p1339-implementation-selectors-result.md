# P1339 — implementação subordinada de seletores

Executor `/root/p1339_observation_design`, implementador dos sete owners
delegados. Regime sem atestação de isolamento. Não é selo, aceitação de
contrato nem veredito independente. Não houve commit nem alteração de L0,
oráculo ou contrato por este executor.

## Medição antes da decisão

Base HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não
commitado com integração concorrente. Autoridade
`p1339-implementation-authorities.json` SHA-256
`cab15a1dce423a12fe198a640b541ee00b3f8743ec0f6a9e97519416d940f3c0`;
selo `35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee`;
RED independente aceito `e4e564e606283c15ef792d59de2d36f1a402d6d5d5077f7d6581b23219076f7d`.
O RED local real e sua proveniência estão no recibo
`p1339-implementation-selectors-red.md`; não foi mera falha de compilação.

- `entities/selector.rs:32` contém o grupo ordenado `Element`;
  `entities/show.rs:70` contém a base `NativeElement` sem segundo grupo.
- `compiler/eval/selector_matching.rs:48` transporta grupo vazio como base
  nativa e grupo preenchido como filtros, sem produzir `And` vazio.
- `selector_matching.rs:208` preserva o caminho especial de Par;
  `rules.rs:476,833` são os únicos pontos de lógica ajustados nesse owner.
- `selector_matching.rs:220` identifica funções por ponteiros tipados e
  delega origens semânticas/listas ao predicado NodeKind antecedente.
  `page` tem ABI NativeWithEngine e comparação de ponteiro na assinatura
  correspondente, não cast para o ABI comum.
- `selector_matching.rs:301,319` distingue projeção de Content e snapshot
  Some autoritativo: campo ausente no snapshot não aciona fallback nem
  equivale a Value::None. O caminho Where legado conserva sua projeção e
  comparação anteriores.
- `operators/equality.rs:157,189` compara campos em ordem por `values_eq`
  e reutiliza a mesma relação para CounterKey. Não há atalho reflexivo
  capaz de tornar NaN igual a si. A igualdade global/Hash de entidades não
  foi modificada; pares antigos de seletores conservam a comparação antiga.
- `repr.rs:1043` formata um grupo explícito, inclusive `(:)`, com o
  formatter multiline já existente. `foundations/selector.rs:29` já
  transportava Value::Selector inteiro; sua lógica produtiva não precisou
  ser substituída para preservar Element.

## Decisão implementada e fronteiras

Implementada a extensão interna nos owners selados, sem novo registro,
despacho dinâmico ou identificação por nome no matcher. A igualdade de
Func usada na igualdade pública é a vigente, conforme L0 equality:167;
ela não é usada para reconhecer o receiver como elemento.

O escopo de construção de filtros não é promessa de reconhecimento de
todo produtor já cristalizado. `elements/shape.rs:22-29` só conserva a
geometria; `stdlib/shapes.rs:141,219` compartilha Rect para rect/square e
`:268,331` compartilha Ellipse para ellipse/circle. `elements/transform.rs:19-22`
só conserva matrix/body para transformações. Logo o matcher não infere
identidade de construtor a partir de dimensões ou matriz: Shape/Transform
retornam false. Isso é uma fronteira real, não paridade geral de show/query
nem êxito silenciosamente comprovado desses produtores. A coordenação
encaminhou essa fronteira ao verificador; ampliar entidades ou esconder o
limite alterando oráculos não está na implementação subordinada.

Snapshot Some permite usar os campos efetivamente capturados, não defaults
reconstruídos. Content sem snapshot usa apenas campos representados:
projeção anterior, text.text, raw.text/lang/block e body de origens Styled
medidas. Campos adicionais não representados não são inventados. Não há
ampliação para corrigir lacunas legadas de campos/show/query.

## Estado de verificação local

Acrescentados testes para transporte constructor/parser, grupo repr
vazio/ordenado/multiline, igualdade recursiva e CounterKey, NaN não
reflexivo, ordem dos campos, legado preservado, snapshot autoritativo,
text como nó completo, identidade que não aceita nome forjado, Par e
campos body/raw. A expectativa pública que falhou no RED permanece intacta.

Primeira compilação integrada de testes detectou `EcoVec.reverse` sem
mutabilidade no novo teste; corrigido para `make_mut().reverse()`. O matcher
page também recebeu o ABI correto. As demais falhas do build naquele
instante estavam em conexões de owners sendo integrados; não são usadas
como evidência de resultado semântico. Execução GREEN e suíte integrada
ainda pendentes nesta versão do recibo. `git diff --check` dos sete owners
não produziu erro.

Snapshot dos sete consumers em `2026-09-10T05:56:22Z` (não é snapshot final
da árvore concorrente):

```text
86ca290128cb6e5d2f306c0beeafbf5a71a0b0d00d7cfb7f1125ae2fa55c8024  entities/selector.rs
26372fc47b01a1684329c94942d50ef5326c2f758c5134d87dab1630bedd5ecc  entities/show.rs
3c2120fd4861e4884f95c39242d2270738cb3036d8f2e461d289524d39736fd7  compiler/eval/selector_matching.rs
5356a3fb89b257a9edb6770a148cf959b4abe53da97c69011f3dc46f9cbae47d  compiler/eval/operators/equality.rs
ecb694bda499d5f1afb5b7b453de2f927fc18f303d39290277ab31cfc09f72e6  compiler/eval/repr.rs
b4bb1f81da20435ace5ad90194edbcc022a1ba53602a02cf9cccdb50afdcc639  compiler/eval/rules.rs
397b593d847d704a500a699e98c44e54881c6fc054d40b912159423a1fa2a5df  compiler/stdlib/foundations/selector.rs
```

Paths de código acima são relativos a `01_core/src/`. Não usar esta lista
como substituto da captura final de árvore/build pelo coordenador.

## Handoff técnico — `2026-09-10T05:59:42Z`

Depois do snapshot acima foi acrescentado o teste de identidade distinta
table.header/grid.header com nomes iguais e atualizados comentários do
matcher. Seu SHA-256 corrente é
`e1bed9945e8ab7ac69c11841025d6c531d2f7604ec3ffbd036ed149c47777c1c`;
os outros seis hashes acima não mudaram. Os arquivos estão estabilizados
para integração pelo coordenador, sem reivindicar GREEN. O comando
relevante após completar as conexões globais é:

```text
CARGO_TARGET_DIR=/tmp/p1339-target.UD8gh7 cargo test -p typst-core --lib p1339_
```

A suíte antecedente de selector_matching e repr também deve permanecer
verde; testes legados fora destes owners não foram editados aqui. O diff
local dos sete consumers neste handoff (incluindo os headers previamente
ressellados pelo coordenador) é exatamente:

```text
01_core/src/compiler/eval/operators/equality.rs     | 126 ++++++-
01_core/src/compiler/eval/repr.rs                   |  42 ++-
01_core/src/compiler/eval/rules.rs                  |   6 +-
01_core/src/compiler/eval/selector_matching.rs      | 377 ++++++++++++++++++++-
01_core/src/compiler/stdlib/foundations/selector.rs |  23 +-
01_core/src/entities/selector.rs                    |   5 +-
01_core/src/entities/show.rs                        |   4 +-
7 files changed, 562 insertions(+), 21 deletions(-)
```

Esse diff não descreve nem congela alterações de outros owners. Não é
evidência final da árvore usada pelo build integrado.
