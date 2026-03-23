# Passo 6 — eval() e Module

## Contexto

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md` (Opção C para comemo)
- `lab/typst-original/crates/typst-eval/Cargo.toml`
- `lab/typst-original/crates/typst-eval/src/lib.rs`

Pré-condição: Passos 4 e 5 concluídos — `parse()`, `Source` real,
AST tipada em L1.

Este passo tem uma decisão de camada que pode ser a mais difícil
da migração: `eval()` pertence a L1 ou L3?

---

## Diagnóstico obrigatório — typst-eval

**Parar aqui. Este diagnóstico determina o passo inteiro.**

```bash
# Dependências directas de typst-eval
cat lab/typst-original/crates/typst-eval/Cargo.toml

# Quantos ficheiros
find lab/typst-original/crates/typst-eval/src -name "*.rs" | wc -l
find lab/typst-original/crates/typst-eval/src -name "*.rs" | head -20

# Assinatura de eval()
grep -n "^pub fn eval\|^pub fn compile" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -10

# comemo em eval()
grep -n "Tracked\|comemo\|#\[comemo" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -20

# typst-library em eval
grep -n "typst_library\|typst_utils" \
  lab/typst-original/crates/typst-eval/src/lib.rs | head -10

# O tipo Module — onde é definido?
grep -rn "^pub struct Module\|^pub enum Module" \
  lab/typst-original/crates/ | head -5
```

**Reportar o output completo antes de continuar.**

---

## Decisão de camada (após diagnósticos)

### Se typst-eval depende de typst-library (provável)

`typst-library` tem 40+ externos incluindo I/O. `eval()` não pode
ser L1 sem trazer esses externos para o domínio.

**Decisão: eval() vai para L3.**

```
03_infra/src/eval/mod.rs   ← motor de avaliação
03_infra/src/eval/vm.rs    ← Vm, stack de scopes
```

`Module` e `Value` precisam de avaliação individual:
- Se são tipos de domínio puro → L1 (com stubs se necessário)
- Se dependem de `typst-library` → L3 também

### Se typst-eval é surpreendentemente limpo

Avaliar cada externo individualmente com o mesmo processo dos
passos anteriores: Opção C para dados, B3 para mecanismos,
ADR para cada decisão de `[l1_allowed_external]`.

**Não assumir. Os diagnósticos decidem.**

---

## Tarefa 1 — Module em L1 ou L3?

`Module` é o resultado de avaliar um ficheiro Typst — contém
`Content` e `Scope` (bindings definidos no ficheiro).

```bash
# Onde Module é definido e o que contém
grep -rn "pub struct Module" lab/typst-original/crates/ | head -5
grep -n "struct Module\|impl Module\|Content\|Scope\|Value" \
  lab/typst-original/crates/typst-eval/src/module.rs 2>/dev/null | head -20
```

Se `Module` contém apenas `Content` e `Scope` sem dependências
externas — é um tipo de domínio, vai para L1 (possivelmente como
stub até `Content` ser migrado).

Se `Module` tem dependências externas — avalia individualmente
ou vai para L3 como stub.

---

## Tarefa 2 — Prompt L0 (após decisão)

**Se eval() em L3:**
**Criar**: `00_nucleo/prompts/infra/eval.md`

```
Interface pública de L3:
pub fn eval(
    world: comemo::Tracked<dyn TrackedWorld>,
    source: &Source,
) -> SourceResult<Module>
```

**Se eval() em L1:**
**Criar**: `00_nucleo/prompts/rules/eval.md`

Documentar cada externo autorizado com justificação.

---

## Tarefa 3 — Migrar eval()

### Se L3:

**Destino**: `03_infra/src/eval/`

```toml
# 03_infra/Cargo.toml — adicionar dependências reais de typst-eval
# Cada linha é uma decisão — não copiar Cargo.toml original cegamente
```

A fronteira L1→L3 em eval():
- `Source` (L1) entra em L3 para avaliação
- `Module` (L1 ou L3) sai como resultado
- `World` (L1) é injectado via `TrackedWorld`

### Se L1:

Seguir o mesmo protocolo dos passos anteriores:
1. V14 sinaliza externo → avaliar
2. I/O → L3
3. Utilitário puro → ADR + `[l1_allowed_external]`
4. Nunca adicionar por conveniência

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra    # se eval() foi para L3
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Teste de integração mínimo (se eval() funcionar end-to-end):
```rust
#[test]
fn parse_eval_texto_simples() {
    // Criar um MockWorld mínimo
    // parse("Hello") → Source → eval() → Module com Content
    // Não precisa de fontes reais nem filesystem
}
```

---

## Ao terminar, reportar

- Decisão tomada (L1 ou L3) e justificação com base nos diagnósticos
- Se `Module`, `Value`, `Content` foram para L1 ou L3
- Externos novos que V14 sinalizou e como foram tratados
- Se o teste de integração parse→eval passou
- Número total de testes

Esta informação vai para ADR-0006 e para o Passo 7
(SystemWorld em L3 — implementação real de World).
