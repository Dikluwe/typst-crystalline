# L0 — svg/groups
Hash do Código: 27cad24e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:96fe00a0c6071bb88ffc63806b3e42c23b78c398fa26d46866e0f099ab0d4e36

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/groups.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/groups.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Groups and local clips

Own render_group, geometric clip validation/writing and matrix-identity checks.
Recurse through façade dispatch. Position/transform apply once; local clips and
children share the same spatial basis. Nested clips intersect with ancestors.
None adds no clip. Finite representable Rect/RoundedRect/Ellipse/Path creates a
local clipPath and resolved reference: dimensions use inner_width/height, corners
remain distinct and paths preserve subpaths/curves. Share collision-free deterministic
clip IDs with images. Allocate a new definition for each accepted clip; this
consumer does not deduplicate equivalent clip definitions.

clip_mask carries no fill-rule. Paths requiring evenodd, open lines without area,
nonfinite/unrepresentable or absent information remain Unknown/CONTRACT-GAP;
do not claim missing clipping is equivalent or silently drop the content. Alpha masks,
filters, opacity and rasterization are not substitutes for geometric clipping.

## Verification

Check supported shapes, transformed/nested clips, unique clip IDs, reference
closure and opaque/nonfinite/fill-rule gaps.


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

- `C067`: `03_infra/src/export/svg/groups.rs:16`, `render_group`, 14 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C067` — `render_group`: Correção interna prescrita

Group reenvia sete componentes do mesmo contexto SVG a cada filho, além da geometria local. A sessão de definições/fontes/IDs/modo deve ser emprestada como contexto privado do hub; as dimensões internas formam Size existente.

Consumir o contexto privado de sessão definido pelo owner SVG; combinar inner_width/inner_height em Size existente. A free function conserva geometria/filhos explícitos e permanece descendente estática.

Aceitação: Clips/matrizes aplicados uma vez, IDs únicos compartilhados com imagens, escopo local de paint_root restaurado, destinos/fontes/HTML preservados.

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

Concretizar C067 com render_group(xml,pos,matrix,clip_mask,size,items,session) e Size existente (Pt internos sem recálculo). Ler width/height e reusar o mesmo contador de clips; manter serialização/matriz/fallback/ordem dos filhos e helpers geométricos intactos. Consumir a sessão privada do owner 00_nucleo/prompts/infra/export/svg.md por descendência estática.

Este adendo materializa somente o escopo nominal e supersede seu estado pendente da auditoria. Fluxo contínuo ADR-0127: sem contrato externo, default, fase ou compatibilidade novos. L0 primeiro e resselo restrito. RED de too_many_arguments antes do código, GREEN sem novas supressões ou alteração de limiar; expectativas anteriores devem continuar satisfeitas. Conferir corpos e transporte de valores, preservar fixtures/esperados, sem testes que espelhem carriers. Executar controles existentes de Grid/Table/outline e SVG/tiling antes/depois, suíte workspace completa, build, Clippy all-targets, Rustdoc, formato, linhagem estrita/reversa e Tekt sem novos avisos, sob 12 GiB, swap zero e um job. Resultados não certificam paridade global ou conformidade PDF/UA.
