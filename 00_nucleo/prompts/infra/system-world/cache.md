# L0 — infra/system-world/cache
Hash do Código: a61c983f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:7ec137b6996e0ea442e3cdda52ce723fd32cb67338ab363b7ff2d733458ef8fa

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/cache.rs`

## Current evidence

- `03_infra/src/world/cache.rs:20` — `SourceSlot`.
- `03_infra/src/world/cache.rs:27` — `new`.
- `03_infra/src/world/cache.rs:31` — `synthetic`.
- `03_infra/src/world/cache.rs:40` — `get`.
- `03_infra/src/world/cache.rs:58` — `register_file`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/cache.rs`.

## Owned contract

- Own SourceSlot/OnceLock, file registration, dependency enumeration and byte/source cache helpers; per-world storage remains in the facade.
- Synthetic sources are already loaded; physical sources load lazily except main. OnceLock memoizes the first Source result, including errors. Canonicalize registration when possible, otherwise retain the original path; allocation follows the checked identity contract below. Registration must atomically associate one canonical path with its FileId and initialized slot under concurrent calls; keep the existing fast lookup and serialize the publication under next_id → path_to_id → slots, rechecking identity after acquiring the allocation lock.
- Successful byte reads share Arc by canonical FileId across path/id access. Do not cache failed byte reads, so they may retry; failed Source reads remain memoized. Preserve FileError/Bytes conversion behavior.
- Dependencies are sorted and unique and include main, sources, assets and failed attempts, not system fonts. Equivalent physical paths share identity without an independent cache.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json`
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

Adicionar a fábrica pública de Source code transiente:

```rust
pub fn source_for_expression(&self, text: String) -> FileResult<Source>;
```

Alocar um id novo sob next_id, construir uma única Source::new_with_parser
com parse_code, publicar slot já carregado em slots com caminho
root/<input-expression> e bytes UTF-8 em read_cache. Retornar o mesmo Source
resolúvel por World::source; World::file usa os bytes em RAM. Não registrar o
marcador em path_to_id, canonicalizá-lo ou lê-lo no disco. Chamadas distintas
recebem ids distintos mesmo quando textos são iguais. A Source de stdin e o
main vazio também têm bytes pré-carregados; os helpers privados de SourceSlot
podem receber uma Source pronta sem alterar a API pública de Source.

Revogar preserve id allocation/wrapping: usar contador verificado comum a
fontes físicas e transientes, nunca reutilizar id publicado. Limite FileId
permanece NonZeroU16. Na exaustão, source_for_expression retorna
FileError::Other("FileId counter exhausted") antes de publicar qualquer slot.
register_file mantém a assinatura e retorna ids já conhecidos mesmo esgotado;
se uma identidade física nova não cabe, falha com panic explícito
"FileId counter exhausted", sem sobrescrever Source ou caches existentes.
A mudança do comportamento extremo de exaustão também está incluída no gate.
Preservar fast lookup e ordem de locks next_id → path_to_id → slots para
registro físico; transientes usam next_id → read_cache → slots, sem caminho
que adquira next_id depois de um desses locks. Não envenenar locks ao testar
esgotamento: detectar a condição, soltar guards e só então emitir o panic.
Dependências físicas e memoização de erros/sucessos mantêm o contrato anterior.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.

## Reuso sintático de watch — lote A

### Medição anterior à decisão

lab/results/construction/diagnostics/incremental-prioridade6-2026-10-08/before.json.gz,
measurement.json.gz, control-measurement.json.gz e count.json registram árvore,
HEAD, UTC, comandos e releases. Source::new reconstrói CST; World novo constrói
Source main/import a cada edição. O import mínimo já absorve recursos no eval_ms;
não atribuir toda a métrica a parsing. O contrato causal de Source/id/caminho vem
dos owners existentes; o reuso é mecânica e precisa preservar observáveis de língua.
Hipótese de benefício refutada por custo, diagnóstico, efeito, stale source,
geometria ou identificação diferente. Não inferir intenção upstream.

### Contrato ratificado

SourceSlot físico pode transportar Arc opcional privada da sessão. Após read_to_string
normal, fornecer caminho canônico/id/texto completos ao owner source_session;
sem sessão, usar Source::new vigente. Registrar IDs/deps normalmente. OnceLock de
cada World mantém seu snapshot/error; nenhum erro de leitura persiste na sessão.
Synthetic/from_source/code/expression permanecem fora do cache inter-World.
Conservar interfaces privadas existentes quando isso evitar reparos externos.

Aceitação: RED→GREEN, contagem de produção separada de tempos, três rodadas CLI
pareadas contra release P5, startup/RSS e controles de invalidação/identidade/
efeitos/Unicode/diagnóstico/assets/fontes/geométricas. Resselo apenas deste owner,
build/suítes pertinentes e lint zero erros; não fechar incrementalidade completa.
