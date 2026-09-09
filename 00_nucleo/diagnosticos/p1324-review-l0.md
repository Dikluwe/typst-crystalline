# P1324 — L0 ex-ante aprovado no recorte

Revisor independente `/root/p1324_review`, 2026-09-09T00:06:59.571Z;
working tree não commitada sobre HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`.
Baseline e diff/stat em `p1324-review-preflight.json`; neste instante somente
o novo L0 se somou ao diff inicial. Source owner e seis arquivos dirty prévios
foram comparados byte a byte/hash e permaneciam intactos.

O amendment lido integralmente é §P1324 de
`00_nucleo/prompts/compiler/eval/bindings/field_access.md`, SHA-256 bruto
`7c67e25db651a628ac25a67ed15d10439e6e8b5ea349e2a528e5c189b359630e`;
corpo sem exclusivamente `Hash do Código:` SHA-256
`4b85d3606fc3dc9e1390449bb856b0fcde5959b644294ba6f728c04b33f807c3`, igual
ao manifesto. A medição precede classificação e decisão; a inferência de
suficiência explicita refutação. A fonte e sondas independentes sustentam
a mensagem e span, sem deduzir intenção histórica da implementação vanilla.

P1311 foi sucedido expressamente apenas para Some ausente, inclusive a
expectativa antiga do teste. Preserva sucesso, None, categorias não nativas,
contexto text, scopes/snapshots e demais owners. Some vazio e populado,
Native/NativeWithEngine, nomes qualificados e With estão cobertos. Nenhuma
API, entidade, feature/default ou fase nova foi autorizada. Owner 1:1 e
ADR-0127 fluxo contínuo são suficientes; não há bloqueio humano novo.

Cálculo independente conforme `tekt-linter/03_infra/nucleus.rs` e
`prompt_io.rs`: Núcleo efetivo
`5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24`, pin
intacto; novo hash efetivo do prompt `740a39f5`. Hash normalizado do código
antigo `5a601982`, igual à metadata recíproca. O header antigo `15b5f7af`
ainda aguardava resselo nessa medição; não é aprovação de linhagem final.

Veredito: **L0 ex-ante aprovado no recorte**. Prosseguir com resselo e cadeia
de testes independente congelada antes de candidato, incluindo sucessão
de teste registrada. Este parecer não julga implementação nem substitui
RED→GREEN, preservação e veredito final. Regime A/B sem atestação técnica
de isolamento; revisor não editou material julgado.
