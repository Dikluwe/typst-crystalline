# P1325 — revisão prévia do cohort math-symbol-binding-warning

Regime: A/B, executado sem atestação técnica de isolamento; sem selo. Revisor
`/root/p1325_review`. Entradas: instruções CLAUDE, skill Tekt e suas duas
referências, ADR-0107/0108/0127/0129, fontes e L0 vigentes relacionados abaixo.
O revisor não escreveu L0, implementação ou testes julgados. Capacidade de
escrita concedida: somente diagnósticos `p1325-review-*`. Contexto recebido:
objetivo de implementar o próximo cohort após P1324 e hipótese ainda não
decidida de usar `Module.content()` como discriminante. Root comunicou a
refutação em imports; a fonte foi lida diretamente pelo revisor.

## Proveniência anterior à decisão

Leitura em `2026-09-08T21:46:36-03:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada.
`git diff HEAD --stat`:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  66 ++-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |  53 ++-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 198 ++++++++-
 00_nucleo/prompts/wiring.md                        |  56 ++-
 01_core/src/compiler/eval/bindings/field_access.rs | 258 +++++++++++-
 01_core/src/compiler/eval/call_dispatch.rs         |  92 +++-
 01_core/src/compiler/stdlib/loading.rs             | 466 +++++++++++++++++++--
 04_wiring/src/main.rs                              |  30 +-
 8 files changed, 1144 insertions(+), 75 deletions(-)
```

SHA-256 das entradas causais:

| Entrada | SHA-256 |
|---|---|
| prompts/compiler/eval/bindings/field_access.md | 7573dd209b35e33b3c6627dd14eaea8ef63c819ad7fe7079438baa263c0760e4 |
| prompts/compiler/stdlib/sym.md | 42a2127215e9a37d69c90695e1be66a61320fa086ffd36b51a1ac3441c816f3d |
| prompts/compiler/stdlib/structural/math.md | 4c0fe0f10d5d673f5de36ce48ceef9871307b13d2d229dad0a458062f9cdc7fb |
| prompts/entities/module.md | 17d75e8c05e28546b7212ae232fa5d9e0474eda2b5bd7b67740e54ec3c1659a9 |
| prompts/compiler/eval.md | 80412483c8353a736f5c11842511cdef6c5e854728281a780c0183d6b73875cf |
| prompts/compiler/eval/modules.md | bba521089d35fcd8db850a60a790309105b73786c818f45ee9a71872366f1577 |
| 01_core/src/compiler/eval/bindings/field_access.rs | 88f3e17807b1fba83eec1dd70f9cfd0238f3c5d431682f286a8a485ba4ad2de1 |
| 01_core/src/compiler/eval/modules.rs | a855f51063310ec00a1106e6447f7a6a3e753ef8ccbb5e17f37a2319da1c3594 |
| 01_core/src/entities/module.rs | da6380bef91ebaa1166ba8f76cba953671e9bab210fbcd2a8b22e667802ab49f |
| 01_core/src/compiler/stdlib/structural/math.rs | cc901a05d573fa2cef0753f85ffcef4584682aa8dd9e0cd727d5094d7d22c03d |

Paths abreviados `prompts/` pertencem a `00_nucleo/`. São hashes de arquivo
bruto para a auditoria, não o hash efetivo de linhagem com pins.

## Medição de fonte

O registro `math-symbol-binding-warning` em
`p1322-classification-selection-r2.json:346-420` enumera `math.join`,
`math.join.l`, `math.join.l.r`, `math.join.r`, owner field_access e hipótese
de um guard com warning carrier existente. O próprio registro define como
refutação a ausência da categoria/origem nativa ou necessidade de outro owner.

`field_access.rs:475-499` obtém a mensagem da tabela de símbolos somente
quando `m.name() == "sym"`. `stdlib/sym.rs:497-513` consulta a depreciação
por nome; isso fornece mensagem, não origem do binding no Module consultado.
`stdlib/structural/math.rs:1322-1329` copia os valores e constrói
`Module::new("math", scope)`. `entities/scope.rs:32-46` transporta somente
Value no Binding, sem metadado de depreciação.

`entities/module.rs:63-73` inicializa content e introspection_content como
None. A distinção proposta entre builtin None e arquivo Some é refutada por
`eval/modules.rs:105-114`: o markup importado é avaliado e descartado; o
resultado é `Module::new(name.to_string(), module_scopes.exit())`. Logo um
arquivo ordinário `math.typ` também chega ao lookup com nome `math` e content
None, inclusive quando exporta um binding `join`. O L0 de imports determina
expressamente exportação isolada de bindings e preservação do nome próprio
do arquivo; não autoriza classificar origem pelo conteúdo exportado.

Também não há evidência de que uma comparação com Module recém-construído
recupere origem: `entities/module.rs:56-59` usa identidade Arc, enquanto
`eval/mod.rs:393-406`, `:626-662` e `eval/modules.rs:60-78` criam bibliotecas
locais próprias. O owner de field access não recebe uma identidade canônica
partilhada de todos esses módulos por simples consulta ao World.

No vanilla ratificado, `typst-eval/src/code.rs:347-366` passa field.span()
ao lookup com binding_guard; a emissão decorre do binding acessado. Os
contratos sym e math distinguem obrigação de valores/variants da dívida de
diagnósticos. Nada nesses contratos autoriza fabricar identidade por nome,
valor Unicode, lista de membros ou forma de arquivo.

## Decisão de revisão

`REOPEN_OWNER_SUFFICIENCY_IDENTITY_PENDING`. A hipótese concreta
nome+content-None está REFUTADA por um caminho produtivo existente. A
completude do owner único reivindicada em P1322 não pode ser transportada
como elegibilidade atual. Isso não é demonstração universal de que nenhuma
solução privada existe: é ausência de solução com origem preservada e
contraprova da solução estrutural proposta. Identidade não demonstrada
permanece Unknown obrigatório, nunca risco baixo.

O cohort fica pendente, sem patch nominal de `sym | math`. Reabrir exige
medir e especificar transporte causal que distinga o módulo nativo de
`math.typ`, preservando alias, sombra legítima, captura, import e reexport.
Se isso requerer novo contrato/carrier público ou outro owner produtivo,
reclassificar e observar ADR-0127 antes da implementação; este recibo não
autoriza tal ampliação. Não regredir silenciosamente `sym` para corrigir
`math` nem inferir warning da igualdade de Symbol.

É apropriado examinar o próximo cohort elegível na ordem vigente, com nova
medição e L0 próprios. Este recibo não aprova o próximo cohort nem lhe
empresta as conclusões de P1322. Não executei probes bilaterais aqui e não
alego paridade/fechamento de math, somente a refutação de fonte descrita.
