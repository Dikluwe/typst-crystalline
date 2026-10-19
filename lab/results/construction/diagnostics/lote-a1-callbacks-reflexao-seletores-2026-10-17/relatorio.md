# Lote A1 — callbacks, reflexão e seletores

Etapa entregue no recorte validado. **State por leitor, suplementos de Heading e admissão nominal de seletores corrigidos.** Há pendências identificadas abaixo; A1 não é uma declaração de paridade global. O gate global Tekt continua aberto pelos V16 anteriores. Sem commit, push, merge ou release nesta etapa.

## Proveniência

- HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch Tekt, **working tree não commitado**. Congelamento `2026-10-18T00:07:40.168158+00:00`; entrega `2026-10-18T00:20:57.713977+00:00`. A lista exata de fontes, arquivos novos, status e `git diff HEAD --stat` está em [final-state.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/final-state.json.gz), junto com o congelamento verificado. Fontes produtivas e L0 não mudaram durante a rodada final.
- Vanilla pin **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore `cac836c07e4dec2a89219370428292393a3216b5`, binário `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Não se usa versão textual como prova.
- Tekt anterior arquivado `5cdfaa01cec0b3035ca1069b8d013bf014bf6f93d119f52fc651d46da28ec0f8`; candidato `321de82f1d5485f077b4aa58558acc2b63d845d5847c53463e9ab28fd74b5d2b`. O oráculo e o anterior permaneceram intactos. Scripts, fontes de sondas, argv, UTC, saídas e logs estão nos artefatos desta pasta e no manifesto.
- Aprovações: A1-S2 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`; A1-H2 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`. Revisões preservadas em materialization. **A1-S1 foi retirada:** não se adicionou init a StateUpdateElem/ElementPayload. A contraprova do vanilla exige init no leitor.

## O que mudou

1. **State:** `state(key)` usa init none. `.update(func)` registra transformação, sem executá-la na construção ou walk. Cada objeto leitor conserva seu init, mesmo com a mesma chave; get/at/final resolvem as ações na ordem causal. Set/Func mistos, reuso de conteúdo, descarte e chaves independentes têm controles. Uma chave não lida não executa callbacks. A sequência demandada inteira pode propagar erro posterior, como no vanilla, embora o valor selecionado seja o do prefixo. Contexto do leitor não é herdado pela callback e é restaurado no sucesso/erro.
2. **Heading:** suplemento é realizado antes de query/context em PDF e HTML, com captura léxica e estilos causais. Erros propagam. A ocorrência conserva identidade Heading e Location; apenas sua pendência privada é consumida, sem repetir callback na mesma entrada. Defaults/auto/none, numeração e escopo local têm controles. Styled ordinário permanece opaco. Não se anteciparam callbacks de Counter, State ou equações.
3. **Reflexão:** campos de texto, amount/weak de H/V, dimensões de Box/Block e body de Block usam valores da linguagem e presença prescrita. O transporte específico de Heading preserva func, supplement/numbering explícitos e ordem dos campos. `assert.eq` passou a reutilizar a igualdade de linguagem existente, necessária para comprimentos relativos e valores numéricos aninhados; não redefiniu a igualdade Rust geral.
4. **Seletores:** show de text/bibliography, selector de funções nativas de elementos e counter de equation reconhecem identidade real do catálogo. Aliases são aceitos; closures, funções parciais e homônimos que não são elementos são rejeitados. A extensão de contadores textuais e os nomes canônicos Footnote/Equation foram preservados.
5. **HTML:** a projeção visual descarta apenas marcadores invisíveis State/StateUpdate/Metadata e preserva o body de Label. Não há avaliação de callbacks no encoder. Conteúdo ainda não suportado continua produzindo erro.

Os L0 proprietários foram editados antes do código. Novos helpers têm owners exclusivos; V15/V26 foram validados antes do resselo. [public-contracts.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/public-contracts.json.gz) registra as cinco assinaturas State migradas e a fachada Heading aprovadas. A nova representação do snapshot é privada.

Fontes principais para revisão: [State](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/state.rs), [resolvedor](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/state/resolve.rs), [suplementos de Heading](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/from_tags/headings.rs), [reflexão](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/bindings/field_access/content_methods.rs) e [projeção HTML](/repos/Antigravity/typst-crystalline/03_infra/src/export/html/document.rs).

## O que faz sentido manter no Tekt

- Despacho estático e separação entre índice semântico, render e composição dos targets; nenhuma necessidade de copiar o algoritmo ou o vetor de estados intermediários do vanilla.
- Payloads Set compartilhados por Arc entre histórico, ações e clones de snapshots. O resolvedor usa empréstimos quando possível e mantém somente acumulador/resultado selecionado. Styles pendentes de Heading ficam fora do Content canônico, no cabeçalho privado do snapshot.
- APIs legadas `state_update`, `state_update_with`, `state_final`, `state_at`, `state_display`, a extensão `.display` e o helper explícito apply_state_funcs conservam seus contratos declarados. Seus controles são **compatibilidade Tekt**, não crédito de paridade com o vanilla.
- Notas V19/V20 devem ser avaliadas pela coesão e exaustividade, não removidas em massa. Os novos grupos/fluxos têm prescrição no L0; não foram adicionadas supressões genéricas.

## Validação final

**Workspace: 8941 testes passaram, 0 falharam, 7 ignorados. Doctests: 5 passaram, 0 falharam, 0 ignorados.** Os ignorados não foram introduzidos por esta etapa. A suíte não foi enfraquecida para esconder erros: o teste histórico do fixpoint que esperava callback sem leitura foi migrado para demandar a chave e preservar a mesma asserção de propagação; o caso sem leitura tem controle separado.

113 observações bilaterais finais nos arquivos main/extra/state/targets, incluindo 6 de extensões Tekt. São controles delimitados, alguns repetem a mesma fonte; não representam percentual de cobertura da linguagem. A auditoria automática de status não encontrou resultado inesperado fora das 7 exceções nomeadas. Isso não fecha as diferenças de texto e diagnóstico registradas separadamente.

| Gate | Exit | Comando |
|---|---:|---|
| build | 0 | `cargo build --offline --locked --workspace` |
| probe | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/probe.py complete` |
| state-extended | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/state-extended.py` |
| extra-probe | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/extra-probe.py` |
| value-shape | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/value-shape.py after` |
| target-controls | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/target-controls.py` |
| runtime-memory | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/runtime-memory.py` |
| probe-audit | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/audit-probes.py` |
| workspace | 0 | `cargo test --offline --locked --workspace --lib --bins --tests --no-fail-fast -- --test-threads=1` |
| doctest | 0 | `cargo test --offline --locked --workspace --doc --no-fail-fast -- --test-threads=1` |
| fmt | 0 | `cargo fmt --all --check` |
| clippy | 0 | `cargo clippy --offline --locked --workspace --all-targets -- -D warnings -D unfulfilled_lint_expectations` |
| rustdoc | 0 | `cargo doc --offline --locked --workspace --no-deps` |
| lineage | 0 | `crystalline-lint . --checks v1,v5,v15,v26 --fail-on warning` |
| reverse | 0 | `python3 tools/ci/check_reverse_lineage.py` |
| architecture | 1 | `crystalline-lint . --format sarif` |
| lint-diff | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/lint-diff.py` |
| diff | 0 | `git diff --check` |

Tekt antes: `{'V16:error': 106, 'V19:note': 1099, 'V20:note': 873}`. Depois: `{'V16:error': 106, 'V19:note': 1106, 'V20:note': 874}`. O diferencial compara regra/nível/consumer/mensagem, desconsiderando deslocamentos de linha; **nenhum erro/warning novo**. V1/V5/V15/V26 estritos e linhagem reversa passaram. O exit global não foi relaxado.

Notas adicionadas ou modificadas no diferencial (incluem deslocamentos entre padrões, não apenas aumento líquido):

| Regra | Consumer | Observações adicionadas/modificadas |
|---|---|---:|
| V19 | [01_core/src/compiler/eval/bindings/field_access/content_methods.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/bindings/field_access/content_methods.rs) | 4 |
| V19 | [01_core/src/compiler/eval/bindings/field_access/fields.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/bindings/field_access/fields.rs) | 2 |
| V19 | [01_core/src/compiler/introspect/heading.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/introspect/heading.rs) | 1 |
| V19 | [01_core/src/compiler/stdlib/state.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/state.rs) | 1 |
| V19 | [03_infra/src/export/html/document.rs](/repos/Antigravity/typst-crystalline/03_infra/src/export/html/document.rs) | 1 |
| V20 | [01_core/src/compiler/stdlib/state.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/stdlib/state.rs) | 1 |

O braço residual de campos mantém enumeração nominal exaustiva. Sua mudança de alternativas foi registrada como nota modificada; não é um novo default genérico. Grupos de campos presentes e marcadores invisíveis têm escopo prescrito nos L0s. Mensagens e padrões completos estão em [lint-diff.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/lint-diff.json.gz).

### Rustdoc

A primeira execução combinada da etapa apresentou E0463 em doctests. O teste p1295 invoca Cargo/Rustdoc JSON recursivamente com RUSTC_BOOTSTRAP e target compartilhado. A hipótese de invalidação de artefatos é compatível com os registros; não foi provada por isolamento causal completo. A hipótese de TMPDIR foi refutada pelos controles normal/dev-shm. A receita validada executa testes comuns e, depois, doctests em outra invocação Cargo, sem excluir o teste p1295 ou os doctests. Não se declara correção geral do problema de concorrência/artefatos do Cargo.

## RAM

Teto por execução pesada **12 GiB**, MemoryHigh **8 GiB**, swap **0**, Cargo **1 job**, tarefas pesadas sequenciais. DEBUG=0 em dev/test e incremental desativado nesta validação. Maior pico registrado nesta etapa, incluindo tentativas RED/falhas e validações intermediárias: **4.079 GiB** (4380168192 bytes). Eventos high/max/OOM/OOM-kill: zero. [resources.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/resources.json.gz) conserva cada cgroup, comando, horário, HEAD/diffstat e pico.

`Value` permaneceu **144 bytes/alinhamento 16**; `IntrospectedContent`, **128/alinhamento 8**, antes/depois no host x86_64. Isso não significa ausência de custo no heap: o cabeçalho privado do mapa inclui os styles pendentes.

Máximo RSS de processos CLI, em MiB, uma amostra por caso/binário:

| Documento | Tekt anterior | Candidato | Vanilla |
|---|---:|---:|---:|
| minimal | 62.56 | 60.15 | 30.75 |
| plain | 64.77 | 63.35 | 30.64 |
| state-set-heavy | 62.05 | 58.78 | 29.83 |
| heading | 63.73 | 61.03 | 31.79 |

Os documentos acima terminaram com o mesmo texto extraído e sucesso nos três binários. Anterior debug com símbolos, candidato debug sem símbolos e vanilla release: perfis diferentes e amostra única impedem atribuir a diferença inteira ao patch ou declarar ganho geral. O caso state-set-heavy mede explicitamente o caminho com payloads Set repetidos (oito updates de arrays com 512 valores); os arquivos preservam fontes, hashes, argv e saída. Pico de cgroup de compilação e RSS de um documento são medidas distintas.

## Pendências e ordem sugerida

1. **Pilha no ensaio maior de State:** 64 updates com arrays de 2048 valores abortaram por stack overflow nos dois binários Tekt debug medidos; vanilla release passou. O caso menor da tabela não fecha esse limite. Evidência completa em pre-repair-runtime-memory.json.gz; release do Tekt não foi medido nesse ensaio. Não houve evento de OOM.
2. **Presença de Box.body:** o carrier atual não distingue corpo omitido de vazio explícito. box().fields() continua correto, mas box[A].body/fields ainda divergem. Exige proposta de transporte/contrato própria, sem inventar presença.
3. **Equation aninhada:** o caso math.equation(block:true)[$x$] duplica ocorrências no snapshot. Queries de equações diretas e contador paginado passam; isso não fecha normalização/morfologia de equações aninhadas.
4. **HTML:** Ref/âncoras e matemática continuam sem implementação suficiente. As sondas combinadas com esses nós falham mesmo com o suplemento/consulta corretos; os controles isolados de Heading e State passam. state_display legado continua pendente no HTML; sua compatibilidade PDF foi preservada.
5. **Mensagens e apresentação:** as rejeições abaixo conservam texto distinto do vanilla. Não são certificadas como igualdade de diagnóstico. O caso metadata entre espaços também conserva diferença de extração PDF já presente no anterior; HTML desse controle passou. O controle de counter-footnote conserva a apresentação anterior [1] em vez de 1; a chave do contador foi validada separadamente, sem certificar esse formato de nota.
6. **Demais lotes e arquitetura:** igualdade semântica de suplementos/Styled e demais A2/A3/A4 do inventário anterior, além dos V16 restantes, continuam abertos. Nenhuma dessas dívidas foi silenciada ou tratada como escolha Tekt correta sem nova medição.

| Controle | Target | Mensagem Tekt | Mensagem vanilla |
|---|---|---|---|
| main/fake-page-counter | pdf | error: counter() requer string, selector ou função de elemento, recebeu function | error: expected element |
| main/non-element-selector | pdf | error: o selector de show rule deve ser uma função nativa ou uma string literal. Closures não são suportadas. | error: only element functions can be used as selectors |
| extra/empty-text-field-negative | pdf | error: empty does not have field "text" | error: sequence does not have field "text" |
| extra/counter-equation-homonym-negative | pdf | error: counter() requer string, selector ou função de elemento, recebeu function | error: expected element |
| state/context-not-inherited | pdf | error: here() chamado fora de contexto locatable — current_location não populado (P208B: infra minimal; captura automática no walk é deferred) | error: can only be used when context is known |
| state/context-not-inherited | html | error: here() chamado fora de contexto locatable — current_location não populado (P208B: infra minimal; captura automática no walk é deferred) | error: can only be used when context is known |
| state/update-extra-named | pdf | error: state.update() requer 1 argumento | error: unexpected argument: extra |
| state/update-extra-named | html | error: state.update() requer 1 argumento | error: unexpected argument: extra |
| targets/heading-context-absent | pdf | error: here() chamado fora de contexto locatable — current_location não populado (P208B: infra minimal; captura automática no walk é deferred) | error: can only be used when context is known |
| targets/heading-context-absent | html | error: here() chamado fora de contexto locatable — current_location não populado (P208B: infra minimal; captura automática no walk é deferred) | error: can only be used when context is known |

[probe-audit.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/probe-audit.json.gz) enumera cada controle, exceção, diferença de mensagem/texto e status anterior. [lint-diff.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/lint-diff.json.gz), [evidence.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/evidence.json.gz), [final-state.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/final-state.json.gz) e [manifest.sha256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/lote-a1-callbacks-reflexao-seletores-2026-10-17/manifest.sha256) permitem conferir o fechamento. O objetivo amplo do projeto permanece aberto.
