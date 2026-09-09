# P1336 — ressalva do resselo recíproco

Manifesto `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`. Veredicto: `Nothing to fix` não satisfaz a validação bidirecional neste estado; metadata B deve ser corrigida e novamente validada, sem mudar intenção.

O candidato SHA `2fa98b5a5d5c5ed05a5d6a2541a64a42452279b172a6ba9468edebbad614321d` ainda tinha `Hash do Código: 3a6ed279` no L0. O recibo do operador `p1336-reseal-final-dry-run.json` registra `Nothing to fix`. A regra ADR-0129 exige coerência bidirecional, portanto não se pode usar essa mensagem para fechar B.

Leitura independente em `/repos/Antigravity/tekt-linter`, HEAD `49f46885fd03f753e9f1cf37e271d6baf30ed725`: `02_shell/fix_hashes.rs:295–305` forma o plano apenas a partir de V5; `04_wiring/main.rs:807` invoca esse plano. `03_infra/hash_writer.rs:16–19` calcula B por SHA-256 dos bytes após remover a linha canônica `//! @prompt-hash ` do header, truncando em oito hexadecimais. `03_infra/prompt_io.rs:151,166–234` remove a linha com seu newline e preserva todos os demais bytes, inclusive newline final. Essa fonte explica o resultado observado; não foi demonstrada reprodução bit a bit do binário instalado a partir desse HEAD.

Em `2026-09-09T18:18:57.198Z`, reviewer calculou independentemente B do candidato: SHA completo dos bytes canônicos `4e4810f397ba2d8df592b1339de55386093f159c47f72bce0a34875b116922b9`, logo metadata correta `4e4810f3`. Foram removidos apenas os 26 bytes da linha de header; nenhum trim geral ou remoção de strings interiores foi usado. Proveniência do produto: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, árvore não commitada/diff-stat em `p1336-review-candidate-audit.json` e fonte C integral no recibo candidato.

O manifesto permite exclusivamente resselo mecânico de `Hash do Código` e `@prompt-hash`, com recibos das revisões integrais. Portanto a correção manual de B pelo operador está no escopo autorizado; o reviewer não altera L0 nem source. A seguir devem ser conferidos B, A efetivo com núcleo, corpo normativo congelado e V5/V15/V26. Nenhuma alteração ou reparo global do linter é autorizada por este parecer.
