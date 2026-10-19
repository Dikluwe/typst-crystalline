---
# P698 — Host `wasmi` em L3, protocolo `typst_env` completo (nível 3 de P696)

> **Passo:** 698
> **Data:** 2026-07-10
> **Foco:** Segundo dos cinco passos propostos por P696. Implementa o runtime `wasmi` em L3, atrás de uma trait `PluginHost`, com o protocolo `typst_env` completo (`wasm_minimal_protocol_write_args_to_buffer`, `wasm_minimal_protocol_send_result_to_host`), validação de assinatura, códigos de retorno 0/1/erro, e as mensagens de erro confirmadas por P696 contra o código fonte do vanilla. Testado com o `hello.wasm` já construído por P696.
> **Tipo:** Implementação directa. Sonda já feita em P696, com `file:line` do vanilla.
> **Tamanho:** M. O núcleo do trabalho desta linha.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P696 (protocolo mapeado com `file:line`, `hello.wasm` reproduzível), P697 (leitura de bytes já funcional).

---

## Contexto

Confirmado por P696, com `file:line` do vanilla (`plugin.rs`):

- Runtime `wasmi 1.0.9`, `wasm_relaxed_simd(false)` para determinismo.
- Módulo tem de exportar `memory`, senão erro "plugin does not export its memory".
- Dois imports do host sob o módulo `"typst_env"`.
- Validação de assinatura lazy: todos os parâmetros `i32`, exactamente um resultado `i32`.
- Códigos de retorno: `0` sucesso, `1` erro (UTF-8), outro valor "plugin did not respect the protocol"; trap → "plugin panicked: {err}"; acesso fora de limites → "plugin tried to {read|write} out of bounds: pointer …".

---

## Implementação

### Dependência `wasmi`

Adicionar `wasmi = "1.0.9"` como dependência de `03_infra` (L3), seguindo a recomendação de arquitectura de P696 (L3, não L1).

### Trait `PluginHost` em L1

```rust
// Esboço, a confirmar contra a estrutura real:
pub trait PluginHost {
    fn load(&self, bytes: &[u8]) -> Result<PluginModuleId, PluginError>;
    fn call(&self, module: PluginModuleId, func_name: &str, args: &[Bytes]) -> Result<Bytes, PluginError>;
}
```

Tipos de fronteira (`PluginModuleId`, `PluginError`) vivem em L1; `wasmi::*` nunca atravessa a fronteira.

### Implementação em L3

`WasmiPluginHost` implementa `PluginHost`, usando `wasmi::Engine`/`Module`/`Linker`/`Store`/`Instance`/`Memory`, replicando exactamente o comportamento já confirmado por P696:

- Exigir `memory` exportada, com a mensagem exacta do vanilla.
- Registar os dois imports `typst_env`.
- Validar assinatura das funções exportadas (todos `i32` nos parâmetros, um `i32` no resultado).
- Implementar `write_args_to_buffer`/`send_result_to_host` conforme o protocolo.
- Traduzir códigos de retorno e erros para as mensagens exactas já confirmadas por P696.

### Critério de fecho da implementação

- [ ] `PluginHost` implementado, testado isoladamente (sem passar ainda pela sintaxe Typst completa — pode ser testado directamente em Rust).
- [ ] Testado com `hello.wasm` (P696), confirmando que a chamada devolve os bytes certos.
- [ ] Mensagens de erro testadas uma a uma contra as confirmadas por P696 (memória em falta, assinatura errada, trap, fora de limites, código de retorno inválido).

---

## Validação

```bash
cargo test -p typst-infra plugin_host
```

Testes directos em Rust, sem passar pela sintaxe Typst (essa integração fica para P699).

Reconstruir o `hello.wasm` de P696 (usando o script reproduzível do anexo) e confirmar que `PluginHost::call` devolve os bytes "hello".

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `wasmi` adicionado como dependência de L3.
- [ ] `PluginHost` (trait em L1, implementação em L3) construído e testado isoladamente.
- [ ] `hello.wasm` chamado com sucesso através de `PluginHost`.
- [ ] Todas as mensagens de erro confirmadas por P696 replicadas e testadas.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p698.md`, com hash do commit.

---

## Próximo passo

P699 (níveis 4-5 de P696): `plugin()` passa a devolver um `Module` real, ligando `PluginHost` à sintaxe Typst (`p.funcao(bytes)`), com cache via `comemo::memoize`.
