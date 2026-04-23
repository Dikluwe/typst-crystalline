# Passo 96.8 — Reestruturação de `math/layout.rs` em submódulos por fase

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `EM VIGOR` com 7 regras + 4 ajustes + nota de visibilidade.
- `00_nucleo/DEBT.md` — DEBT-46 checkbox 96.8 pendente.
- `01_core/src/rules/math/layout.rs` — ficheiro actual, 1806
  linhas.
- `01_core/src/rules/math/` — directório com possíveis ficheiros
  irmãos (verificar em Fase 0).
- Reporte do Passo 96.7 — referência de padrão aplicado em
  `layout/mod.rs`, com a nota de visibilidade em vigor.

Pré-condição: `cargo test` — 753 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96.7 concluído.

---

## Natureza deste passo

Passo único de reestruturação. Aplica ADR-0037 a
`math/layout.rs` (layout matemático).

Similaridades com Passo 96.7:
- Ficheiro dentro de um directório já existente (`math/`).
- Pode ter struct central com muitos métodos.
- Nota de visibilidade em vigor.

Diferenças potenciais:
- Provavelmente **menos** armos de dispatcher (math tem tipos
  de expressão mais concentrados: fracção, raiz, atacamento,
  delimitadores).
- Pode haver reutilização de infraestrutura do layout genérico
  (ex: `Layouter<M>` de `layout/mod.rs`) ou pode ter struct
  própria (`MathLayouter`?).

---

## Clusters propostos

Hipóteses, a confirmar em Fase 0:

```
01_core/src/rules/math/layout.rs (antes: 1806 linhas)
    ↓ transforma-se em:
01_core/src/rules/math/layout/
    mod.rs          — struct central + entry point (layout_math)
    frac.rs         — fracções (numerador/denominador)
    attach.rs       — sub/super scripts, limits
    delimited.rs    — parênteses, chavetas, colchetes escaláveis
    root.rs         — raiz quadrada, n-ésima
    matrix.rs       — matrizes, casos, vectores (se existirem)
    tests.rs        — testes se secção grande (>500 linhas)
```

Alternativa possível: se `math/layout.rs` já reutiliza muito de
`layout/mod.rs` e contém apenas dispatch específico, os clusters
podem ser menores e reunidos em 2-3 submódulos.

**Ajustes durante execução**: reportar em Fase 0 qual a
estrutura real e ajustar o plano.

---

## Fase 0 — Diagnóstico

### 0.1 — Inventário do directório `math/`

```bash
# Tamanho:
wc -l 01_core/src/rules/math/layout.rs

# Ficheiros irmãos:
find 01_core/src/rules/math/ -name "*.rs"
ls -la 01_core/src/rules/math/

# Estrutura top-level:
grep -n "^pub fn\|^fn\|^pub struct\|^struct\|^impl\|^pub enum\|^enum" \
    01_core/src/rules/math/layout.rs | head -50

# Dependências de outros módulos:
grep -n "^use " 01_core/src/rules/math/layout.rs | head -20

# Testes:
grep -c "^\s*#\[test\]" 01_core/src/rules/math/layout.rs
```

Reportar:
- Linhas confirmadas (esperado 1806).
- Ficheiros pré-existentes em `math/`.
- Se há struct central (`MathLayouter`? ou reutiliza
  `Layouter<M>`?).
- Número de métodos/funções top-level.
- Número de testes e sua organização.
- Se importa `super::layout::*` ou `crate::rules::layout::*`.

### 0.2 — Mapear funções por fase matemática

Com base no inventário, propor clusters:

| Cluster | Funções candidatas |
|---------|---------------------|
| frac | layout_frac, place_numerator, ... |
| attach | layout_attach, layout_limits, ... |
| delimited | layout_delimited, scale_paren, ... |
| root | layout_root, ... |
| matrix | layout_matrix, layout_cases, ... |
| ... | ... |

Se há cluster com apenas 1-2 funções pequenas, considerar fusão
com cluster próximo (ex: `root` + `frac` como `vertical.rs`).
Documentar ajustes.

---

## Fase 1 — Extracções incrementais

### Estratégia

Seguir exactamente o padrão do Passo 96.7:

1. **Se `math/layout.rs` fica sozinho no directório**: converter
   para `math/layout/mod.rs` via `git mv`, depois extrair.
2. **Se há ficheiros irmãos em `math/`**: extrair pedaços para
   ficheiros irmãos directamente (pode ser fora de um
   sub-directório `layout/`, dependendo da organização).

### Procedimento por cluster

Idêntico ao Passo 96.7:

- Identificar métodos/funções.
- Criar submódulo com cabeçalho ADR-0037.
- Mover métodos mantendo generics se aplicável
  (`impl<M: FontMetrics> Layouter<M> { ... }`).
- Aplicar visibilidade pela ordem de preferência: privado →
  método `pub(super)` → `pub(in path)` → campo `pub(super)` →
  `pub(crate)` → `pub`.
- Verificar `cargo check` + `cargo test` após cada cluster.
- Rollback do cluster se falhar.

### Visibilidade

A nota de visibilidade da ADR-0037 Regra 3 está em vigor desde
o Passo 96.6. Segue as mesmas orientações do Passo 96.7.

Se encontrar padrão análogo aos 14 campos `pub(super)` do
`Layouter` (grupos de campos que são actualizados em conjunto
sem invariante individual), considerar se há operação de alto
nível que os torna atómicos. Se não houver, documentar
`pub(super)` no comentário-bloco da struct.

---

## Fase 2 — Verificação final

### 2.1 — Tamanhos

```bash
wc -l 01_core/src/rules/math/*.rs 2>/dev/null | sort -rn
# Se math/layout/ for criado:
wc -l 01_core/src/rules/math/layout/*.rs 2>/dev/null | sort -rn
```

Alvo: nenhum submódulo acima de 800 linhas sem excepção Regra 6.

### 2.2 — Testes e linter

```bash
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado: 753 + possivelmente +N smoke tests V2 (se novos
submódulos criados). Zero violations.

### 2.3 — Testes específicos de matemática

```bash
cargo test --package typst-core math 2>&1 | tail -15
cargo test --package typst-core equation 2>&1 | tail -10
cargo test --package typst-core frac 2>&1 | tail -5
```

Todos devem passar sem alteração.

### 2.4 — Visibilidade

```bash
grep -rn "pub(super)" 01_core/src/rules/math/ | wc -l
grep -rn "pub(super)\s*fn" 01_core/src/rules/math/ | wc -l    # métodos
grep -rn "pub(super)\s*\w*:\s" 01_core/src/rules/math/ | wc -l # campos
```

Reportar a proporção métodos/campos. Alvo: métodos > campos,
como no Passo 96.7.

---

## Fase 3 — Actualizar DEBT-46

Marcar o oitavo checkbox:

```markdown
- [x] `math/layout.rs` reestruturado ou marcado como excepção
      Regra 6. Passo 96.8. **Concluído** — N submódulos,
      todos abaixo de 800 linhas. Rácio métodos/campos
      `pub(super)`: X/Y.
```

DEBT-46 não fecha (2 checkboxes restantes: 96.9 lexer, 96.10
verificação final).

---

## Critérios de conclusão

- [ ] `math/layout.rs` reestruturado (decomposto em submódulos
      ou convertido em directório).
- [ ] Nenhum submódulo acima de 800 linhas sem excepção Regra 6.
- [ ] Visibilidade seguiu ordem de preferência da ADR-0037.
- [ ] Sem bulk replace de `pub(super)`.
- [ ] Testes de matemática passam sem alteração.
- [ ] `cargo test --workspace` preservado (753 ± smoke tests V2).
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo.
- [ ] DEBT-46 com oitavo checkbox marcado.
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

Fase 0:
- Lista de ficheiros pré-existentes em `math/`.
- Estrutura da struct central (se existir).
- Se reutiliza `Layouter<M>` ou tem layouter próprio.

Fase 1 (por cluster):
- Cluster extraído, tamanho final.
- Visibilidade aplicada (contagem de métodos vs campos
  `pub(super)`).
- Rollbacks se necessário.

Fase 2:
- Tamanhos finais.
- Rácio métodos/campos `pub(super)`.
- Testes verdes, zero violations.

Fase 3:
- Confirmação DEBT-46 actualizado.

Go/No-Go para Passo 96.9:
- **Go incondicional** se reestruturação foi limpa. Passo 96.9
  = `lexer/mod.rs` (1250 linhas), possivelmente excepção Regra 6
  (lexers têm frequentemente tabelas de estados grandes).
- **Go com observações** se houve fricção nova.
- **No-Go parcial** se layout matemático tem dependência forte
  e não-decomponível do `Layouter<M>` principal. Nesse caso,
  reavaliar estratégia.
