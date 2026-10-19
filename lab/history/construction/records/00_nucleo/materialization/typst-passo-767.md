---
# P767 — L0: `Content::Shape` como bloco que quebra parágrafo + arqueologia da decisão original

> **Passo:** 767
> **Data:** 2026-07-15
> **Foco:** P763h confirmou, por leitura directa do código-fonte do vanilla, que **todas** as primitivas de desenho (`line`, `curve`, `polygon`, `rect`, `square`, `circle`, `ellipse`) são `BlockElem` — quebram parágrafo por construção. No cristalino, todas passam por `Content::Shape`/`ShapeElem`, mas continuam dentro do fluxo contínuo de texto, produzindo divergências de AE 1864–7070 em qualquer documento que misture texto e forma. Isto sai do âmbito de P763 (download de pacotes) — é uma questão do realizador/layout de parágrafo, não de `cetz` especificamente. Este passo escreve o L0 correspondente e regista onde a decisão original (tratar formas como fluxo contínuo) foi tomada, para a correcção ficar rastreável à sua origem, não solta.
> **Tipo:** L0 + Sonda arqueológica. Sem implementação — a implementação é um passo seguinte, fora da numeração P763.
> **Tamanho:** M para este passo (arqueologia + L0); a implementação em si (passo seguinte) é potencialmente L/XL, por mexer no realizador.
> **ADR-0108 EM VIGOR** — confirmar a origem por leitura directa (histórico + código), não por suposição. **Regra 1 do handoff** — decisão nova regista-se explicitamente. **Regra 9 do handoff ("Registo, não reconstrução")** — não reescrever retroactivamente os passos antigos onde a decisão foi tomada; registar como linha numa tabela `passo → commit → relatório`, mantendo a sequência auditável tal como aconteceu.
> **Dependências:** P763h (achado do padrão sistémico, decisão de scope-out registada lá).

---

## Parte A — Arqueologia: onde a decisão foi tomada

Já há um indício forte, encontrado em conversas anteriores desta linha de trabalho (não no repositório directamente — precisa de confirmação):

- O **Layouter** original (fase inicial do projecto, "Passo 10-23", antes da numeração P590+) foi construído desde o início como um motor de **fluxo contínuo único**: um único `cursor_x`/`cursor_y` que avança item a item, com `flush_line()` a processar tudo — texto e, mais tarde, formas — pela mesma mecânica de linha.
- As primitivas de desenho (`rect`, `line`, depois `ellipse`/`circle`) foram introduzidas mais tarde ("Passo 74-82", também numeração antiga) como mais um tipo de item dentro desse mesmo fluxo — não há registo de uma decisão explícita de "formas quebram parágrafo como blocos"; a mecânica pré-existente do Layouter (cursor único, contínuo) simplesmente absorveu as formas da mesma maneira que absorvia palavras.

**Hipótese a confirmar, não a aceitar directamente:** a divergência não foi uma escolha consciente contra o vanilla — foi uma consequência não avaliada da arquitectura do Layouter, que já era block-agnóstica desde a Fase 1 do projecto, muito antes de o vanilla ter sido consultado para esta questão específica.

### Confirmar contra o repositório real

```bash
git log --all --oneline --grep="ShapeElem\|ShapeKind\|native_rect\|native_circle" | tail -30
git log --all --follow --oneline -- "01_core/src/engine/layout/shape.rs" | tail -20
```

```bash
# Se existirem, ler os prompts/relatórios históricos referenciados
find . -iname "*passo-7[6-9]*" -o -iname "*passo-8[0-2]*" 2>/dev/null
find 00_nucleo -iname "*shape*" -o -iname "*visualize*" 2>/dev/null
```

Confirmar:
1. O commit onde `ShapeElem`/`ShapeKind` foi introduzido pela primeira vez.
2. Se o prompt/relatório desse passo original menciona, de alguma forma, a questão inline vs block — ou se de facto nunca foi avaliada (confirmando a hipótese acima).
3. Se houve algum passo posterior (entre a introdução original e P763h) que tenha tocado nesta questão sem a resolver.

### Registo na tabela de rastreabilidade

Preencher, sem reescrever os passos antigos (regra 9):

| Passo (numeração da época) | Commit | O que decidiu (de facto, não retroactivamente) |
|---|---|---|
| Introdução de `rect`/`line` (~"Passo 76") | a confirmar | Formas adicionadas ao fluxo do Layouter, sem avaliação inline/block contra o vanilla |
| Introdução de `ellipse`/`circle` (~"Passo 77-82") | a confirmar | Mesmo padrão, replicado de `native_rect` |
| P763h | commit do relatório P763h | Divergência sistémica confirmada; scope-out registado, decisão de corrigir adiada para L0 dedicado |
| P767 (este passo) | — | L0 escrito; arqueologia registada |

---

## Parte B — L0: `Content::Shape` deve quebrar parágrafo

Escrever `00_nucleo/prompts/engine/layout/shape_block_behaviour.md` (caminho a confirmar contra a convenção real de `00_nucleo/prompts/`), cobrindo:

1. **Contrato**: qualquer `Content::Shape` (todas as `ShapeKind`) força o fecho do parágrafo corrente antes e depois de si, replicando `BlockElem::single_layouter` do vanilla — mesmo comportamento que gerou o aviso `block may not occur inside of a paragraph and was ignored` observado por P763h quando testado dentro de `#par[...]` explícito (confirmar se esse aviso também deve ser replicado como mensagem observável, ou se o cristalino deve simplesmente quebrar sem avisar — decisão a registar, não assumir).
2. **Ponto de intercepção**: onde no realizador/`Content::Sequence` a distinção bloco/inline é decidida hoje para outros elementos (heading, list, etc. — se já existir mecanismo de "isto quebra parágrafo" para esses, reutilizar o mesmo, não inventar um novo).
3. **Espaçamento above/below**: o vanilla usa `BlockElem` com `above`/`below` implícitos entre blocos — confirmar os valores por defeito reais (sonda contra o vanilla) antes de hardcodar.
4. **Impacto em `place()`**: formas dentro de `place()` já não seguem o fluxo normal (são posicionamento absoluto) — confirmar que esta mudança não afecta esse caminho, que já foi corrigido separadamente em P763f.
5. **Impacto nos testes de regressão de texto** (P745-762): confirmar que nenhum documento de teste da linha de layout vertical combina texto com formas de forma que dependa do comportamento actual (fluxo contínuo) — se depender, esses testes vão quebrar e precisam de actualização consciente, não seriam uma regressão real.

---

## Critério de fecho do passo

- [ ] Commit de introdução de `ShapeElem`/`ShapeKind` confirmado por `git log`, não por memória de conversa.
- [ ] Confirmado se a decisão inline/block foi avaliada nesse passo original ou nunca avaliada (hipótese da Parte A).
- [ ] Tabela de rastreabilidade preenchida, sem reescrever os passos antigos.
- [ ] L0 escrito em `00_nucleo/prompts/engine/layout/shape_block_behaviour.md`, com hash calculado, cobrindo os 5 pontos da Parte B.
- [ ] Nenhum código L1/L2/L3 escrito neste passo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p767.md`.

---

## Próximo passo

P767a (implementação): fazer `Content::Shape` quebrar parágrafo conforme o L0, reutilizando o mecanismo já existente para outros elementos de bloco se a Parte B (ponto 2) confirmar que ele existe. Validação: repetir as medições de AE de P763h (isoladas e misturadas com texto) para todas as primitivas, mais o checklist de sub-layouts (regra 5), mais os testes de regressão de P745-762 (ponto 5 do L0).
