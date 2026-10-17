# L0 — svg/navigation
Hash do Código: 4c3e29ad

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/navigation.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36

## Consumer evidence

`03_infra/src/export/svg/navigation.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Page-local destination context

Own SvgDestinationContext/SvgDestination, label normalization, lookup, destination
nodes and render_link. Consume the pinned destination-context nucleus rather than
duplicating its shared page-local registry/routing obligations in the façade.
The immutable context performs no layout/filesystem lookup and carries no filenames.

Emit one local node per present destination at its supplied position and bind the
link edge using the same collision-free identity. Missing/cross-page destinations
without an explicit route preserve visual children and remain Unknown, not dangling
fragments. External URLs retain their target. Preserve hit area, escaping, nested
children and transforms. Caller-owned bundle/cross-page filenames are not inferred.
SvgDestinationContext constructors and normalization belong here. SvgOptions and
export wrapper defaults remain in `00_nucleo/prompts/infra/export/svg.md`.

## Verification

Check same-page closure, deterministic IDs, missing/cross-page fallback, URL targets,
escaped labels, nesting and hit-area transforms.


## Binding da face ratificada — 2026-10-05

Medição: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/probe.txt`. Transportar o contexto privado PreparedFonts pelos wrappers SVG; preservar geometria/semântica/API pública. Contexto discrimina face original/eixos e fornece bytes preparados, sem resolução nova por nome. O campo TextShaped.font foi confirmado pelo humano.

Claims compartilhadas desta migração pertencem ao núcleo shaped-face-identity pinado acima; este owner materializa somente as referências da sua unidade.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C069`: `03_infra/src/export/svg/navigation.rs:83`, `render_link`, 12 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C069` — `render_link`: Correção interna prescrita

Link reenvia a mesma sessão SVG usada pelo dispatcher/grupos. Manter target, área de hit e filhos explícitos, mas transportar as definições/fontes/IDs/modo no contexto privado do hub.

Receber a sessão privada do owner SVG, mantendo target, items, pos e size separados; continuar free function de navegação.

Aceitação: URLs, destinos presentes/ausentes, área de hit, transforms, conteúdo e modo HTML iguais; não derivar filenames ou fazer novo layout.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Aridade final — materialização interna, 2026-10-16

### Medição anterior à decisão

Fontes e L0 vigentes conferidos antes do código; snapshot 00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16/before.json.gz conserva HEAD, working tree não commitado, UTC, status, diffstat e hashes globais. callers.txt registra busca no workspace, incluindo fixtures; as chamadas homônimas do math::MathLayouter são outro motor e permanecem intactas. Inventário anterior tem 22 avisos, cinco destas aridades. Trata-se de estrutura privada Rust, sem nova afirmação de paridade ou intenção upstream.

### Responsabilidade exclusiva e aceitação

Concretizar C069 com render_link(xml,target,items,pos,size,session). Destinos vêm de session.destinations; filhos reusam a sessão. Preservar registro imutável e sua API, href/fallback, hit-area, transformação e serialização. Sessão privada definida em 00_nucleo/prompts/infra/export/svg.md.

Este adendo materializa somente o escopo nominal e supersede seu estado pendente da auditoria. Fluxo contínuo ADR-0127: sem contrato externo, default, fase ou compatibilidade novos. L0 primeiro e resselo restrito. RED de too_many_arguments antes do código, GREEN sem novas supressões ou alteração de limiar; expectativas anteriores devem continuar satisfeitas. Conferir corpos e transporte de valores, preservar fixtures/esperados, sem testes que espelhem carriers. Executar controles existentes de Grid/Table/outline e SVG/tiling antes/depois, suíte workspace completa, build, Clippy all-targets, Rustdoc, formato, linhagem estrita/reversa e Tekt sem novos avisos, sob 12 GiB, swap zero e um job. Resultados não certificam paridade global ou conformidade PDF/UA.
