# L0 — infra/export/gradients/relative
Hash do Código: 106b75e8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/relative.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/relative.rs:30` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Relative-coordinate helpers

Own only resolve_relative and apply_parent_transform, with their current tuple
signatures. resolve_relative accepts Option<RelativeTo> and uses unwrap_or_default
(None resolves to Self_). It does not itself choose a geometry bbox or resolve
an on-text policy.

apply_parent_transform accepts four f32 unit-space coordinates and optional
(px0,py0,px1,py1). Some maps each point by parent origin and span; None returns the
tuple unchanged. Preserve arithmetic and coordinate units.

`00_nucleo/prompts/infra/export/gradients/mod.md` owns effective bbox discovery/
keys; `00_nucleo/prompts/infra/export/builder/resources.md` owns shading placement;
`00_nucleo/prompts/infra/export/stream/shape.md` owns paint lookup.
Do not duplicate those rules or redefine their helper signatures.

## Verification

Check default resolution, identity without parent and translation/scale with parent.
Transform mechanics are local implementation contracts, not language identity.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/relative.rs` nas linhas 12 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.
