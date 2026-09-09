# P1334 — parecer de escopo anterior ao candidato

Revisor `/root/p1334_review`, regime A/B executado sem atestação de isolamento.
Li a skill `tekt-materializacao-segregada`, ambas as referências de papéis/gates,
instruções de repositório e L1, ADRs 0107/0108/0127/0129 e os quatro L0 indicados
integralmente. Não localizei ADR de segregação pela busca textual em `adr/`.
Entradas: fonte vigente, vanilla local ratificado, L0 e relatório P1333; escrita
limitada a `diagnosticos/p1334-review-*`. Não escrevi produto, L0 ou testes. Não há
candidato julgado, teste executado, selo de refinamento ou mutation score aqui.

## Fonte medida

- Vanilla ratificado upstream/main `a51e02804`: `foundations/calc.rs:73-94`
  declara `value: ToAbs`; `typst-macros/src/func.rs:463-473` escolhe `expect`
  para parâmetro obrigatório e `:390-391` gera validação final das sobras.
  `typst-library/src/foundations/args.rs:111-120,150-173` procura o primeiro
  posicional e faz cast antes de qualquer missing/finish. Sem posicional,
  procura a primeira ocorrência named `value`, mesmo depois de outro named:
  `the argument `value` is positional`, hint `try removing `value:`` e span
  do argumento completo. Sem esse named, `missing argument: value` no agregado.
- `foundations/args.rs:259-266` rejeita a primeira ocorrência que restou após
  consumir o primeiro posicional: `unexpected argument` ou
  `unexpected argument: NAME`, no span do argumento completo. Não existe
  prioridade universal de named sobre extra posicional. Um named `value`
  junto de um primeiro posicional válido é sobra comum, sem hint de missing.
- Cristalino `compiler/stdlib/calc.rs:139-214` já processa o primeiro valor
  antes dos guards. `:207-221` ainda produz named/aridade antigos. As falhas
  existentes de tipo/overflow/comprimento usam `value_span`, que deve continuar
  distinto do `span` das sobras e do agregado do missing.
- `compiler/eval/call_dispatch.rs:398,451` começa com agregado da lista AST.
  `:455-484` possui whitelist por ponteiro, recursão With e transporte
  incondicional apenas de `Args.span`. `:1634` já recebe `call.span()`.
  `:1058-1064` concatena pre/new e preserva `new.span`. Em vanilla,
  `typst-eval/src/call.rs:56,64,78` usa chamada inteira no agregado.
- `compiler/eval/math.rs:581-588,1319-1326` já usa o mesmo transporte nos
  caminhos genéricos. O resultado ainda passa pela exigência existente de
  content. A resolução especial de abs importado em math continua dívida
  anterior à nativa, conforme P1333; não pode virar sucesso por reclassificação.
- `stdlib/mod.rs:24` torna `calc` privado; `:77` reexporta apenas
  `make_calc_module`. `calc.rs:133` já declara `calc_abs` em `pub(crate)`.
- `entities/args.rs:53-80` oferece sequência causal e síntese None com
  posicionais antes de named e spans individuais detached. Seu L0 proíbe
  resolver inconsistência entre sequência e views escolhendo uma cópia.

## Parecer e objeções concretas

O escopo é viável em fluxo contínuo ADR-0127: completar diagnóstico de paridade
em calc, estender a whitelist existente por identidade de `calc_abs` e acrescentar
reexport interno explícito no hub. Isso exige L0-first nos owners `calc.md`,
`call_dispatch.md` e `_comum.md`. Não exige campo/API pública, mudança de assinatura,
novo modo ou fase. O hub deve conter somente a ligação `pub(crate)`, sem wrapper,
lookup ou validação. Args e math podem permanecer sem edição. Esta suficiência é
inferência refutável: perda de origem em rota obrigatória, necessidade de mudar
writers/entidades/math ou assinatura pública obriga reavaliar antes de expandir.

Não transplantar os seletores históricos P1293 que inferem âncora a partir de
aridade: a whitelist de transporte agregado já permite que calc seja o único
dono de ordem e diagnóstico. Nome `abs` do usuário ou de outra nativa não pode
ativar transporte; alias, import e With da identidade real devem ativá-lo mesmo
com nome divergente. Preservar encoders, panic e CSV já cobertos pelos testes
`call_dispatch.rs:1778-1858`, bem como preargs/ocorrências/ordem de avaliação.

Os testes históricos requerem migração explícita, não supressão: missing em
`calc.rs:1730,2664,3077`; sobras dimensionais em `:2230`; guard/trace público em
`:3533`; fronteiras de primeiro válido e ausência em `:3910-3970`. Em ausência
real com chamada, a nova âncora elimina naturalmente trace da própria chamada
pelo critério de contenção em `call_dispatch.rs:1031-1049`. Com named ou extra
originado em With/arguments de outra chamada/fonte, o erro continua naquela
origem e o trace externo permanece. Migrar mensagem sem migrar span/trace seria
erro de contrato.

Há fixtures históricas intencionalmente incoerentes (`calc.rs:3953-3969`, além
dos casos documentados em `:2206` e `:2619`). Não usá-las para obrigar uma nova
política de reader que legitime views stale. Os testes normativos novos devem
construir Some por `from_occurrences`, None por `from_parts`/`positional`, e
distinguir síntese sem origem de carrier inconsistente. Se mantidas, as fixtures
incoerentes precisam continuar explicitamente fora do domínio causal de paridade.

Antes de C, congelar ao menos: ausência com/sem named `value`; `value` posterior
a outro named e duplicado por With/spread; primeiro posicional válido antes e
depois de named; duas ordens das sobras; primeiro inválido com sobras; With
aninhado e Args preservando origem externa; None com span agregado conhecido;
spans do argumento e do valor diferentes; controles de identidade falsa e de
eager `panic`. Uma sobra inválida não deve sofrer cast. Não converter valores
math em números para satisfazer esse recorte. Unknown obrigatório não fecha.

## Proveniência desta inspeção

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
captura final `2026-09-09T15:50:44Z`. `git diff HEAD --stat` com exclusão das
pastas restritas registrou:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  187 +-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |   53 +-
 00_nucleo/prompts/compiler/eval/modules.md         |   47 +-
 00_nucleo/prompts/compiler/eval/tests.md           |   78 +-
 00_nucleo/prompts/compiler/stdlib/calc.md          |  461 +++-
 00_nucleo/prompts/compiler/stdlib/loading.md       |  198 +-
 00_nucleo/prompts/wiring.md                        |  104 +-
 01_core/src/compiler/eval/bindings/field_access.rs |  731 +++++-
 01_core/src/compiler/eval/call_dispatch.rs         |   92 +-
 01_core/src/compiler/eval/modules.rs               |    5 +-
 01_core/src/compiler/eval/tests.rs                 |  236 +-
 01_core/src/compiler/stdlib/calc.rs                | 2592 +++++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs             |  466 +++-
 04_wiring/src/main.rs                              |   50 +-
```

SHA-256 bruto das entradas L0 lidas, não hashes canônicos de linhagem:

```text
calc.md          059187f40ed91e0cf157dbed96f013d429dafe1075cb388b485c6acddf8e2b26
call_dispatch.md d11ca271808ccdda640fea58a3bfa0354816299a42c3736c72373c76859f5ebf
_comum.md        6401779014c0b3ad8a1e4d1d4e0b6fe990597150423fdf3b6b97ea3d258668e3
args.md          1a78d43dcc7c1a72c45848494b86281aa9b8a13d03abfaa8fb6baf141a5511aa
calc.rs          c6630647094dbaa1008e8516a1d8f4480ff1834d04a810a9fa8355dc788ddf06
call_dispatch.rs 9b11c388cf50666beecad9a6c92fbebfb323cdf5edbb550f4be04a96e0c3e9d4
stdlib/mod.rs    8246c2553ca325ab39f0425fc154ee856e7e231941cddf2638fa624261402e81
args.rs          f3e77a9bf2d038cbede40c8b5304dd00cb18f7c2f5b5cd9d53822a44ba7e0c4b
```

`/usr/local/bin/typst` confere o SHA ratificado P1333
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Não usei a string de versão como proveniência nem produzi números de paridade.
Incidente operacional: `git status --short` inicial exibiu incidentalmente nomes
de arquivos das pastas restritas. Nenhum conteúdo dessas pastas foi lido; o
incidente foi informado ao coordenador e as consultas posteriores as excluíram.
