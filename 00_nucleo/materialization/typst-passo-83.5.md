# Passo 83.5 — Auditoria dos DEBTs em aberto

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/DEBT.md` — Localização actual do ficheiro (incorrecta). Ver
  Tarefa de movimentação abaixo.
- `00_nucleo/` — Directório onde `DEBT.md` deveria estar. Confirmar
  estrutura antes de mover.
- `00_nucleo/adr/typst-adr-0006-typst-timing.md` — Referenciado pela
  secção "Dívida de instrumentação — ADR-0006" em `DEBT.md`.

Pré-condição: `cargo test` — 901 testes (732 L1 + 169 L3, 6 ignorados
pré-existentes), zero violations. Passo 83 concluído.

---

## Natureza deste passo

Este é um passo de **auditoria e reporte**, não de implementação. O
objectivo é:

1. **Mover** `DEBT.md` de `01_core/` para `00_nucleo/` (localização correcta).
2. **Reorganizar** o ficheiro (está desordenado: encerrados misturados com
   abertos, DEBTs numéricos fora de ordem, secção ADR-0006 órfã no meio).
3. **Auditar** cada DEBT marcado como aberto ou parcialmente resolvido,
   confirmando no código se ainda está de facto em aberto ou se foi
   fechado sem ser sinalizado no `DEBT.md`.
4. **Reportar** apenas a lista final de DEBTs confirmadamente em aberto.

Claude Code **não altera código** neste passo. **Não fecha DEBTs no
ficheiro**. **Não propõe correcções**. O output é uma lista curada para
informar a decisão sobre quais atacar no Passo 84.

A única alteração permitida a ficheiros neste passo é a movimentação e
reorganização do `DEBT.md` (Tarefas 1 e 2).

---

## Tarefa 1 — Movimentação do ficheiro

Mover `00_nucleo/DEBT.md` para `00_nucleo/DEBT.md`.

Procurar e actualizar todas as referências ao caminho antigo:

```bash
# Ocorrências de "00_nucleo/DEBT.md" no repositório.
grep -rn "00_nucleo/DEBT.md" . --include="*.md" --include="*.rs" --include="*.toml"
```

Actualizar cada referência para `00_nucleo/DEBT.md`. Incluir:
- Comentários em código-fonte (`// ver 00_nucleo/DEBT.md`).
- Ficheiros de ADR em `00_nucleo/adr/`.
- `typst-migracao-estado.md` e outros ficheiros de estado.
- READMEs e documentação auxiliar.

Depois da movimentação, confirmar que `grep -rn "00_nucleo/DEBT.md"` retorna
zero resultados.

---

## Tarefa 2 — Reorganização do ficheiro

O `DEBT.md` actual tem três problemas estruturais:

1. **Numeração fora de ordem**: DEBT-2 aparece antes de DEBT-6, DEBT-3
   aparece depois de DEBT-7, DEBT-10 aparece antes de DEBT-8, etc.
2. **Encerrados misturados com abertos**: DEBT-6 (encerrado) aparece no
   meio de DEBTs em aberto; DEBT-34b (encerrado) aparece antes de
   DEBT-34d (aberto).
3. **Secção ADR-0006 órfã**: o bloco "Dívida de instrumentação —
   ADR-0006" aparece no meio do ficheiro sem número de DEBT e sem
   relação directa com os DEBTs numéricos.

Aplicar a seguinte estrutura:

```markdown
# Inventário de dívida técnica

## Secção 1 — DEBTs em aberto ou parcialmente resolvidos

Ordenados por número crescente. Cada entrada mantém o seu texto actual
(não resumir nem reescrever). Se um DEBT está "parcialmente resolvido",
o histórico de passos que o atacaram fica preservado.

[DEBT-1, DEBT-2, DEBT-8, DEBT-9, DEBT-21, DEBT-22, DEBT-33, DEBT-34d,
 DEBT-34e, DEBT-35b, DEBT-36, DEBT-37, DEBT-38, ...]

## Secção 2 — DEBTs encerrados

Ordenados por número crescente. Cada entrada mantém o seu texto actual.
Preservar o histórico porque futuras regressões podem precisar de
consultar a resolução original.

[DEBT-3, DEBT-6, DEBT-7, DEBT-10, DEBT-11, DEBT-12, DEBT-13, DEBT-14,
 DEBT-15, DEBT-16, DEBT-17, DEBT-18, DEBT-19, DEBT-20, DEBT-23, DEBT-24b,
 DEBT-24c, DEBT-25, DEBT-26, DEBT-27, DEBT-28, DEBT-29, DEBT-30, DEBT-31,
 DEBT-32, DEBT-34b, DEBT-34c, ...]

## Secção 3 — Dívida de instrumentação

A secção "Dívida de instrumentação — ADR-0006" mantém-se separada das
secções 1 e 2. Não é um DEBT numerado — é uma nota de coordenação sobre
pontos de timing removidos à espera de religação. Preservar como está.
```

**Regras estritas**:

- Não alterar o texto de nenhuma entrada. Apenas mover e ordenar.
- Não renumerar DEBTs. Se DEBT-4 ou DEBT-5 não existem, a numeração
  mantém-se descontínua — o histórico é mais valioso que a continuidade.
- DEBT-3 está marcado "RESOLVIDO (estrutura)" com uma subsecção
  "Pendente (não é DEBT — é feature futura)". A subsecção explica que o
  pendente não é dívida. Colocar em "Secção 2 — encerrados".
- DEBT-1 e DEBT-2 estão "PARCIALMENTE RESOLVIDO" com subsecções
  "Pendente". Colocar em "Secção 1 — em aberto".
- DEBT-8 está "PARCIALMENTE RESOLVIDO" com subsecção "Ainda pendente".
  Colocar em "Secção 1 — em aberto".
- DEBT-9 ("tracking contínuo") é aberto por natureza — nunca encerra
  enquanto houver novos SyntaxKind. Colocar em "Secção 1".
- DEBT-21 está "MITIGADO" mas não encerrado. Colocar em "Secção 1" com
  nota da mitigação.
- DEBT-22 não tem marca explícita de encerramento. Colocar em "Secção 1".
- DEBT-35b é preventivo (cache não existe ainda). Colocar em "Secção 1"
  com nota do estado preventivo.

---

## Tarefa 3 — Auditoria de cada DEBT aberto

Para cada DEBT da lista abaixo, Claude Code executa uma verificação no
código e reporta se o estado no `DEBT.md` corresponde à realidade.

**Regra absoluta**: Claude Code **não fecha nada**, **não altera o
`DEBT.md`**, **não altera código**. Apenas reporta o resultado da
verificação. Se um DEBT parece já ter sido fechado sem ser sinalizado,
isso entra no relatório como observação — a decisão de fechar é do
utilizador no Passo 84.

### Lista de DEBTs a auditar

#### DEBT-1 — StyleChain

Estado no ficheiro: PARCIALMENTE RESOLVIDO (Passo 30). Pendências listadas:
scoping de `#set` por bloco, propriedades adicionais (fill, font-family,
weight numérico), `#show` rules, paridade total.

Verificar:
- `#show` rules: foram implementadas no Passo 68 (ver entrada
  DEBT-19/DEBT-20 em Secção 2). Confirmar se a pendência "`#show` rules"
  em DEBT-1 ainda faz sentido ou se deve ser riscada.
- Scoping de `#set` por bloco: procurar `save/restore` de `ctx.styles`
  em `Expr::CodeBlock`/`Expr::ContentBlock` (DEBT-7 menciona isto como
  resolvido). Verificar se o item "scoping de `#set` por bloco" em
  DEBT-1 já foi coberto pelo DEBT-7.

```bash
grep -n "styles" 01_core/src/rules/eval.rs | grep -i "save\|restore\|push\|pop"
grep -rn "StyleChain" 01_core/src/ | head -20
```

Reportar: quais das pendências listadas em DEBT-1 já foram resolvidas
implicitamente por outros passos, e quais continuam em aberto.

#### DEBT-2 — Closures eager vs lazy capture

Estado: PARCIALMENTE RESOLVIDO (Passo 31). Pendente: integração com
`comemo` para tracking semântico real, testes de paridade avançados de
shadowing.

Verificar:
- Existe `TrackedWorld` real no código? Procurar em L3.
- Os testes de paridade em `lab/parity/` incluem cenários de shadowing?

```bash
grep -rn "TrackedWorld" 03_infra/src/ 01_core/src/
grep -rn "shadow\|capture" lab/parity/tests/ 2>/dev/null
```

#### DEBT-8 — Motor de equações

Estado: PARCIALMENTE RESOLVIDO (Passos 36-40). Pendente: kern matemático,
fontes OpenType MATH, `MathPrimes`, `MathAlignPoint`, baseline por x-height.

Verificar:
- `MathPrimes` existe no AST mas não é tratado no layouter?
- `MathAlignPoint` idem?

```bash
grep -rn "MathPrimes\|MathAlignPoint" 01_core/src/
```

#### DEBT-9 — Cobertura de paridade

Estado: tracking contínuo. Aberto por natureza.

Verificar:
- Quantos testes de paridade existem hoje? O número aumentou desde o
  baseline do Passo 35 (50 inputs)?

```bash
find lab/parity/tests -name "*.rs" -exec wc -l {} \;
grep -c "#\[test\]" lab/parity/tests/parse_parity.rs 2>/dev/null
```

#### DEBT-21 — Resolução de NodeKind por string

Estado: MITIGADO (Passo 70). Requer Rust >= 1.85 para `fn_addr_eq`
estável.

Verificar:
- Versão actual do Rust no `rust-toolchain.toml` ou `Cargo.toml`.
- Se >= 1.85, o DEBT pode ser efectivamente fechado com pouco trabalho.

```bash
cat rust-toolchain.toml 2>/dev/null
grep "edition\|rust-version" Cargo.toml 01_core/Cargo.toml 03_infra/Cargo.toml
rustc --version
```

#### DEBT-22 — Clone de show_rules por nó

Estado: registado no Passo 68, sem marca de encerramento.

Verificar:
- O código actual ainda faz `ctx.show_rules.clone()` em
  `intercept_content` ou já foi substituído por `Rc<[ShowRule]>`?

```bash
grep -n "show_rules" 01_core/src/rules/eval.rs 01_core/src/rules/show.rs 2>/dev/null
```

#### DEBT-33 — AABB de curvas Bézier

Estado: EM ABERTO (Passo 79).

Verificar:
- O código em `rules/layout/mod.rs` ou equivalente que calcula AABB de
  `ShapeKind::Path` ainda usa min/max dos pontos de controlo, ou já faz
  cálculo analítico?

```bash
grep -rn "CubicTo\|bounding\|aabb" 01_core/src/rules/layout/ | head -10
```

#### DEBT-34d — Auto não encolhe antes de matar fr

Estado: EM ABERTO (Passo 80).

Verificar:
- A resolução de `columns` tem lógica de min-content/max-content ou
  apenas medição por `layout_sub_frame_with_width`?

```bash
grep -A 20 "TrackSizing::Auto" 01_core/src/rules/layout/mod.rs | head -30
```

#### DEBT-34e — colspan e rowspan

Estado: EM ABERTO (Passo 80).

Verificar:
- `Content::Grid` tem campos `colspan`/`rowspan`?

```bash
grep -A 5 "Grid {" 01_core/src/entities/content.rs
```

#### DEBT-35b — Invalidação de cache de available_width após SetPage

Estado: EM ABERTO, preventivo.

Verificar:
- `available_width` é calculado em tempo real ou existe campo de cache?

```bash
grep -n "available_width" 01_core/src/rules/layout/mod.rs | head -10
```

#### DEBT-36 — Operadores simbólicos de alinhamento

Estado: EM ABERTO (Passo 82).

Verificar:
- O parser tem `Value::Align` com composição simbólica (`center + bottom`)?
- `Align2D::from_string` ainda é usado na stdlib?

```bash
grep -rn "Value::Align\|Align2D::from_string" 01_core/src/
```

#### DEBT-37 — Place relativo ao contentor pai

Estado: EM ABERTO (Passo 82).

Verificar:
- `Content::Place` no layouter ancora a `line_start_x` e margem da
  página, ou recebe área de âncora por parâmetro?

```bash
grep -A 10 "Content::Place" 01_core/src/rules/layout/mod.rs | head -20
```

#### DEBT-38 — Cache de sub-frames no Grid Auto

Estado: EM ABERTO (Passo 83, aberto no passo corrente).

Verificar:
- Existe cache `(Content*, width) → (height, Vec<FrameItem>)` ou a
  medição ainda descarta os `FrameItem` produzidos?

```bash
grep -B 2 -A 15 "TrackSizing::Auto" 01_core/src/rules/layout/mod.rs | head -40
```

---

## Tarefa 4 — Formato do relatório

Claude Code produz um único documento de relatório com a seguinte
estrutura:

```markdown
# Relatório de auditoria de DEBTs — após Passo 83

## 1. Movimentação e reorganização

- Ficheiro movido: 00_nucleo/DEBT.md → 00_nucleo/DEBT.md
- Referências actualizadas: [lista de ficheiros tocados, sem diff
  completo — só caminhos]
- Reorganização: [confirmar que Secção 1 / Secção 2 / Secção 3 foram
  aplicadas, com contagem de DEBTs em cada secção]

## 2. DEBTs confirmadamente em aberto

Para cada DEBT que a auditoria confirmou estar realmente em aberto:

### DEBT-N — [título]
- Estado no ficheiro: [copiar literal do DEBT.md]
- Verificação: [comando executado + output resumido]
- Confirmação: ABERTO — [razão]
- Sugestão para Passo 84: [FÁCIL | MÉDIO | DIFÍCIL | FORA DE ESCOPO]
  - FÁCIL: resolução local, <50 linhas, sem nova dependência.
  - MÉDIO: resolução com alterações em 2-3 ficheiros, requer testes.
  - DIFÍCIL: resolução arquitectural, requer ADR ou decisão de design.
  - FORA DE ESCOPO: depende de infra externa (TrackedWorld, comemo real,
    Rust version bump, etc.).

## 3. DEBTs com divergência entre ficheiro e código

Para cada DEBT onde a auditoria encontrou que o estado no ficheiro não
corresponde ao código:

### DEBT-N — [título]
- Estado no ficheiro: [copiar literal]
- Verificação: [comando executado + output resumido]
- Observação: [descrever a divergência — pode ser "parece fechado sem
  ser sinalizado", "pendências listadas já foram resolvidas por outro
  passo", "estado mudou entre passos sem actualização do DEBT.md"]
- Acção sugerida para o utilizador: [fechar | actualizar estado |
  dividir em sub-DEBTs | outra]

## 4. Sumário para o Passo 84

Lista ordenada por dificuldade crescente:

### Candidatos FÁCEIS (atacar primeiro no Passo 84)
- DEBT-N: [título curto]
- DEBT-M: [título curto]

### Candidatos MÉDIOS
- DEBT-X: [título curto]

### Candidatos DIFÍCEIS (adiar ou escrever ADR primeiro)
- DEBT-Y: [título curto]

### FORA DE ESCOPO do Passo 84
- DEBT-Z: [título curto] — razão

## 5. Anexo — comandos de verificação executados

Para reprodutibilidade, listar todos os comandos `grep`/`find`/`cat`
executados durante a auditoria. Não incluir os outputs completos — só
os comandos.
```

---

## Critérios de conclusão

- [ ] `DEBT.md` movido de `01_core/` para `00_nucleo/`.
- [ ] Todas as referências ao caminho antigo actualizadas (`grep -rn
  "00_nucleo/DEBT.md"` retorna zero).
- [ ] Ficheiro reorganizado nas três secções (abertos / encerrados /
  instrumentação) sem alterar o texto de nenhuma entrada.
- [ ] Cada DEBT da lista da Tarefa 3 foi auditado com um comando de
  verificação concreto.
- [ ] Relatório produzido seguindo o formato da Tarefa 4.
- [ ] Nenhum DEBT foi fechado no ficheiro durante este passo.
- [ ] Nenhuma linha de código-fonte foi alterada (excepto actualização
  de comentários com o novo caminho de `DEBT.md`).
- [ ] Número total de testes após o passo é idêntico ao anterior
  (732 L1 + 169 L3). Zero violations.

---

## Ao terminar, reportar

- Número de ficheiros cujas referências a `DEBT.md` foram actualizadas.
- Número de DEBTs em cada secção do ficheiro reorganizado.
- Lista de DEBTs confirmadamente em aberto (copiar directamente da
  Secção 2 do relatório).
- Lista de divergências encontradas entre ficheiro e código (copiar
  directamente da Secção 3 do relatório).
- Recomendação final para o Passo 84: quais DEBTs são candidatos a
  fecho imediato, quais beneficiam de um ADR prévio, quais precisam de
  ser adiados.

Este passo não tem Go/No-Go para o Passo 84 no sentido habitual —
produz antes o **menu de opções** que o Passo 84 vai consumir. A
decisão sobre o que atacar no Passo 84 é do utilizador, informada pelo
relatório.
