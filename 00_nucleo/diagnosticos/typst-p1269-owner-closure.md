# P1269 — fechamento do retorno ao owner para iniciar P1270

**Estado medido:** working tree não commitado sobre
`3bc6f5a683cd2df9b4281f1debf46f1b052ad463`, em
`2026-08-29T08:10:27-03:00`. O diff produtivo/L0 no momento da medição contém
120 inserções e 39 remoções nos seis owners listados abaixo. Os receipts desta
secção foram produzidos pela mesma working tree após rebuild do binário debug.

Inventário exato do estado não commitado: os três L0
`compiler/eval/tests.md`, `compiler/stdlib/gradients.md` e
`entities/gradient.md`; os três consumers `compiler/eval/tests.rs`,
`compiler/stdlib/gradients.rs` e `entities/gradient.rs`; este relatório; e os
quinze TSVs `p1269-owner-{attacks,coincidence,determinism,fixture-gates,
input-manifest,interval-cost,invalid-domain,mutants,numeric-failure-freeze,
numeric,opaque,pairs,public-stops,role-capabilities,route}.tsv`.

## Veredito

**FECHADO PARA P1270.** A reexecução segregada devolveu
`Numeric-Failures-Frozen`, rota `P1270`: nos 96 fixtures, todas as obrigações
não numéricas ficaram preservadas. Restam exclusivamente as 28 violações G05
já congeladas em 24 fixtures para P1270.

Receipt de rota: `p1269-owner-route.tsv`,
sha256 `56c5ebe68d31c15f57e3920766687121105663aa2e6f544b6e3c1c127c75571a`.

## Obrigação e causa

P1269 devolveu `S14-linear-oklab` e `S14-linear-linear-rgb` ao owner porque
`sample(g.stops().at(i).at(1))` selecionava o último stop numa coincidência
interna não diádica. O sampling Linear preservava fronteiras canônicas em
`f64`, enquanto `stops()` expunha a identidade pública histórica via `f32`
alargado. A razão pública ficava numericamente à direita de `1/3` e `2/3`.

A implementação final mantém intacto o carrier de `stops()`. Para Linear,
somente uma igualdade exata com um offset público efetivo é mapeada para a
fronteira `Ratio(f64)` correspondente antes de `sample_precise`; um epsilon
positivo não é normalizado e continua no ramo direito. `samples` reutiliza o
mesmo caminho.

## L0, teste e implementação

- L0 atualizado e ressellado: `compiler/stdlib/gradients.md`,
  `entities/gradient.md` e `compiler/eval/tests.md`.
- RED observado antes do código: coincidências Oklab `0/2`, forma estática
  falsa, `samples` falso; coincidências Linear RGB `0/2`.
- GREEN final: o teste `p1269_linear_sample_coincidencia_publica` passa em
  Oklab e Linear RGB para `1/3`, `2/3`, epsilon à direita, forma estática e
  `samples`.
- Código: `effective_offsets_precise` centraliza a resolução `f64` sem mudar
  os métodos Rust históricos; `sample_gradient` reconhece apenas identidades
  públicas exatas.

## Ataques e refinamento

O ataque A02 matou a primeira solução, que expunha offsets Linear precisos em
`stops()`: embora fechasse G04A, reabriu G04B, G08 e G10 nos pares Linear. O
refinamento final preservou o carrier público e sobreviveu à matriz completa.
O recibo `p1269-owner-attacks.tsv` registra 2/2 mutações rejeitadas e a
candidata final sobrevivente. Os 24 mutantes do oráculo selado continuaram
24/24 mortos; dez casos opacos permaneceram excluídos de sucesso.

## Reexecução final

- rota: `P1270`; veredito `Numeric-Failures-Frozen`;
- fixtures: 96;
- falhas numéricas congeladas: 24 fixtures, 28 métricas G05;
- coincidências exatas e epsilon: preservadas em toda a população aplicável;
- domínio inválido: 28/28 rejeitado;
- determinismo direto/inverso/repetição: 192/192;
- mutation score selado: 24/24;
- controle: `Preserved`;
- sem promoção produtiva, alegação de equivalência ou resselo de P1268.

Artefatos finais têm prefixo `p1269-owner-`; os artefatos selados de P1268 e
P1269 não foram alterados. O freeze que inicia P1270 está em
`p1269-owner-numeric-failure-freeze.tsv`, sha256
`cb68244dbc3d4469c2e75a0af66f5c0d7885a0ee6d137c89710621e377dce020`.

## Validação

- `cargo fmt --all -- --check`: passou;
- teste direcionado RED→GREEN: passou no estado final;
- `cargo test --workspace`: passou (exit 0);
- `cargo build --workspace`: passou (exit 0; warnings preexistentes);
- `crystalline-lint --checks v15,v26 .`: zero violações antes do resselo;
- `crystalline-lint --fix-hashes .`: três owners ressellados, zero drift;
- `crystalline-lint .`: exit 0, sem violações bloqueantes; avisos V16/V19/V20
  preexistentes permanecem fora deste owner;
- `git diff --check`: passou.

## Atestação de segregação

O regime foi causal/processual, conforme
`p1269-owner-role-capabilities.tsv`. `/root` e os executores compartilham o
mesmo workspace e capacidade de leitura; portanto não há alegação de isolamento
técnico. O manifesto `p1269-owner-input-manifest.tsv` congelou os oráculos como
read-only antes da implementação, e a reexecução final adjudicou a candidata
sem modificar esses inputs.
