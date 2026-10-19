# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/compile.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `2a036902b37e289e0edd6a7ea1375202f5127537f99dfe28ee98555756b5fbc9`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — wiring/compile
Hash do Código: 24a92fbf

**Camada:** L4
**Consumer exclusivo:** `04_wiring/src/compile.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

## Medição anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada em 2026-09-26T12:30:13Z. `04_wiring/src/main.rs` tem 1.350 linhas: constantes :72–86, sete testes :88–325, dispatcher :327–338, watch/helpers :340–500, init/info/completions/fonts :502–607, Published :609–612, publicação de bundle :614–648, compile :650–1114, eval :1116–1215, query :1217–1289 e diagnósticos :1291–1350. Fonte e estado exatos no diagnóstico `00_nucleo/diagnosticos/wiring-main-atomization.md`. L0 vigente foi lido integralmente antes da decisão.

## Classificação e fronteira

Atomização interna ADR-0037/0109/0127 contínua. Não adicionar contrato público, defaults, flags, fase ou compatibilidade novos. L4 só compõe intents L2 com operações L3 e valores L1; não ganha parser CLI, serializer, política de linguagem, renderização, TLS ou watcher. Preservar os corpos medidos, inclusive a composição de bundle já presente; não antecipar nem declarar aprovados gates históricos marcados PROPOSTO. Arquivos de integração mantêm owners próprios.

## Contrato proprietário

Consumer exclusivo `04_wiring/src/compile.rs`. Único dono de run_compile, run_compile_observed, report_in_order, write_bundle_output e constantes privadas BUNDLE_EXPERIMENTAL_WARNING/HTML_EXPERIMENTAL_WARNING. Somente run_compile e run_compile_observed passam a pub(super) para main/watch; nenhum campo, trait, assinatura externa ou algoritmo muda. Published continua privado no parent main, importável por descendência.

run_compile usa callback vazia e ignora dados de observação. observed preserva destructuring completo do CompileIntent e seu retorno (ExitCode, Vec<PathBuf>, bool, Option<Published>). Bundle rejeita stdout/feature ausente sem ativar feature; PDF valida pages + pdf_tagged explícito, depois tagged CLI-only com perfis acessíveis e hints de pages, depois conflitos PdfStandards, antes de World/pipeline. Não reordenar early returns nem sua callback. Validar std::Path apenas como já medido.

SystemWorld recebe root/main, fontes, inputs, CA e plugin host; Source/dependências/coverage permanecem no fluxo vigente. compact só mapeia StreamMode; tagged preserva Some explícito, legado false e None; pretty/pages/standard/PPI seguem tri-state/tipos intactos até L3. A opção oracle PDF só vale para PDF. Match de OutputFormat fechado Bundle/Pdf/Png/Svg/Html fica neste owner, incluindo todos os caminhos com/sem timings; não mover fases ou formar documento aqui. HTML serialization só é mapeada para enum L3. BundleCompileOptions mantém todos os campos e defaults medidos.

Saída bem-sucedida escreve bytes ou árvore de BundleFile; bundle mantém criação de pais, validação de Component::Normal, canonicalização e rejeição de symlinks/escape exatamente como a fonte, sem hardening incidental nem mudança de atomicidade. Falha de write/timings vira exit 2; erro de compilação exit 1; sucesso exit 0. Só sucesso produz Published. report_in_order chama status e diagnostics uma vez nessa ordem; na drenagem, warning experimental condicionado ao target/feature, warnings antes de errors e erro operacional por último. Preservar ausência de medições opt-in e saída CRYSTALLINE_MEASUREMENTS=1, delegada às mesmas APIs L3.

## Aceitação

Os testes P1323 de envelope HTML e P1366 de ordem de callbacks são movidos sem modificar corpos. Suítes CLI e p1462–p1469 conferem resultados e neutralidade entre targets; bundle/watch e controles de feature preservam publicação. O owner de drenagem é `00_nucleo/prompts/wiring/diagnostics.md`; formatter/serializer continuam L2 e pipeline L3. Não duplicar essas regras aqui.
