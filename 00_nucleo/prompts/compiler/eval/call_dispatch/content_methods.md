# L0 — compiler/eval/call_dispatch/content_methods
Hash do Código: da1c69dc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/content_methods.rs`. Comportamento original em `01_core/src/compiler/eval/call_dispatch/call.rs:323-421` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own Location, Content/LocatedContent and OutlineEntry method interception in the existing order. Preserve method gates, receiver and argument evaluation, query morphology, spans and fallthrough before field-callee errors. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.


## OutlineEntry com carriers de origem/identidade — 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, métodos/fontes/UTC/hashes/diffstat em before.json.gz e methods-before.json.gz no diagnóstico tekt-v16-introspect-walk-2026-10-16. A primeira correção pura passou nove controles e seis barreiras, mas a sonda CLI direct.typ produziu element entry has no method body, enquanto o vanilla pinado 9f2b6e871 aceita. attempt-method-carriers-measurement-after.json conserva argv, binários, fontes e saídas: consulta direta ainda 0 contra 1 e mista 1 contra 2; não foram declaradas resolvidas. call_dispatch/content_methods.rs:68 exige variante OutlineEntry bruta. rules/apply.rs:504/564 fornece Value::Content do Located criado para uma ocorrência nativa; Source também é transparente e não pode impedir o método. Não se infere paridade global nem intenção upstream dos mecanismos de armazenamento.

### Contrato exclusivo

Adicionar helper privado emprestado outline_entry_receiver: normalizar Source pelo source_body existente, recursar em Located, devolver Some da referência OutlineEntry nua e None para o complemento nominal das 118 variantes atuais. Sem clone, alocação, avaliação ou uso de Location para escolher semântica. Realized/ShowRecipe/Styled/Strong não são novos unwraps por este helper. No interceptor, após o match Value atual, usar esse helper para reconhecer o receiver e passar a entry nua aos mesmos métodos. Preservar gates de nomes, ordem e número de avaliações de target/args, spans, dispatch e fallthrough de outros tipos; somente o conteúdo da identificação do receiver muda. Não criar API/campo/trait/default/fase ou mudar assinaturas; correção interna/paridade ADR-0127.

O complemento nominal tem consequência None uniforme e mantém V19 informativo, sem supressão; não prova execução de todas as alternativas. Uma nova variante de Content exige rever esse match, com sonda sentinela real. A responsabilidade de body/prefix/fill e Location é dos owners existentes, sem nova tabela de métodos.

### Aceitação

Testes co-localizados antes do código: Entry crua, Source/Located em ambas as ordens e aliases resolvem a mesma referência; outros tipos e Realized não são convertidos. Os controles e as seis sondas CLI originais permanecem, inclusive consultas direta/sintetizada/mista e body, sem relaxar os esperados. Sonda Content deve falhar também neste novo helper. Build/fmt/Clippy estrito/Rustdoc estrito/workspace/linhagem V1/V5/V15/V26/reversa/diferencial e 12 GiB, swap zero, um job; resselo só do escopo ampliado para seis pares. V16 esperado 239→233; V19 900→907 (seis grupos anteriores e este novo), três mensagens de grupos já existentes ajustadas; V20 sem aumento. O objetivo geral e as sondas CLI seguem abertos até GREEN real.


## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/call-once-2026-10-17/before.json.gz registra UTC,
status, diff HEAD --stat, fontes/L0/config e hashes. Os 2124 hashes da etapa
anterior foram verificados exatamente antes do baseline. before-probe.json.gz
e before-math-probe.json.gz conservam binários/argv/fixtures/saídas bilaterais.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
calc.abs/array.len/str.from-unicode medem receiver=3/args=2 contra 1/1;
module.measure/layout/within/before/after/func/has/body chegam a receiver=4;
fallback mutante mede args=3 contra 1. Métodos ausentes executam panic de
argumento antes do erro causal. math.mat soma duas vezes; array prévio (1,)
fica (1,1,1) contra (1,1). Array vazio mascara o defeito com truncate.
call.rs:70–118 perde receiver; collection_methods.rs:43 avalia args antes de
reconhecer operação; contextual/content/selector reavaliam AST no fallthrough;
method_dispatch.rs:247 descarta args; invoke.rs:66–120 compensa por nome.
O L0 call_dispatch.md já exige receiver/args uma vez. pinned-source.json.gz
fixa call.rs/methods.rs upstream: interesse é ordem/efeito/diagnóstico da
linguagem, não copiar Vm, storage ou igualdade Rust.

A primeira fixture declarava palavra reservada or como função: é inválida e
está arquivada em invalid-fixture-attempt, sem crédito para efeitos de chamada.
Ela revelou panic 101 em import inválido; manter aberto para correção causal
própria no owner de parsing/import. O baseline válido substitui só a fixture.

### Decisão e responsabilidade exclusiva

Receber cache e usar referência ao receiver resolvido em Location, Content/LocatedContent e OutlineEntry, sem reavaliar AST em cada grupo. Preservar helpers de transparência, snapshot/Location, args, traces e schemas; ajustar somente empréstimos/derefs necessários.

Consumir o núcleo eval/call-once.toml pinado. Mudança mecânica interna e
correção de paridade ADR-0127 em fluxo contínuo. Assinaturas pub fn/pub use e
structs públicos não mudam; os parâmetros novos/retornos/helpers acima têm
visibilidade crate/eval, sem novo trait público, campo público, default ou
fase. Esta cláusula supersede somente restrições históricas conflitantes de
assinatura interna e compensação por nome; manter extensões Tekt existentes.

### Aceitação e limites

Testes funcionais co-localizados antes do código: efeitos de receiver/args,
fallback mutante, erro antes de args, where/with/selector/coleções preservados
e matriz sem truncar efeito legítimo. Confirmar RED funcional e GREEN.
Comparar casos bilaterais válidos e controles; hits e misses de todos os
métodos de coleção, erros, spans/occurrences e complementos de Value. Não
mascarar dívida residual de diagnóstico como igualdade. Testar dispatch
Unhandled sem perder receiver ou executar callback de args; contar avaliações
pelo efeito observado, não por bytes de PDF, Rust Eq ou microbenchmark.

Ownership/V26 antes de resselo só dos owners do lote. Build, suíte workspace,
Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
fmt, V1/V5/V15/V26 e reverse, SARIF diferencial sem erro novo. V19 uniforme
continua informativo, com matriz de alternativas e relação de comportamento;
sem supressão nem mudança de configuração. Comandos pesados sequenciais com
12 GiB/high10GiB/swap0/jobs1. Não publicar/commit/merge/release nem declarar o
objetivo global fechado por esta etapa.
