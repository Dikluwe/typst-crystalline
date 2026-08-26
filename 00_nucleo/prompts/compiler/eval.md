# Prompt L0 — `compiler/eval` — dispatcher e contexto
Hash do Código: 4a222767

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/eval/mod.rs`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0024, ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

O consumer possui `EvalContext`, os entrypoints públicos, a criação do scope
base, a passagem dupla, `eval_markup` e o dispatcher exaustivo de `Expr`; os
corpos especializados já delegam aos seus módulos donos.

## Contrato

Os entrypoints constroem scope fresco e avaliam `Source` sem I/O direto.
`eval_expression` avalia código isolado. O dispatcher preserva spans, scopes,
short-circuit, joins e eventos de fluxo. `EvalContext` transporta limites,
metadados, target, features e `FlowEvent`, sem estado global mutável. Conteúdo
de introspecção pré-show e conteúdo final pós-show permanecem distintos;
evento residual no entrypoint é erro. Mudança pública, de default ou fase para
no gate ADR-0127.

## Aceitação

Entrypoints, scope base, duas passagens, dispatcher, spans e fluxo residual são
cobertos pela suíte de eval; L1 permanece puro.

## P1215 — source numerizada em `eval_expression`

Medição mostrou que `eval_expression` avaliava via `native_eval` com
`Args::positional`, portanto com span detached. O entrypoint deve criar uma
`Source` em modo code com `world.main()` e o texto integral, avaliar os filhos
da raiz numerizada no mesmo scope fresco e devolver diagnósticos cujos spans
resolvem contra uma `Source` idêntica. Isto preserva assinatura, valores,
features e fase; apenas deixa de descartar localização pública no CLI `eval`.
