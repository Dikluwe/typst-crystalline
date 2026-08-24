# Passo 1140.26 — Constructor público `page` e fecho da série

**Estado:** executado e fechado; série P1140 permanece aberta  
**Data:** 2026-08-24  
**Continua:** P1140.25  
**Numeração:** este passo não cria subdivisões adicionais

## 1. Medição antes da decisão

No vanilla ratificado `a51e02804`:

- `lab/typst-original/crates/typst-library/src/layout/page.rs:54-503`
  declara a superfície de `PageElem`;
- `lab/typst-original/crates/typst-library/src/layout/page.rs:504-523`
  implementa `Construct`: aplica o mapa local de estilos e envolve o body com
  pagebreak fraco, marker invisível e boundary final;
- o constructor exige `body` e aceita `paper`, `width`, `height`, `flipped`,
  `margin`, `bleed`, `binding`, `columns`, `fill`, `numbering`, `supplement`,
  `number-align`, `header`, `header-ascent`, `footer`, `footer-descent`,
  `background` e `foreground`;
- `page` é função no scope global e membro de `std`; não produz um elemento
  selecionável por show rules.

Sondas adicionais no mesmo binário: `page("a5", [x])` compila;
`page(body: [x])` falha com ``the
argument `body` is positional``; `page([x], [y])` falha com ``unexpected
argument``; `std.page(width: 100pt, height: 100pt, [x])` compila.

No cristalino:

- `01_core/src/entities/elements/page_run.rs:20-42` já representa os 18 deltas
  e o body em `PageRunElem`;
- `01_core/src/compiler/layout/page_run.rs` já aplica e restaura lexicalmente
  esses deltas, inclusive runs vazios e aninhados;
- `01_core/src/compiler/eval/mod.rs:1391-2030` constrói um único scope que é
  clonado para `std`, portanto uma definição nesse scope materializa as duas
  superfícies;
- `01_core/src/compiler/stdlib/layout.rs:430-432` conserva a remoção histórica
  do antigo `native_page`, que produzia `SetPage` e não satisfaz o constructor;
- a superfície diferencial ainda reporta ausência de `page`/`std.page`.

Classificação: expor o binding e a assinatura é mudança de contrato público e
comportamento por defeito da linguagem. O gate ADR-0127 é obrigatório.

Proveniência desta medição: HEAD
`45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree não commitado,
`2026-08-24T17:39:40-03:00`; `git diff HEAD --stat` registrou
`126 files changed, 3251 insertions(+), 657 deletions(-)`.

## 2. Objetivo

Materializar `page(.., body)` como constructor público que produz exatamente
`Content::PageRun`, registrar o mesmo `Func` em `page` e `std.page`, validar
todos os argumentos reconhecidos sem aceitar e ignorar nenhum, medir a
paridade diferencial e decidir o fechamento da série P1140.

## 3. Fase A — L0 e gate

1. atualizar `00_nucleo/prompts/compiler/stdlib/layout.md`;
2. atualizar `00_nucleo/prompts/compiler/eval.md`;
3. atualizar `00_nucleo/prompts/entities/elements/page_run.md`;
4. especificar assinatura, estados de omissão e diagnósticos;
5. ressellar hashes;
6. parar para confirmação humana ADR-0127.

## 4. Fase B — RED

Escrever e executar testes que falhem para:

1. `type(page) == function` e `type(std.page) == function`;
2. identidade observável das duas rotas;
3. paper posicional opcional, body obrigatório e rejeição dos excedentes;
4. transporte de cada um dos 18 named arguments;
5. rejeição de named desconhecido e tipo inválido;
6. body vazio conservando exatamente uma página;
7. isolamento e restauração após o run;
8. runs consecutivos, multipágina e aninhados;
9. proibição dentro de block/columns;
10. `repr(page([body]))` expondo a sequência delimitada e a forma `styled`
    quando houver propriedades, conforme a morfologia vanilla medida.

Registrar o RED específico antes do GREEN.

## 5. Fase C — implementação atomizada

- criar `native_page` em `compiler/stdlib/layout.rs` como parser/constructor
  puro de argumentos já avaliados;
- reutilizar owners tipados de geometria, canvas, running matter e supplement;
- construir `Content::PageRun(Arc<PageRunElem>)`, nunca `Content::SetPage`;
- registrar a função uma vez no scope que origina global e `std`;
- manter aplicação/restauração em `compiler/layout/page_run.rs`;
- não introduzir mapa dinâmico, `dyn`, strings de dispatch internas, I/O ou
  estado global em L1;
- não alterar `#set page`, `#show page` nem o default exterior ao run.

## 6. Critérios de paridade

A aceitação é no nível da linguagem:

- disponibilidade e tipo de `page`/`std.page`;
- aceitação, rejeição e diagnóstico dos argumentos;
- morfologia e isolamento do conteúdo produzido;
- páginas observáveis, configuração lexical e restauração.

Não são gates: identidade de structs Rust, igualdade de bytes PDF, sequência
interna de operações ou texto de debug.

## 7. Rebaseline diferencial

Depois do GREEN:

1. executar as probes de `page` e `std.page` contra cristalino e vanilla
   ratificado;
2. atualizar o inventário mecânico somente com resultados medidos;
3. registrar HEAD, hora e stat do working tree;
4. classificar qualquer divergência restante como língua ou mecânica;
5. fechar P1140 apenas se não restar falha conhecida da frente de superfície
   pública coberta pela série.

## 8. Gates

1. testes específicos P1140.26;
2. testes de page-run, eval, stdlib e superfície;
3. `cargo test -p typst-core -- --test-threads=1`;
4. `cargo test -p typst-infra -- --test-threads=1`;
5. `cargo test --workspace -- --test-threads=1`;
6. `cargo build --workspace`;
7. `crystalline-lint .`;
8. `git diff --check`.

## 9. Relatório e condição de fecho

Produzir
`00_nucleo/diagnosticos/typst-p1140.26-constructor-page-fecho-serie.md` com
RED→GREEN, matriz de argumentos, probes diferenciais, contagens dos gates e
proveniência.

P1140.26 fecha quando ambas as rotas públicas constroem page-runs completos,
os diagnósticos e a restauração estão provados, o rebaseline está registrado e
os gates estão verdes. O relatório deve dizer explicitamente se a série P1140
fechou ou qual divergência mensurável ainda permanece.

## 10. Execução e decisão de fecho

O dono confirmou o gate ADR-0127 em 2026-08-24. O RED demonstrou ausência das
duas rotas públicas; o GREEN materializou `native_page`, `page`, `std.page` e
a morfologia de `repr`. O inventário e as probes foram regenerados.

P1140.26 está fechado, mas a série P1140 não fecha neste estado: o rebaseline
mantém duas ausências globais (`html`, feature-gated, e `path`) e nove frentes
de membros/aliases nas probes. Além disso, a forma função de `numbering`
documentada pelo vanilla ainda não cabe no domínio cristalino de numbering,
que aceita string/none. Ver o diagnóstico de fecho do passo.
