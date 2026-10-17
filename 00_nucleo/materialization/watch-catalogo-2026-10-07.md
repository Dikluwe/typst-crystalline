# Prioridade 4 — pacote de materialização do catálogo no watch

Aguardando confirmação concreta do dono. O contrato revisável está em
[watch-catalogo-2026-10-07/contract.md](watch-catalogo-2026-10-07/contract.md).
O manifesto e seu SHA-256 estão em
[watch-catalogo-2026-10-07/manifest.json](watch-catalogo-2026-10-07/manifest.json) e
[watch-catalogo-2026-10-07/manifest.sha256](watch-catalogo-2026-10-07/manifest.sha256).

Evidência e limites: [validação](../diagnosticos/watch-validacao-2026-10-07.md)
e [baseline de watch](../diagnosticos/watch-retencao-2026-10-07.md).

Escopo: sete L0 proprietários candidatos e um Núcleo pinado, sem instalar nos
prompts ativos antes da ratificação. Nova API L3 opaca FontCatalogSession::new e
SystemWorld::with_font_catalog_session; World L1 permanece igual. Watch conserva
uma geração válida de fontes e cria Sources/bytes/dependências novos por ciclo.
Compile comum conserva aquisição atual. Mudanças/recusas invalidam reuso.

O gate vem de ADR-0127 §2.1 (contrato público) e §2.2 (caminho de aquisição padrão
do watch). Este passo não é L0. Aprovar o contrato/manifesto autoriza instalar
os candidatos, testes RED→GREEN e materialização estritamente no pacote, seguida
de validação e medição. Ganho isolado de validação não fecha D10.
