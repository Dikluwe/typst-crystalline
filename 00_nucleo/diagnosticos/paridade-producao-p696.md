# Paridade produção — P696 — Sonda: suporte a plugins WASM (`plugin()`)

**Commit deste passo:** `__P696_COMMIT__` (preenchido no 2º commit; ver §Proveniência).
**Passo:** `00_nucleo/materialization/typst-passo-696.md`. **Tipo:** sonda (sem implementação).
**ADRs:** ADR-0108 (medir antes de decidir), ADR-0114 (sonda obrigatória para
funcionalidade nova nunca tocada), ADR-0107 (paridade com a linguagem).
**Dependências:** P694 (bloqueio `cetz` confirmado: `unknown variable: plugin`),
P678 (padrão de sonda para funcionalidades grandes).

---

## Proveniência das medições (regra de proveniência)

- **Estado:** working tree em `HEAD = f90f4dae5a9d464b281bb31e9e69e9cf82c0b827`
  (P695). Sonda **sem código**: `git diff --stat` vazio (working tree tracked
  limpa). Hora `2026-07-10T21:51:41-03:00`.
- **Binários:** vanilla `lab/typst-original/target/release/typst`; cristalino
  `target/debug/typst` (reconstruído em P695, 21:24).
- **Fixture `hello.wasm` (192 B):** gerada por `temp_p696/gen_wasm.py` (anexo §A),
  sem toolchain WASM (não havia `wat2wasm`/`wasm-tools`/`wasm32-*`). Reproduzível:
  `python3 gen_wasm.py` → `hello.wasm`. O scratch `temp_p696/` é apagado no fim;
  o módulo é auto-contido e descrito em §3 + §A.

---

## §1 — Protocolo confirmado no **código fonte** do vanilla

Fonte: `lab/typst-original/crates/typst-library/src/foundations/plugin.rs` (612 linhas).
Confirmado directamente (não só por pesquisa externa):

- **Entrada:** `#[func(scope)] pub fn plugin(engine, source: Spanned<DataSource>)
  -> SourceResult<Module>` (`plugin.rs:148-156`). Lê bytes via `source.load(engine.world)`
  (`DataSource`) e chama `Plugin::module(bytes)`. → `plugin("caminho.wasm")` ou
  `plugin(bytes(...))`; como devolve `Module`, também serve com `#import`.
- **Runtime:** `wasmi` (`plugin.rs:7`, `269-297`) — `Config`/`Engine`/`Module`/
  `Linker`/`Store`/`Instance`/`Memory`. `config.wasm_relaxed_simd(false)` para
  determinismo (`plugin.rs:272`). **Versão: `wasmi = 1.0.9`** (`Cargo.lock:3938-3943`).
- **Memória obrigatória:** se o módulo não exporta `memory` →
  `bail!("plugin does not export its memory")` (`plugin.rs:279-281`).
- **Imports do host** registados sob o módulo **`"typst_env"`** (`plugin.rs:283-297`):
  - `wasm_minimal_protocol_write_args_to_buffer(ptr)` (`plugin.rs:577-595`) —
    escreve os args concatenados no buffer do plugin; em falha de escrita regista
    `MemoryError{ write:true }`.
  - `wasm_minimal_protocol_send_result_to_host(ptr, len)` (`plugin.rs:598-612`) —
    lê `len` bytes de `ptr` para `CallData.output`; em falha regista
    `MemoryError{ write:false }`.
- **Despacho para módulo Typst:** `into_module` (`plugin.rs:366-380`) itera
  `module.exports()` e, para cada `ExternType::Func`, cria `PluginFunc` e liga-o
  ao `Scope` → `Module::anonymous(scope)`. Logo `p.nome` é uma função por export
  WASM do tipo função (inclui nomes como `_initialize`, mas esses falham na
  validação de assinatura quando chamados).
- **Validação de assinatura (lazy, na chamada):** todos os params `i32`, exactamente
  um resultado `i32`; contagem de args == nº de params (`plugin.rs:459-477`).
- **Passagem de args:** as **lengths** dos buffers são passadas como `i32`
  (`plugin.rs:479-491`); os bytes ficam em `CallData.args` e o host escreve-os
  quando o plugin chama `write_args_to_buffer`.
- **Código de retorno:** `0` = sucesso (output = bytes); `1` = erro
  (`"plugin errored with: {message}"`, UTF-8; se não UTF-8 →
  `"plugin errored, but did not return a valid error message"`); outro valor →
  `"plugin did not respect the protocol"` (`plugin.rs:508-519`). Trap WASM →
  `"plugin panicked: {err}"` (`plugin.rs:491-492`); acesso fora de limites →
  `"plugin tried to {read|write} out of bounds: pointer …"` (`plugin.rs:494-502`).
- **Cache / pureza:** `PluginFunc::call` e `PluginFunc::transition` são
  `#[comemo::memoize]` (`plugin.rs:220-231`) e `Plugin::module` também
  (`plugin.rs:261-263`). A pureza é **exigida ao autor** (não enforced); a cache
  vem "de graça" do comemo — que o cristalino **já usa**.
- **Transition API (estado):** `plugin.transition(func, *args)` (`plugin.rs:158-202`)
  + `Plugin::transition` (`plugin.rs:328-352`) com `Snapshot`/`restore` da memória
  (`plugin.rs:525-545`) e `fingerprint` u128 para hash/eq determinista entre
  "irmãos" transitados. Limitação documentada: só reflecte mudanças em **memória**,
  não em globais WASM (`plugin.rs:178-182`).

## §2 — Teste real com plugin mínimo (evidência executável)

`hello.wasm` (192 B; ver §A): exporta `memory` (1 página) e `hello() -> i32`,
com segmento de dados `"hello"` no offset 0. `hello()` chama
`send_result_to_host(0, 5)` e devolve `0`. Fixture `#let p = plugin("hello.wasm")\n#str(p.hello())`:

- **vanilla:** exit 0, saída `hello`. ✅ — confirma o protocolo end-to-end e a
  correcção do módulo gerado.
- **cristalino:** exit 1, `p696-hello.typ:1:10: error: unknown variable: plugin`.
  Confirma ausência total (não há builtin nem infra parcial).

## §3 — Estado exacto do cristalino hoje

- `grep` por `"plugin"`/`fn.*plugin`/`native_plugin` em `01_core`–`04_wiring`:
  **zero ocorrências**. `plugin` não existe como builtin, tipo ou módulo.
- `wasmi` **ausente** do `Cargo.lock` cristalino (não é dependência hoje).
- Reaproveitável: `World::read_bytes(current_file, path)`/`World::file(id)`
  (`01_core/src/contracts/world.rs:47,54`) e o pipeline de `#include`/`#import`
  (P679/P686) já resolvem e lêem ficheiros (incluindo absolutos `/...` dentro de
  pacotes). A leitura dos bytes do `.wasm` **não** é trabalho novo.

## §4 — `cetz` precisa só de plugins **puros** (decisivo para o âmbito)

- `cetz 0.5.2` carrega **um** plugin: `cetz-core/cetz_core.wasm` (343 529 B),
  via `plugin("../cetz-core/cetz_core.wasm")` e `plugin("/cetz-core/cetz_core.wasm")`
  (`src/aabb.typ:3`, `src/bezier.typ:7`, `src/draw/boolean.typ:8`,
  `src/lib/tree.typ:13`, `src/matrix.typ:3`).
- Imports de `cetz_core.wasm` (extraídos do binário): **apenas**
  `wasm_minimal_protocol_send_result_to_host` e
  `wasm_minimal_protocol_write_args_to_buffer`. **Sem** qualquer símbolo de
  `transition`.
- **Conclusão:** o âmbito mínimo para desbloquear `cetz` é **plugins puros** —
  `plugin()` + chamadas com `bytes` → `bytes`, sem transition API, snapshot ou
  fingerprint. A transition API fica **scope-out** inicial.

---

## §5 — Mapa dos seis níveis (com estimativa e camada sugerida)

| # | Nível | O que é | Camada | Estimativa | Risco/depende de |
|---|-------|---------|--------|-----------|------------------|
| 1 | Dependência `wasmi` | adicionar `wasmi = 1.0.9` ao workspace | manifestos | **XS** | policy: L1 whitelist vs L3 (ver §6) |
| 2 | Ler bytes do `.wasm` | `plugin("path")`/`plugin(bytes)` via `World::read_bytes`/`file` | L1 (builtin) + L3 | **S** | reutiliza `#include`/`#import` (P679/P686) |
| 3 | Protocolo de chamada | host `typst_env` (2 imports), `memory`, validação de assinatura, 0/1/erro, bounds | L3 (recomendado) | **M** | exactidão das mensagens (mecânica observável) |
| 4 | Tipo de retorno | `plugin()`→`Module` de `PluginFunc`; `p.f(*bytes)`→`bytes`; `Value::Bytes` | L1 | **M** | `PluginFunc` como `Value::Func`/cast; field access em módulo |
| 5 | Pureza / cache | `#[comemo::memoize]` no `call`/`module` | L1/L3 | **XS** | comemo já em uso; sem trabalho de cache manual |
| 6 | Transition API | snapshot/restore memória, fingerprint, `plugin.transition` | L3 | **L** | **scope-out** (cetz não precisa) |

Tamanho total do âmbito mínimo (1–5): **M** (com risco concentrado no nível 3 —
paridade exacta do protocolo e das mensagens). Nível 6 fica para passo próprio
e posterior, só se um pacote-alvo o exigir.

## §6 — Arquitetura proposta (recomendação)

`wasmi` é um runtime grande e impuro-adjacente (JIT/interprete, memória
mutável). **Recomendação:** viver em **L3**, atrás de uma trait
`PluginHost`/`WasmHost` injectada no pipeline (espelha fontes `fontdb` e
`hayagriva` em L3; mantém L1 sem um runtime WASM). L1 vê só o contrato:
`trait PluginHost { fn load(&self, bytes: &[u8]) -> Result<PluginModule, …>;
fn call(&self, …) -> Result<Vec<u8>, …> }` — tipos de fronteira em L1 (`Bytes`,
nomes `EcoString`), `wasmi::*` nunca atravessa a fronteira (evita V14 em L1).

**Alternativa (não recomendada para já):** `wasmi` em L1 com entrada em
`[l1_allowed_external]`. Mais simples de despachar (como vanilla), mas introduz
um runtime grande na camada pura e alarga a whitelist — contra o espírito de
"L1 mínimo". Só considerar se a injectão em L3 se provar inviável.

A cache/pureza (nível 5) mapeia naturalmente para `#[comemo::memoize]` (já em
L1/L3), sem mecanismo novo.

## §7 — Proposta de divisão em passos (padrão P614/P628/P678)

1. **P697 (S)** — builtin `plugin(path|bytes)` que lê bytes (via `World`) e, por
   agora, devolve erro controlado "plugins ainda não implementados" **ou** módulo
   vazio; fixa a sintaxe e a leitura (nível 2) + teste de leitura. L0 próprio.
2. **P698 (M)** — host `wasmi` puro em L3 atrás de trait `PluginHost`; implementa
   `typst_env` (2 imports), exigência de `memory`, validação de assinatura e
   códigos 0/1/erro com mensagens à vanilla (nível 3). Teste com `hello.wasm`.
3. **P699 (M)** — `plugin()` devolve `Module` de `PluginFunc`; `p.f(*bytes)`→`bytes`;
   `#[comemo::memoize]`; integração com eval/field-access (níveis 4–5). E2E:
   `#import plugin("hello.wasm"): hello`.
4. **P700 (S–M)** — validação contra `cetz_core.wasm`: chamar uma função pura
   real e `#import "@preview/cetz:0.5.2"`; medir até onde avança (provável novo
   bloqueio noutra funcionalidade — registar honesto, como P694).
5. **P701+ (L, só se necessário)** — transition API (nível 6), **fora** do
   caminho para `cetz`.

Cada passo com L0 + `crystalline-lint --fix-hashes` + testes primeiro, per a
Trava Arquitetural.

## §8 — Critério de âmbito mínimo (decisão da sonda)

Para desbloquear `cetz` (o objectivo real herdado de P694): **níveis 1–5**
(plugins puros). Nível 6 (transition) é **não necessário** — `cetz_core.wasm`
não o usa. Esta é a decisão que reduz o trabalho de "L" para "M" no caminho do
objectivo.

## §9 — Honestidade / riscos

- A paridade exacta das **mensagens** do host (nível 3) é mecânica-que-é-
  observável (ADR-0107): mensagens de erro são o observável e devem coincidir
  com o vanilla (citadas com `file:line` em §1). Implementação divergente aqui
  seria paridade falha ao nível da linguagem.
- Não foi testado `cetz_core.wasm` neste passo (sonda); a assinatura/exports
  concretos das suas funções só se confirmam em P700.
- A escolha L3-vs-L1 para `wasmi` é recomendação de sonda, não decisão de L0 —
  fica para o L0 de P697/P698.

---

## §A — Anexo: `gen_wasm.py` (reproduz `hello.wasm`, 192 B)

```python
def uleb(n):
    out=[]
    while True:
        b=n&0x7f; n>>=7
        out.append(b|0x80 if n else b)
        if not n: break
    return bytes(out)
def sleb(n):
    out=[]
    while True:
        b=n&0x7f; n>>=7
        done=(n==0 and (b&0x40)==0) or (n==-1 and (b&0x40)!=0)
        out.append(b if done else b|0x80)
        if done: break
    return bytes(out)
def vbytes(v): return uleb(len(v))+bytes(v)
def vec_raw(c): return uleb(len(c))+b"".join(c)
def section(i,p): return bytes([i])+uleb(len(p))+p
def name(s): b=s.encode(); return uleb(len(b))+b
I32=0x7f
def ft(p,r): return bytes([0x60])+vbytes([I32]*len(p))+vbytes([I32]*len(r))
t0,t1,t2=ft([I32],[]),ft([I32,I32],[]),ft([],[I32])
type_sec=section(1,vec_raw([t0,t1,t2]))
def imp(m,n,ti): return name(m)+name(n)+bytes([0])+uleb(ti)
import_sec=section(2,vec_raw([imp("typst_env","wasm_minimal_protocol_write_args_to_buffer",0),
                              imp("typst_env","wasm_minimal_protocol_send_result_to_host",1)]))
func_sec=section(3,vbytes([2]))                 # hello -> type 2 (func idx 2)
mem_sec=section(5,vbytes([0])+uleb(1))          # 1 mem, flags=0, min=1
def em(n,i): return name(n)+bytes([2])+uleb(i)  # export memory
def ef(n,i): return name(n)+bytes([0])+uleb(i)  # export func
export_sec=section(7,vec_raw([em("memory",0),ef("hello",2)]))
body=(bytes([0x41])+sleb(0)+bytes([0x41])+sleb(5)+bytes([0x10])+uleb(1)+
      bytes([0x41])+sleb(0)+bytes([0x0b]))
code_sec=section(10,vec_raw([uleb(len(uleb(0)+body))+uleb(0)+body]))
off=bytes([0x41])+sleb(0)+bytes([0x0b])
data_sec=section(11,vec_raw([uleb(0)+off+vbytes(list(b"hello"))]))
wasm=b"\x00asm\x01\x00\x00\x00"+type_sec+import_sec+func_sec+mem_sec+export_sec+code_sec+data_sec
open("hello.wasm","wb").write(wasm)
```
