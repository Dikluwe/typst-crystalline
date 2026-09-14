# Passo 1359 — `--pdf-tagged` tri-state

## Objetivo

Fechar o recorte owner-confirmed seguinte ao P1358: aceitar e transportar
`--pdf-tagged[=true|=false]` com a semântica pública do vanilla ratificado,
reutilizando o eixo `PdfTags` já materializado. Este passo não implementa
`--pretty`.

## Entradas congeladas

- preflight: `00_nucleo/diagnosticos/p1359-preflight.md`;
- baseline: vanilla ratificado
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- owners L0: `00_nucleo/prompts/shell/cli.md` e
  `00_nucleo/prompts/wiring.md`;
- consumers: `02_shell/src/cli.rs` e `04_wiring/src/main.rs`;
- testes de processo podem viver em `04_wiring/tests/cli.rs`, sem adquirir
  ownership produtivo.

## Contrato observável

1. `compile`, `watch` e `w` aceitam `--pdf-tagged`,
   `--pdf-tagged=true` e `--pdf-tagged=false`.
2. Bare equivale a `true`; valor separado sem `=` é rejeitado.
3. Ausência e `true` produzem `PdfTags::Enabled`; `false` produz
   `PdfTags::Disabled`.
4. `--pdf-tagged` explícito vence o legado `--no-pdf-tags`; sem valor
   explícito, o legado continua desabilitando tags.
5. A opção não altera `--compact` e não muda PNG, SVG ou HTML.
6. PDFs enabled contêm estrutura lógica; disabled não contêm
   `/StructTreeRoot` nem operadores de marcação do eixo tagueado.

### Segurança do observador

Os casos `watch` e `w` são exclusivamente de parsing e devem usar
`Args::try_parse_from` no mesmo processo. É proibido lançá-los como subprocesso
de duração aberta ou usar captura bloqueante de stdout/stderr. A matriz de
processo executa somente `compile`; `watch --help` é permitido por terminar.
Esta regra corrige a causa raiz documentada em
`diagnosticos/p1359-memory-root-cause.md`.

## Fora do recorte

- `--pretty` (P1360);
- seleção de páginas e suas interações com tags;
- standards PDF/A e PDF/UA;
- remoção ou warning de depreciação de `--no-pdf-tags`;
- mudanças nos exportadores, entidades ou fases do pipeline.

## Ordem de materialização

1. atualizar os dois L0 owners e ressellar linhagem;
2. congelar manifesto, contrato, oráculos positivos/opacos e mutantes;
3. exigir gate discriminatório 1.0 e determinismo antes de código;
4. escrever testes RED independentes;
5. implementar apenas L2 e a tradução thin em L4;
6. executar matriz bilateral nos quatro perfis, testes, build, lint e
   `git diff --check`;
7. emitir veredito restrito ao recorte `--pdf-tagged`.

## Gate ADR-0127

Mudança de contrato público: `CompileIntent` ganha um campo tri-state e a CLI
ganha uma flag. Gate confirmado pelo dono em 2026-09-12 pela instrução
explícita “Escreva o passo e implemente”.

## Regime

Protocolo completo de materialização segregada, executado sem atestação de
isolamento. `Unknown` nunca recebe crédito; primeira selagem exige
`mutation_score = 1.0`.
