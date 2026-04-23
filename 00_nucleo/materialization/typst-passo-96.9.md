# Passo 96.9 — Reestruturação de `lexer/mod.rs` ou aplicação de Regra 6

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `EM VIGOR`. Regra 6 autoriza excepções para ficheiros
  intrinsecamente densos (código gerado, tabelas de estados,
  enums com muitas variantes).
- `00_nucleo/DEBT.md` — DEBT-46 checkbox 96.9 pendente.
- `01_core/src/rules/lexer/mod.rs` — ficheiro actual, 1250
  linhas.
- `01_core/src/rules/lexer/scanner.rs` — ficheiro irmão (645
  linhas), contém parte da lógica de scanning/DEBT-42 bloqueado.

Pré-condição: `cargo test` — 761 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.8 concluído.

---

## Natureza deste passo

Passo único com duas vias possíveis, decidida em Fase 0:

1. **Via A — Decomposição**: se o lexer tem clusters claros
   (lex_string, lex_number, lex_identifier, etc.), aplicar
   ADR-0037 como nos passos anteriores.

2. **Via B — Excepção Regra 6**: se o lexer é código denso
   monolítico (tabela de transições, grande `match` sobre
   caracteres sem clusters naturais), documentar como excepção
   Regra 6 com justificativa no topo do ficheiro.

Lexers têm frequentemente características que justificam Via B:
- Tabelas de estados ou transições geradas por ferramenta.
- Função `next_token` massiva com `match` sobre bytes/chars.
- Código intrinsecamente sequencial (um lexer é, por natureza,
  um autómato monolítico).

Mas não assumir Via B sem inspeccionar — alguns lexers
modernos (especialmente escritos à mão) têm divisão clara por
tipo de token. O diagnóstico em Fase 0 decide.

---

## Fase 0 — Diagnóstico obrigatório

### 0.1 — Inventário

```bash
# Tamanho:
wc -l 01_core/src/rules/lexer/mod.rs

# Ficheiros no directório:
ls -la 01_core/src/rules/lexer/

# Estrutura top-level:
grep -n "^pub fn\|^fn\|^pub struct\|^struct\|^impl\|^pub enum\|^enum" \
    01_core/src/rules/lexer/mod.rs | head -40

# Métodos privados/públicos:
grep -c "^\s*fn \|^\s*pub fn " 01_core/src/rules/lexer/mod.rs

# Tamanho dos `match` (indicador de monolito):
grep -c "=>\s*{" 01_core/src/rules/lexer/mod.rs

# Testes:
grep -c "^\s*#\[test\]" 01_core/src/rules/lexer/mod.rs
```

Reportar:
- Linhas confirmadas.
- Lista de ficheiros existentes em `lexer/`.
- Número de funções top-level e métodos.
- Número de `match` e armos.
- Organização dos testes.

### 0.2 — Análise de clusters potenciais

Procurar nomes de funções que sugerem domínios:

```bash
# Funções potencialmente agrupáveis:
grep -n "fn lex_\|fn tokenize_\|fn scan_\|fn consume_" \
    01_core/src/rules/lexer/mod.rs
```

Se houver 6+ funções com nomes `lex_string`, `lex_number`,
`lex_identifier`, `lex_operator`, etc., há clusters claros
→ **Via A**.

Se a maior parte do ficheiro é:
- Uma função gigante (`next_token` com 500+ linhas).
- Uma tabela estática (`const TRANSITIONS: ...`).
- Métodos privados que só fazem sentido em conjunto.

Então não há clusters → **Via B**.

### 0.3 — Análise da struct central (se existir)

```bash
grep -B 2 -A 20 "^pub struct Lexer\|^pub struct Scanner" \
    01_core/src/rules/lexer/mod.rs
```

Reportar: se há struct com muitos campos, se é genérica, e se
faz sentido separar métodos por domínio.

### 0.4 — Decisão

Com base no diagnóstico, escolher Via A ou Via B e reportar **antes**
de avançar. A decisão é revertível — se Via A revelar que a
decomposição produz submódulos artificiais, voltar a Via B.

---

## Via A — Decomposição por cluster

Se a análise revelar clusters claros, aplicar padrão dos passos
anteriores (96.1, 96.4, 96.5, 96.7, 96.8):

### A.1 — Criar estrutura de submódulos

```bash
git mv 01_core/src/rules/lexer/mod.rs 01_core/src/rules/lexer/old_mod.rs
# (nome temporário; vai voltar a mod.rs depois)
```

Não — melhor padrão: criar directamente os submódulos extraindo
pedaços do `mod.rs` actual, mantendo `mod.rs` como ponto de
entrada.

### A.2 — Clusters hipotéticos

```
01_core/src/rules/lexer/
    mod.rs           — Lexer struct + next_token dispatcher +
                       entry points
    string.rs        — lex_string, escape sequences
    number.rs        — lex_number, parse de literais
    identifier.rs    — lex_identifier, palavras-chave
    operator.rs      — lex_operator, pontuação
    comment.rs       — lex_comment (bloco/linha)
    whitespace.rs    — consumo de espaços
    tests.rs         — se secção grande
```

Ajustar conforme realidade.

### A.3 — Procedimento

Idêntico aos passos anteriores. Visibilidade pela ordem de
preferência da nota da Regra 3 (Passo 96.6). Verificação após
cada cluster.

### A.4 — Atualizar DEBT-46

```markdown
- [x] `lexer/mod.rs` reestruturado por tipo de token (string,
      number, identifier, operator, etc.). Passo 96.9.
      **Concluído** — N submódulos, todos abaixo de 800
      linhas. Rácio métodos/campos pub(super): X/Y.
```

---

## Via B — Excepção Regra 6

Se a análise revelar que o lexer é monolítico por natureza,
aplicar Regra 6 sem decomposição.

### B.1 — Adicionar justificativa no topo de `lexer/mod.rs`

```rust
//! Lexer do Typst.
//!
//! Este ficheiro excede o limite orientativo de 800 linhas da
//! ADR-0037 Regra 2 (~1250 linhas actuais). Aplica-se a
//! **Regra 6 (excepções permitidas)** com a seguinte
//! justificativa:
//!
//! O lexer é um autómato de estados que processa fluxo de
//! caracteres sequencialmente. A função principal `next_token`
//! (ou equivalente) contém um `match` exaustivo sobre os
//! caracteres de entrada que não se decompõe naturalmente em
//! clusters por domínio — cada caso é uma transição de estado
//! ligada às outras por contexto partilhado (posição,
//! lookahead, flags de modo).
//!
//! Tentar dividir em submódulos produziria ou:
//! - Fragmentação artificial (um submódulo por token type
//!   sem valor real, porque cada submódulo só teria uma
//!   função pequena).
//! - Acoplamento crescente (submódulos a consultar estado do
//!   lexer via `pub(super)` campos ou métodos, anulando o
//!   ganho de decomposição).
//!
//! A decomposição fica possível se no futuro o lexer ganhar
//! funcionalidades claramente separáveis (ex: modo matemático
//! vs modo markup com regras totalmente distintas). Até lá,
//! manter monolítico é a decisão correcta.
```

Ajustar texto conforme realidade observada no diagnóstico.

### B.2 — Actualizar DEBT-46

```markdown
- [x] `lexer/mod.rs` marcado como excepção Regra 6 da
      ADR-0037. Passo 96.9. **Concluído** — justificativa no
      topo do ficheiro: lexer monolítico por natureza, sem
      clusters decomponíveis sem fragmentação artificial.
```

### B.3 — Não tocar em código

Via B é 100% governança. Não altera nenhuma linha do lexer.
Só adiciona comentário no topo do ficheiro.

---

## Fase 2 — Verificação final

Independentemente da via escolhida:

```bash
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado:
- **Via A**: 761 L1 + possivelmente +N smoke tests V2.
- **Via B**: 761 L1 inalterado (só comentário foi alterado).
- Zero violations.

---

## Critérios de conclusão

**Se Via A**:
- [ ] `lexer/mod.rs` reduzido ou dividido.
- [ ] Submódulos criados, todos abaixo de 800 linhas.
- [ ] Visibilidade seguiu nota Regra 3.
- [ ] Testes preservados.
- [ ] DEBT-46 checkbox 96.9 marcado com registo da via.

**Se Via B**:
- [ ] Comentário de justificativa Regra 6 adicionado no topo
      de `lexer/mod.rs`.
- [ ] Nenhuma outra alteração.
- [ ] DEBT-46 checkbox 96.9 marcado com registo da via.

**Ambas as vias**:
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Fase 0:
- Número de funções top-level, tamanhos de match, padrões.
- **Via escolhida (A ou B)** com razão factual.

Se Via A:
- Submódulos criados, tamanhos finais.
- Visibilidade aplicada.
- Fricções encontradas.

Se Via B:
- Texto final da justificativa Regra 6.

Verificação:
- Contagem de testes.
- Zero violations.

Go/No-Go para Passo 96.10:
- **Go incondicional** em ambas as vias. Passo 96.10 =
  verificação final do DEBT-46 e encerramento. Inclui:
  - Inventário de todos os ficheiros acima de 800 linhas em
    `01_core/src/`.
  - Cada um está abaixo de 800 OU tem excepção Regra 6
    documentada.
  - Mover DEBT-46 para Secção 2 (encerrado).
