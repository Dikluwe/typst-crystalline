# P1334 — recibo B dos resultados CLI públicos

Veredito: **PASS**, limitado ao corpus congelado. Auditoria UTC `2026-09-09T16:18:50.583380+00:00`.

Manifesto SHA-256 `2cac9aab9dc8e911e2a14e932515db3efd5be64b10b1089d06caac0b310be1da`. Freeze R1 SHA-256 `684143d5fdbb4005e5d641f772e603f48da1c32c928840c89873bbcff5fa50b5`. Auditoria reproduzível: `python3 00_nucleo/diagnosticos/p1334-ab-audit-public.py` (evidências imutáveis; execução subsequente verifica e recusa sobrescrever). JSON detalhado: `p1334-ab-public-audit.json`, SHA-256 `a54393293fca5675e64375055ecd7d7d1b569e8ec5ffcecc9d12cbf386ff4f36`.

As 828 células distintas passaram nas três ordens: 2484 comparações integrais de exit/stdout/stderr, zero diferenças e zero Unknown. Suite principal: 816 células por rodada. Suplemento real cross-source: 12 por rodada. Resultados normal/repetido/invertido coincidem por caso e perfil, com reversão efetiva da ordem de expressões.

As 712 células históricas estão presentes: 56 migram somente pelos guards autorizados no ledger congelado, e 656 preservam exatamente as expectativas anteriores. Conferidos 47 hashes declarados de inputs/artefatos e as três normas L0 sem a linha Hash do Código. As observações BASE/VANILLA nos seis recibos são cópias integrais dos caches congelados, incluindo argv e UTC originais.

Os seis recibos identificam o candidato `/tmp/p1334-target.Ujq0xU/release/typst`, SHA-256 `11e3164fa509030cc78dc048d5bb4f2426348e32a24edc7cd2f320c704e6ef61`. Cada comando candidato registra argv e UTC posterior ao freeze. O estado inicial de referência é HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado, com diff/stat e inventário preservados no baseline público pinado; esta auditoria não infere a identidade do candidato a partir desse HEAD.

| Recibo público | Células | SHA-256 |
|---|---:|---|
| p1334-ab-cli-normal.json | 816 | `eeb317721dfe2ecc103a85e30992913dc11498c6eca41326e511c0772e804840` |
| p1334-ab-cli-repeat.json | 816 | `669e134d67bdd359ba9402d256bd681f0c92d07aea7e9ee877dd1024eeb3c68c` |
| p1334-ab-cli-reverse.json | 816 | `7028d7f3cc0a05f532abe8f4319db289ff1653f487579d7d6b00716bce5b014a` |
| p1334-ab-cross-cli-normal.json | 12 | `4a1888585c6bcd492c2c0461c130b779d8cf137941103d92ba43827b0b97c406` |
| p1334-ab-cross-cli-repeat.json | 12 | `c049bf700e34c2702b5ba3d2374210df2108235b775d3606c31579d19eded46a` |
| p1334-ab-cross-cli-reverse.json | 12 | `afa9f1e055fd79783360e03aceb03780404233543bd90fa0e2948c82155b4f4e` |

Regime A/B sem atestação técnica de isolamento. Não há selo de refinamento nem alegação de paridade geral. B leu somente os outputs públicos autorizados e inputs/freezes, sem ler runtime produtivo ou recibos privados RED/GREEN. A identidade do executável é a declarada consistentemente nos recibos; B não executou nem inspecionou seu conteúdo. Este recibo não certifica gates nativos, build, workspace ou lint. Fixtures Some históricos incoerentes continuam classificados apenas como robustez local fora do domínio causal.
