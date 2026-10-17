# L0 — break selection
Hash do Código: 134cdb8c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:7e41c96527dd5936bebe6916f2a9ea93477c057495845c7e8b52ce3c5917855a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/break_selection.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Separation — 2026-09-28

### Evidence before decision

The current cursor measures and chooses word/chunk breaks at `cursor.rs:195–271`, emits highlighted text at `cursor.rs:149–193`, and transitions regions at `cursor.rs:314–417`. Metrics defines default width at `metrics.rs:249–262`; LineCommit requests post-line progress at `line_commit.rs:696–703`. The 63 controlled source-to-PDF observations and isolated pre-export measurements are archived in `00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28/`. They expose behavior limitations but this operation separates ownership while preserving those exact decisions. Initial code, L0 and provenance for this operation are archived in `00_nucleo/diagnosticos/separacao-texto-linhas-regioes-2026-09-28/`.

### Internal ownership decision

Own private Decision { Keep, Wrap, Hyphenate { prefix, width, rest } }, should_wrap and word selection. Receive injected metrics/style, word, its already measured width and numeric cursor/right-edge/wrap-threshold values; access no Layouter or mutable state. Use the exact overflow test cursor_x + width > right_edge AND cursor_x > the supplied line origin. The local-origin correction below supersedes the historical page-margin threshold. If no overflow, Keep. At overflow, use the existing lang/hyphenation points in reverse order, measure each prefix plus literal hyphen through text_measure::word, choose the first that fits and return an owned prefix/rest split by Unicode character indices. If no usable point, Wrap. Chunk selection uses only should_wrap and does not hyphenate. Do not flush, paginate, alter style, consume text, add optimized selection or honor previously discarded properties. No public strategy or trait is introduced.

## Acceptance

Preserve the existing observable text, positions, styles, pages and errors in the controlled corpus and current suites. Structural tests refute reintroduction of algorithms in adapters. Pure-selection tests discriminate no-wrap, wrap and Unicode-safe prefix decisions; measurement tests retain overrides and shaped/fallback paths. Each consumer is owned exclusively by this prompt. This is internal atomization, not completion of the audit findings.

## Política de hifenização e validação — 2026-09-28

### Medição anterior à decisão

`correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json` conserva 52 entradas bilaterais, fontes, comandos e caixas, medidas no estado `initial.json`. As combinações absent/auto/false/true eram equivalentes no Tekt. O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` rejeita hyphenate integer/string, justify integer/auto e linebreaks fora de simple/optimized/auto; os diagnósticos literais e spans estão nos stderr do lote. Fonte: eval/rules.rs:2972-3010/3407-3420 descarta opções; stdlib/structural/par.rs:119-126 aceita/ignora; layout/break_selection.rs:39 não consulta política; vanilla text/mod.rs:545-565 declara Smart<bool> e model/par.rs:227-242/627-636 declara justify e Linebreaks. Isso mede comportamento e tipos, não demonstra a intenção histórica dos prompts.

### Decisão e escopo vigente

Correção interna de paridade ADR-0127: assinaturas públicas, entidades e fases permanecem; os owners abaixo legitimam apenas sua unidade. A restrição da atomização de não honrar propriedades descartadas é supersedida somente por este lote. O núcleo paragraph-break-policy fixa o invariante compartilhado; não é owner de código.

O seletor privado word recebe também o boolean resolvido. Depois do predicado atual de overflow, false devolve Wrap sem consultar padrões ou medir prefixos; true mantém o percurso Unicode existente. Keep, limites, progressão na origem da linha e ausência de padrões ficam iguais. Chunk não hifeniza. Testes mantêm o controle Unicode habilitado explicitamente e comprovam que o mesmo problema resolve Wrap quando desabilitado.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Decision::Hyphenate privado ganha replacement String, prefixo lexical seguido de U+00AD, separado do prefix visual medido com hífen literal. Produzir ambos na escolha do mesmo ponto, sem heurística posterior.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

## Origem local da linha — correção medida

### Evidência anterior à decisão

`00_nucleo/diagnosticos/transformacoes-2026-10-14/margin-before/results.json.gz`
registra estado exato, comandos, binários e imagens bilaterais. plain-box e
first-overlong com margem de 70pt mantêm palavras fora da caixa no Tekt; no pin
as palavras seguintes quebram. As imagens foram inspecionadas. Em
`word_layout.rs:44,81` o limiar é page_config.margin.left, mas sub_frame.rs:196–217
usa origem local zero. break_selection.rs:25 permite overflow enquanto o cursor
não excede esse limiar. Isso explica o RED das regressões em
implementation/reflow-green-first.log, não é motivo para relaxar os testes.
A antiga cláusula de preservar a margem foi uma restrição de atomização; agora
está refutada para regiões locais. Geometria e quebras são observáveis da língua.

### Contrato substitutivo

O limiar de progresso da primeira palavra é a origem real da linha no mesmo
referencial do cursor e da borda direita. O predicado continua estrito:
cursor + largura > borda direita e cursor > origem. Assim uma primeira palavra
maior que a caixa progride sem linha vazia, mas não bloqueia quebras posteriores.
Nenhum novo algoritmo de hifenização, API pública ou fase do pipeline. Preservar
tracking e os controles de palavra longa, exact-fit, chunks e região ilimitada.

Receber o limiar numérico do chamador; não acessar Layouter nem inferir margem.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C016`: `01_core/src/compiler/layout/break_selection.rs:27`, `word`, 8 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C016` — `word`: Exceção pontual justificada

Seletor puro isolado do Layouter por decisão explícita do L0: recebe palavra/largura medida, métricas/estilo, cursor/borda/origem reais e política de hifenização. Inferir esses valores novamente recriaria o defeito de origem local.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C016: Seletor puro isolado do Layouter por decisão explícita do L0: recebe palavra/largura medida, métricas/estilo, cursor/borda/origem reais e política de hifenização")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Exceções nominais materializadas — 2026-10-16

### Medição anterior à decisão

O inventário validado em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json` conserva 79 avisos. A reconfirmação de fontes/spans, motivos vigentes, assinaturas e hashes deste lote está em `00_nucleo/diagnosticos/clippy-excecoes-2026-10-16/before.json.gz`, com HEAD, working tree não commitado, UTC e git diff HEAD --stat. As funções e os dados concretos foram lidos antes desta decisão. Aridade e heurísticas de tipos são mecânica Rust; não há nova prova de paridade ou benefício cognitivo medido.

### Responsabilidade e aceitação

Materializar somente as expectativas diretas nominais já justificadas neste owner: C016. Reconfirmadas contra os spans atuais e as cláusulas individuais. Usar exatamente #[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")] na função, nunca módulo/impl/crate; preservar assinatura, corpo, chamadas, cfg e atributos alheios. As entradas classificadas corrigir, inclusive C047/C066 condicionais, continuam pendentes e não recebem anotação neste lote.

Fluxo contínuo ADR-0127: este lote acrescenta apenas metadados de lint, não muda contratos públicos, corpo executável, cfg, defaults, fase, dependências ou arquitetura. A cláusula materializa somente as exceções nominais expressas e supersede proibições genéricas de supressão apenas para os itens/lints identificados; não autoriza silenciar as correções pendentes. L0 precede código, resselo restrito e prova de que remover os atributos acrescentados e o header de linhagem recupera a fonte anterior, normalizada pelo rustfmt. Medir RED antes; depois conferir inventário all-targets, exatamente 46 aridades e cinco avisos de testes satisfeitos por 49 expectativas, sem aviso novo e com unfulfilled_lint_expectations como erro. As 11 aridades destinadas a correção devem continuar visíveis. Validar build, formato, linhagens estrita/reversa, Tekt sem erros/avisos novos, testes existentes de Counter e cobertura sob 12 GiB, swap zero e um job. A suíte completa anterior permanece evidência do mesmo código executável; não criar testes que espelhem atributos de lint nem exigir nova comparação de performance para metadados.
