# Passo 1140.24 — Running matter de página

**Estado:** executado — fechado  
**Data:** 2026-08-24  
**Continua:** P1140.23  
**Numeração:** este passo não cria subdivisões adicionais

## 1. Medição antes da decisão

No vanilla ratificado:

- `lab/typst-original/crates/typst-library/src/layout/page.rs:315-450`
  define `numbering`, `number-align`, `header`, `header-ascent`, `footer` e
  `footer-descent`;
- `lab/typst-original/crates/typst-layout/src/pages/run.rs:141-185` resolve os
  offsets contra as margens, cria a numeração marginal e dá precedência ao
  header/footer explícito;
- `lab/typst-original/crates/typst-layout/src/pages/run.rs:226-233` dimensiona
  as regiões marginais e preserva a numeração no snapshot da página.

No cristalino:

- `01_core/src/entities/layout_types.rs:700-760` possui somente
  `numbering: Option<EcoString>` em `PageConfig` e `Page`;
- `01_core/src/entities/elements/page_run.rs:21-39` transporta `numbering`, mas
  não os outros cinco argumentos;
- `01_core/src/compiler/layout/page_run.rs:45-77` só aplica/restaura o padrão;
- não há owner para composição marginal de página.

Conclusão: é divergência de semântica e morfologia da linguagem, não de
mecânica Rust. Acrescentar os campos altera contratos públicos e o
comportamento padrão, portanto a parada ADR-0127 é obrigatória.

## 2. Objetivo

Implementar de ponta a ponta:

- `numbering` como `none`, padrão ou função suportada pelo domínio vigente;
- `number-align` horizontal + vertical top/bottom, rejeitando horizon;
- `header` e `footer` com estados distintos auto, none e content;
- `header-ascent` e `footer-descent` como comprimentos relativos às margens;
- numeração automática na região marginal correta;
- precedência de header/footer explícito sobre a numeração automática;
- restauração lexical integral em page-runs aninhados.

Não expor ainda `page`/`std.page`; isso pertence à frente pública final.

## 3. Fase A — L0 e gate

1. Guardar `00_nucleo/prompts/entities/page_running.md`.
2. Auditar e, durante a implementação, atualizar primeiro os L0 de
   `content`, `layout_types`, `page_run`, eval e layout.
3. Ressellar hashes.
4. Parar e obter confirmação humana por se tratar de contrato público e
   comportamento por defeito.

## 4. Fase B — RED

Escrever testes que falhem para:

1. transporte distinto de omitido/auto/none/content;
2. defaults de alinhamento center+bottom e offsets de 30%;
3. rejeição de alinhamento vertical horizon;
4. numeração no header quando top e no footer quando bottom;
5. supressão da numeração por header/footer explícito correspondente;
6. resolução percentual de ascent/descent contra a margem correta;
7. páginas físicas consecutivas e page-runs vazios, multipágina e aninhados;
8. restauração LIFO dos seis argumentos;
9. ausência do running matter em plain text, query, introspecção e estrutura
   acessível do body, sem perder o texto visual exportado.

Registrar RED antes do GREEN.

## 5. Fase C — implementação

- criar o domínio atomizado em `entities/page_running.rs`;
- ampliar `PageConfig`, `Page`, `Content::SetPage` e `PageRunElem`;
- transportar clone, hash, igualdade, map_content/map_text e repr;
- aceitar e validar os named arguments em eval sem ignorar nenhum;
- criar `compiler/layout/page_running.rs` como owner forma B;
- resolver a numeração por página lógica e total quando disponível;
- compor header/body/footer sem contaminar os observáveis semânticos do body;
- manter exporters consumidores do snapshot final, sem lógica de negócio.

## 6. Gates

Executar, nesta ordem:

1. testes específicos P1140.24;
2. `cargo test -p typst-core`;
3. `cargo test -p typst-infra -- --test-threads=1`;
4. `cargo test --workspace -- --test-threads=1`;
5. `cargo build --workspace`;
6. `crystalline-lint .`;
7. `git diff --check`.

Produzir `00_nucleo/diagnosticos/typst-p1140.24-running-matter-page.md` com
HEAD, árvore não commitada, hora, stat, matriz de argumentos, RED→GREEN e
resultados dos gates.

## 7. Condição de fecho

P1140.24 fecha somente quando os seis argumentos alcançarem o snapshot e a
saída visual, a precedência e restauração estiverem provadas e todos os gates
estiverem verdes. O passo seguinte será P1140.25 (`supplement` e referências).
