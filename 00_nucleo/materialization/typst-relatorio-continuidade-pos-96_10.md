# Typst Cristalino — Relatório de Continuidade (pós-Passo 96.10)

**Data**: 2026-04-23
**Último passo executado**: 96.10 (encerramento do DEBT-46)
**Próximo passo**: a decidir no novo chat

---

## Como usar este relatório

Este documento é handoff de sessão. Foi escrito no fim de uma
conversa longa onde a série de passos 85–96.10 foi planeada e
executada com o Claude Code. Ao colar este relatório no início
de um novo chat, a assistente deve:

1. Ler integralmente antes de responder.
2. Confirmar que compreende o estado actual.
3. Esperar a minha indicação de qual trabalho quero seguir.
4. Não assumir contexto que não está aqui.

---

## Identidade do projecto

Fork de `typst/typst` sob arquitectura cristalina (padrão Tekt)
com camadas L0–L4. Guardião: `crystalline-lint` em projecto
separado. Testes e linter passam em todos os passos (regra
incondicional).

Ficheiros de outputs neste formato: `typst-passo-NN.md` para
enunciados, `typst-adr-NNNN-*.md` para ADRs. O utilizador executa
os passos localmente via Claude Code; a conversa serve para
planeamento e análise de reportes.

---

## Preferências do utilizador (cumprir sempre)

- **Português europeu**.
- **Linguagem literal**, sem figuras de linguagem nem bajulação
  (dislexia). Respeitar definições literais de palavras.
- **Política de passos**: passos de construção são únicos.
  Sub-passos só existem para pagar DEBTs ou fazer verificações.
  Não usar numeração decimal interna (ex: 96.5.5); usar inteiros
  contíguos em ordem linear.
- **Decisões com factos empíricos**, não estimativas. Se não sei,
  pedir os números reais antes de opinar.
- **Mensagens com opções**: usar `ask_user_input_v0` com opções
  curtas para decisões do utilizador.
- **Atomização**: o utilizador quer "caixas pretas com contrato
  explícito na assinatura" — não é jargão. Significa funções
  cujas dependências estão na assinatura, sem estado escondido.

---

## Arquitectura actual

### Camadas

- **L0** — fundações (tipos básicos, sem dependências externas).
- **L1** — core (regras da linguagem; puro, sem I/O).
- **L2** — shell (orquestração).
- **L3** — infra (I/O, fontes, exportação PDF).
- **L4** — wiring (composição final).

### Estrutura de ficheiros relevantes

`01_core/src/rules/` tem agora a seguinte organização (após o
DEBT-46 encerrado):

```
rules/
├── eval/
│   ├── mod.rs          (520 linhas; dispatcher + EvalContext)
│   ├── markup.rs
│   ├── math.rs
│   ├── modules.rs
│   ├── rules.rs
│   ├── closures.rs
│   ├── control_flow.rs
│   ├── bindings.rs
│   ├── operators.rs
│   └── tests.rs        (2100 linhas, Regra 6 — testes E2E)
├── parse/
│   ├── mod.rs          (156 linhas)
│   ├── parser.rs       (700 linhas)
│   ├── math.rs
│   ├── markup.rs
│   ├── code.rs
│   ├── rules.rs
│   └── patterns.rs
├── stdlib/
│   ├── mod.rs          (617 linhas)
│   ├── foundations.rs
│   ├── calc.rs
│   ├── text.rs
│   ├── assert.rs
│   ├── structural.rs
│   ├── figure_image.rs
│   ├── shapes.rs
│   ├── transforms.rs
│   └── layout.rs
├── layout/
│   ├── mod.rs          (756 linhas)
│   ├── metrics.rs
│   ├── measure.rs
│   ├── cursor.rs
│   ├── grid.rs
│   ├── placement.rs
│   ├── equation.rs
│   ├── helpers.rs
│   └── tests.rs        (1399 linhas, Regra 6)
├── math/
│   └── layout/
│       ├── mod.rs      (484 linhas)
│       ├── frac.rs
│       ├── attach.rs
│       ├── root.rs
│       ├── delimited.rs
│       ├── matrix.rs
│       ├── cases.rs
│       ├── stretchy.rs
│       ├── assembly.rs
│       └── tests.rs    (761 linhas)
└── lexer/
    ├── mod.rs          (468 linhas)
    ├── markup.rs
    ├── code.rs
    ├── math.rs
    └── scanner.rs      (645 linhas, intocado; DEBT-42 bloqueado)
```

### Ficheiros > 800 linhas com excepção Regra 6 documentada

- `rules/eval/tests.rs` (2100) — testes E2E cross-cutting.
- `rules/layout/tests.rs` (1399) — testes E2E cross-cutting.
- `entities/syntax_node.rs` (1095) — árvore sintáctica fundamental.
- `entities/content.rs` (1072) — enum central visual.
- `entities/layout_types.rs` (850) — vocabulário geométrico.
- `entities/ast/expr.rs` (845) — enum AST com ~50 variantes.

### Contagem de testes actual

**764 L1 + 174 L3 + 6 ignorados**. Zero violations no
`crystalline-lint`.

---

## ADRs em vigor (relevantes)

- **ADR-0018** — sem I/O em L1, determinismo.
- **ADR-0029** — pureza física de L1.
- **ADR-0030** — performance é domínio de L1.
- **ADR-0032** — política de `unsafe` em L1 (reduzir
  progressivamente).
- **ADR-0033** — paridade funcional com vanilla.
- **ADR-0034** — diagnóstico obrigatório antes de
  materializações.
- **ADR-0035** — EcoVec autorizado em L1 (Passo 89).
- **ADR-0036** — atomização progressiva (Passo 91.5). 5 regras
  operacionais. Funções declaram dependências na assinatura;
  estado partilhado mutável é dívida a reduzir.
- **ADR-0037** — coesão por domínio (Passo 96; promovida a
  `EM VIGOR` no Passo 96.3). 7 regras + 4 ajustes + nota de
  visibilidade (Passo 96.6). Primeira ADR validada
  empiricamente antes de promoção.

### Meta-regras em vigor (ADR-0036 e ADR-0037)

- **Declaração explícita de dependências** (Regra 1 da 0036).
- **Agrupamento deliberado** (Regra 2).
- **Redução progressiva** (Regra 3).
- **Excepções Regra 4** (estado genuinamente cross-cutting:
  colectores, limites, configuração estática).
- **Divergência vs vanilla registada** (Regra 5).
- **Coesão por domínio** em ficheiros (Regras 1-7 da 0037).
- **Visibilidade preferida**: privado → método `pub(super)` →
  `pub(in path)` → campo `pub(super)` justificado → `pub(crate)` → `pub`.

---

## DEBTs abertos (Secção 1)

| DEBT | Título | Estado | Último passo |
|------|--------|--------|--------------|
| DEBT-1 | StyleChain — pendências residuais | PARCIALMENTE RESOLVIDO | Passo 95 |
| DEBT-2 | Closures eager vs lazy | PARCIALMENTE RESOLVIDO | Passo 31 |
| DEBT-8 | Motor de equações | PARCIALMENTE RESOLVIDO | — |
| DEBT-9 | Cobertura de paridade | Tracking contínuo | — |
| DEBT-33, 34d, 34e, 35b, 39 | Diversos | EM ABERTO | Passos 79–84 |
| DEBT-42 | scanner get_unchecked | Bloqueado por benchmark | — |
| DEBT-43 | linter whitelist crate-level | EM ABERTO | Passo 89 |
| DEBT-45 | check_*_depth não chamados em alguns pontos | Parcialmente pago | Passo 93 |
| DEBT-47 | Auditoria de visibilidade pub(super) | EM ABERTO | Passo 96.6 |

### DEBTs encerrados recentemente

- **DEBT-40** (ImportGuard + unsafe) — Passo 90.
- **DEBT-41** (sealed traits) — Passo 85.
- **DEBT-44** (Route<'a> não estrutural) — Passo 92.
- **DEBT-39** (active_guards) — Passo 95.
- **DEBT-46** (ficheiros grandes) — Passo 96.10.

---

## Estado do `EvalContext`

Após extracções progressivas (ADR-0036), `EvalContext<'w>` tem
agora **6 campos**:

```rust
pub struct EvalContext<'w> {
    pub world: &'w dyn World,
    pub loop_iterations: usize,
    pub max_loop_iterations: usize,
    pub next_rule_id: RuleId,
    pub current_file: FileId,
    pub figure_numbering: Option<String>,
}
```

Campos removidos progressivamente: `route`, `styles`,
`show_rules`, `active_guards`, `depth`, `max_call_depth`.

**Candidatos restantes a extracção**: `current_file`,
`figure_numbering`. Os outros 4 são Regra 4 legítima da
ADR-0036 (handle externo, cross-cutting, alocador monotónico).

### Funções `eval_*` recebem agora

Desde o Passo 95, a assinatura típica de função `eval_*` é:

```rust
fn eval_expr(
    ctx: &mut EvalContext,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    expr: &Expr,
) -> SourceResult<Value>
```

---

## Stubs `comemo-tracked` por materializar (em L1)

- `Traced` — materializado (Passo 88).
- `Route<'a>` — materializado (Passo 90), integrado estruturalmente (Passo 92).
- **`Sink`** — stub, bloqueado por `Introspection`.
- **`Styles`** — stub, bloqueado por `Style` + `LazyHash`.
- **`Routines`** — stub.
- **`Engine<'a>`** — stub. Aglomera os anteriores no vanilla.

---

## Fricções técnicas descobertas (documentadas)

### Padrão `<T<'static> as Validate>::Constraint`

Auto-referência em tipos `#[comemo::track]` (ex:
`Route<'a> { outer: Option<Tracked<'a, Self>> }`) requer
override explícito do `Constraint`. Documentado em
`00_nucleo/diagnosticos/diagnostico-constraint-tracked-recursivo-passo-93.md`.

### Funções livres para operações sobre `Tracked<T>`

`Tracked<T>` só expõe métodos do bloco `#[comemo::track]`.
Operações que produzem tipos não-memoizáveis (ex:
`SourceDiagnostic`) têm de ser funções livres, não métodos.
Descoberto no Passo 93.

### Interacções com `crystalline-lint`

- **V14 (ForbiddenImport)**: `use self::X` é flagged; usar
  `use crate::...` ou `use super::...` (paths absolutos ou
  relativos para cima, não para dentro).
- **V2 (PromptDrift)**: cada ficheiro novo precisa de bloco
  `#[cfg(test)]` ou smoke test. Resulta em +N testes
  artificiais por reestruturação. 18 smoke tests V2 no total
  da série 96.x.

### Extractor Python para reestruturação

Durante Passos 96.4 e 96.8, scripts Python para mover blocos
de código entre ficheiros tiveram bugs por contagem naïve de
chavetas (char literal `'{'` confundido com delimitador).
Para refactorings grandes futuros, usar parser AST (syn,
tree-sitter) ou sanitizar string/chars literais antes.

---

## Série 96.x — resumo de execução

Sequência completa para referência:

1. **96** — governança (ADR-0037 `PROPOSTO` + DEBT-46).
2. **96.1** — reestruturação `eval.rs`.
3. **96.2** — completar delegação dos armos de `eval_expr`.
4. **96.3** — promover ADR-0037 a `EM VIGOR` com 4 ajustes.
5. **96.4** — reestruturação `parse.rs`.
6. **96.5** — reestruturação `stdlib.rs`.
7. **96.6** — nota de visibilidade na Regra 3 + DEBT-47 aberto.
8. **96.7** — reestruturação `layout/mod.rs`.
9. **96.8** — reestruturação `math/layout.rs`.
10. **96.9** — reestruturação `lexer/mod.rs`.
11. **96.10** — verificação final e encerramento DEBT-46.

### Rácio métodos/campos `pub(super)` ao longo dos passos

- 96.7: 2.1:1 (primeiro passo sob nota de visibilidade).
- 96.8: 2.6:1.
- 96.9: 4.0:1 (melhor — lexer tem struct simples).

Tendência crescente sugere que DEBT-47 deve priorizar auditoria
dos passos 96.1, 96.2, 96.4 (antes da nota), onde bulk replace
de `pub(super)` foi aplicado.

---

## Candidatos para próximo trabalho

O utilizador decide em conversa. Opções:

### 1. DEBT-47 — auditoria de visibilidade

**Escopo**: revisão dos `pub(super)` introduzidos nos Passos
96.1–96.5. Converter campos em métodos quando preservar
invariantes. Restringir com `pub(in path)` quando aplicável.
Remover residuais de bulk replace.

**Custo**: médio. Auditoria é pesquisa + edição localizada.

**Ganho**: código com invariantes protegidos; dívida histórica
liquidada antes de esquecer.

### 2. Materializar `Style` + `LazyHash`

**Escopo**: folhas da cadeia `Styles`. Pré-requisito para
materializar `Styles` (stub actual).

**Custo**: médio. Cada uma é materialização pontual.

**Ganho**: desbloqueia `Styles`. Abre caminho para
incrementalidade.

### 3. Materializar `Introspection`

**Escopo**: infraestrutura de introspecção. Desbloqueia `Sink`.

**Custo**: alto. `Introspection` tem acoplamento com layout
(figuras, referências, etiquetas).

**Ganho**: desbloqueia `Sink` (diagnósticos memoizáveis) e
counters/figuras corretos.

### 4. Materializar `Engine<'a>`

**Escopo**: agregador vanilla. Unifica `route`, `styles`,
`show_rules`, etc. numa estrutura.

**Custo**: alto. Toca em todas as funções `eval_*` + layout.

**Ganho**: convergência arquitectural com vanilla. Prepara
#[memoize] real.

**Dependência**: melhor fazer depois de `Introspection`
materializada.

### 5. Continuar ADR-0036 com `figure_numbering` e `current_file`

**Escopo**: últimos candidatos de extracção do `EvalContext`.

**Custo**: baixo (análogo a Passos 92-95).

**Ganho**: marginal. Depois destes, `EvalContext` fica só com
campos Regra 4 legítimos.

---

## Sugestão de ordem (opinião)

1. **DEBT-47** primeiro — limpeza antes de avançar. Valida que
   a nota da Regra 3 funciona retroactivamente.
2. **Materializar folhas** (`Style`, `LazyHash`) — trabalho
   isolado, desbloqueia `Styles`.
3. **Materializar `Introspection`** — segue naturalmente.
4. **`Engine<'a>`** — depois de ter `Sink` e `Styles`
   materializados.
5. Extracções residuais do `EvalContext` — por último ou nunca.

Mas isto é opinião. A decisão é do utilizador.

---

## Instruções finais para a assistente do novo chat

- Cumprimentar breve.
- Confirmar que leu este relatório.
- Não reiterar informação do relatório sem ser perguntada.
- Esperar indicação do utilizador sobre qual caminho seguir.
- Quando o utilizador decidir, escrever o enunciado do próximo
  passo com o mesmo rigor dos passos anteriores.
- Usar `ask_user_input_v0` para decisões binárias/múltiplas.
- Respeitar preferências de linguagem (literal, sem bajulação).
