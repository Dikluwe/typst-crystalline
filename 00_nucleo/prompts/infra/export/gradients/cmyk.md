# L0 — infra/export/gradients/cmyk
Hash do Código: 8418324e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/cmyk.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/cmyk.rs:22` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## CMYK sampling and fallback conversion

Own rgb_to_cmyk and linear/radial four-component samplers. Use max(n_samples,2)
uniform f32 samples via each gradient's declared interpolation. Preserve native
CMYK components when returned; clamp channels to [0,1]. Otherwise use the existing
sRGB fallback conversion, not an ICC conversion or a new output-intent policy.

Set k=1-max(r,g,b). For k>=1-1e-6 return (0,0,0,1); otherwise use
c=(1-r-k)/(1-k), m=(1-g-k)/(1-k), y=(1-b-k)/(1-k), clamping all channels.
Black yields full K and white zero ink. Alpha is not represented by this sampler.
The conic emitter may reuse rgb_to_cmyk without this owner deciding shading topology.

## Verification

Check black/white, native versus fallback CMYK, endpoints and bounded samples.
Print component identity is observable; ICC-managed color fidelity or universal
upstream conversion parity is not asserted.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/cmyk.rs` nas linhas 12, 13 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.
