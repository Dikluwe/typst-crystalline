# P1287 — receipt de contrato e congelamento

**Estado:** `CONTRACT_FROZEN_BEFORE_ORACLES`  
**Contrato:** `P1287-GLOBAL-PARITY-v1`  
**Manifesto:** `00_nucleo/diagnosticos/p1287-manifest.json`  
**SHA-256 do manifesto:** `5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b`  
**Congelado em:** `2026-08-30T22:34:42-03:00`  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`  
**Árvore:** working tree não commitida e compartilhada.

## Papel e atestação

O executor `/root/contrato_p1287` atuou exclusivamente como Autor de
Manifesto/Contrato sob o protocolo completo da skill
`tekt-materializacao-segregada`. Leu somente as entradas autorizadas e não leu
código candidato em `01_core`–`04_wiring`, não executou
`target/debug/typst` nem `target/release/typst` e não escreveu L0, código,
testes, oráculos, baselines, ataques ou veredito.

A escrita ficou limitada a este receipt e ao manifesto acima. A segregação é
por papel, capacidade, ordem causal e hashes; o checkout compartilhado impede
alegação de isolamento ambiental forte. A inexistência de ADR específica de
materialização segregada foi transportada do receipt independente P1286.

## Entradas pinadas

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| Passo 1287 | `eb0622dd0fe195dfb4ed0dcdae079d0bc8c0162837531358a3598b7c9f0e32a8` |
| baseline receipt P1287 | `3bbba47235e4865bb3c9e788bb709215dd0a53297ee587d4334a01ca42126cd6` |
| verification receipt P1286 | `d02c7cdd0fa7579b7489b3862caa74ec847aaa6116dcb2d30dcc77a8e1c1fd8f` |
| relatório P1281 | `6e33212c9ac5de11c3e0dd37a073dea745504f19dc3e0b33d8e4f192ba9c5555` |
| promoção P1280 | `42d245b5606e9cc605b3965723862828a693c16c0117201a0e88e5de344bab1c` |
| certificado P1246 v2 | `195e8413bebd9a81850bdb4f2d131df02310ffcc76c7f7cac227b6588ab6278a` |
| certificado P1247 | `e7356842f34ca8933a0d38eee851dd3061d1c26419f26c001883756e34f74248` |
| certificado P1248 | `b7683baff761b46bf8074639b8749953ee172ed51c07c3d2a4b9addb8d39438d` |
| certificado P1249 | `b6e29b625008f796e9c770f8cfa11ebb0536a03f2dd95c784ec465fea2e20f6e` |
| certificado P1254 | `c96118840781518373d7d29a298420b21844cc5f50e75ee9e6709beeccc6b9f6` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| papéis/capacidades | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| artefactos/gates | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| `/usr/local/bin/typst` transportado do baseline | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

## Decisões congeladas

- `Unknown` nunca é sucesso; `NotApplicable` exige irrelevância estrutural
  declarada e não pode esconder capacidade obrigatória.
- PNG usa igualdade RGBA exata por defeito. A única exceção possível é E01,
  com máscara independente e pré-candidata de no máximo 11 coordenadas,
  valores RGBA permitidos por coordenada e causa de borda raster comprovada;
  não existe tolerância geral.
- PDF separa texto, páginas/boxes, fontes e acessibilidade; identidade nominal
  de fonte não substitui os outros eixos.
- SVG cobre gradientes, tiling, links/bundle, imagens opacas/dependências,
  SVGZ, glyph/fontes, alfa, máscaras e clipping/even-odd sem promover
  disponibilidade unilateral.
- Corpus C01–C12, exportadores E01–E11 e inventários padrão/HTML são o mínimo;
  suficiência é checklist observável, não percentagem da linguagem.
- `PARIDADE PARCIAL` exige ao menos uma diferença de linguagem reproduzível e
  com proveniência válida. `PARIDADE DEMONSTRADA` exige zero divergência de
  linguagem, cobertura suficiente, identidade do baseline provada e harness/
  proveniência decisórios íntegros. Nos restantes casos, `INCONCLUSIVO`.

## Gate causal

O manifesto foi materializado e validado como JSON antes deste receipt. O hash
acima o congela **antes de qualquer Autor de Oráculos**. Esse autor só pode
receber manifesto e receipt já pinados; qualquer alteração posterior de bytes
invalida oráculos, ataques, selo, candidato e veredito derivados.

Nenhum gate funcional, mutação, implementação ou veredito foi executado por
este papel. Resultado proporcional: **contrato segregado e congelado por
capacidade e artefactos, sem atestação de isolamento forte do host**.
