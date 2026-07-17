# P765 — Sonda inicial: metodologia de varredura sistemática da stdlib

**Tipo**: Diagnóstico / Sonda de âmbito  
**Data**: 2026-07-15  
**Passo**: 765

---

## Proveniência da medição

| Item | Valor |
|------|-------|
| HEAD cristalino | `82e84356fb0c80a58f0da21c28a5a0c476937d32` |
| Vanilla (`lab/typst-original/target/release/typst`) | `typst 0.15.0 (969087ec)` |
| Cristalino (`./target/release/typst`) | `typst 0.1.0` |
| Data/hora da sonda | `2026-07-15T14:10:39-03:00` (início) |
| Metodologia base | P663 (auditoria por auto-rotulagem) e P664 (teste directo de argumentos nomeados) |
| Entrada auxiliar | Diagnóstico `lente-falta-migrar-2026-07-15.md` (achados reais: `#title()` e `repr(Symbol)` variantes) |

## Metodologia confirmada por leitura directa

Releu-se:

- `00_nucleo/materialization/typst-passo-663.md` — auditoria por auto-rotulagem (palavras como "extensão", "capacidade nova").
- `00_nucleo/materialization/typst-passo-664.md` — teste directo de cada argumento nomeado contra o vanilla.

Ambos compartilham o princípio: **não confiar em relatórios anteriores**, confirmar directamente contra o binário vanilla.

## Amostra executada

A amostra sugerida no passo (`typst_library::diag`) é infraestrutura Rust de propagação de erros, não símbolo de língua Typst (conforme já assinalado no diagnóstico da lente). Por isso, a amostra foi complementada com os dois achados reais de nível-de-língua do diagnóstico da lente.

### 1. `#title()` — elemento de markup

Documento de teste:

```typ
#title("Hello")
```

Resultado:

| Compilador | Resultado |
|---|---|
| Vanilla | `exit=0` (compila com sucesso) |
| Cristalino | `error: unknown variable: title` |

Classificação: **bug real de linguagem** — `#title()` existe no vanilla e falta no cristalino.

### 2. `symbol` — construtor e modificadores

#### 2.1 Construtor `symbol("α")`

Documento de teste:

```typ
#repr(symbol("α"))
```

Resultado:

| Compilador | Resultado |
|---|---|
| Vanilla | `exit=0` |
| Cristalino | `error: type symbol does not have a constructor` |

Classificação: **bug real de linguagem** — o construtor `symbol(...)` existe no vanilla.

#### 2.2 Modificadores via field access (`sym.arrow.r.filled`)

Documento de teste:

```typ
#repr(sym.arrow.r)
#repr(sym.arrow.r.filled)
```

Resultado:

| Compilador | Resultado |
|---|---|
| Vanilla | `exit=0` |
| Cristalino | `error: field access não suportado em symbol` |

Classificação: **bug real de linguagem** — o cristalino não suporta modificadores de símbolo.

#### 2.3 `repr(sym.alpha)` simples

Documento de teste:

```typ
#repr(sym.alpha)
```

Resultado:

| Compilador | Resultado |
|---|---|
| Vanilla | `exit=0` |
| Cristalino | `exit=0` |

Classificação: **paridade** — caso simples já funciona.

### 3. Mensagem de erro como observável

Documento de teste:

```typ
#import "@preview/xyzdoesnotexist:1.0.0": x
```

Resultado:

| Compilador | Mensagem |
|---|---|
| Vanilla | `error: package not found (searched for @preview/xyzdoesnotexist:1.0.0)` |
| Cristalino | `error: pacote '@preview/xyzdoesnotexist:1.0.0' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)` |

Classificação: **diferença no observável mecânico-legítimo** — o texto da mensagem de erro diverge (língua e conteúdo). Conforme CLAUDE.md, mensagens de erro são observáveis legítimos de paridade.

## Custo medido na amostra

| Caso | Tempo aproximado | Esforço |
|---|---|---|
| `#title()` | ~2 min | Manual: escrever documento, compilar ambos, classificar |
| `symbol` construtor + modificadores | ~5 min | Manual: várias sintaxes testadas até confirmar a real divergência |
| Mensagem de erro de pacote | ~1 min | Manual: forçar erro conhecido |

Total por elemento isolado: **2–5 minutos**. A varredura completa de toda a stdlib seria impraticável num único passo; o trabalho deve ser dividido em lotes pequenos.

## Decisões registadas no L0

Todas as decisões foram registadas em `00_nucleo/prompts/engine/stdlib_audit_methodology.md` (hash `0683fad7`):

| Decisão | Resolução |
|---|---|
| Âmbito | Namespaces parciais — começar pelos itens `ausente`/`parcial` do Inventário 148 e achados da lente (`#title()`, `symbol`) |
| Critério de comparação | Resultado do documento + texto de mensagem de erro (quando a mecânica é o observável) |
| Formato de registo | `00_nucleo/diagnosticos/achados-stdlib-<lote>.md` por lote |
| Geração de casos | Manual para elementos isolados; semi-automática para argumentos nomeados (metodologia P664) |

## Estado do passo

- [x] Metodologia de P663/P664 confirmada por leitura directa.
- [x] Amostra varrida, com achados classificados.
- [x] Cada decisão da tabela resolvida com base no custo medido na amostra.
- [x] L0 escrito em `00_nucleo/prompts/engine/stdlib_audit_methodology.md`, com hash `0683fad7`.
- [x] Nenhuma correcção de bug feita neste passo — só registo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p765.md`.

## Próximo passo

P765a (lote 0): varredura detalhada de `#title()` e `symbol`, com um relatório próprio em `00_nucleo/diagnosticos/achados-stdlib-lote0.md` e passos de correcção individuais com L0 próprio cada um.
