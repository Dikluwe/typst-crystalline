---
# P697 — Builtin `plugin()`: sintaxe e leitura de bytes (nível 2 de P696)

> **Passo:** 697
> **Data:** 2026-07-10
> **Foco:** Primeiro dos cinco passos propostos por P696. Fixa `plugin(path|bytes)` como builtin reconhecido, reaproveitando `World::read_bytes`/`#include`/`#import` (P679/P686) já existentes para ler os bytes do `.wasm`. Não implementa o runtime WASM em si — devolve erro controlado ou módulo vazio por agora, deixando o protocolo de chamada (nível 3) para P698.
> **Tipo:** Implementação directa. Sonda já feita em P696.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P696 (sonda e mapa completo), P679/P686 (mecanismo de leitura de ficheiros a reaproveitar).

---

## Contexto

`plugin(source: str | bytes) -> Module` no vanilla. Por agora, só a parte de resolução de `source` para bytes — o `Module` devolvido pode ser um valor provisório, desde que a sintaxe e a leitura de ficheiro estejam correctas e testadas.

---

## Implementação

### Reconhecer `plugin(...)` como builtin

Adicionar `plugin` ao scope global (`01_core/src/rules/eval/mod.rs` ou `stdlib/`), aceitando `Value::Str` (caminho) ou `Value::Bytes` (conteúdo directo), seguindo a mesma resolução de `DataSource` já confirmada por P696 (`plugin.rs:148-156` do vanilla).

### Ler os bytes

Reaproveitar `World::read_bytes(current_file, path)` (já usado por `#include`/`#import`, P679/P686) para o caso de caminho string. Para o caso `bytes(...)` directo, usar o valor já disponível.

### Devolver valor provisório

Por agora, devolver um erro claro ("plugin: runtime WASM ainda não implementado, ficheiro lido com sucesso") em vez de um `Module` funcional — isto confirma que a leitura funciona, sem fingir que o resto está pronto.

### Critério de fecho da implementação

- [ ] `plugin("caminho.wasm")` lê o ficheiro com sucesso (reaproveitando o mecanismo já existente), e produz o erro provisório claro, não um erro de "variável desconhecida".
- [ ] `plugin(bytes(...))` funciona da mesma forma com bytes directos.
- [ ] Ficheiro inexistente produz o mesmo tipo de erro já usado por `#include`/`#import` para esse caso.
- [ ] Caminho absoluto dentro de pacote (P686) funciona para `.wasm` também, testado com o caminho real usado por `cetz` (`/cetz-core/cetz_core.wasm`).

---

## Validação

```bash
cat > /tmp/p697-plugin-simples.typ <<'EOF'
#let p = plugin("hello.wasm")
EOF
./target/release/typst /tmp/p697-plugin-simples.typ /tmp/p697.pdf
echo "Exit code: $?"
```

Confirmar que o erro é o provisório claro ("runtime ainda não implementado"), não "variável desconhecida" — prova de que a leitura de ficheiro já funciona.

```bash
cat > /tmp/p697-inexistente.typ <<'EOF'
#let p = plugin("nao-existe.wasm")
EOF
./target/release/typst /tmp/p697-inexistente.typ /tmp/p697-erro.pdf
echo "Exit code: $?"
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `plugin(...)` reconhecido como builtin, sintaxe e leitura de ficheiro confirmadas.
- [ ] Erro provisório claro, distinto de "variável desconhecida" e de "ficheiro não encontrado".
- [ ] Caminho absoluto dentro de pacote testado com o caso real de `cetz`.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p697.md`, com hash do commit.
- [ ] L0 próprio para `plugin`, conforme a Trava Arquitetural.

---

## Próximo passo

P698 (nível 3 de P696): host `wasmi` em L3, atrás de uma trait `PluginHost`, implementando o protocolo `typst_env` completo, testado com o `hello.wasm` já construído por P696.
