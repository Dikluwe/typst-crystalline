# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `5d506068404b00765dd8cbb87bef0e4872668a53c0303daf682c54634540a48b`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/main
Hash do Código: 3918c50f

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/main.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/main.rs`. Declara módulos privados compile/watch/eval/query/fonts/completions/info/init/diagnostics e mantém eviction sob seu owner vigente. main faz cli::parse() e match exaustivo de RunIntent, delegando cada braço ao comando correspondente. Não usar wildcard, registro dinâmico ou wrappers de sucesso.

Conserva o carrier privado Published { Html(String), Bundle(Vec<BundleFile>) } exatamente como medido. É dado transitório preexistente compartilhado apenas entre compile/watch, não entidade nova; não torná-lo público nem justificar política nova com ele. Esta tarefa não corrige o aviso V12 preexistente movendo tipo para outra camada, pois isso exigiria contrato separado.

Não conserva corpos run_*, constantes de warnings, I/O ou testes de outro domínio. Teste local structural exige os módulos e rejeita handlers inline, passando RED→GREEN sem criar comportamento. Owners exclusivos: `00_nucleo/prompts/wiring/compile.md`, `00_nucleo/prompts/wiring/watch.md`, `00_nucleo/prompts/wiring/eval.md`, `00_nucleo/prompts/wiring/query.md`, `00_nucleo/prompts/wiring/fonts.md`, `00_nucleo/prompts/wiring/completions.md`, `00_nucleo/prompts/wiring/info.md`, `00_nucleo/prompts/wiring/init.md`, `00_nucleo/prompts/wiring/diagnostics.md`. Cláusulas históricas do L0 monolítico (P1137–P1485) são transferidas a esses owners; não há ownership 1:N.

## Aceitação

Conservar os sete testes inline nos domínios donos, mais os dois de eviction sem alteração; adicionar controle structural do entrypoint. Validar comandos, todos os targets, overrides PDF/SVG/PNG/HTML, stdout/stderr/exit, source transitória, watch, bundle, completions e info/init/fonts nos testes de processo. Build, V15/V26 e hashes individuais; preservar edits preexistentes da working tree.
