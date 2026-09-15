# Passo 1410 — limitar cobertura tipográfica ao catálogo já medido

**Estado:** concluído.

## Objetivo

Corrigir o estouro de memória exposto por P1409 sem elevar o teto: impedir que
a primeira consulta de fallback mapeie todos os ficheiros de fontes instaladas.

## Proveniência congelada

- HEAD inicial: `425e3cb26fc3569a7aea1ecddd13722d6cf25dcb`.
- Working tree inicial: limpo antes das cinco fixtures P1410.
- Primeira medição: `2026-09-15T16:07:20-03:00`; redução causal adicional na
  mesma working tree não commitada.
- Vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`.
- Binário cristalino inicial: `target/release/typst`, SHA-256
  `21edbc26682f290db91d85455ce96bb6b372edcb1f9edcac4bf8d38fad38059d`.
- L0 vigente lido: `infra/fontdb.md` e `infra/system-world.md`; consumers:
  `03_infra/src/fontdb.rs` e `03_infra/src/world.rs`.

## Medição antes da decisão

Todos os probes receberam um thread e timeout de 15 s. Os REDs usaram
`RLIMIT_AS=512 MiB`; a inspeção posterior sem esse limite só foi usada para
medir `/proc/<pid>/status` e `/proc/<pid>/maps`, sem repetir a matriz ampla.

| Documento/operação | Resultado inicial |
|---|---|
| simples + expressão | termina |
| heading + expressão | termina |
| outline + expressão constante | SIGABRT; alocação de 210.496 bytes |
| outline + query | SIGABRT; alocação de 210.496 bytes |
| raw simples | SIGABRT; alocação de 103.488 bytes |
| raw com `show raw.line` | SIGABRT; alocação de 107.712 bytes |

Uma reprodução de outline + expressão sob 2 GiB também abortou, embora o RSS
máximo ficasse em 163.316 KiB. A hipótese inicial de bibliotecas stdlib
redundantes foi testada e refutada: compartilhar o base manteve ambos os REDs e
fez o outline compilado alcançar `VmSize` de 4.821.924 KiB. A mudança foi
integralmente retirada antes da decisão abaixo.

Com o binário sem limite artificial, outline e raw terminaram com RSS máximo de
276.496 KiB e 274.792 KiB. Durante outline, a amostragem capturou um único
thread, `VmSize` de 4.821.924 KiB e 945 mapeamentos de fontes: 475 em
`/usr/share/fonts` (754.516 KiB) e 470 em outros paths, sobretudo
`~/.local/share/fonts` (aproximadamente 3,7 GiB). O código causal é:

- `03_infra/src/world.rs:candidates_for_char` percorre todos os slots cuja
  coverage está vazia e chama `source_bytes()`;
- `03_infra/src/fonts.rs:font_info_from_bytes` deixa toda coverage vazia;
- `03_infra/src/fontdb.rs:load_system_fonts` já possui os bytes de cada face via
  `with_face_data`, mas descarta essa oportunidade após extrair metadados.

O enquadramento “bug no teste” é refutado: o corpus apenas torna observável uma
reserva virtual proporcional ao catálogo de fontes do host.

## Decisão e contrato

1. `load_system_fonts` e `load_fonts_from_dir` extraem `FontInfo` e coverage
   exacta dentro da mesma chamada `fontdb::Database::with_face_data`.
2. `SystemWorld::candidates_for_char` usa primeiro a coverage não vazia do
   `FontBook`; não abre o `FontSlot` nesse caso.
3. Slots sem coverage pré-calculada preservam o caminho lazy P938 e seu cache.
4. A seleção, a exactidão Unicode, a ordem e os índices emparelhados permanecem
   iguais. Não há cache global, mudança de trait, default ou fase de pipeline.
5. Trata-se de correção interna de recurso/paridade; segue fluxo contínuo pela
   ADR-0127.

## RED → GREEN e ataques

- RED integrado: outline + expressão e raw simples abortam sob 512 MiB;
- GREEN focal: ambos terminam sob o mesmo teto, um thread e 15 s;
- controle: simples, heading e outline compilado continuam funcionais;
- bilateral: a campanha P1409 alcança os observáveis sem OOM; divergências de
  língua permanecem abertas, nunca são promovidas por esta correção de recurso;
- ataque estrutural: fonte de sistema válida devolvida pelo loader contém
  coverage de `A` antes de qualquer `FontSlot::get()`;
- mutantes rejeitados: manter coverage vazia no loader, ignorar a coverage do
  `FontBook`, mapear todos os slots antes de filtrar ou elevar o teto.

## Política de execução

- probes: 512 MiB, `RAYON_NUM_THREADS=1`, timeout 15 s e saída limitada;
- build/teste focal: 2 GiB, um job, incremental/debug desligados, 512 codegen
  units e linker gold;
- proibidos: harness monolítico de `typst-core`, `cargo check --tests`, suíte
  integral P1400 e repetição sem nova hipótese;
- sem isolamento de autoridades, o veredito será “executado sem atestação de
  isolamento”.

## Execução e veredito

- O primeiro build release terminou em 3 min 11 s sob 2 GiB, um job,
  incremental/debug desligados e 512 codegen units. Depois do resselo, o build
  final terminou em 3 min 07 s sob a mesma política; o binário final tem
  SHA-256
  `6610e74ee29f78b6f6969b3d449e8bc030209127af92ec2cde97df2e486c9daf`.
- Os dois REDs focais ficaram GREEN sob 512 MiB: outline + expressão devolveu
  `ok`; raw simples gerou PDF; ambos terminaram com código zero e stderr vazio.
- A amostragem pós-correção mediu `VmSize` máximo de 91.072 KiB para outline +
  expressão e 91.080 KiB para raw, contra 4.821.924 KiB antes. RSS amostrado
  ficou em 41.764 KiB e 38.396 KiB; um thread em ambos.
- Os controles simples + expressão, heading + expressão e outline compilado
  terminaram com código zero sob 512 MiB. `show raw.line` chegou ao diagnóstico
  normal `function raw does not contain field line`, código 1, sem sinal/OOM.
- O teste focal `fontdb::tests::p1410_info_de_sistema_conserva_coverage_exacta`
  passou sob 2 GiB; 1 passou, 0 falhou e 946 foram filtrados.
- A campanha bilateral P1409 refeita em `/tmp` executou 84 processos sob
  512 MiB, máximo de 42.636 KiB RSS, swap zero e resultado do runner `PASS`.
  `par.line` e `raw.line` agora são `ABSENT`; os quatro campos de
  `outline.entry` deixam de ser OOM, mas permanecem `UNKNOWN` por
  `NON_BILATERAL_EVIDENCE`. O resultado compacto temporário tem SHA-256
  `9df9f7d10aaf0f11c96ecd40f6e8912326376c8bd59927a53ab1f15f0aff0a36`.
- Raw simples também revelou divergência morfológica independente: o texto
  extraído cristalino foi `xy` numa linha, enquanto o vanila preservou `x` e
  `y` em linhas separadas. P1410 não mascara nem fecha essa lacuna.
- O `cargo fmt --all --check`, `git diff --check` e `crystalline-lint .`
  terminaram com código zero; o linter reportou zero violações e conservou
  somente warnings preexistentes. Após o build final, os dois probes focais
  foram repetidos uma vez e mantiveram código zero e stderr vazio.
- Estado medido em `2026-09-15T16:32:28-03:00`, sobre o HEAD inicial e working
  tree não commitada contendo os dois L0, seus dois consumers, este passo e as
  cinco fixtures P1410. Os experimentos brutos e PDFs ficaram em `/tmp`.

**Veredito:** `PASS` para eliminar o crescimento virtual proporcional ao
catálogo de fontes e para tornar o recorte executável no teto original.
As divergências semânticas/morfológicas reveladas continuam abertas. Protocolo
Tekt executado sem atestação de isolamento.
