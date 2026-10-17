# L0 — infra/export/gradients/linear
Hash do Código: 7237adb4

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/linear.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/linear.rs:14` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Axial coordinates and RGB samples

Own compute_axial_coords and multispace_sample_stops for PDF axial shading.
Endpoints are symmetric around (x0+w/2,y0+h/2), using dx=cos(angle), dy=sin(angle)
and half offsets (w*dx/2,h*dy/2). Preserve horizontal/vertical cases and bbox units.

Sample max(n_samples,2) points uniformly from 0 to 1 in f32. Use Linear::sample
in its declared color space, then to_rgba_f32; discard alpha in this PDF RGB
sampler and clamp each RGB channel to [0,1]. Do not force Oklab or special-case
two original stops as two samples. PDF shading/function objects belong to
`00_nucleo/prompts/infra/export/builder/resources.md`; native interpolation
belongs to the gradient entity, not this adapter.

## Verification

Check centered endpoints, sample endpoints/cardinality, each declared interpolation
space and bounded RGB channels. The sampler's alpha omission is a limitation;
SVG's adaptive alpha-preserving path has a different owner.

## Imports e documentação de CI — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/linear.rs` nas linhas 12 contém os avisos selecionados. Fontes/owners, spans, saída original do compilador, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/infra-imports-docs-ci-2026-10-13/before.json.gz`. A pesquisa distingue referências produtivas, imports necessários somente à suíte descendente e menções em comentários; não constitui nova medição de paridade.

### Decisão e aceitação

Remover somente os imports privados comprovadamente sem uso neste consumer; manter os reexports públicos e os imports exigidos por chamadas ou contratos. A lista exata de spans anterior à edição está no diagnóstico citado.

Manutenção interna sem mudança de assinatura pública, default, fase, fórmulas ou critérios de aceitação (fluxo contínuo ADR-0127). Não introduzir supressões de lint. Revalidar compilação de todos os targets, nomes e execução da suíte de infraestrutura, build, Clippy do workspace, Rustdoc e linhagem com resselo restrito. Não criar testes que apenas espelhem remoção de imports ou formatação de comentários.

## Cálculos numéricos dos exportadores — 2026-10-13

### Medição anterior à decisão

`03_infra/src/export/gradients/linear.rs` nas linhas [21, 22] usa métodos apontados pelo Clippy. `clippy.toml` exige alternativas sem dependência da matemática da plataforma; libm =0.2.11 já está pinado no core/Cargo.lock. Os fontes da biblioteca, consumer, owner, manifest/lock, spans, HEAD, working tree não commitado, UTC, status e diffstat estão em `00_nucleo/diagnosticos/gradientes-numericos-ci-2026-10-13/before.json.gz`.

`before-renders.json` registra 24 fontes renderizadas em PDF/SVG por Tekt e pelo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, com hashes dos binários e comandos. `before-comparison.json` mede o canvas inteiro sem máscaras ajustadas pelo erro. Existem divergências anteriores; o corpus não é prova de paridade integral. A primeira tentativa de fixture usava hsv fora do namespace color, foi rejeitada pelo parser antes de qualquer mudança produtiva e está preservada em initial-fixture-error.log; cases.json contém as entradas válidas.

### Decisão e aceitação

Usar libm::cos e libm::sin de libm 0.2.11 nos componentes f64 do eixo. Preservar centro, dimensões, sinais, sequência aritmética e todos os samplers; não transferir fórmulas para L1 nem criar assinatura pública.

Dependência direta interna de L3 fixada em =0.2.11 no manifest, reutilizando a versão já no lock. Não atualizar versões de pacotes nem adicionar I/O. Mudança interna em fluxo contínuo ADR-0127, sem contrato público, default, fase ou estratégia novos. L0 precede código e resselo é limitado aos owners envolvidos.

Validar as fórmulas com a suíte existente, exportação dos mesmos 24 documentos antes/depois e leitura raster/estrutural de PDF/SVG. Comparar a preservação local separadamente das divergências contra o pin vanilla; nenhuma diferença nova de render é aceita sem análise. Não alargar tolerâncias/limites, mudar fixtures ou supressões para esconder uma falha. Compilar todos os targets, executar a suíte infra, verificar build, Clippy, Rustdoc, formato e linhagem. Contagens e identidade de arquivos não certificam a paridade da linguagem.
