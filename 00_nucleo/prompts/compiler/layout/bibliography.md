# L0 — bibliography
Hash do Código: 150eb680

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/bibliography.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/bibliography.rs:25`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render an optional title and close its line. When the precomputed CSL cache supplies bibliography Content, dispatch it directly. Otherwise stably order entries by first citation, leave uncited entries in original order, add sequential `[N]` prefixes and the existing back-reference arrows. Own the `format_bib_entry_body` compatibility helper and its conditional field composition. The hub delegates fallback formatting here; preserve the existing field order and organization-as-publisher rule. This fallback is a local compatibility renderer, not evidence of complete vanilla CSL parity. Citation rendering belongs only to `00_nucleo/prompts/compiler/layout/cite.md`; cache construction belongs to `00_nucleo/prompts/compiler/layout/bib_csl.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## A2 — fronteira de bloco e validação de hanging-indent (2026-10-18)

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-indent-2026-10-18/before.json.gz contém
60 pares de sondas no HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c,
working tree não commitado/UTC/status/diff HEAD --stat, fontes/argv/binários,
texto e caixas. before-sources.json.gz preserva fontes/L0. Oracle pinado
bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, executável SHA-256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9;
Tekt baseline 97f6fa3078bdd6c40aabb80d6f93f3fd019b405ce70fbf2266686d8fb42271cb.

MLA sem título começa na linha da citação no Tekt (Silva x41.84/y5.168)
contra x8/y27.463995 no vanilla. Com título, o Tekt mistura título e citação
(y6.7568/y8.108) e intercala a extração no CSL customizado; o vanilla os
separa. Oito positivos de par medem recuo não implementado. Dez negativos
none/auto/int/ratio/string de set e constructor são aceitos no Tekt, mas o
oracle rejeita expected length. bibliography.rs:106–123 despacha o título sem
fechar a linha anterior e o cache como fluxo inline. set.rs:1180–1192 trata
hanging-indent como desconhecido; structural/par.rs:173 ignora o valor.

No pin, typst-layout/src/rules.rs:463–531 realiza título e blocos de entradas;
sem prefixo, o recuo é inset mais HElem compensador; prefixos usam duas colunas.
model/par.rs:431 declara Length e inline/collect.rs:146–148 conserva avanço
inicial absoluto. São fatos de linguagem/posição/erro; não obrigação de copiar
seu grafo Rust, bytes ou algoritmo de armazenamento. Hipótese causal desta
subetapa: ausência de fronteira de bloco mistura título/citação/corpo, e falta
de validação admite tipos inválidos. Medições de posições, tipos/hints/spans e
os controles restantes podem refutá-la; não concluir que um offset único resolve
recuo ou que toda diferença de extração PDF é só mecânica.

### Classe e divisão explícita

Correção interna de paridade em fluxo contínuo ADR-0127: sem campo/trait ou
assinatura pública nova, default/flag adicional, compatibilidade Rust ou fase
nova. O L0 precede testes/código. Esta revisão é incompleta para A2:
A2-Structure deve preservar hanging_indent/prefix/body do CSL e compor entradas
com suas margens/colunas; A2-HangingTransport deve transportar overrides por
ocorrência e resolver prioridades; A2-ParIndent deve completar composição de
parágrafos comuns (set/constructor), inclusive estratégias e direções. A2-Multi
continua atribuição/cache por bibliografia. Não silenciar warnings de suporte
incompleto antes de materializar seus consumidores, nem considerar positivos
hanging-indent corrigidos pela rejeição dos negativos.

### Responsabilidade exclusiva

Antes de emitir título ou corpo não vazio, fechar o segmento anterior pelo helper paragraph::close_in_chain, usando text_segment_chain quando disponível e a chain atual como fallback. Isso conserva o escopo causal da citação anterior mesmo quando o show da bibliografia muda a fonte; o helper continua governado por 00_nucleo/prompts/compiler/layout/paragraph.md e delega a parbreak. Despachar o corpo bibliográfico dentro de um Block breakable de largura/altura automáticas e inset zero, delegando toda geometria/margens ao owner 00_nucleo/prompts/compiler/layout/block.md. O corpo cacheado continua a ser o Content fornecido pelo cache; não mudar sua API ou morfologia pública. O fallback monta as mesmas linhas, ordem/campos/arrows e quebras anteriores, e usa a mesma fronteira de corpo. Não emitir Block para corpo vazio; título continua sua ocorrência existente, sem tags falsas, nova realização ou interpretação CSL. Não inventar constante vertical para ajustar um caso. Os recuos/per-entry/prefixos ainda requerem A2-Structure/A2-HangingTransport; global block defaults e controles de paginação não são presumidos equivalentes pelo template.

### Aceitação

Testes RED funcionais antes da implementação; GREEN de fronteira título/citação/corpo/fallback e negativos com controles de zero/negativo/affine e vazio. Medir novamente caixas PDF/extração, manter os REDs de recuo e múltiplas bibliografias explícitos, não atribuir vantagem ao Tekt sem fonte/controle. Revalidar build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e reversa, SARIF sem erro/warning novo. Teto 12 GiB/High8 GiB/swap0/jobs1/debug0/incremental0; comandos pesados sequenciais. Sem commit/push/release.
