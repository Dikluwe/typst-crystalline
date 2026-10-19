# Passo 415 — ADR "Stub Transparente vs Fallback Software" + Atualização Inventário Cobertura (XS)

**Tipo**: Administrativo-documental (zero código de produção; zero tipo novo; zero I/O).  
**Data**: 2026-06-22.  
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).  
**ADRs relevantes**: ADR-0054 (graded), ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla).  
**Sonda fonte**: Princípio emergente aplicado implicitamente em P408 (smallcaps), P295 (footnote), P414 (text.font dict); carece de formalização canônica.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

O princípio da **honestidade epistêmica** — "não fingir que se sabe fazer algo que não se sabe fazer bem" — já opera no cristalino desde o P408 (stub transparente para `smallcaps` sem shaping) e P295 (marker-only para `footnote` sem layout-time). No entanto, carece de formalização arquitetural. Sem ADR, cada passo futuro que enfrente a mesma escolha (stub vs fallback) terá de re-discutir o critério do zero.

Este passo cria a ADR canônica, cita os passos históricos que a aplicaram, e atualiza o inventário de cobertura com as materializações P391–P414.

---

## 2. Decisão de engenharia

A ADR estabelece:

> **Quando um consumer requer infraestrutura ausente** (shaping, runtime layout, CSL parser, variant-aware font selection, gradient render, etc.), **prefere-se stub transparente** — o elemento existe no pipeline (parse → eval → Content), mas o consumer emite o body inalterado ou vazio. **Nunca fallback software** a menos que o fallback seja bit-exact com vanilla ou explicitamente justificado como ADR-0054 graded com ressalva.

**Razão**: "quase funciona" é mais difícil de remover do que adicionar. Stub permite evolução por adição pura; fallback exige refactor destrutivo no futuro.

**Passos históricos que aplicam este princípio** (não alterados; apenas citados na ADR):
- **P408** (`smallcaps`): stub transparente — consumer emite `layout_content(&body)` inalterado; não finge OpenType `smcp`/`c2sc` sem rustybuzz.
- **P295** (`footnote`): Fase 1 marker only — consumer emite `[N]` superscript; nota rodapé real adiada até layout-time suportar two-pass.
- **P414** (`text.font` dict): campos `variant`/`weight`/`style` parseados em `FontFamily` mas variant-aware selection não ativada — honestidade sobre o que `FontBook::select` realmente faz.
- **P157B** (`table.cell`): `colspan`/`rowspan` armazenados mas ignored em layout até placement algorítmico (P224.C/P234).
- **P224.B** (`GridHeader`/`GridFooter`): `repeat: bool` armazenado mas semantic adiada até multi-region flow real.
- **P223** (`place`): `float`/`clearance` armazenados mas semantic adiada até consumer geometric real (P245).
- **P156G** (`block`): `breakable` armazenado mas semantic adiada até consumer real (P248).
- **P231** (`block`/`box`): `outset`/`radius`/`clip` armazenados mas semantic adiada até infrastructure real (P242/P247/P248).

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.0 — Sonda do substrato (obrigatória; 5 min)

```bash
# 1. Verificar que ADR-0054 existe (referência para graded)
grep -n "ADR-0054" 00_nucleo/adr/*.md | head -5
# Esperado: 1+ hit

# 2. Verificar que P408 relatório existe (prova de aplicação)
grep -n "stub\|transparente\|honestidade" 00_nucleo/relatorios/typst-passo-408-relatorio.md 2>/dev/null || echo "relatório P408 não encontrado"
# Esperado: referência a stub transparente

# 3. Verificar que P295 relatório existe
grep -n "marker only\|Fase 1" 00_nucleo/relatorios/typst-passo-295-relatorio.md 2>/dev/null || echo "relatório P295 não encontrado"
# Esperado: referência a marker only

# 4. Verificar que inventário de cobertura existe
ls 00_nucleo/typst-cobertura-vanilla-vs-cristalino.md 2>/dev/null || echo "inventário não encontrado"
# Esperado: arquivo existe

# 5. Verificar que nenhuma ADR com este tema já existe
grep -rl "stub transparente\|fallback software\|honestidade epistêmica" 00_nucleo/adr/
# Esperado: zero hits (confirma ausência)
```

**Critério de passagem**: (1) ADR-0054 existe; (2)-(4) provas de aplicação existem; (5) nenhuma ADR duplicada. Se (5) encontrar hit, **parar** — ADR já existe.

### A.1 — Prompt L0 `adr-stub-vs-fallback.md`

Novo em `00_nucleo/prompts/adr/adr-stub-vs-fallback.md`:

- **Paridade**: não aplica (ADR é cristalino-native; não existe no vanilla).
- **Decisão**: stub transparente preferido sobre fallback software quando infraestrutura está ausente.
- **Exceção**: fallback software justificado apenas se (a) bit-exact com vanilla, ou (b) ADR-0054 graded com ressalva documentada.
- **Razão**: evolução por adição pura vs refactor destrutivo; honestidade epistêmica.
- **Passos citados**: P408, P295, P414, P157B, P224.B, P223, P156G, P231 (lista não exaustiva; novos passos podem ser adicionados via PR).
- **Testes**: não aplica (ADR é documental).

### A.2 — CHECKPOINT

Parar. Apresentar `adr-stub-vs-fallback.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Implementação (após confirmação humana)

### B.1 — Criar ADR

Novo arquivo: `00_nucleo/adr/typst-adr-00XX-stub-transparente-vs-fallback-software.md` (substituir XX pelo próximo número sequencial disponível; verificar `ls 00_nucleo/adr/` para confirmar).

Estrutura da ADR:

```markdown
# ADR-00XX — Stub Transparente vs. Fallback Software

**Status**: IMPLEMENTADO  
**Data**: 2026-06-22  
**Decisão**: Stub transparente preferido sobre fallback software quando infraestrutura está ausente.  
**Decisor**: Passo 415 (administrativo-documental).

## 1. Contexto

Vários passos do cristalino enfrentaram a escolha: quando um consumer requer infraestrutura que ainda não existe (shaping, runtime layout, CSL parser, etc.), implementar um fallback software aproximado ou um stub transparente que admite a limitação?

## 2. Decisão

**Stub transparente é preferido.** O elemento existe no pipeline completo (parse → eval → Content → walk → layout), mas o consumer emite o body inalterado ou vazio. Não finge funcionalidade que não existe.

**Fallback software é rejeitado** salvo duas exceções:
- (a) O fallback é **bit-exact com vanilla** (ex.: `lorem` é helper puro, não fallback).
- (b) O fallback é **ADR-0054 graded com ressalva documentada** — aproximação aceitável com plano de remoção.

## 3. Consequências

- **Positiva**: evolução por adição pura; nenhum refactor destrutivo no futuro.
- **Positiva**: honestidade epistêmica — o sistema não engana o usuário.
- **Negativa**: valor visual imediato pode ser zero para features stubbed.
- **Negativa**: testes E2E de features stubbed precisam ser escritos com expectativas ajustadas (body inalterado, não erro).

## 4. Passos que aplicam este princípio

| Passo | Elemento | Infra ausente | Comportamento do stub |
|-------|----------|---------------|----------------------|
| P408 | `smallcaps` | OpenType shaping (rustybuzz) | Emite `body` inalterado |
| P295 | `footnote` | Layout-time two-pass | Emite `[N]` marker apenas |
| P414 | `text.font` dict | Variant-aware selection | Parseia campos mas não ativa selection |
| P157B | `table.cell` | Placement algorítmico | Armazena `colspan`/`rowspan` mas ignora em layout |
| P224.B | `GridHeader`/`GridFooter` | Multi-region flow | Armazena `repeat` mas ignora em layout |
| P223 | `place` | Float consumer geometric | Armazena `float`/`clearance` mas ignora |
| P156G | `block` | Breakable layout | Armazena `breakable` mas ignora até P248 |
| P231 | `block`/`box` | Radius/clip infrastructure | Armazena `radius`/`clip`/`outset` mas ignora até P242/P247 |

## 5. Anti-padrões

- **Não** implementar "quase funciona" sem documentar como scope-out.
- **Não** criar fallback visual (uppercase+scale, fake small caps, etc.) sem ADR-0054 graded explícito.
- **Não** deixar o usuário achar que a feature está completa quando é stub.

## 6. Referências

- ADR-0054 — Graded (aproximações aceitáveis com ressalva).
- P408 — `smallcaps` stub transparente.
- P295 — `footnote` marker only.
- P414 — `text.font` dict honestidade epistêmica.
```

### B.2 — Atualizar inventário de cobertura

Editar `00_nucleo/typst-cobertura-vanilla-vs-cristalino.md`:

1. **Adicionar seção "Atualizações P391–P414"** no topo, após o header de status:
   ```markdown
   > **Atualização P415 (2026-06-22)** — cluster P391–P414 materializado:
   > - Foundations: `lorem`, `panic`, `eval`, `read`, `cbor`, `bytes`, `csv`, `xml`, `json`, `yaml`, `toml` implementados.
   > - Tipos primitivos: `Decimal`, `Duration`, `Version` (constructors + aritmética + field access + comparações).
   > - Text: `smallcaps` implementado (stub transparente per ADR-00XX).
   > - Math: `accent`, `cancel`, `underover`, `op` implementados.
   > - Layout/Model: `document`, `asset`, `tiling`, `state`/`counter` display, `place` float, `Block`/`Boxed`/`Grid`/`Table` refinos.
   > - DEBT-52 (gap 8, `text.font` dict) **FECHADO** em P414.
   ```

2. **Atualizar Tabela A.3 (Text)**:
   - `smallcaps`: `ausente` → `implementado` (nota: stub transparente, aguarda shaping real).
   - `lorem`: adicionar linha se ausente → `implementado`.

3. **Atualizar Tabela A.8 (Foundations)**:
   - Adicionar `eval`, `read`, `cbor`, `bytes` se ausentes → `implementado`.
   - Adicionar `csv`, `xml`, `json`, `yaml`, `toml` se ausentes → `implementado`.
   - Adicionar `lorem`, `panic` se ausentes → `implementado`.

4. **Atualizar Tabela B.1 (Value variants)**:
   - `Decimal`: `implementado` (P399).
   - `Duration`: `implementado` (P400).
   - `Version`: `implementado` (P401).
   - `Tiling`: `implementado` (P395).
   - `Bytes`: `implementado` (P398).
   - `Regex`: `implementado` (P402).

5. **Atualizar Tabela B.2 (Content variants)**:
   - `Document`: `implementado` (P397).
   - `Asset`: `implementado` (P397).
   - `SmallCaps` (se existir): `implementado` (P408).

6. **Atualizar resumo agregado**:
   - Recalcular contagens com base nas mudanças acima.
   - Cobertura user-facing estimada: ~81% (impl + impl+).

### B.3 — Linhagem e validação

- **Linhagem**: `@prompt` aponta para `adr-stub-vs-fallback.md`; `@prompt-hash` via `--fix-hashes`.
- **Validação**: `crystalline-lint .` — zero violations (ADR é markdown; não afeta código Rust).
- **Git diff**: `git diff --stat` mostra apenas `adr-00XX.md` + `cobertura.md`.

---

## 5. O que NÃO fazer (scope-out)

- **Não** alterar código de nenhum passo histórico (P408, P295, etc.) — a ADR cita, não modifica.
- **Não** criar código de produção — este passo é puramente documental.
- **Não** adicionar novos tipos, variants, ou funções stdlib.
- **Não** promover ADR-0054 ou outras ADRs — apenas criar a nova.
- **Não** inventar passos futuros que aplicarão a ADR — deixar para serem citados quando materializados.

---

## 6. Critérios de aceitação

1. ADR `typst-adr-00XX-stub-transparente-vs-fallback-software.md` criada em `00_nucleo/adr/`.
2. ADR cita pelo menos 5 passos históricos que aplicam o princípio (P408, P295, P414, P157B, P224.B, etc.).
3. Inventário `typst-cobertura-vanilla-vs-cristalino.md` atualizado com P391–P414.
4. DEBT-52 marcado como FECHADO no inventário.
5. `smallcaps` transita `ausente` → `implementado` no inventário (com nota de stub).
6. Tipos primitivos (`Decimal`, `Duration`, `Version`, `Tiling`, `Bytes`, `Regex`) marcados como implementados no inventário.
7. Zero código Rust alterado; zero testes novos; zero I/O.
8. L0 salvo e hashado antes da ADR; sonda A.0 documentada.
9. `crystalline-lint` zero violations.

---

## 7. O que pode sair errado

- **Número de ADR já existe** (conflito de sequência). Mitigação: verificar `ls 00_nucleo/adr/` antes de nomear.
- **Inventário muito grande para editar sem erro**. Mitigação: editar seções específicas apenas; não reescrever o arquivo inteiro.
- **Tentação de adicionar código junto**. Mitigação: scope-out claro; este passo é documental puro.

---

## 8. Referências

- P408 — `smallcaps` stub transparente (prova de aplicação).
- P295 — `footnote` marker only (prova de aplicação).
- P414 — `text.font` dict honestidade epistêmica (prova de aplicação).
- ADR-0054 — Graded (exceção para fallback justificado).
- ADR-0108 — Medir-antes-de-decidir (sonda A.0).
- `00_nucleo/typst-cobertura-vanilla-vs-cristalino.md` — inventário a atualizar.

---

## 9. Nota sobre o Tekt

Este passo é **XS** — puramente documental. O valor está na **centralização de um princípio emergente** que já operava implicitamente. Sem esta ADR, cada passo futuro (P500+) que enfrente a escolha stub vs fallback teria de re-discutir o critério. Com a ADR, a decisão é **lookup de um linha**.

A atualização do inventário é **sincronização factual** — garantir que o documento de cobertura reflete o estado real do código após P391–P414.
