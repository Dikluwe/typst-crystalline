# L0 — infra/export/gradients/radial
Hash do Código: 7ddce23d

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/radial.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/radial.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Radial coordinates and RGB samples

Own compute_radial_coords and multispace_sample_stops_radial for PDF radial
shading. Preserve the current coordinate-helper signature: center, radius,
focal_center, focal_radius, width and height.
Resolve center/focal ratios against width/height; both radii scale by min(width,height).
Return (fx,fy,fr,cx,cy,r). Default focal=center and focal_radius=0 are supplied
by the domain/caller, not a second option policy here.

Sample max(n_samples,2) uniform f32 points through Radial::sample, convert to
RGBA, discard alpha and clamp RGB to [0,1]. Function/shading dictionaries and
bbox placement are owned by
`00_nucleo/prompts/infra/export/builder/resources.md`.

## Verification

Check independent focal circles, unequal bbox dimensions, centered defaults and
bounded multispace RGB samples. Do not claim alpha-preserving SVG support from
this PDF sampler or exact upstream algorithm parity.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/radial.rs` nas linhas 12, 13, 14 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.
