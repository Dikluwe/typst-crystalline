# P1328 — revisão preliminar do escopo calc-abs-content-diagnostic

Revisor: `/root/p1328_review`. Auditoria somente leitura de produto/L0/oráculos;
escrita limitada a `p1328-review-*`. Regime recebido: A/B, executado sem
atestação técnica de isolamento. A leitura do baseline não torna este revisor
um autor independente de oráculos. Nenhum candidato C foi lido ou escrito.

## Proveniência e entradas

Inspeção em `2026-09-09T11:35:37Z`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
`git diff HEAD --stat` observado antes de editar este diagnóstico:

```text
 .../prompts/compiler/eval/bindings/field_access.md | 187 +++++-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |  53 +-
 00_nucleo/prompts/compiler/eval/modules.md         |  47 +-
 00_nucleo/prompts/compiler/eval/tests.md           |  78 ++-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 198 +++++-
 00_nucleo/prompts/wiring.md                        | 104 ++-
 01_core/src/compiler/eval/bindings/field_access.rs | 731 ++++++++++++++++++++-
 01_core/src/compiler/eval/call_dispatch.rs         |  92 ++-
 01_core/src/compiler/eval/modules.rs               |   5 +-
 01_core/src/compiler/eval/tests.rs                 | 236 ++++++-
 01_core/src/compiler/stdlib/loading.rs             | 466 +++++++++++--
 04_wiring/src/main.rs                              |  50 +-
 12 files changed, 2145 insertions(+), 102 deletions(-)
```

O primeiro path abreviado é
`00_nucleo/prompts/compiler/eval/bindings/field_access.md`.
Hashes SHA-256 obtidos com `sha256sum`:

- L0 `00_nucleo/prompts/compiler/stdlib/calc.md`:
  `0fc04b14b3cca19a203823ef392b56509d7ea2ff12a43550b2654fa2bb120f81`.
- Produto `01_core/src/compiler/stdlib/calc.rs`:
  `091b5db1fa5e00b0ee05f09578c909fd75642b3fb4a255e56b0c75f9c6715580`.
- Referência `lab/typst-original/crates/typst-library/src/foundations/calc.rs`:
  `0eb0836dc8bf17ba214d04be2c14f137350766ec44b6f5afe238cf467f78f484`.

Skill Tekt e ambas referências lidas; não foi localizada ADR de materialização
segregada na pesquisa de ADRs por `segrega`. ADR-0127 lida. L0 calc e `_comum.md`
lidos integralmente. Args e seus invariantes de origem foram consultados.
P1327 closure declara manifesto
`9ebf13c2cf9a1c1daf186671194c7c4e15db16d8d4e47a12ad5ff4fb597f8ad0`
e mantém limites de A/B sem atestação; esse fechamento não é prova desta coorte.

## Medição de fonte anterior à decisão

- `calc.rs:133–149`: `calc_abs` rejeita named antes de discriminar a lista;
  os braços Int/Float/Decimal são independentes do fallback `[other]`.
  O fallback em `:146` produz `calc.abs() requer Int ou Float, recebeu content`
  para Content. O helper `stdlib/mod.rs:189–191` sempre cria span detached.
- Vanilla pinado `a51e02804`, fonte `foundations/calc.rs:74–93`: `abs`
  recebe `ToAbs`, cuja união de cast contém integer, float, length, angle,
  ratio, fraction e decimal. `foundations/args.rs:111–120` ancora falha de
  cast no span do valor consumido. A mensagem de Content guardada nas
  testemunhas históricas P1322 é
  `expected integer, float, length, angle, ratio, fraction, or decimal, found content`.
- `eval/math.rs:329–348`: argumentos posicionais recebem `ArgOccurrence`
  com `value_span: expr.span()`. A rota qualificada usa esse construtor
  em `:583–590`; o fallback de função bare usa o mesmo em `:1320–1327`.
  Não falta transporte para as testemunhas `$std.calc.abs(-1)$` e
  `{ let enc = calc.abs; $enc(-1)$ }`.
- `entities/args.rs:15–33,66–80`: a origem de cada valor está em
  `occurrences`; síntese não possui origem individual. O L0 Args explicita
  que `args.span` não reconstrói uma origem perdida, nem substitui uma
  ocorrência cujo `value_span` é detached.
- O L0 calc vigente explicita o sucesso Int/Float/Decimal e saturating_abs,
  mas não contém o diagnóstico/spans de Content. Precisa amendment antes
  de código. `_comum.md` possui somente o hub/helpers; não deve receber a
  decisão específica de abs.

## Decisão de escopo

O owner `compiler/stdlib/calc.md` → `compiler/stdlib/calc.rs` basta para o
recorte: Content como único argumento posicional, sem named, rejeitado com
mensagem vanilla exata e origem da primeira ocorrência positional. Na
ausência de origem, ou se essa origem for explicitamente detached, o erro
fica detached. Não usar span agregado, origem interna de Content, comparação
de valores ou busca textual para fabricar âncora.

É correção de paridade ADR-0127 em fluxo contínuo, condicionada a L0 primeiro,
resselo, RED→GREEN e revalidação. As duas rotas já convergem na nativa; mover
math para code/eval/layout mudaria a causa e refutaria este escopo.

Int mínimo/overflow (`saturating_abs` versus `checked_abs`) e suporte ausente
a Length/Angle/Ratio/Fraction têm diferença de sucesso numérico, não são
necessários à correção do Content. O texto vanilla lista esses tipos porque
esse é o diagnóstico observado; não equivale a afirmar que estão implementados.
Erro de outros tipos, aridade, named, valores válidos e demais funções calc
ficam como controles de preservação, sem ampliação silenciosa desta coorte.

## Testes legados e suficiência pendente

Pesquisa em código Rust L1–L4 pelo diagnóstico antigo e por `abs.*content`
não encontrou assert legado de erro Content. Os testes diretos existentes
`stdlib/mod.rs:1893` e `:1914` cobrem Int/Float; os pipelines em
`eval/tests.rs:3924` e `:4074` cobrem Int/Decimal. Não há motivo demonstrado
para editar esses owners de testes. A pesquisa adicional em alguns paths
`tests/` inexistentes devolveu erro; não se alega cobertura desses diretórios.

O pacote A/B deve congelar antes de C: testemunhas math qualificada e bare,
Content em chamada de código, seleção do value-span distinguível do arg-span
e span agregado, FileId preservado, origem detached/sintética e controles
de sucesso/named/aridade/outros tipos. With/spread com origem já preservada
são controles úteis, sem prometer reparo de transporte se uma rota falhar.
As medições bilaterais atuais e o RED ainda são responsabilidade das fases
em curso; este documento não fecha o diagnóstico a partir de fonte ou P1322.

Refutadores: necessidade de outro consumer produtivo; diagnóstico de Content
dependente de mudança de avaliação; baseline fresco sem a divergência;
necessidade causal de ampliar tipos/overflow; origem não disponível no
carrier da rota selecionada. Qualquer um reabre a decisão antes de C.
