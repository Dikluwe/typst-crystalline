# L0 — Domain entity facade
Hash do Código: c6772200

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/mod.rs`

## Responsibility

The consumer declares the static L1 entity modules, the inline `frame_visitor`
reexport facade (`walk_frame_items`, `FrameVisitor` from `layout_types`) and
the private `p1431_line_cap_tests` module, gated by its own inner `#![cfg(test)]`.

Preserve its public module surface, including `font_variations` and
`document_format_options`. Each child owns its domain behavior. The facade
must not duplicate defaults, parsing, document extraction or export precedence.
Format carriers belong to `00_nucleo/prompts/entities/document_format_options.md`;
font axes belong to `00_nucleo/prompts/entities/font_variations.md`.

Verify static declarations and test-only gating. No new reexports, removal of
modules or business logic is authorized by this documentary cleanup.

## Fachada de contexto bidi — contrato ratificado ADR-0127, 2026-10-05

### Medição anterior à decisão

Medição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/initial.json.gz` preserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree integral, UTC, fontes e binários; treze entradas CLI com vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. rtl_auto_mixed põe ABC em x=124.020510 no Tekt contra152.949220 no vanilla; ltr_numeric põe árabe à esquerda e inverte 123/456, contra números à esquerda no vanilla. `layout_bidi.rs:73–82` usa primeiro caractere forte da linha e `:227–240` usa qualquer style.dir RTL como nível padrão; `shaper/bidi.rs:34` resolve cada string novamente com base automática. O contexto privado de paragraph.rs não chega a esses consumers. Vanilla `inline/mod.rs:189,232` resolve direção compartilhada; `inline/collect.rs:156–178` conserva embedding explícito por TextElem com direção diferente da base. `probe.json.gz` registra receita, rustc, hashes da biblioteca unicode-bidi 0.3.18 e sonda independente: dentro de ABC 123 456 789 مرحبا, linha 456 789 é um run LTR; a mesma string isolada com a mesma base RTL tem runs 789, espaço, 456. Isso refuta o transporte somente de Dir e a resolução por linha/palavra isolada. Não é prova de geometria de um candidato produtivo. Semântica/sintaxe/morfologia são obrigações; biblioteca, ranges, Arc e estratégia de cache são mecânica interna (ADR-0107).

### Ampliação pública ratificada

Acrescentar somente `pub mod bidi_context;`, cujo consumer único é `01_core/src/entities/bidi_context.rs` e owner é `00_nucleo/prompts/entities/bidi_context.md`. Esta seção supersede a proibição de novos módulos da limpeza documental apenas para essa declaração após a confirmação do pacote. Não reexportar tipos, decidir níveis Unicode, adicionar estado ou duplicar as validações do filho. Manter todas as demais declarações.

Ratificação recebida: “confirmo”, manifesto 1ae7513eb3af426754909954aef165c97257a9f62d3911fbb111e2002fac4043, evidência em `00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json`. Versões congeladas estão no receipt before.json.gz da implementação; alterações de Hash do Código não substituem a prova da aprovação.

## Integração condicional dos módulos de teste — 2026-10-13

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado. A duplicação cfg(test) ocorre em `01_core/src/entities/p1431_line_cap_tests.rs:5` / pai :128.
Fontes, L0, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/cfg-test-ci-2026-10-13/before.json.gz` e
`refinement-before.json.gz`. O Clippy pinado reproduz os atributos duplicados.
A tentativa de retirar a condição interna mantém os testes Rust, mas o linter
arquitetural pinado não propaga a condição do pai e acusa V2 nos helpers.

Neste owner, retirar apenas o atributo externo cfg(test) das declarações
listadas acima. Manter os caminhos, nomes e visibilidades dos módulos e seus
atributos path. A condição interna `#![cfg(test)]` de cada arquivo listado
continua a excluir integralmente o módulo do build de produção, inclusive
seus imports e helpers. Os consumers de teste e seus L0 permanecem intactos.
Não acrescentar dependências, lógica, API, modo padrão ou fase. Validar a
mesma lista de testes, suíte core, build sem cfg(test), Clippy e arquitetura.

## Transporte de célula preparada — candidato ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/tiling-transporte-2026-10-14/{parent-before,solid-controls}/results.json.gz registra UTC, diffstat/status, fontes/owners, argv e hashes de binários. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8; SHA-256 do oráculo 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, árvore 6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489.
Layout tiling.rs:124–127 devolve None acima de 10000 células; shape.rs:151–185 só materializa fill, conservando stroke declarativo; export/stream/shape.rs:151/201 reduz Tiling a cor. layout_types.rs:505/531 não transporta célula nem raiz de pintura. render.rs:217 reduz fill a cor e :579 ignora clip_mask. Esses limites são do código/contrato de transporte, não só da fórmula do offset.
A medição anterior tiling-cobertura-2026-10-14 registra over-limit200×100pitch1 sólido e stroke sem motivo. Fontes respectivas permanecem idênticas; esse registro não é uma nova medição de binário.
No SVG atual 240×192 branco/librsvg, place-parent diverge em1194 pixels >16, box-fixed/auto-parent em1192; seus controles Solid coincidem pixel a pixel. block-parent tem2 pixels >16, máximo23. Os SVGs vanilla explicitam T(offset)*R e inversa de transformações herdadas. Fonte pinada typst-svg/src/lib.rs:346–358 e typst-layout/src/inline/box.rs:35–49 confirma raízes locais antes do inset final; FrameKind em typst-library/src/layout/frame.rs:459–474 distingue Hard/Soft. É semântica geométrica, não obrigação de copiar FrameKind, Packed ou igualdade Rust.
Controles Solid também refutam geometria geral em box-place e rotate: respectivamente3200/5296/3645 pixels >16. Portanto o transporte não será usado para declarar essas diferenças automaticamente resolvidas. Registrar a separação e corrigi-las com seus owners. No controle anterior quarter, Tekt PDF/SVG coincide com vanilla SVG, mas vanilla PDF diverge; não copiar essa diferença conhecida.

### Contrato candidato exclusivo

Acrescentar apenas pub mod frame_paint; não duplicar lógica do filho nem criar reexports gerais. Isto supersede a vedação histórica a módulo novo somente para o carrier aprovado.

Gate obrigatório: campos e tipos públicos/compatibilidade mudam. Não escrever L1–L4 antes da aprovação humana do manifesto. Owners de implementação serão atualizados antes de cada consumer, com claims compartilhadas pinadas e RED→GREEN.
