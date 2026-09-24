# Passo 1495 — auditar o catálogo de símbolos e decidir compatibilidade

**Estado:** PLANEADO; etapa de L0/decisão, sem troca automática de dependência. Dependência: P1489.

## Medição anterior à decisão

O vanilla pinado `9f2b6e87` usa `codex` git `2e43ab9` (`lab/typst-original/Cargo.toml:51`); o cristalino fixa `codex = 0.3.0` (`Cargo.toml:74`). P1489 mediu seis módulos-pai ausentes (`{sym,math}.{chess,keyboard,ordinal}`), 42 descendentes bloqueados e 24 `VERIFIED_METADATA_DIFFERENCE`: são 12 grupos duplicados pelo espelho `sym → math`, não 24 bugs independentes. Sondas de linguagem já demonstram `sym.chess`, `sym.not.inv` e `sym.tack.r.double` divergentes. Proveniência integral: `00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md`; revalidar o estado antes da spec.

O novo catálogo remove `join` e variantes antigas `gt.tri.*`/`lt.tri.*`; o cristalino ainda possui rotas históricas em `01_core/src/compiler/stdlib/sym.rs:533,610-646`. Uma troca mecânica de `codex` poderia fazer `#sym.join` sumir enquanto `$join$` sobrevivesse pelo fallback. `01_core/src/compiler/stdlib/structural/math.rs:3123-3131` espelha o módulo; `01_core/src/compiler/eval/math.rs:577-592` consulta uma tabela estática antes de `sym_lookup`. Uma auditoria preliminar encontrou `emoji.txt` byte-idêntico entre os pins; confirmar os dois SHA-256 no relatório deste passo, pois a dependência é compartilhada.

Inferência: o pin explica a maior parte da diferença; refutação: valores ainda distintos com o mesmo catálogo por overrides ou precedência local. Valor novo do vanilla não implica copiar defeito: confrontar changelog, testes e sondas.

## Contrato de preparação

1. Auditar L0 1:1 `00_nucleo/prompts/compiler/stdlib/sym.md`, `00_nucleo/prompts/compiler/stdlib/structural/math.md` e `00_nucleo/prompts/compiler/stdlib/emoji.md`, todos com menções antigas ao pin. Só tocar `00_nucleo/prompts/entities/symbol.md` se representação/algoritmo/contrato desse consumer mudar; se precedência bare ou warnings exigirem código em `eval/math.rs` ou `eval/bindings/field_access.rs`, auditar também seus L0 canônicos `00_nucleo/prompts/compiler/eval/math.md` e `00_nucleo/prompts/compiler/eval/bindings/field_access.md`. Redigir a revisão L0 antes de L1; atualizar pins de Núcleos somente se claims compartilhadas mudarem.
2. Montar sondas RED recursivas para os seis pais e seus 42 filhos, com valores/modificadores/`repr`; sondar os 12 grupos `eq, equiv, gt, lat, lt, not, prec, smt, succ, tack, tilde, trademark` nos dois namespaces. Incluir aliases bare em matemática, `not.inv` e os cinco valores `tack.*.double`. Não equiparar presença de binding a comportamento.
3. Medir rotas legadas `join`, `gt.tri.*`/`lt.tri.*` no novo vanilla e no cristalino. Registrar opções: preservar aliases cristalinos como extensão compatível, com `#sym` e matemática coerentes, ou removê-los. **Remoção pública é quebra de compatibilidade: redigir L0 causal e parar para confirmação do dono (ADR-0127).** O changelog do novo `codex` chama as alterações `tack.*.double` de breaking; classificar glifos em caminhos existentes como correção de tabela ou mudança de contrato; se houver dúvida, parar.
4. Entregar decisão registrada e lista exata de testes/owners para P1496. Não atualizar o lockfile nem código produtivo neste passo enquanto o gate de compatibilidade estiver aberto; sondas de laboratório RED são permitidas e não legitimam L1.

Nenhum dos 469 `UNVERIFIED_METADATA` do perfil default é tratado como bug por inferência.
