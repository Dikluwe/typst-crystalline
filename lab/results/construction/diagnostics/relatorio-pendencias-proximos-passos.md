# Relatório de pendências — próximos passos (pós-P762)

**Data:** 2026-07-15  
**Commit base:** `82e84356fb0c80a58f0da21c28a5a0c476937d32`  
**Working tree:** modificado — deleções em `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md`; ficheiros untracked de materialização P589–P762, diagnósticos e ADRs.  
**Último passo fechado:** P762  
**Fontes:** `00_nucleo/handoff-novo-chat-p762.md`, `00_nucleo/estado-geral-p695.md`, `00_nucleo/diagnosticos/achados-adiados-cetz.md`, medição da lente de 2026-07-15 (`/tmp/comparar-typst-itens.json`).

---

## 1. Resumo executivo

O projecto está num estado avançado de migração do núcleo da linguagem, mas a **lente de comparação mecânica distorce a percepção**: a grande maioria dos "só-vanilla" são símbolos de áreas já migradas que a lente não consegue parear por diferença de path/nome (reescrita de `Content` como enum, `*Elem` structs → variants, atomização em camadas L1–L4).

O que falta **de facto** divide-se em três grupos:

1. **Bloqueio funcional imediato** — `cetz` (e qualquer pacote com plugins WASM) não produz PDF porque `plugin()` / `PluginHost` ainda não está ligado à linguagem.
2. **Continuidade de funcionalidades já em curso** — download de pacotes, resolução `"@preview/nome"` sem versão, e pequenos débitos de `str`/regex.
3. **Grandes áreas por iniciar** — HTML, SVG, PNG, IDE/LSP, PDF Tagged/UA, fontes emoji COLR/CPAL, Type1/PostScript.

---

## 2. Estado actual (o que está fechado ou estável)

| Área | Estado | Último passo |
|---|---|---|
| Core da linguagem (eval, closures, destructuring, assignment, short-circuit) | Fechado | P728, P733 |
| `#import` de ficheiros locais e pacotes offline | Fechado | P678–P695 |
| `sys.inputs`, namespaces (`calc`, `math`, `sym`, `emoji`, `pdf`) | Fechado | P694, P731, P735 |
| Métodos de `str` (lista completa confirmada) | Fechado | P689–P693 |
| Métodos de `color`/`gradient` | Fechado | P736, P742, P744 |
| `array.slice`, `line(end:)`, `block(fill/stroke: none)`, hline/vline | Fechado | P730, P726, P739A |
| Fontes variáveis (peso/estilo) | Fechado | P666–P669 |
| Layout vertical (top-edge/bottom-edge/leading) | Fechado | P762 |
| CJK/Thai básico | Fechado | P755–P759 |
| `cetz` com desenho vectorial simples | Paridade visual alcançada | P744 |
| Desempenho | Estável (≤1,3× vanilla, por vezes mais rápido) | P657–P677 |

Validação técnica no P762:
- `cargo build --release` OK
- `cargo test --workspace` OK: 4121 passed
- `crystalline-lint .` zero violations

---

## 3. O que a lente diz — e o que isso significa

Medição de 2026-07-15 contra `lab/typst-original` (Typst 0.15.0):

| Métrica | Valor |
|---|---:|
| Pareados | 1.738 |
| Só-vanilla (reais) | 5.493 |
| Só-cristalino (reais) | 1.665 |
| Ambíguos | 134 |

### Decomposição do só-vanilla

| Categoria | Itens | % |
|---|---:|---:|
| Crates parcialmente migrados (provável arquitetura/nomenclatura) | 4.783 | **87,1%** |
| Crates inteiramente não migrados | 696 | **12,7%** |
| CLI / macros | 14 | 0,3% |

Dentro dos crates parciais, **84,6%** dos só-vanilla estão em módulos que já têm pelo menos 1 item pareado, indicando migração real com reescrita de símbolos.

### Conclusão da análise

- **~74% do só-vanilla total** são símbolos de áreas já migradas que a lente não pareia por path.
- **~13%** são potencialmente funcionalidade em falta dentro de áreas parciais.
- **~13%** são crates inteiros ainda não migrados.

A lente **não** deve ser usada como medida de "percentagem de migração". O sinal útil é o cruzamento com a coluna declarada (ver `lab/mapa-migracao/README.md`).

---

## 4. Pendências identificadas

### 4.1 Bloqueio imediato — plugins WASM (`cetz`)

**Problema:** `cetz` 0.5.2 usa `plugin()` para carregar módulos WASM. O cristalino tem infraestrutura de host (`WasmiPluginHost`, P698) mas ainda **não ligou** `plugin()` à linguagem.

**Impacto:** Qualquer documento que use plugins WASM falha. É o bloqueio que impede validar `cetz` até ao fim.

**Estado dos L0:**
- `00_nucleo/prompts/engine/stdlib/plugin.md` — existe e está actualizado (hash `9b34a5db`).
- `00_nucleo/prompts/infra/plugin_host.md` — existe e está actualizado (hash `386e15d7`).
- `00_nucleo/prompts/entities/plugin_func.md` — existe e está actualizado (hash `21230a71`).
- `00_nucleo/prompts/infra/system-world.md` — existe, prevê `with_plugin_host` (P699).
- `00_nucleo/prompts/contracts/plugin_host.md` e `00_nucleo/prompts/contracts/world.md` — contratos existem.

**Próximo passo sugerido:** P763 — implementar P699 (ligar `PluginHost` à linguagem). Os L0 já existem; falta executar a implementação e validar contra `cetz`.

---

### 4.2 Funcionalidades médias em curso

#### 4.2.1 Download de pacotes (P-γ de P678)

**Problema:** Pacotes `@preview` só funcionam se já estiverem em `~/.cache/typst/packages`. O vanilla descarrega automaticamente do registo oficial.

**Impacto:** Utilizador tem de pre-popular a cache manualmente.

**Estado do L0:** `00_nucleo/prompts/infra/system-world.md` menciona "download ainda não implementado (ver P-γ de P678)" mas **não existe prompt L0 dedicado** ao downloader.

**Próximo passo sugerido:** Escrever L0 para download de pacotes antes de codificar. Envolve rede em L3, registo de pacotes, cache, e semântica de erro.

#### 4.2.2 Resolução `"@preview/nome"` sem versão (P-δ de P678)

**Problema:** `#import "@preview/cetz"` sem versão falha. O vanilla resolve para a versão mais recente instalada/cacheada.

**Impacto:** Incompatível com documentos Typst comuns.

**Estado do L0:** Não existe prompt dedicado.

**Próximo passo sugerido:** Escrever L0 após decidir se resolve offline (mais recente na cache) ou online (requer download).

#### 4.2.3 Débitos pequenos registados

| Item | Onde | Prioridade |
|---|---|---|
| `polygon` com vértices `Ratio` (`50%`) | `achados-adiados-cetz.md` | Baixa — sem consumidor em `cetz` |
| Ordem de erros de argumento extra (`f(1, z: 2, 3)`) | `achados-adiados-cetz.md` | Baixa — caso de canto |
| `str.replace` / flags de regex | `estado-geral-p695.md` | Média |
| Auditoria sistemática da stdlib inteira para divergências de linguagem | `estado-geral-p695.md` | Alta (risco de mais surpresas) |

---

### 4.3 Grandes áreas ainda não iniciadas

| Área | Itens só-vanilla (aprox.) | Notas |
|---|---|---|
| Exportação HTML | 303 | Crate `typst_html` inteiro |
| Exportação SVG | 101 | Crate `typst_svg` inteiro |
| Renderização PNG/raster | 37 | Crate `typst_render` |
| IDE / LSP | 97 | Crate `typst_ide` |
| Documentação (`typst docs`) | 98 | Crate `typst_docs` |
| PDF Tagged / PDF-UA | 25+ | `typst_library::pdf::accessibility` |
| Fontes emoji (COLR/CPAL) | — | scope-out consciente |
| Fontes Type1/PostScript | — | scope-out consciente |
| Object streams / compressão PDF | — | scope-out consciente |
| Escrita vertical CJK | — | confirmado ausente no vanilla também |

---

### 4.4 Débitos técnicos visíveis (warnings)

`cargo build` gera dezenas de warnings que não são violações do `crystalline-lint` mas indicam qualidade a limpar:

- Imports não usados em `typst-core` e `typst-infra`
- Código morto (`dead_code`)
- Uso de variantes/fields deprecados (`FrameItem::Text`)
- Doc comments aplicados a statements

**Sugestão:** Não tratar como próximo passo prioritário, mas incluir num passo de limpeza após P763.

---

## 5. Próximos passos propostos

### Passo imediato: P763 — Plugins WASM

**Objectivo:** Fechar a cadeia `plugin()` → `Module` → chamada de export, validando `cetz` com plugins.

**L0 necessário:** Já existe (`rules/stdlib/plugin.md`, `entities/plugin_func.md`, `infra/plugin_host.md`, `infra/system-world.md`).

**Tarefas:**
1. Implementar `PluginFunc` em `01_core/src/entities/plugin_func.rs`.
2. Adicionar `FuncRepr::Plugin` em `01_core/src/entities/func.rs`.
3. Implementar despacho em `01_core/src/engine/eval/closures.rs`.
4. Actualizar `native_plugin` em `01_core/src/engine/stdlib/plugin.rs` para devolver `Value::Module`.
5. Instalar host no `SystemWorld` via `with_plugin_host` em `03_infra/src/world.rs`.
6. Encadear no CLI em `04_wiring/src/main.rs`.
7. Escrever testes unitários e E2E.
8. Validar com `cetz` real (caso de teste com plugin WASM).

**Critério de fecho:** `cetz` com plugins compila e produz PDF com paridade visual aceitável; `cargo test --workspace` verde; `crystalline-lint .` zero violations.

---

### Passo seguinte: P764 — Download de pacotes

**Objectivo:** Permitir que o cristalino descarregue pacotes `@preview` automaticamente.

**L0 necessário:** **Não existe**. Deve ser redigido antes de qualquer código.

**Decisões a tomar no L0:**
- Registo de pacotes a usar (oficial Typst, mirror, configurável?)
- Cache: `~/.cache/typst/packages` apenas ou também `~/.local/share/typst/packages`?
- Semântica de concorrência/locking
- Erros observáveis (mensagens exactas)
- Integração com `SystemWorld::resolve_package`

---

### Passo seguinte: P765 — Resolução de versão implícita

**Objectivo:** `#import "@preview/nome"` resolve para a versão mais recente disponível.

**L0 necessário:** **Não existe**.

**Dependência:** P764 (download) ou, no mínimo, enumeração da cache local.

---

### Passo de auditoria: P766 — Varredura sistemática da stdlib

**Objectivo:** Aplicar a metodologia de P663/P664 ao resto da stdlib (não só às funções já tocadas) para encontrar divergências de linguagem.

**L0 necessário:** **Não existe**.

**Motivação:** Vários bugs graves foram encontrados por acidente ao perseguir `cetz`. Uma varredura proactiva reduz o risco de surpresas.

---

## 6. Dependências e riscos

| Risco | Impacto | Mitigação |
|---|---|---|
| `wasmi 1.0.9` pode não ter `Linker: Send` no ambiente actual | Build de P763 falha | Prompt L0 `plugin_host.md` já prevê fallback: reconstruir linker dentro de `call` |
| Colisão de cache `comemo::memoize` entre testes | Testes flaky | L0 `plugin_func.md` prevê nomes de export únicos por teste |
| `cetz` pode revelar mais bugs de semântica após P763 | Próximos passos imprevisíveis | Manter disciplina de sondagem (ADR-0108, ADR-0114) |
| Download de pacotes envolve rede e políticas de confiança | Decisão arquitetural | Escrever L0 explícito; não improvisar em L3 |

---

## 7. Recomendação

1. **Executar P763 imediatamente** — é o único passo cujos L0 já existem e cujo fecho desbloqueia a validação completa de `cetz`.
2. **Não iniciar P764/P765 sem L0 novo** — o protocolo exige prompt antes de código (regra de ouro do `AGENTS.md`).
3. **Manter a lente como detector de deriva, não como medida de progresso** — os números só-vanilla são dominados pela arquitetura cristalina.
4. **Após P763, considerar P766 antes de grandes features** — o risco de divergências de linguagem escondidas é alto e o custo de os encontrar tarde é maior.

---

## 8. Medição de proveniência

- Lente: `lente --comparar --antes lab/typst-original --depois .`, commit `82e84356fb0c80a58f0da21c28a5a0c476937d32`, working tree modificado (deleções em `00_nucleo/0.15.0.typ` e `00_nucleo/testing/fontes-padrao-teste.md`).
- Estado P762: `00_nucleo/diagnosticos/paridade-producao-p762.md`.
- Handoff: `00_nucleo/handoff-novo-chat-p762.md`.
- Estado geral: `00_nucleo/estado-geral-p695.md`.
- Achados adiados: `00_nucleo/diagnosticos/achados-adiados-cetz.md`.
