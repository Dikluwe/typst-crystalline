# Passo 1291 — membros realmente ausentes do namespace `math`

> Documento de execução; não é Prompt L0. Frente C, iniciável em paralelo com
> P1288 e P1290 em worktree separado. Esta frente não possui `repr.rs`.

## Objetivo

Materializar e verificar os oito membros classificados como ausentes na
amostra padrão:

```text
math.bb
math.cancel
math.frak
math.inline
math.scripts
math.serif
math.underline
math.vec
```

Presença e `repr(function)` são apenas a primeira obrigação. Cada membro deve
ter assinatura, conteúdo produzido, morfologia, erros e efeitos de layout
compatíveis com a linguagem vanilla. Igualdade de nome com uma função global
cristalina não autoriza alias automático.

## Estado congelado de partida

Medição anterior à decisão, árvore não commitada em
`2026-08-31T10:25:52-03:00`, `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`:

- vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- L0 `00_nucleo/prompts/compiler/stdlib/structural/math.md`: SHA-256
  `4441fe00c99c3475179c4a90060f5698e4472a368756ead6e4312781229617bf`;
- consumer `01_core/src/compiler/stdlib/structural/math.rs`: SHA-256
  `8e03f4e7e87cbeb91045842fda784310e2e112d1716fdc775f51812976330873`;
- inventário P1284: SHA-256
  `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb`;
- para cada um dos oito paths, o vanilla devolve `function` e nome curto; o
  cristalino devolve `module 'math' does not contain field ...`;
- `bb`, `frak`, `inline` e `serif` têm nativas cristalinas em
  `compiler/stdlib/math_style.rs`, SHA-256
  `a29f32e37b2ba4911ab2bcd99a16551d7ded8418fee71d4250513c0043e8b92a`;
- `cancel` já tem nativa em `structural/math.rs`;
- existe `underline` textual em `compiler/stdlib/text/deco.rs`, SHA-256
  `991752687b93b9d3ac93aa768033a029e3fd0b4be4c2179f66eab784359b40e4`,
  mas ainda não há prova de equivalência com `math.underline`;
- a sintaxe matemática já reconhece caminhos de `scripts(...)` e `vec(...)`
  em `compiler/eval/math.rs`, mas não há função pública correspondente
  registrada no módulo.

## Classificação e regime

Usar regime Tekt A/B por membro: contrato e testes independentes do patch,
implementação sem escrita nos testes protegidos e verificação somente leitora.
As oito linhas podem compartilhar a montagem de namespace, mas não podem
compartilhar uma suposição semântica não medida.

Correções de paridade e entradas em tabela seguem fluxo contínuo ADR-0127.
Se algum membro exigir nova variante/campo/método Rust público, mudança de fase
ou comportamento padrão, separar esse membro, atualizar L0 e parar no gate
humano. Os demais membros seguros podem continuar.

## Proibições de desenho

- não registrar todas as funções globais por reflexão;
- não copiar o scope global inteiro para `math`;
- não usar nome igual como prova de semântica igual;
- não criar wrappers duplicando construtores que já pertencem a outro owner;
- não editar `repr.rs` para esconder morfologia incorreta;
- não acrescentar variantes `Content` somente para satisfazer `type(...)`.

## Fase 1 — matriz de medição e ownership

Criar `00_nucleo/diagnosticos/p1291-manifest.json` e uma matriz com uma linha
por membro:

| Campo | Medir obrigatoriamente |
|---|---|
| kind/nome | `type`, `repr` e presença no namespace |
| chamada mínima | argumentos obrigatórios e resultado morfológico |
| chamada completa | named args, defaults e ordem pública |
| erros | ausência, excesso, tipo errado e named desconhecido |
| semântica | variante/conteúdo produzido e efeito observável |
| ownership | função existente reutilizável ou nova unidade dona |

Casos específicos:

- estilos `bb`, `frak`, `inline`, `serif`: medir nesting, override de estilo,
  `cramped` quando aplicável e diferença entre texto e math;
- `cancel`: medir body e opções públicas do vanilla, sem presumir que a nativa
  global atual tem contrato completo;
- `scripts`: medir função direta, interação com attachments e erros; comparar
  com o braço sintático já existente sem duplicá-lo;
- `vec`: medir zero/uma/múltiplas células, delimitadores, alinhamento e
  diagnósticos; comparar com o constructor sintático existente;
- `underline`: provar se é decoração textual ou elemento matemático. Se forem
  semanticamente distintos, é proibido registrar `text::native_underline`.

Medir primeiro na fonte e nos binários vanilla; registrar file:line e o que
refutaria cada inferência.

## Fase 2 — L0 antes de código

Atualizar `compiler/stdlib/structural/math.md` para enumerar os bindings e a
forma de montagem do namespace. Cada owner semântico adicional precisa do seu
Prompt L0 `1:1` vigente. O owner de `structural/math.rs` possui a tabela e
construções que já lhe pertencem; não absorve lógica de style, texto ou matrix
por conveniência.

Ressellar hashes antes dos testes/implementação. Se a auditoria revelar que
`scripts`, `vec` ou `underline` necessita contrato público novo, removê-lo do
lote contínuo e abrir subpasso com gate ADR-0127; registrar explicitamente a
incompletude no L0 inicial.

## Fase 3 — Testes A RED

O Testador A recebe passo, matriz medida, L0s ressellados e baseline, mas não o
patch. Congelar testes por membro antes da implementação e demonstrar RED.

Mutações mínimas que os testes devem rejeitar:

- binding presente mas não chamável;
- binding que aponta para a função global errada;
- nome público qualificado em vez do nome curto medido;
- argumentos/defaults aceitos e ignorados;
- `scripts` que não altera a política de attachments;
- `vec` que produz sequência comum em vez da morfologia vetorial;
- `underline` textual usado onde o vanilla produz conteúdo math;
- cópia do scope global que introduz extras não autorizados.

## Fase 4 — Implementação B por lotes seguros

O implementador recebe somente L0s e testes selados. Ordem recomendada:

1. registrar/reutilizar estilos já provados equivalentes;
2. registrar `cancel` depois de validar contrato completo;
3. fatorar construtores compartilhados para `scripts`/`vec`, se a medição
   provar que sintaxe e função têm a mesma unidade dona;
4. implementar `underline` apenas após a decisão semântica própria.

Cada lote deve ficar GREEN antes do seguinte. Um membro bloqueado não impede o
fecho parcial dos membros já certificados, mas o relatório final deve manter o
resíduo aberto explicitamente.

## Fase 5 — verificação e integração

Verificar hashes protegidos, testes focais, repetição bilateral,
`cargo build`, `cargo test --workspace` e `crystalline-lint .`. Confirmar que
os bindings já existentes do módulo `math`, inclusive espelho `sym → math`,
não mudaram de kind, valor ou precedência.

Depois que P1288 estabilizar o harness e P1290 integrar a representação
pública, reexecutar os oito probes e os casos funcionais. Diferença causada
somente por `repr` permanece dependência de P1290; P1291 não a corrige.

## Coordenação paralela

- worktree e target dir próprios;
- não editar harness/fixtures compartilhados com P1288;
- não editar `repr.rs` ou `compiler/eval/repr.md`, que pertencem a P1290;
- P1290 não edita `make_math_module`;
- integrar por lotes e revalidar o hash de `structural/math.rs` antes de cada
  cherry-pick/rebase; drift invalida o recibo daquele lote;
- comunicar somente por manifestos, L0s ressellados, testes selados e recibos.

## Critério de fecho

Cada membro fecha individualmente quando presença, chamada, morfologia,
diagnósticos e semântica observável coincidem nos casos selados, com testes A
RED→GREEN e gates arquiteturais verdes. O teto projetado é `+8 MATCH` na
amostra anterior; qualquer membro separado por gate reduz o ganho deste passo
e permanece declarado, nunca contado como sucesso.

## Subpasso `P1291.cancel-angle-runtime` — correção arquitetural no gate

**Estado:** materializado e certificado em 2026-08-31 após novo selo humano
ADR-0127. Não é um novo passo numerado. O certificado é
`00_nucleo/diagnosticos/p1291-callback-runtime-certificate.json`.

A primeira proposta de bridge síncrona por `dyn MathCallbackResolver` foi
rejeitada na revisão porque preservava a direção dos imports, mas reintroduzia
execução eval durante layout. Os L0s foram substituídos pela forma já
implementada no projeto: passagem de layout pura produz requests, a pipeline
L3 realiza as funções, e uma passagem seguinte consome store selada.

O contrato corrigido inclui:

- `MathLayoutPassOutcome::Pending(requests) | Complete(document)`, sem acesso
  ao documento provisório;
- identidade `Location` da equação + occurrence estrutural + linha `0|1`;
- duas requests para `cross=true`, como as duas chamadas do vanilla;
- `StyleChain` efetiva e span original transportados;
- store integral, stale rejeitada e teto de convergência com erro;
- `vec.gap` relativo à `Regions::effective().height`, inclusive política
  medida para região não-finita;
- Núcleo Tekt `math/callback-realization.toml` pinado pelos owners.

A revisão segregada final também congelou a fronteira operacional: cada
tentativa concreta de `Layouter::new` cria estado local novo; o occurrence de
cancel é reservado antes de descer no body; tentativas internas de TOC
rejeitadas descartam seus transcripts; e cada relayout externo de P1159 volta
a estabilizar callbacks math antes de page numbering. A entrypoint compatível
retorna `SourceResult<PagedDocument>` e `Pending` vira
`Err(vec![diagnostic])` sem valor ou documento, portanto nenhuma geometria
provisória pode chegar ao exporter.

Após o selo, testes RED e implementação continuam dentro deste subpasso sob o
regime segregado. A confirmação anterior fica invalidada porque os bytes do
contrato mudaram durante a revisão; nenhum hash de código deve ser reparado
antes da nova confirmação.

### Resultado do subpasso

O novo selo foi confirmado pelo humano e a implementação passou por autoria de
testes, implementação, ataque e verificação final separados logicamente. O
runtime de `cancel.angle` agora produz requests puras em L1, realiza `Func` em
L3, repete layout até estabilidade e impede que geometria provisória alcance
numbering ou export. `cross` preserva as duas chamadas com o mesmo default
positivo; o estilo efetivo de script é observável; tentativas rejeitadas de TOC
descartam seus transcripts.

As entrypoints compatíveis falham fechadas quando não podem realizar callback.
Em particular, `layout_with_font` mantém a assinatura legada
`PagedDocument`, mas entra pelo boundary callback-aware e não retorna documento
provisório. O teste protegido demonstrou RED com uma página provisória e GREEN
após a correção.

Gates finais do fragmento: P1291 core 22/22, black-box 5/5,
`layout_with_font` 1/1 e controles 4/4, V1/V2/V5/V15/V21/V26 verdes,
`cargo build --workspace`, formatação e `git diff --check` verdes. A suíte
workspace paralela passou núcleo e infra, mas o teste de watch P1137 expirou
duas vezes; isolado e a suíte CLI serial passaram. Isto fica registrado como
limitação, sem alegação de workspace paralelo integralmente verde.

O Passo 1291 completo permanece parcialmente aberto: os quatro V7 restantes
pertencem exclusivamente aos owners de `math.vec` e `math.underline`. Nenhum
crédito de fecho foi atribuído a esses dois membros.
