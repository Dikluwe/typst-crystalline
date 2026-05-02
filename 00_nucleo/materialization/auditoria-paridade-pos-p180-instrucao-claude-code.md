# Auditoria de paridade — estado pós-P180

## Contexto

Após 20 passos executados (P161-P180), o módulo Introspection
cristalino sofreu refactor significativo: walk puro, fixpoint,
sub-stores isolados, 5 features M9 implementadas. Esta
auditoria mede o estado actual em três dimensões.

A meta é produzir números literais que respondam:

- Quantos tipos cristalino tem agora vs início?
- Quantos tipos vanilla têm equivalente cristalino?
- Quantas features Introspection vanilla são cobertas?
- A auditoria de isolamento (executada antes de P161) ainda
  é válida ou mudou?
- Snapshot tests vanilla — quantos passam?

Output: ficheiro markdown único em
`00_nucleo/diagnosticos/paridade-pos-p175.md` com 4 secções.

---

## Material a inspeccionar

**Cristalino**:
- `01_core/src/entities/` — todas as structs e enums `pub`.
- `01_core/src/rules/introspect/` — todas as funções/tipos
  `pub`.
- `01_core/src/rules/layout/mod.rs` — Layouter actual.
- `01_core/src/rules/layout/references.rs` — consumer migrado
  P168.
- Tests em `01_core/tests/` — contagem total e snapshot
  tests.

**Vanilla** (referência):
- `lab/typst-original/crates/typst-library/src/introspection/` —
  44 tipos `pub` identificados em inventário anterior.
- `lab/typst-original/crates/typst-library/src/foundations/` —
  Content e tipos relacionados.

**Documentos prévios**:
- `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md`
  (44 tipos pub vanilla).
- `00_nucleo/diagnosticos/auditoria-isolamento-vs-vanilla.md`
  (5 estruturas piores antes do refactor).
- `00_nucleo/diagnosticos/m1-lacunas-captura.md` (7 lacunas
  identificadas durante M1).
- `00_nucleo/materialization/typst-passo-161-relatorio.md` a
  `typst-passo-175-relatorio.md` (15 relatórios).

---

## Output esperado

### Secção 1 — Contagem de tipos cristalino

Tabela com todos os tipos `pub` actuais em
`01_core/src/entities/` e `01_core/src/rules/introspect/`,
com:
- Nome do tipo.
- Ficheiro.
- Linhas no ficheiro.
- Passo onde foi criado (P161-P175 ou anterior).
- Equivalente vanilla (nome do tipo vanilla mais próximo, ou
  "sem equivalente" se for tipo novo).

Total esperado: ~25-30 tipos novos pós-refactor + tipos
pré-existentes de Introspection.

Sub-tabela com totais:
- Tipos pré-existentes (antes de P161).
- Tipos novos (P161-P175).
- Tipos com equivalente vanilla.
- Tipos sem equivalente vanilla (decomposição cristalina).

### Secção 2 — Cobertura vanilla → cristalino

Para cada um dos 44 tipos vanilla do inventário, indicar:
- Tem equivalente cristalino? (Sim/Não/Parcial)
- Se sim, qual é.
- Se não, está deferido conscientemente (com referência ao
  passo ou diagnóstico) ou ainda não considerado?

Sub-tabela com totais:
- Cobertos: N de 44.
- Deferidos: M de 44.
- Não considerados: K de 44.

### Secção 3 — Cobertura de features Introspection

Tabela com as 11 features Introspection vanilla, com:
- Feature.
- Estado (✅ completa / 🟡 parcial / ⏳ pendente).
- Passo de implementação (se aplicável).
- Subset implementado (descrição em uma linha).
- Subset deferido (descrição em uma linha).

Total: features completas + parciais + pendentes = 11.

### Secção 4 — Auditoria de isolamento revisitada

Para cada uma das 5 estruturas piores identificadas na
auditoria anterior:

- Nome.
- Estado anterior (número de fields/variants/linhas).
- Estado actual (número de fields/variants/linhas).
- Classificação anterior: Pior / Igual / Melhor que vanilla.
- Classificação actual: Pior / Igual / Melhor que vanilla.
- Justificação literal da mudança ou não-mudança.

Critérios A-D da auditoria anterior aplicam-se. Regra "pior
dos 4" mantém-se.

Para tipos novos criados em P161-P175 (sub-stores, Tag,
ElementInfo, etc.), aplicar mesma classificação contra
vanilla equivalente.

Sub-tabela com distribuição:
- Pior que vanilla: X (era Y).
- Igual a vanilla: A (era B).
- Melhor que vanilla: M (era N).

---

## Restrições

- **Não escrever ADR** sobre os achados.
- **Não criar reservas** de identificadores.
- **Não propor refactor** ou plano de acção.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio".
- **Não classificar como passo PNNN**: é auditoria.
- **Sem código novo escrito**: trabalho de leitura apenas.
- **Reportar números literais**, não estimativas qualitativas.

---

## Critério de conclusão

- Ficheiro `paridade-pos-p175.md` produzido em
  `00_nucleo/diagnosticos/`.
- 4 secções presentes.
- Sub-tabelas de totais em cada secção.
- Cada tipo vanilla do inventário aparece exactamente uma vez
  na secção 2.
- Cada feature das 11 aparece exactamente uma vez na secção 3.
- Cada estrutura das 5 anteriores aparece na secção 4.
- Sem ADR nova criada.
- Sem reservas.

A auditoria é instrumento. Decisões sobre o que fazer com os
resultados ficam para depois.
