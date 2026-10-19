# L0 — infra/system-world
Hash do Código: 9f650c8e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world.rs`

## Current evidence

- `03_infra/src/world.rs:35` — `SystemWorldError`.
- `03_infra/src/world.rs:43` — `fmt`.
- `03_infra/src/world.rs:58` — `SystemWorld`.
- `03_infra/src/world.rs:97` — `for_eval`.
- `03_infra/src/world.rs:132` — `new`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world.rs`.

## Owned contract

- Own SystemWorld state, SystemWorldError/Display, new/for_eval and input/plugin builders only. Cache, fonts, paths, packages and World implementation have independent descendant owners.
- new accepts the existing root/main inputs, canonicalizes the physical main, eagerly reads it, registers FileId 1 and starts subsequent ids at 2. Preserve main-not-found and I/O errors.
- for_eval creates a preloaded synthetic source at root/<input-expression> without file reads; imports still resolve against the supplied root.
- Initialize the current per-world maps/locks, library, empty SysInputs, absent plugin host and default package downloader. Font state consists of a once-initialized Arc of a paired catalog and a private discovery closure returning that Arc; an unconfigured world has an empty catalog. The font child owns discovery and replacement. with_inputs stores strings and with_plugin_host stores the supplied Arc; no I/O enters the L1 contracts.
- Preserve public paths/builders and private descendant access. Do not add a new trait or global cache. The ratified session extension below is the only new export; child-to-child helpers remain private to L3 descendants.

## Child owners

- `00_nucleo/prompts/infra/system-world/cache.md` owns only `03_infra/src/world/cache.rs`.
- `00_nucleo/prompts/infra/system-world/fonts.md` owns only `03_infra/src/world/fonts.rs`.
- `00_nucleo/prompts/infra/system-world/packages.md` owns only `03_infra/src/world/packages.rs`.
- `00_nucleo/prompts/infra/system-world/paths.md` owns only `03_infra/src/world/paths.rs`.
- `00_nucleo/prompts/infra/system-world/tests.md` owns only `03_infra/src/world/tests.rs`.
- `00_nucleo/prompts/infra/system-world/world_impl.md` owns only `03_infra/src/world/world_impl.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`00_nucleo/diagnosticos/diagnostic-paths-2026-10-06/identity-next-audit.json`
conserva comandos, fontes, UTC, HEAD, árvore alterada e hashes dos binários do
Tekt e do vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8. Os casos
separam erro/warning do documento de erro da expressão: o documento perde sua
fonte, enquanto a expressão recebe indevidamente o nome do documento.
`01_core/src/compiler/eval/mod.rs:609-615` usa world.main() para Source e base;
`04_wiring/src/eval.rs:66-95` repete esse id ao fornecer o primary ao formatter.
`03_infra/src/world/cache.rs:58-78` registra arquivos e recicla o contador;
`lab/typst-original/crates/typst-cli/src/eval.rs:159-179` e
`lab/typst-original/crates/typst-cli/src/world.rs:176-190` distinguem expressão,
stdin e ausência de entrada por identidade. A igualdade dos inteiros e o
interner global upstream são mecânica, não requisito (ADR-0107). A origem
mostrada, fonte, posição e resolução relativa são observáveis de linguagem.

### Decisão e escopo do gate

Adicionar o constructor público:

```rust
pub fn for_stdin(root: impl Into<PathBuf>, text: String) -> Self;
```

Cria main FileId 1, Source de markup já carregado com caminho root/<stdin>,
bytes UTF-8 correspondentes e a mesma configuração inicial de for_eval.
Não lê stdin: o caller fornece o texto. Imports/read usam a raiz fornecida;
main() e world.source(main()) referem o documento stdin verdadeiro. Nenhuma
entrada transiente pertence a path_to_id. for_eval conserva assinatura/base e
Source vazio, mas também deixa de registrar seu marcador como arquivo físico.
Isso permite importar um arquivo real com o mesmo nome sem alias de Source.

next_id passa a contador privado capaz de representar o estado esgotado após
65535; inicialização permanece 2, main permanece 1 e os mapas são por World.
A operação de alocação/publicação pertence ao owner cache; não adiciona estado
global. for_stdin não habilita compile stdin nem muda CLI flags ou defaults.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509

## Medição anterior à decisão de retenção (candidato não ratificado)

00_nucleo/diagnosticos/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
00_nucleo/diagnosticos/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão proposta sob ADR-0127 §2.1/§2.2

Aguardar ratificação do pacote 00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json
antes de instalar este candidato ou escrever código produtivo. Cada owner continua 1:1.

## Extensão exclusiva do estado e export

Declare filho font_session e reexporte seu tipo FontCatalogSession no caminho
público typst_infra::world::FontCatalogSession. A definição e métodos pertencem
somente ao child owner 00_nucleo/prompts/infra/system-world/font_session.md.
font_catalog torna-se OnceLock<Arc<(Vec<FontSlot>, FontBook)>>; font_loader retorna
esse Arc. Constructors continuam com catálogo vazio, sem aquisição de fontes.
Sources/bytes/mapas/contador/inputs/plugins/downloader e coverage_cache mantêm
inicialização e duração por World. Não adicionar reset, método em World ou
mudança de identidades. O builder novo pertence somente ao owner fonts.
