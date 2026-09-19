# Certificação segregada — Passo 1459

**Veredito:** VERIFIED no recorte ordinário declarado

**Produto certificado:** `b975b651fd65845cf01dfe1e6d47b250ef1d1065`

**Árvore certificada:** `769594ce12d2ef7556ac627c97e9721ff2a1bb54`

**Medição final:** `2026-09-19T02:46:39Z`

**Regime:** executado sem atestação de isolamento

## Escopo verificado

O produto P1459 elimina as decisões bilingues produtivas de suplemento
automático de heading dos consumers do compiler L1. O owner interno contém as
45 traduções base do pin vanilla ratificado; snapshot, transporte labelled e
layout delegam a ele. Referências ordinárias preservam a língua causal
transportada pelo alvo, inclusive em ordens forward e backward, sem perder a
precedência de supplement explícito/vazio, U+00A0, número ou link.

Esta certificação não transforma o preseal em atestação integral. Prompts e
runner foram fortalecidos depois dele; a emenda pós-selo registra as
identidades finais, mas não equivale a novo selo. O veredito é um ensaio
reproduzido da árvore acima, não uma alegação de isolamento.

## Identidades e campanha pública

- preseal preservado: SHA-256
  `068cbed4cc304cf9ce231519ca1fad4e32a0c69abdaf1ef7cea79c6d0202259e`;
- passo materializado: SHA-256
  `04c852d54526010f0b972cc6dec8a6dda097919ff51dc0cde3dfda2e2c6982aa`;
- emenda pós-selo: SHA-256
  `a7a176f94c41891f476315e52a3f902e88756d7a0a3598d852c9027d84f04d26`;
- vanilla ratificado: SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino release reconstruído em `b975b651f`: SHA-256
  `bc51055a97e85c1fccdcae8f51c0d21fe928b58eb0d6557eaf68ca3f3a7e68b5`;
- fixture: `lab/parity/p1459_reference_supplements_de.typ`;
- forward, reverse e forward-repeat: os seis processos terminaram com sucesso
  e cada par vanilla/cristalino teve diff de texto extraído igual a zero;
- os seis textos extraídos tiveram SHA-256
  `25a41e975e2adb0bc19925124535f9c078612a5c88636d8c3f00baaf448ef918`.

## Testes e ataques

- `cargo test -p typst-core p1459_ --quiet`: 3 aprovados, 0 falhas;
- `cargo test --workspace --quiet`: 5919 aprovados, 8 falhas, 0 falhas P1459;
- as oito falhas são as mesmas do baseline: P1292 repr, quatro casos
  P981/P1132R de `lr`, align point e duas contagens/bindings HTML;
- runner estático/de modelo reproduzido na árvore commitada: 14 ataques
  rejeitados, 0 sobreviventes, score discriminatório 1,0;
- os 14 ataques acima **não** são apresentados como mutation testing
  compilado;
- `cargo-mutants 27.1.0` focal, registado no produto: 3 mutantes compilados,
  3 apanhados, 0 missed/timeout/unviable, score 1,0;
- outcomes brutos dessa campanha: SHA-256
  `fe341704b7b637d8fd18163b6fb1133a25f4cabe3ab9374e74c88efe40399dd5`.

## Gates da árvore certificada

- release e `cargo build --workspace`: sucesso;
- `cargo fmt --all -- --check`: sucesso;
- `git diff --check`: sucesso;
- todos os JSON P1459: válidos;
- runner estático: reprodução byte a byte do resultado versionado;
- `crystalline-lint .`: exit 0, sem violations e sem bloqueio V5/V15/V26;
- working tree limpa antes da criação deste certificado.

A revisão final por verificador separado, sem atestação de isolamento,
conferiu hashes, pares L0↔consumer, campanha pública e limites do veredito,
sem encontrar bloqueio funcional ou arquitetural no recorte.

## Não verificado e não fechado

1. `#show heading: set text(lang: "de")` permanece divergente: vanilla
   produz `Abschnitt`, cristalino produz `Section`. Corrigir isso altera a
   fronteira pré-show/render e exige L0 próprio mais confirmação ADR-0127.
2. `02_shell/src/cli.rs` ainda contém o literal produtivo
   `Content::text("Section")` no serializer raw de L2, dívida já registrada em
   `shell/cli.md` P1307-R5. Este certificado não a chama de eliminada nem
   promove o owner interno a owner global.

Logo, **VERIFIED** significa apenas: owner e transporte causal ordinário de
heading no compiler L1 estão materializados e reproduzidos. Não significa
paridade show-set universal, eliminação global de todo hardcode ou cadeia
integralmente selada/isolada.
