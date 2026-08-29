# P1246 — proposta de contrato para clip SVG

**Veredito final:** `PASS — SEGREGATED_AND_CERTIFIED`.

## Medição

O cristalino transporta `clip_mask: Option<ShapeKind<Pt>>`, dimensões internas,
posição e transform em `FrameItem::Group`. O exporter recebe esses dados em
`03_infra/src/export/svg.rs:1086-1117`, mas ignora a máscara. O vanilla
ratificado cria um `clipPath`, referencia-o no grupo e conserva o recorte no
espaço local (`lab/typst-original/crates/typst-svg/src/lib.rs:326-359,421-433`).

O carrier atual não transporta fill-rule para clip nem máscara alpha. A proposta
L0 cobre Rect, RoundedRect, Ellipse e Path não-zero representáveis; exige base
local única, nesting por interseção, grafo resolvido e deduplicação com chave
semântica completa. Even-odd dependente, linha sem área e alpha mask permanecem
não positivos.

## Evidência materializada

- 12 cláusulas de contrato em `p1246-clip-contract-draft.tsv`;
- 10 oráculos em `p1246-clip-oracles-draft.tsv`;
- 12 ataques em `p1246-clip-attacks-draft.tsv`.

Todos são rascunhos `NOT_EXECUTED`. Não houve autoria segregada, campanha de
mutação, mutation score, preseal ou código produtivo. Duas execuções do auditor
produziram resumo byte-idêntico.

## Gate

Implementar clip altera o comportamento SVG padrão. O dono aprovou o contrato
em 2026-08-28; o próximo gate é o preseal segregado. O V5 adicional do consumer
SVG é esperado neste intervalo e não deve ser ressellado.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`: os rascunhos atuais continuam ensaio
diagnóstico, não protocolo completo atestado.

## Materialização v2

Os rascunhos foram substituídos por contrato, oráculos e ataques canônicos sob
autoridades separadas. O preseal v2 passou 12/12 mutações, score 1.0. A
implementação e os testes foram escritos somente depois do selo; o verificador
final confirmou 4/4 testes P1246, 23/23 testes SVG, build workspace, V15/V26,
V5 do owner e `git diff --check`. O certificado v2 limita a alegação a clips
geométricos locais; even-odd sem carrier e máscara alpha continuam `Unknown`.
