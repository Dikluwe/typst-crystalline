# P425 — Varredura Mecânica: agentes para gaps parciais e ausentes (L)

**Título**: Varredura mecânica — delegação de gaps mecânicos parciais e ausentes a agentes autônomos durante ausência do dono  
**Tipo**: Materialização (L) — múltiplos consumers + infraestrutura de verificação automática  
**Bloqueadores**: Nenhum externo; depende do estado do Inventário post-P423  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), ADR-0108 §resíduo (modos de falha novos), P421 (repr completo), P422 (link render), P423 (Selector And/Or)

---

## FASE A.0 — Sonda do substrato: inventário mecânico (obrigatória; 10 min)

Execute os 8 grep/scripts abaixo para **mapear gaps mecânicos** — itens que são paridade mecânica (não linguagem) e podem ser resolvidos sem decisão arquitetural.

```bash
# 1. Quais variants de Content NÃO têm layout arm implementado?
grep -n "Content::" 01_core/src/rules/layout/mod.rs | grep -v "todo\|unimplemented\|panic" | wc -l
grep -n "enum Content" 01_core/src/entities/content.rs | head -5
# Comparar: se o número de arms em layout/mod.rs < número de variants em content.rs → gap mecânico

# 2. Quais variants de Value NÃO têm repr implementado?
grep -n "Value::" 01_core/src/rules/eval/repr.rs | wc -l
grep -n "enum Value" 01_core/src/entities/value.rs | head -5

# 3. Quais elementos NÃO têm arquivo próprio na camada de render (atomização ADR-0109)?
ls 01_core/src/rules/layout/*.rs | wc -l
grep -n "enum Content" 01_core/src/entities/content.rs | grep -o "[A-Z][a-zA-Z]*" | sort -u | wc -l
# Comparar: se arquivos de layout < variants de Content → gap de atomização

# 4. Quais L0 prompts NÃO têm @prompt-hash sincronizado?
crystalline-lint . 2>&1 | grep "drift\|hash" | head -20

# 5. Quais tests estão faltando para variants implementados?
grep -rn "#\[test\]" 01_core/src/rules/layout/ | wc -l
grep -rn "#\[test\]" 01_core/src/rules/eval/ | wc -l
# Se < 100 tests em layout ou < 200 em eval → gap mecânico de cobertura

# 6. FrameItem::Link tem consumer downstream (PDF writer)?
grep -rn "FrameItem::Link" 01_core/src/ | grep -v "layout_types\|link.rs" | head -10

# 7. Quais warnings de compilador persistem?
cargo check -p typst-core 2>&1 | grep "warning:" | wc -l

# 8. Quais ADRs estão PROPOSTO vs IMPLEMENTADO?
grep -rn "PROPOSTO\|IMPLEMENTADO" 00_nucleo/adr/ | head -20
```

**Output esperado**: lista numerada de gaps mecânicos, cada um classificado como:
- **P** (parcial) — existe mas incompleto
- **A** (ausente) — não existe
- **M** (mecânico) — pode ser resolvido sem decisão arquitetural (ADR-0107)

**Critério de parada**: se a sonda encontrar > 30 gaps mecânicos, reclassificar para **XL** (muitos para 4h).

---

## FASE A.1 — L0 (hash obrigatório) + Plano de Delegação

**Documentar no L0** (`00_nucleo/prompts/meta/p425-varredura-mecanica.md`):

### A.1.1 — Princípio: mecânica vs linguagem (ADR-0107)

Este passo **só** toca gaps **mecânicos**:
- **Mecânico**: implementar arm faltante em `match`, adicionar test, sincronizar hash, mover lógica para free function (atomização), adicionar consumer downstream.
- **NÃO mecânico**: decidir se um tipo novo entra no enum, escolher crate externa, redefinir API pública, alterar contrato de paridade linguagem.

**Regra de ouro**: se um gap exige a frase "deveríamos..." ou "qual a melhor forma de...", é **linguagem** → **não tocar**. Se exige "falta o arm no match" ou "falta o test para", é **mecânico** → **delegar**.

### A.1.2 — Agentes definidos

Cada agente é uma **tarefa autônoma** com entrada, algoritmo, e critério de aceitação. O agente (IA assistente) executa sem intervenção do dono.

| Agente | Tarefa | Entrada | Critério de aceitação | Tempo estimado |
|--------|--------|---------|----------------------|----------------|
| **A1** — Layout stub hunter | Para cada variant de `Content` sem arm em `layout_content`, criar arm que delega para free function (forma B) | Lista de variants faltantes do grep A.0.1 | `cargo check` passa; `match` exaustivo; 0 `todo!()` | 30 min |
| **A2** — Repr gap filler | Para cada variant de `Value`/`Content`/`Selector` sem repr, adicionar arm em `repr.rs` | Lista de variants faltantes do grep A.0.2 | `cargo test -- repr` passa; match exaustivo | 20 min |
| **A3** — Atomizador mecânico | Para cada arm monolítico (> 20 LOC) em `layout/mod.rs`, extrair para `rules/layout/<elem>.rs` (forma B) | Lista de arms gordos do grep A.0.3 | Lint zero; `cargo test` passa; arquivo novo < 100 LOC | 45 min |
| **A4** — Test gap filler | Para cada variant implementado sem test, adicionar test unitário mínimo | Lista de variants sem test do grep A.0.5 | +N tests verdes; 0 regressões | 30 min |
| **A5** — Hash sync | Sincronizar todos os `@prompt-hash` driftados | Lista de drift do grep A.0.4 | `crystalline-lint --fix-hashes` → 0 drift | 10 min |
| **A6** — Warning hunter | Corrigir warnings de compilador que são mecânicos (unused, dead_code, etc.) | Lista de warnings do grep A.0.7 | `cargo check` → 0 warnings mecânicos | 20 min |
| **A7** — PDF link consumer | Implementar consumo de `FrameItem::Link` no PDF writer (se existir) | `FrameItem::Link` + PDF writer path | `FrameItem::Link` → annotation URI no output | 40 min |

**Regra de orquestração**: agentes A1–A6 são **independentes** (podem rodar em paralelo). A7 depende de A1 (layout estável).

### A.1.3 — Algoritmo de execução (para o agente IA)

```
PARA cada agente A1..A7:
  1. LER entrada (lista de gaps do grep)
  2. SE lista vazia → PULAR (nada a fazer)
  3. PARA cada item na lista:
     a. MEDIR: abrir arquivo, verificar contexto (3 linhas antes/depois)
     b. DECIDIR: é mecânico? (não envolve API pública, enum novo, ou decisão arquitetural)
     c. SE sim → IMPLEMENTAR (código + test mínimo)
     d. SE não → DOCUMENTAR no scope-out do P425 (não tocar)
  4. VALIDAR: cargo check / cargo test / crystalline-lint
  5. SE falha → REVERTER (git stash) e DOCUMENTAR como bloqueador
  6. COMMIT: git add -A && git commit -m "P425: <agente> — <descrição>"
```

### A.1.4 — Critério de parada do agente

- **Tempo**: 4 horas máximo (ausência do dono). Se um agente exceder 45 min, parar e documentar como "incompleto — requer decisão arquitetural".
- **Qualidade**: `cargo test` deve passar em cada commit. Nenhum commit quebrando a build.
- **Escopo**: se um gap mecânico revelar sub-gaps linguísticos (ex.: "para implementar este arm, preciso de um novo tipo"), parar e documentar.

### A.1.5 — Scope-out explícito (não tocar)

- **Decisões arquiteturais**: adicionar novo variant a `Content`, `Value`, ou `Selector`; escolher crate externa; alterar contrato de paridade.
- **Refactor estrutural**: mudar de enum para vtable, de `match` para `dyn`, de `Box` para `Arc`.
- **Features linguísticas novas**: `text.lang` rustybuzz, `bibliography` CSL-JSON, `cite` supplement CSL-native.
- **Performance**: otimizações de cache, lazy evaluation, multi-threading.
- **Documentação de usuário**: README, tutoriais, exemplos.

---

## CHECKPOINT A

**Só prosseguir para Fase B (execução por agentes) quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `meta/p425-varredura-mecanica.md`
2. Sonda A.0 produziu lista de gaps ≤ 30
3. Agentes A1–A7 estão definidos com entradas e critérios claros
4. O agente IA confirmou que entende a distinção mecânico vs linguagem (ADR-0107)

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Execução por Agentes (autônoma, 4h)

### B.1 — Preparação do ambiente (5 min)

```bash
# Criar branch de trabalho
git checkout -b p425-varredura-mecanica

# Snapshot do estado inicial
git log --oneline -1 > /tmp/p425-base-commit.txt
cargo test -p typst-core --lib 2>&1 | tail -5 > /tmp/p425-base-tests.txt
```

### B.2 — Execução sequencial dos agentes

**Ordem recomendada** (minimiza conflitos):
1. **A5** (Hash sync) — 10 min — limpa drift antes de tocar código
2. **A6** (Warning hunter) — 20 min — limpa warnings antes de novos tests
3. **A1** (Layout stub hunter) — 30 min — implementa arms faltantes
4. **A2** (Repr gap filler) — 20 min — completa repr
5. **A3** (Atomizador mecânico) — 45 min — move lógica gorda para free functions
6. **A4** (Test gap filler) — 30 min — adiciona tests para o que A1–A3 criaram
7. **A7** (PDF link consumer) — 40 min — consumer downstream (se aplicável)

**Total**: ~3h 15 min. Buffer de 45 min para imprevistos.

### B.3 — Protocolo de commit

```bash
# Para cada agente:
git add -A
git commit -m "P425-A<N>: <agente> — <resumo>"
# Ex: "P425-A1: Layout stub hunter — arms para Content::Table, Content::Grid"
```

### B.4 — Protocolo de falha

Se um agente falhar (`cargo test` quebra, lint não passa):
1. `git stash` ou `git checkout -- .` (reverter alterações do agente)
2. Documentar no relatório: "A<N> falhou: <causa> — requer decisão arquitetural"
3. Prosseguir para o próximo agente (não bloquear pipeline)

---

## FASE C — Relatório de Fecho (ao retorno do dono)

### C.1 — Coleta de métricas

```bash
cargo test -p typst-core --lib 2>&1 | tail -5 > /tmp/p425-final-tests.txt
crystalline-lint . 2>&1 | grep -E "error|drift|warning" | head -20 > /tmp/p425-final-lint.txt
git log --oneline p425-varredura-mecanica --not $(cat /tmp/p425-base-commit.txt) > /tmp/p425-commits.txt
```

### C.2 — Relatório mínimo

```markdown
# Relatório P425 — Varredura Mecânica

**Data**: <data>
**Tempo de execução**: <X>h <Y>m
**Commits**: <N> (ver /tmp/p425-commits.txt)

## Agentes executados
| Agente | Status | Itens tratados | Itens scope-out |
|--------|--------|----------------|-----------------|
| A1 | ✅/❌ | N | M |
| A2 | ✅/❌ | N | M |
| ... | ... | ... | ... |

## Métricas
- Tests antes: <N> passed
- Tests depois: <N+Δ> passed
- Lint drift antes: <N>
- Lint drift depois: <N>
- Warnings compilador antes: <N>
- Warnings compilador depois: <N>

## Bloqueadores (requerem decisão do dono)
1. <descrição> — <arquivo:linha>
2. ...

## Próximo passo sugerido
P426: <baseado nos bloqueadores restantes>
```

### C.3 — Critério de fecho

- [ ] Todos os agentes executados (sucesso ou falha documentada)
- [ ] `cargo test` passa no branch p425 (ou no mínimo não regressa do base)
- [ ] `cargo check` passa
- [ ] Lint zero errors (drift pode persistir se agente A5 falhou)
- [ ] Relatório escrito em `/tmp/p425-relatorio.md` (ou similar)
- [ ] Branch p425-varredura-mecanica pronto para merge/review

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 mede exatamente quais gaps existem antes de delegar. Nenhum agente trabalha no escuro.
- **Paridade linguagem (ADR-0107)**: Agentes A1–A7 são explicitamente restritos a mecânica. Se um gap revelar ser linguagem, o agente para e documenta. O dono decide ao retornar.
- **Atomização (ADR-0109)**: A3 é o agente de atomização mecânica. Ele move lógica gorda para free functions sem alterar comportamento (content-preserving).
- **Resíduo (ADR-0108 §resíduo)**: Este passo não pega modos de falha novos, reenquadramento de objetivo, ou cumprimento ritual. O dono, ao retornar, audita a substância (não a forma).
- **Honestidade epistêmica**: Se um agente falhar, não "forçar" a solução. Documentar e passar adiante. O objetivo é progresso incremental, não heroísmo.
- **Próximo passo P426**: dependerá do relatório do P425. Possíveis: decisões arquiteturais pendentes, consumer PDF, `text.lang` rustybuzz, ou continuação da varredura.
