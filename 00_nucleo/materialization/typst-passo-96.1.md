# Passo 96.1 — Reestruturação de `eval.rs` em submódulos por domínio

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md` — ADR
  `PROPOSTO`. Este passo é a primeira aplicação concreta e
  valida (ou ajusta) o princípio.
- `00_nucleo/adr/typst-adr-0036-*.md` — atomização progressiva,
  complementar.
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional,
  comportamento observável preservado.
- `00_nucleo/DEBT.md` — entrada DEBT-46 com 8 checkboxes. Este
  passo marca o primeiro checkbox (`eval.rs` reestruturado).
- `01_core/src/rules/eval.rs` — ficheiro actual, 3780 linhas,
  368 ocorrências de padrões `match`.

Pré-condição: `cargo test` — 746 L1 + 174 L3 + 6 ignorados,
zero violations. Passo 96 concluído (DEBT-46 aberto, ADR-0037
`PROPOSTO`).

---

## Natureza deste passo

Passo único de reestruturação (pagamento parcial de DEBT-46).
Movimenta código entre ficheiros. **Não altera semântica
observável**. Cria estrutura de submódulos sob
`01_core/src/rules/eval/`.

Foi a opção mais arriscada do plano: o `eval.rs` é o maior e o
mais central do projecto. Por isso, a execução é incremental —
cada bloco de movimentação é validado antes do próximo. Se
qualquer bloco falhar a compilação ou testes, parar e reportar
antes de prosseguir.

O sucesso deste passo valida ADR-0037 (a promoção fica para
Passo 96.2).

---

## Decisões formalizadas neste passo

- ADR-0037 — primeira aplicação. Regras 1–7 aplicadas.
- ADR-0033 — comportamento observável preservado (testes
  iguais).
- ADR-0036 — atomização preservada; extracções dos Passos 92–95
  continuam activas.

---

## Clusters propostos

Com base em análise do `eval.rs` actual (funções principais
identificadas na recolha de dados do Passo 96), a decomposição
proposta é:

```
01_core/src/rules/eval.rs (antes: 3780 linhas)
    ↓ transforma-se em:
01_core/src/rules/eval/
    mod.rs          — EvalContext, pub fn eval, dispatcher eval_expr
    markup.rs       — Expr::Text, Strong, Emph, Heading, Link, Raw,
                      list/enum items, Ref, Label
    math.rs         — todas as Expr::Math* (delegação para o eval
                      matemático existente)
    modules.rs      — Expr::ModuleInclude, Expr::ModuleImport, lógica
                      que interage com Route
    rules.rs        — Expr::SetRule, Expr::ShowRule, apply_show_rules,
                      intercept_content
    closures.rs     — Expr::Closure, Expr::FuncCall, apply_func,
                      apply_closure, eval_args
    control_flow.rs — Expr::Conditional, While, For, Break, Continue,
                      Return, eval_conditional, eval_while, eval_for
    bindings.rs     — Expr::Let, Expr::Ident, eval_let, scopes
```

Tamanho estimado após divisão: 7 ficheiros com 300–600 linhas
cada, mais `mod.rs` com 400–700 linhas (contém `EvalContext`,
`pub fn eval`, dispatcher).

**Ajustes possíveis durante execução**: se algum cluster ficar
muito pequeno (< 150 linhas), pode ser absorvido por outro
cluster próximo. Se algum ficar muito grande (> 800 linhas),
pode ser subdividido (ex: `closures.rs` em `closures.rs` +
`args.rs`).

Reportar ajustes no reporte final.

---

## Fase 0 — Preparação

### 0.1 — Verificação do estado actual

```bash
# Tamanho actual:
wc -l 01_core/src/rules/eval.rs

# Contagem de armos:
grep -c "Expr::\|SyntaxKind::\|Value::" 01_core/src/rules/eval.rs

# Funções top-level:
grep -n "^pub fn\|^fn\|^pub struct\|^struct\|^impl" 01_core/src/rules/eval.rs

# Testes no ficheiro (importante para decidir onde ficam após divisão):
grep -n "^\s*#\[test\]\|^\s*#\[cfg(test)\]" 01_core/src/rules/eval.rs | head -30
grep -c "^\s*#\[test\]" 01_core/src/rules/eval.rs
```

Reportar:
- Linhas totais confirmadas (esperado 3780).
- Número de funções top-level.
- Número de testes e se estão num `mod tests` único ou
  espalhados.

### 0.2 — Criar directório e `mod.rs` vazio

```bash
# Criar directório:
mkdir -p 01_core/src/rules/eval

# Mover o ficheiro actual temporariamente para dentro do directório:
git mv 01_core/src/rules/eval.rs 01_core/src/rules/eval/mod.rs
```

**Porquê `git mv`**: preserva história. O `eval.rs` original fica
como `eval/mod.rs`; subsequentes extracções movem pedaços de
`mod.rs` para os submódulos.

Verificar compilação:

```bash
cargo check --package typst-core 2>&1 | tail -15
```

Esperado: compila sem alteração (nenhum import muda porque o
`eval` continua a ser um único módulo — só a localização do
ficheiro mudou).

Se não compilar, algo depende do caminho exacto
`01_core/src/rules/eval.rs`. Investigar e reportar. Rollback:
`git mv` invertido.

---

## Fase 1 — Extracções incrementais

Cada fase extrai um cluster. **Após cada extracção**:

```bash
cargo check --package typst-core 2>&1 | tail -10
cargo test --package typst-core 2>&1 | tail -5
```

Se falhar qualquer uma, parar e reportar antes de prosseguir.

### Ordem recomendada (menos arriscado primeiro)

A ordem foi escolhida por critério de **acoplamento decrescente
com o dispatcher central**. Clusters mais isolados saem primeiro;
clusters mais entrelaçados saem por último.

1. **math** — mais isolado (delegação para eval matemático
   existente, pouco acoplamento).
2. **modules** — bem contido (interage só com `Route`).
3. **bindings** — relativamente isolado (`Expr::Let`, `Ident`).
4. **control_flow** — self-contained (`if`, `while`, `for`).
5. **markup** — médio (interage com `styles`, `show_rules`).
6. **rules** — maior acoplamento com dispatcher (`SetRule`,
   `ShowRule` afectam estado).
7. **closures** — último (mais ligado a `apply_func`,
   `apply_closure`, `eval_args`).

### 1.N — Extracção de cada cluster

Passos genéricos para cada cluster:

#### Sub-passo N.a — Identificar símbolos a mover

```bash
# Funções a extrair, seus spans no ficheiro:
grep -n "^fn eval_math\|^fn apply_math\|...resto do cluster..." \
    01_core/src/rules/eval/mod.rs
```

#### Sub-passo N.b — Criar o submódulo

Criar `01_core/src/rules/eval/<cluster>.rs` com cabeçalho:

```rust
//! Avaliação de <cluster>. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use super::*;

// conteúdo movido aqui
```

#### Sub-passo N.c — Mover funções

Copiar as funções do `eval/mod.rs` para
`eval/<cluster>.rs`. Decidir visibilidade:
- `pub(super)` — usada apenas pelo `mod.rs` (caso comum para
  funções internas).
- `pub(crate)` — usada por outros submódulos ou crates dentro
  do workspace.

Não usar `pub` directo (exposição global) sem razão concreta.

#### Sub-passo N.d — Declarar submódulo em `mod.rs`

No `01_core/src/rules/eval/mod.rs`:

```rust
mod <cluster>;
// ou pub(super) mod <cluster>; se for necessário expor ao eval
```

#### Sub-passo N.e — Actualizar o dispatcher `eval_expr`

No `eval_expr` (que vive em `mod.rs`), substituir o corpo do
armo pela chamada à função do submódulo:

```rust
match expr {
    Expr::Math(m) => math::eval_math(ctx, /* ... */, m),
    // ...
}
```

#### Sub-passo N.f — Mover testes correspondentes

Se havia testes específicos ao cluster, mover para o submódulo:

```rust
#[cfg(test)]
mod tests {
    // testes relativos a este cluster
}
```

Se os testes estavam num `mod tests` único do `eval.rs`, decidir:
- Mover testes especificamente relacionados com este cluster
  para `eval/<cluster>.rs::tests`.
- Manter testes transversais (que exercitam múltiplos clusters)
  no `eval/mod.rs::tests`.

#### Sub-passo N.g — Verificar

```bash
cargo check --package typst-core 2>&1 | tail -10
cargo test --package typst-core 2>&1 | tail -5
```

Ambos devem passar. Se falharem, rollback do cluster e
reportar.

### Progresso de cada fase

Reportar após cada cluster extraído:
- Cluster extraído.
- Tamanho do novo submódulo (linhas).
- Tamanho remanescente do `mod.rs`.
- Compilação e testes verdes.

---

## Fase 2 — Verificação final

### 2.1 — Tamanho dos novos ficheiros

```bash
wc -l 01_core/src/rules/eval/*.rs | sort -rn
```

Esperado: nenhum ficheiro > 800 linhas (Regra 2 da ADR-0037),
ou cada excepção tem comentário no topo (Regra 6).

Se algum ficheiro ultrapassa 800 sem justificativa, reavaliar
a decomposição.

### 2.2 — Dispatcher compacto

```bash
grep -c "Expr::\|SyntaxKind::\|Value::" 01_core/src/rules/eval/mod.rs
```

Esperado: muito menor que as 368 originais. A maioria dos
padrões `match` foi movida para submódulos. O dispatcher em
`mod.rs` terá tipicamente 20–40 padrões (um por armo de
`Expr`, cada armo é uma linha de delegação).

### 2.3 — Testes e linter

```bash
cargo test --workspace 2>&1 | tail -10
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado: 746 L1 + 174 L3 + 6 ignorados (inalterado ou com
pequena diferença explicável por testes renomeados). Zero
violations.

### 2.4 — Zero regressão de comportamento

```bash
# Testes que verificam comportamento crítico:
cargo test --package typst-core styles 2>&1 | tail -5
cargo test --package typst-core show_ 2>&1 | tail -5
cargo test --package typst-core depth 2>&1 | tail -5
cargo test --package typst-core cycle 2>&1 | tail -5
```

Todos devem passar.

---

## Fase 3 — Actualizar DEBT-46

Marcar primeiro checkbox:

```markdown
- [x] `eval.rs` reestruturado em submódulos por domínio (markup,
      math, modules, rules, closures, control_flow, bindings).
      Nenhum submódulo > 800 linhas ou cada excepção tem
      justificativa Regra 6 no topo. (Passo 96.1) **Concluído.
      Ver reporte para detalhes dos tamanhos finais.**
```

Não fechar o DEBT-46 (7 checkboxes pendentes). DEBT-46 continua
na Secção 1.

---

## Critérios de conclusão

- [ ] Directório `01_core/src/rules/eval/` criado com
      `mod.rs` e submódulos por cluster.
- [ ] `01_core/src/rules/eval.rs` já não existe como ficheiro
      (foi renomeado para `eval/mod.rs` e depois decomposto).
- [ ] 7 submódulos criados (ou ajustes documentados no reporte).
- [ ] Nenhum submódulo > 800 linhas, ou excepções com
      comentário Regra 6 no topo.
- [ ] Dispatcher em `mod.rs` compacto (armos de uma linha
      delegando para submódulos).
- [ ] `cargo test --workspace` passa com 746 L1 + 174 L3 + 6
      ignorados (ou diferença explicada).
- [ ] `crystalline-lint` → zero violations.
- [ ] Nenhum `unsafe` novo introduzido.
- [ ] Nenhum ADR alterado (ADR-0037 continua `PROPOSTO`;
      promoção é trabalho do Passo 96.2).
- [ ] DEBT-46 primeiro checkbox marcado.

---

## Ao terminar, reportar

Fase 0:
- Número de funções top-level encontradas.
- Número de testes no ficheiro e sua organização.

Fase 1 (por cluster extraído):
- Cluster, tamanho do novo submódulo, tamanho remanescente do
  `mod.rs` após cada extracção.
- Se algum cluster precisou de ajuste (fusão com outro,
  subdivisão).
- Se algum cluster encontrou fricção não antecipada.

Fase 2:
- Tamanhos finais de todos os ficheiros em
  `01_core/src/rules/eval/`.
- Contagem de armos `match` remanescente no `mod.rs`.
- Testes: contagem final, zero regressão.

Fase 3:
- Confirmação de DEBT-46 actualizado.

Tensões encontradas (relevante para Passo 96.2 — revisão do
ADR):
- Alguma Regra da ADR-0037 foi difícil de aplicar? (ex: ficheiro
  que não cabia num cluster único; delegação que forçou
  `pub(crate)` onde queria `pub(super)`).
- Alguma Regra demonstrou-se desnecessária ou demasiado estrita?
- Há fricções que a ADR não antecipou?

**Estas observações informam directamente o Passo 96.2**
(promoção ou ajuste do ADR-0037).

Go/No-Go para Passo 96.2:
- **Go incondicional** se a reestruturação concluiu limpamente
  e as 7 regras ADR-0037 foram aplicáveis. Passo 96.2 promove
  `PROPOSTO` → `EM VIGOR`.
- **Go com ajuste** se uma ou duas regras precisaram de
  interpretação flexível mas o princípio geral funcionou.
  Passo 96.2 promove com ajustes pontuais.
- **No-Go parcial** se uma regra fundamental se revelou
  impraticável. Passo 96.2 reescreve ou remove a regra antes
  de promover.
- **No-Go total** se a decomposição não foi possível (ex:
  dependências cíclicas intratáveis entre clusters). ADR-0037
  ajustada substancialmente antes de seguir para 96.3.
