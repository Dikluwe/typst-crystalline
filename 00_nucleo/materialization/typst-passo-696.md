---
# P696 — Sonda: suporte a plugins WASM (`plugin()`)

> **Passo:** 696
> **Data:** 2026-07-10
> **Foco:** `plugin()` está confirmado como ausente (P694, bloqueio real de `cetz` 0.5.2). Este passo é só sonda: mapear o protocolo exacto ("wasm-minimal-protocol"), a escolha de runtime do vanilla (`wasmi`), e o estado actual do cristalino, antes de qualquer código.
> **Tipo:** Sonda directa. Sem implementação neste passo.
> **Tamanho:** M para a sonda; a implementação, se avançar, é provavelmente L.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — funcionalidade nova, nunca tocada; sonda obrigatória.
> **Dependências:** P694 (onde o bloqueio de `cetz` foi confirmado), P678 (padrão de sonda para funcionalidades grandes).

---

## Contexto já confirmado por pesquisa externa (a validar contra o código fonte do vanilla, não aceitar sem confirmar)

- **Runtime:** `wasmi`, um interpretador WASM em Rust puro (não `wasmtime`). Escolhido pelos mantenedores do Typst por ser leve e não precisar de compilação nativa.
- **Protocolo ("wasm-minimal-protocol"):**
  - Um módulo plugin exporta funções que recebem N inteiros de 32 bits (os comprimentos de N argumentos de bytes) e devolvem um inteiro de 32 bits.
  - A função importa duas funções do anfitrião: `wasm_minimal_protocol_write_args_to_buffer(ptr)` (escreve os argumentos no buffer que o plugin alocou) e `wasm_minimal_protocol_send_result_to_host(ptr, len)` (envia o resultado de volta).
  - Devolver `0` significa sucesso; `1` significa erro, e o buffer enviado é interpretado como mensagem de erro UTF-8.
  - Funções de plugin têm de ser **puras** — o mesmo input produz sempre o mesmo output; o Typst pode cachear chamadas repetidas.
  - Existe uma "transition API" para plugins com estado (impuros), mais avançada — candidata a scope-out inicial.
- **Sintaxe Typst:** `#let p = plugin("caminho.wasm")`, depois `p.nome_funcao(bytes(a), bytes(b))` devolve `bytes`. Como `plugin()` devolve algo parecido com um módulo, também pode ser usado com `import`.
- **Segurança:** plugins correm isolados — sem ficheiros, sem print, dentro da sandbox do WASM.

---

## Sonda

### Confirmar o protocolo directamente no código fonte do vanilla

```bash
grep -rn "wasm_minimal_protocol\|struct Plugin\|fn plugin" lab/typst-original/crates/typst-library/src/foundations/plugin.rs 2>/dev/null | head -30
grep -n "wasmi" lab/typst-original/Cargo.toml lab/typst-original/crates/typst-library/Cargo.toml 2>/dev/null
```

Confirmar a versão exacta de `wasmi` usada, e a estrutura interna de `Plugin` (como o módulo WASM é carregado, instanciado, e como as funções são despachadas).

### Testar directamente com um plugin mínimo, real

```bash
cat > /tmp/p696-plugin.typ <<'EOF'
#let p = plugin("hello.wasm")
#str(p.hello())
EOF
web_search "typst wasm-minimal-protocol hello.wasm example download"
```

Obter (ou construir, se necessário, com as ferramentas disponíveis) um `.wasm` mínimo de exemplo, e confirmar que o vanilla local o executa correctamente.

### Confirmar o estado exacto do cristalino hoje

```bash
grep -rn "\"plugin\"\|fn.*plugin" 01_core/src/rules/eval/mod.rs 01_core/src/rules/stdlib/*.rs | head -10
./target/release/typst /tmp/p696-plugin.typ /tmp/p696-cristalino.pdf 2>&1
```

Confirmar a mensagem de erro exacta, e se há já alguma infra-estrutura parcial (mesmo que não funcional).

### Mapear o alcance do trabalho, nível a nível

1. **Dependência `wasmi`:** confirmar se pode entrar em L3 (I/O, permitido) sem tocar L1.
2. **`plugin("caminho.wasm")`:** ler bytes do ficheiro (reaproveitar mecanismo de `#import`/`#include` já existente, P679/P686), carregar módulo WASM.
3. **Protocolo de chamada:** implementar as duas funções importadas pelo host, o despacho de argumentos como bytes, e a leitura do resultado/erro.
4. **Tipo de retorno em Typst:** `plugin(...)` devolve um valor que se comporta como módulo, com funções que aceitam/devolvem `bytes`.
5. **Pureza/cache:** confirmar se é preciso implementar cache de chamadas repetidas já nesta fase, ou se pode ficar para depois (funcionalmente correcto sem cache, só menos eficiente).
6. **Transition API (plugins com estado):** confirmar se `cetz` precisa disto, ou só de plugins puros — se só precisar de puros, isto fica de fora do âmbito inicial.

### Confirmar se `cetz_core.wasm` (a dependência real que bloqueia `cetz`) usa a transition API ou só chamadas puras

```bash
find ~/.cache/typst/packages/preview/cetz/0.5.2 -name "*.wasm"
grep -rn "plugin(" ~/.cache/typst/packages/preview/cetz/0.5.2/src/*.typ 2>/dev/null | head -10
```

Confirmar como `cetz` invoca o plugin, e se usa chamadas simples ou a API de transição — isto decide se o âmbito inicial (só funções puras) já é suficiente para desbloquear `cetz`, ou se é preciso mais.

### Critério de fecho da sonda

- [ ] Protocolo confirmado directamente no código fonte do vanilla, não só por pesquisa externa.
- [ ] Testado com um plugin `.wasm` mínimo real, confirmando o comportamento do vanilla.
- [ ] Estado exacto do cristalino confirmado.
- [ ] Os seis níveis do trabalho mapeados, com estimativa de tamanho para cada um.
- [ ] Confirmado se `cetz` precisa da transition API ou só de chamadas puras — decide o âmbito mínimo necessário para desbloquear o objectivo real.

---

## Decisão

Este passo não implementa nada. Produz o mapa e uma proposta de divisão em passos menores, seguindo o mesmo padrão já usado para outras funcionalidades grandes (P614, P628, P678).

---

## Critério de fecho do passo

- [ ] Sonda completa, com evidência directa (código fonte, teste real), não só a pesquisa externa já feita.
- [ ] Mapa dos seis níveis, com estimativa de tamanho para cada um.
- [ ] Confirmado o âmbito mínimo necessário para desbloquear `cetz` especificamente.
- [ ] Proposta de divisão em passos menores.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p696.md`.
