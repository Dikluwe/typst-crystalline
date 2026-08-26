# P1182 — auditoria da bijeção L0

**Medição:** 2026-08-25T21:23:21-03:00  
**Produto HEAD:** `00f402e875956304aa435f749a251f359287e2ba`  
**Estado:** working tree P1181 não commitada; índice vazio  
**Decisão:** corpus classificado; materialização bloqueada até aprovação humana

## Proveniência

Executável usado:

```text
/home/dikluwe/.cargo/bin/crystalline-lint
SHA-256 eb7494979040e70feb6ac3b738c86979488b8c26927480126746aae2ff707c9d
```

O binário foi instalado anteriormente a partir do Tekt Linter em
`b2a2826e540a556081476918f98cb85c5dfe21be`. Durante esta auditoria, o checkout
da fonte avançou para `2874bdad75a0e791b212fa762c99679aff2a0038` e continha somente
`00_nucleo/assessments/0039-mutation-pilot.md` não rastreado. O executável não
mudou: a autoridade dos números é seu SHA-256, não o HEAD posterior do checkout.

Duas execuções de:

```text
crystalline-lint --checks v15,v26 --fail-on warning .
```

terminaram com exit 1, stdout byte-idêntico de 72 linhas, stderr vazio e
SHA-256
`f1feeb0620caa94210e3e6b3aade45f429d5d04729d5570dacde28e98134b70a`.
Resultado: 24 V15, zero V26.

O lint integral anterior, no mesmo produto e executável, terminou com exit 1 e
contou V5=421, V7=2, V15=24, V16=210, V17=36, V18=2, V19=349, V20=600 e
V21=24. V16–V21 são lentes independentes. V5 permanece bloqueada por V15.

## Fechamento mecânico

O [manifesto TSV](typst-p1182-inventario-bijecao-l0.tsv), SHA-256
`589168e74110968260fb3a8e27bce91ad7dd94ffcfffa9706f6f1ff88c49f2c4`,
contém:

- 114 relações consumer→prompt;
- 114 consumers únicos;
- 24 prompts compartilhados;
- zero paths ausentes;
- zero relações duplicadas;
- 114 owners proprietários propostos e distintos;
- 22 consumers preservam o path do prompt atual como owner;
- 92 consumers exigem novo Prompt L0 proprietário;
- 20 Núcleos candidatos dentro dos grupos V15; nenhum foi criado.

Sete dos 24 prompts não têm `Hash do Código` canônico no preâmbulo, afetando
64 relações: `atomizacao_elementos.md`, `eval.md`, `layout.md`,
`stdlib_audit_methodology.md`, `infra/export/builder.md`,
`infra/package_downloader.md` e `infra/shaper.md`. Os outros 17 têm exatamente
uma ocorrência canônica. Metadata não foi reparada porque ownership precede
resselo.

## Classificação dos 24 grupos

| Prompt atual | N | Classe | Decisão posterior à medição |
|---|---:|:---:|---|
| `compiler/layout_references.md` | 2 | A | `references.rs` conserva o owner; `label_kind.rs` ganha owner próprio; auditar Núcleo de semântica label/ref. |
| `compiler/math/layout/_comum.md` | 2 | A | `mod.rs` conserva o owner; testes ganham owner; auditar Núcleo de observáveis math. |
| `compiler/stdlib/context.md` | 2 | A | stdlib e entidade ganham ownership separado; auditar Núcleo de `ContextBlock`. |
| `compiler/stdlib/foundations.md` | 2 | A | hub conserva o owner; `eval/repr.rs` ganha `compiler/eval/repr.md`; nenhum Núcleo necessário. |
| `compiler/stdlib/state.md` | 2 | A | stdlib e entidade separados; auditar Núcleo da semântica de `state`. |
| `compiler/stdlib_audit_methodology.md` | 2 | D | metodologia não legitima `Title`; criar owners de entidade/layout e mover metodologia para diagnóstico/processo. |
| `entities/page_canvas.md` | 2 | A | entidade conserva owner; composição de layout ganha owner; auditar Núcleo de canvas. |
| `entities/page_running.md` | 2 | A | entidade conserva owner; composição de layout ganha owner; auditar Núcleo de running matter. |
| `infra.md` | 2 | A | `lib.rs` conserva owner; testes de integração ganham owner; nenhum Núcleo necessário. |
| `infra/export/builder.md` | 2 | A | builder conserva owner; bitmap glyphs ganha owner; auditar Núcleo de embedding bitmap. |
| `infra/package_downloader.md` | 2 | A | adapter conserva owner; porta L1 ganha owner; auditar Núcleo do contrato da porta. |
| `infra/shaper.md` | 2 | A | shaper conserva owner; catálogo de fallback ganha owner; auditar Núcleo de seleção de fallback. |
| `shell/cli.md` | 2 | A | CLI conserva owner; build script ganha owner; auditar Núcleo de identidade da versão. |
| `shell/info.md` | 2 | A | formatter conserva owner; snapshot L3 ganha owner; auditar Núcleo de projeção. |
| `testing/math_oracle.md` | 2 | A | oráculo conserva owner; hub de testes ganha owner; nenhum Núcleo necessário. |
| `wiring.md` | 3 | A | main conserva owner; duas suítes ganham owners; só main+CLI tests candidatam Núcleo de observáveis. |
| `compiler/lexer/mod.md` | 4 | A | hub conserva owner; cada modo ganha owner; auditar Núcleo de fronteiras do lexer. |
| `compiler/stdlib/primitives-constructors.md` | 4 | A | hub conserva owner; três constructors ganham owners; auditar Núcleo de chamadas. |
| `compiler/lang.md` | 5 | A | hub conserva owner; quatro geradores ganham owners; auditar Núcleo de defaults linguísticos. |
| `entities/f_fronteira_e1.md` | 5 | B | `dynamic.rs` é owner principal; registry e elementos ganham owners; extrair fronteira compartilhada candidata. |
| `compiler/parse.md` | 7 | A | facade conserva owner; seis subunidades ganham owners; auditar Núcleo de modos/parser. |
| `compiler/eval.md` | 10 | A | dispatcher conserva owner; nove unidades ganham owners; `foundations/path.rs` sai do domínio eval; auditar contexto comum. |
| `compiler/layout.md` | 17 | A | `layout/mod.rs` conserva owner; 16 unidades ganham owners; Núcleo de coordenadas só para 12 consumers aplicáveis. |
| `compiler/atomizacao_elementos.md` | 29 | A | documento não conserva owner; distribuir contratos específicos e auditar Núcleo da forma B para os 29 owners. |

Totais por grupo: A=22, B=1, D=1, E=0. Totais por consumer: A=107, B=5,
D=2. Não houve bloqueio E porque headers, responsabilidades observáveis e
seções do L0 permitiram separar os owners; isso não aprova automaticamente o
texto futuro de cada prompt.

## Evidência semântica principal

A classificação não foi inferida apenas dos nomes:

- `eval.md:111-200` define scope/entrypoint global, enquanto `:334-443`
  especifica transporte e consumo de `FlowEvent`; os headers distinguem
  bibliography (`bibliography.rs:7-13`), controle (`control_flow.rs:7-10`),
  markup (`markup.rs:7-10`), math (`math.rs:7-8`) e path
  (`foundations/path.rs:15-37`).
- `layout.md:69-110` é o motor/facade, mas `:830-959`, `:1230-1317` e
  `:1618-1710` especificam unidades diferentes; seus arquivos declaram
  responsabilidades próprias desde o header.
- `atomizacao_elementos.md:58-171` contém a regra transversal da forma B,
  mas `:173-524` acumula fatias e extensões específicas de elementos. Logo não
  é Núcleo puro nem owner único.
- `f_fronteira_e1.md:65-227` torna `DynElement` a fundação central, enquanto
  `:955-981` separa registry e hub e `:744-954` trata `strong`/`emph`; por isso
  classe B, não documento transversal inteiro.
- `stdlib_audit_methodology.md:12-55` descreve processo de varredura, ao passo
  que `layout/title.rs:7-18` e `entities/elements/title.rs:7-19` implementam
  layout e entidade. A associação é documentalmente incorreta, classe D.
- `package_downloader.md:16-57` mistura objetivo da porta com decisões L3 em
  `:59-118`; os consumers confirmam trait L1 (`contracts/package_downloader.rs:21`)
  e adapter HTTP L3 (`infra/package_downloader.rs:7-16`).
- `shell/cli.md:40-105` especifica stamping de versão do build e `:144-632`
  especifica runtime CLI; `build.rs:7-15` e `cli.rs:7-18` são owners distintos.
- `wiring.md:27-80` legitima o entrypoint, mas `tests/cli.rs:7-15` é suíte E2E e
  `tests/crystalline_lint.rs:7-13` testa outra ferramenta; compartilhar o
  mesmo owner não preserva causalidade.

O manifesto registra para cada uma das 114 linhas o hash efetivo do prompt,
hash do consumer sem sua própria linha `@prompt-hash`, metadata canônica,
responsabilidade extraída do header e evidência `file:line`.

## Núcleos candidatos dos grupos V15

Todos permanecem propostas. Modalidade inicial das claims: `must`;
dependências: nenhuma; risco de ciclo inicial: nenhum. O refutador comum é a
leitura do futuro par de prompts demonstrar que a claim pertence a apenas um
owner ou não é verificável independentemente dele.

| Núcleo proposto | Prompts | Claims resumidas e refutador específico |
|---|---:|---|
| `layout-atomization-form-b.toml` | 29 | match exaustivo magro delega estaticamente à free function da feature; refutado por módulo que não seja delegação atomizada. |
| `layout-coordinate-composition.toml` | 12 | coordenadas locais, rebase e composição de regiões; refutado por unidade sem transformação espacial. |
| `eval-engine-context.toml` | 9 | transporte de engine/scopes/span/flow; refutado por unidade pura sem contexto de eval. |
| `parser-mode-contract.toml` | 7 | modos, progresso, spans e morfologia da árvore; refutado por helper sem consumo do parser. |
| `dynamic-element-extension-boundary.toml` | 5 | object safety, identidade e lifecycle E1; refutado por elemento fechado sem caminho dinâmico. |
| `language-text-defaults.toml` | 5 | fallback linguístico determinístico em texto gerado; refutado por unidade que não gera texto. |
| `lexer-mode-contract.toml` | 4 | progresso e fronteiras comuns dos modos; refutado por helper lexical independente de modo. |
| `primitive-constructor-call-contract.toml` | 4 | disciplina comum de argumentos/erros dos constructors; refutado por diferenças que eliminem claim comum. |
| `cli-build-version-identity.toml` | 2 | commit estampado e apresentado pelo CLI são a mesma identidade; refutado se build e runtime tiverem autoridades separadas. |
| `cli-wiring-observables.toml` | 2 | exit/output do entrypoint são verificados pela suíte CLI; não inclui testes do linter. |
| `context-block-semantics.toml` | 2 | entidade transporta expressão delayed consumida pelo constructor/context; refutado se transporte não for obrigação de ambos. |
| `font-fallback-selection.toml` | 2 | lista e shaper concordam em classe, cobertura e math; refutado se lista for mero dado privado do shaper. |
| `math-layout-observables.toml` | 2 | implementação e testes concordam em observáveis geométricos, não mecânica Rust; refutado se o teste puder possuir spec independente completa. |
| `package-downloader-port-contract.toml` | 2 | adapter satisfaz porta/erros/integridade L1; refutado se o trait não prometer esses observáveis. |
| `page-canvas-semantics.toml` | 2 | entidade e compositor concordam em geometria/ordem de layers; refutado por campo sem observável de composição. |
| `page-running-matter.toml` | 2 | transporte e layout concordam em posição/morfologia; refutado por detalhe exclusivo de render. |
| `pdf-bitmap-glyph-embedding.toml` | 2 | coletor e builder concordam em dedup/XObject/IDs; refutado se a interface entre ambos não expuser a obrigação. |
| `reference-label-semantics.toml` | 2 | classificação de label determina diagnóstico/layout de ref; refutado por variante sem consumo em references. |
| `runtime-info-projection.toml` | 2 | snapshot L3 e apresentação L2 concordam no schema projetado; refutado por campo somente interno. |
| `state-language-semantics.toml` | 2 | entidade e stdlib concordam em key/init/observação contextual; refutado por método exclusivo da stdlib. |

Os prompts consumidores exatos de cada candidato estão na coluna
`proposed_nucleus` do TSV. Nenhum candidato tem menos de dois consumers.

## Reconciliação dos dois V7

### `_convencoes.md` — classe C documental

`_convencoes.md:9-22` define obrigação transversal de caminhos completos e
`:26-39` define referências de lineage. Não possui consumer produtivo porque
não é Prompt L0 materializável. Destino proposto:

```text
00_nucleo/prompts/_nuclei/prompt-reference-identity.toml
```

Claims `must`: referências inter-prompt usam path lógico completo; `@prompt`
usa path completo da raiz; nomes folha não constituem identidade. Consumers
iniciais comprovados: futuros owners de `compiler/layout.md` e
`compiler/eval.md`, que contêm referências completas em `layout.md:225-248` e
`eval.md:720-816`. A materialização deve inventariar todos os prompts com
referências e não limitar o Núcleo a esses dois. Refutador: a regra permanecer
governança exclusiva de agente sem obrigação verificável nos prompts; nesse
caso deve ir para `AGENTS.md`, não para Núcleo.

### `shell/custom-ca-cert.md` — classe C documental

O documento é transversal: declara alvos L2/L3/L4 em `:4-8` e o contrato
público/TLS em `:21-49`. A feature está observável em `cli.rs:439`,
`world.rs:278-282` e `wiring/main.rs:127-176`. Destino proposto:

```text
00_nucleo/prompts/_nuclei/custom-ca-certificate.toml
```

Prompts consumidores propostos: `shell/cli.md`, `infra/package_downloader.md`,
`infra/system-world.md` e `wiring.md`. Claims `must`: precedência flag>env;
roots customizadas são adicionadas sem desativar validação; leitura ocorre ao
construir downloader; segredo não aparece em diagnóstico; wiring injeta a
opção. Refutador: alguma obrigação pertencer exclusivamente a um dos quatro
owners; essa claim fica no prompt específico, não no Núcleo.

Assim, ambos os V7 são documentos em categoria errada, não prompts órfãos a
materializar diretamente. Nenhum foi movido nesta auditoria.

## Comparação com P1179

P1179 mediu 22 prompts/107 consumers em HEAD
`ce49041de76eb64c011e990e9774a0339f979211`, usando o reparador anterior. P1182
mede 24/114 em HEAD `00f402e8` com V15 global corrigida. Entraram ou passaram a
ser detectados `compiler/stdlib/foundations.md`, `compiler/layout_references.md`
e mudanças de cardinalidade em `atomizacao_elementos`, `layout` e `wiring`.
Os conjuntos pertencem a estados diferentes; P1182 substitui P1179 como
inventário operativo, preservando P1179 como prova histórica da regressão.

## Ordem proposta dos lotes futuros

1. **D-title:** retirar `Title` da metodologia; dois owners, nenhum Núcleo.
2. **Hubs simples:** `infra.md`, `testing/math_oracle.md`, foundations/repr.
3. **Pares entidade/compilador:** page canvas, page running, context, state,
   references/label.
4. **Fronteiras de camada:** package downloader, info/runtime, CLI/build.
5. **Infra focal:** bitmap glyphs/builder e fallback/shaper.
6. **Wiring e suítes:** main, CLI tests e crystalline-lint tests.
7. **Famílias pequenas:** lexer, constructors e lang.
8. **Parse.**
9. **Eval.**
10. **Layout.**
11. **Atomização**, por último.
12. **V7 transversais**, depois de existirem prompts owners estáveis que possam
    consumir os Núcleos sem pins órfãos.

Cada lote exige L0 primeiro, claims do Núcleo auditadas, gate humano quando
aplicável, headers depois, reparo manifestado e V5/V7/V15/V26 focal. Nenhum
lote pode absorver mudança funcional descoberta durante a divisão.

## Decisão final

O corpus é saneável sem mudança funcional, mas não mecanicamente. Aprovar este
diagnóstico legitima apenas escrever o passo do lote D-title. Não legitima os
20+2 Núcleos candidatos nem a divisão dos demais grupos; cada lote deve reduzir
e auditar suas claims antes da materialização.

