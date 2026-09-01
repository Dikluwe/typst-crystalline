# Prompt L0 — motor geral de layout
Hash do Código: b134c576

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4bf17f1455eef032ab3e30ea038edabed721e8378b913aaecf2b544bf288a917

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/mod.rs`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## Medição e contrato

O consumer define Layouter/PageConfig, entrypoints de layout, dispatcher
exaustivo e composição final de páginas. Domínios atomizados delegam por
funções estáticas descendentes. Métricas, introspecção e imagens são injetadas;
L1 não importa L3. Cursor, regiões, páginas, fixups e conteúdo diferido
preservam referencial e causalidade.

## Aceitação

Empty, texto, parágrafos, paginação, páginas auto, dispatch e composição final
são cobertos pela suíte de layout. Mudança pública/default/fase para no gate
ADR-0127.

## P1291 — passagem pura de requests math (PROPOSTO; GATE ADR-0127)

### Medição anterior à decisão

O caller de math em `01_core/src/compiler/layout/equation.rs:107-120` constrói
`MathLayouter` somente com métricas, modo block e estilo. Neste ponto o
`Layouter` possui região efetiva e `StyleChain`, mas não deve adquirir
capacidade de executar `Func`. Os precedentes P240/P241/P1159 mantêm o
Layouter puro e realizam callbacks na pipeline entre passagens.

### Decisão proposta

Adicionar uma entry point explícita de passagem que recebe
`&SealedMathCallbacks` e devolve `MathLayoutPassOutcome`. O `Layouter` cria um
estado de transcript em memória e o compartilha com todos os `MathLayouter`s
daquela passagem. Ao entrar numa equação, `equation.rs` fornece a `Location`
corrente da equação,
`regions.effective().height`, a `StyleChain` léxica e o mesmo estado; não
fornece função executável.

A entry point compatível devolve `SourceResult<PagedDocument>` e continua
válida para árvores sem requests. Se a passagem retornar `Pending`, descarta
as páginas provisórias e devolve `Err(vec![diagnostic])`, conforme
`SourceResult<T> = Result<T, Vec<SourceDiagnostic>>`, **sem valor e sem
documento**; não existe `PagedDocument` vazio/de-falha capaz de chegar a um
exporter. A entry point de passagem não marca a primeira medição como erro:
retorna `Pending(requests)` ao caller L3, que é obrigado a realizá-las e
repetir.

Nenhum estado de eval é guardado em `Content`, `Region`, `PagedDocument` ou
cache de métricas. Request e store pertencem à invocação e são retornados por
valor. `PagedDocument` só sai na variante `Complete`, depois de o transcript
estar integralmente resolvido, totalmente consumido e estável.

O escopo da passagem é a tentativa concreta criada por cada `Layouter::new`,
não a invocação externa inteira. O loop interno que pode refazer layout por TOC
cria `MathCallbackPassState` novo para cada tentativa, descarta junto com o
documento os transcripts das iterações rejeitadas e conserva somente o estado
da tentativa candidata. Se essa candidata possui pendências, devolve apenas
`Pending`; caso contrário pode devolver `Complete`. Esse ciclo de callbacks é
ortogonal a `compiler::introspect::run_fixpoint` e não é fundido a ele.

A assinatura nova e a ligação de fase exigem selo ADR-0127 antes de código.
Contrato detalhado: `compiler/math/layout/callbacks.md`; implementação da
capacidade: `infra/pipeline.md`.
