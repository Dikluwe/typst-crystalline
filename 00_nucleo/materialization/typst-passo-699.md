---
# P699 — `plugin()` devolve `Module` real, `p.funcao(bytes)`, cache (níveis 4-5 de P696)

> **Passo:** 699
> **Data:** 2026-07-10
> **Foco:** Terceiro dos cinco passos propostos por P696. Liga o `PluginHost` (P698, já testado isoladamente em Rust) à linguagem Typst: `plugin("caminho.wasm")` passa a devolver um `Module` real, cujos campos são funções (`PluginFunc`) correspondentes aos exports do módulo WASM. `p.funcao(bytes1, bytes2, ...)` chama o plugin e devolve `bytes`. Cache de chamadas via `comemo::memoize`, seguindo o padrão já confirmado no vanilla por P696.
> **Tipo:** Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P698 (`PluginHost` funcional e testado isoladamente), P697 (leitura de bytes), P679 (mecanismo de `Value::Module`, já usado por `#import` de ficheiros locais).

---

## Contexto

Confirmado por P696 (`plugin.rs:366-380` do vanilla): `into_module` itera os exports do módulo WASM do tipo `Func` e cria um `PluginFunc` por cada um, ligado ao `Scope` do `Module` devolvido. `PluginFunc::call` é `#[comemo::memoize]` — chamadas repetidas com os mesmos argumentos são cacheadas, sem trabalho de cache manual.

---

## Implementação

### `native_plugin` passa a devolver `Module` real

Em vez do erro provisório de P697, `plugin(...)` agora:
1. Lê os bytes (já feito, P697).
2. Chama `PluginHost::load(bytes)` (P698), obtendo `PluginModuleId`.
3. Consulta os exports do módulo WASM (função nova em `PluginHost`, ou exposta a partir de `load`) para saber os nomes de função disponíveis.
4. Constrói um `Value::Module` cujo `Scope` liga cada nome de export a uma `PluginFunc`.

### `PluginFunc`

Um valor chamável (semelhante a `Value::Func`, mas específico), que ao ser chamado com argumentos `bytes`:
1. Valida que todos os argumentos são `Value::Bytes`.
2. Chama `PluginHost::call(module_id, func_name, args)`.
3. Devolve o resultado como `Value::Bytes`, ou propaga o erro.

### Cache via `comemo::memoize`

Aplicar `#[comemo::memoize]` (já usado noutras partes do cristalino, confirmado por P696 como já em uso) à chamada de `PluginFunc`, para que chamadas repetidas com os mesmos argumentos não repitam o trabalho — reaproveitando infra-estrutura já existente, não construindo cache manual nova.

### Injecção de `Arc<dyn PluginHost>` no eval

Confirmar o ponto certo de injecção (provavelmente no `Engine`, ao lado de `World`), e se `Send + Sync` é necessário nesta altura (P698 já apontou isto como possível ponto de atenção).

### Critério de fecho da implementação

- [ ] `plugin("hello.wasm")` devolve um `Module` com o campo `hello`.
- [ ] `p.hello()` chama o plugin e devolve `bytes`, testado com `hello.wasm` (P696/P698).
- [ ] `#import plugin("hello.wasm"): hello` funciona, reaproveitando o mecanismo de `#import` de módulos já existente (P679/P683).
- [ ] Cache confirmada — chamar a mesma função com os mesmos argumentos duas vezes não invoca o `PluginHost` duas vezes (testável instrumentando temporariamente, ou confirmando por tempo se a segunda chamada for muito mais rápida).

---

## Validação

```bash
cat > /tmp/p699-plugin.typ <<'EOF'
#let p = plugin("hello.wasm")
#str(p.hello())
EOF
./target/release/typst /tmp/p699-plugin.typ /tmp/p699.pdf
pdftotext /tmp/p699.pdf -
```

Comparar com o resultado do vanilla já confirmado por P696 ("hello").

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `plugin()` devolve `Module` real, testado com `hello.wasm`.
- [ ] `p.funcao(bytes)` funciona, comparado com o vanilla.
- [ ] `#import plugin(...): item` funciona.
- [ ] Cache confirmada.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p699.md`, com hash do commit.

---

## Próximo passo

P700 (validação real): chamar uma função real de `cetz_core.wasm`, e tentar `#import "@preview/cetz:0.5.2"` de novo, medindo até onde avança — incluindo, se `cetz` finalmente renderizar, uma medição de desempenho (dado que P698 já assinalou "instância fresca por chamada" como um custo a rever se `cetz` chamar o plugin muitas vezes por documento).
