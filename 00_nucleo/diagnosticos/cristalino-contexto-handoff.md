---

# Documento de Contexto — Projeto Cristalino (Typst)

> **Data:** 2026-07-01
> **Versão:** 1.1
> **Propósito:** Handoff para nova conversa. Este documento contém todo o estado relevante do projeto cristalino para que um novo assistente possa continuar sem perda de contexto.
> **Projeto:** typst-crystalline (reimplementação de Typst 0.15.0 com arquitetura cristalina)
> **Localização:** `/home/dikluwe/Documentos/Antigravity/typst-crystalline`

---

## 1. Resumo Executivo

O projeto cristalino é uma **reimplementação do Typst 0.15.0** com arquitetura cristalina (atomizada, documentada, testada).

**Estado atual:**
- ✅ **Paridade de linguagem:** Completa (sintaxe, semântica, morfologia)
- ⚠️ **Paridade de produção:** Quase completa. Variation Fonts tem fix pronto para implementar (P530).
- ⏸️ **Inovações:** Pausadas
- 📋 **Próximos passos:** A decidir pelo usuário

**Benchmark:** Cristalino vs vanilla 0.15.0 — mediana 1.10×, macro 0.30×.

---

## 2. O que foi Feito (P490–P529)

### 2.1 Paridade de Linguagem (P490–P514)

| Passo | Foco | Resultado |
|-------|------|-----------|
| P490 | Diagnóstico baseline | 20 ficheiros, 5 MATCH / 9 DIFF / 6 AUSENTE |
| P494 | 7 selectors ausentes | 7 AUSENTE → MATCH |
| P495 | 4 args nomeados | 4 DIFF → MATCH |
| P496 | Field access em coleções | 3 DIFF → MATCH |
| P497 | Variáveis de cor + text() | 2 DIFF → MATCH |
| P498 | D3c residual (show-rules) | 1 DIFF → MATCH |
| P500 | Audit expandido | 17 funcionalidades não testadas |
| P501 | str/dict/calc methods | 3 AUSENTE → MATCH |
| P502 | image/raw/footnote/outline | 4 AUSENTE/DIFF → MATCH |
| P503 | Re-baseline 0.15.0 | 20/20 MATCH |
| P504 | Novas funcionalidades 0.15.0 | 16/16 OK |
| P505 | List/enum indent | 2 AUSENTE → MATCH |
| P506 | Runtime state (state/counter/context) | 1 AUSENTE → MATCH |
| P507 | Benchmark (inválido, sem shaping) | Diagnóstico |
| P508 | Diagnóstico real de brechas | Identificou 6 brechas |
| P509 | Stdlib core (str/dict/image/raw/query) | 5 AUSENTE → MATCH |
| P510 | Math styles (12 funções) | 12/12 OK |
| P511 | Math elements granulares | 6/6 OK |
| P512 | Grid/Table HLine/VLine | 4/4 OK |
| P513 | Curve elements | 5/5 OK |
| P514 | Relatório: Paridade de linguagem completa | 37/37 MATCH |

### 2.2 Paridade de Produção (P515–P523)

| Passo | Foco | Resultado |
|-------|------|-----------|
| P515 | fontdb + font fallback | System fonts descobertas |
| P516 | Subsetting TrueType | 559 KB → 30 KB, `AAAAAA+` prefix |
| P517 | System fonts por defeito + marcação de subset | CLI amigável |
| P518 | Benchmark revalidado | Mediana 1.10×, macro 0.30× |
| P519 | Sonda de regressão pós-subsetting | Kerning e ligatures identificados como problemas reais |
| P520 | Fix de kerning (sinal do TJ) + ligatures no subsetting | Ambos corrigidos |
| P521 | ToUnicode completo para ligatures (DEBT-64) | Fechado, incluindo caso RTL |
| P522 | Sonda de CFF | CFF já funciona via `oxifont-subset` |
| P523 | Polimento CFF + correcção de narrativa | Fechado |

### 2.3 Variation Fonts (P524–P530)

| Passo | Foco | Resultado |
|-------|------|-----------|
| P524 | Sonda de VF + correcção de contradição sobre kerning | VF existe no sistema; shaper não aplica coordenadas |
| P525 | MVP: coordenadas de eixo no shaper | Avanços correctos, mas fonte embutida sempre na instância default |
| P527 | Confirmação da regressão visual | `weight:` não muda a aparência do texto para fontes VF |
| P528 | Diagnóstico de instanciação (fonte completa) | Inviável, ~4 minutos por combinação |
| P529 | Diagnóstico de instanciação (subset primeiro) | Viável, ~0,3 segundos por combinação |
| P530 | Implementação do fix de VF | Múltiplas instâncias estáticas no PDF; paridade visual para peso |
| **P530** | Implementação do fix real | **Pronto para executar** |

---

## 3. Arquitetura do Projeto

### 3.1 Estrutura de Diretórios

```
typst-crystalline/
├── 00_nucleo/                    # Documentação, ADRs, diagnósticos
│   ├── adrs/
│   ├── diagnosticos/
│   └── prompts/
├── 01_core/                      # Core do compilador
│   ├── src/
│   │   ├── entities/
│   │   ├── rules/
│   │   │   ├── eval/
│   │   │   ├── layout/
│   │   │   ├── stdlib/
│   │   │   └── lexer/
│   │   └── infra/
│   └── tests/
├── 03_infra/                     # Infraestrutura (typst-wiring, shaper, export)
├── lab/
│   ├── parity/                   # Corpus de paridade
│   │   ├── corpus/p490/
│   │   ├── corpus/p500/
│   │   ├── corpus/p520/
│   │   ├── corpus/p523/
│   │   └── corpus/rtl/
│   ├── typst-original/           # Vanilla 0.15.0
│   └── .venv/                    # Ambiente Python (fontTools, skia-pathops)
├── tools/
│   ├── perf/
│   └── crystalline-lint/
└── Cargo.toml
```

### 3.2 Camadas (L0, L1, L3)

| Camada | Descrição | Ficheiros |
|--------|-----------|-----------|
| **L0** | Prompts, ADRs, documentação | `00_nucleo/` |
| **L1** | Core Rust (entities, rules, eval, layout) | `01_core/src/` |
| **L3** | Infraestrutura (CLI, wiring, helpers, export, shaper) | `03_infra/src/` |

### 3.3 ADRs Relevantes

| ADR | Descrição | Estado |
|-----|-----------|--------|
| ADR-0026 | Estrutura cristalina (L0/L1/L3) | Ativa |
| ADR-0054 | Graded parity (MATCH/DIFF/ERRO/PANIC/AUSENTE) | Ativa |
| ADR-0075 | Comparação via `typst query --format json` | Ativa |
| ADR-0107 | Paridade é de linguagem, não de mecânica | Ativa |
| ADR-0108 | Medir antes de decidir; língua vs mecânica | Ativa |
| ADR-0109 | Atomização de código | Ativa |
| ADR-0114 | Sonda A.0 antes de spec | Ativa |
| ADR-0115 | Infra de benchmark | Ativa |
| ADR-0120 | Shaping via rustybuzz (`FrameItem::TextShaped`) | Ativa |

**Nota P530:** o fix de Variation Fonts introduz uma dependência de Python (`fontTools`) em runtime, não só em desenvolvimento. Confirmar se isto foi registado formalmente como decisão de arquitectura ao rever o relatório de P530.

---

## 4. Estado dos Testes

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

cargo build --release -p typst-wiring
crystalline-lint .
cargo test -p typst-core
cargo test -p typst-infra

for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ lab/parity/corpus/p520/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done

pdffonts /tmp/out.pdf
python3 tools/perf/benchmark-p507.py
```

Números de testes exactos: confirmar no relatório mais recente (`00_nucleo/diagnosticos/`), não copiar valores antigos deste documento sem verificar — já aconteceu mais do que uma vez neste projecto uma afirmação desactualizada persistir num documento depois de já ter sido corrigida noutro.

---

## 5. Brechas Remanescentes

### 5.1 Paridade de Produção

| Brecha | Estado | Notas |
|--------|--------|-------|
| Shaping real (rustybuzz) | Fechado em P515/P520/P521 | Kerning e ligatures corrigidos |
| fontdb system discovery | Fechado em P515 | |
| Subsetting TrueType | Fechado em P516 | |
| System fonts por defeito | Fechado em P517 | |
| CFF subsetting | Fechado em P523 | |
| Variation fonts (VF) | ✅ Fechado em P530 | Subset-first + fontTools instancer; múltiplas instâncias embutidas por (FontList, FontVariant). Itálico requer eixo `ital` na VF. Dependência de Python + fontTools em runtime. |

### 5.2 Fora de Escopo (Declarado)

| Funcionalidade | Razão |
|----------------|-------|
| HTML export | PDF-only (sondado em P526, não implementado) |
| SVG export | PDF-only (sondado em P526, não implementado) |
| Raster render (PNG) | PDF-only (sondado em P526, não implementado) |
| IDE / LSP | Fora de escopo (sondado em P526) |
| Plugin system | Fora de escopo |

---

## 6. Próximos Passos Possíveis

| # | Passo | Tamanho | Descrição |
|---|-------|---------|-----------|
| A | **P519** | L | Lookahead Layout Engine (inovação, já escrito, aguardando execução) |
| B | **SVG export** | M–L | Sondado em P526; especificação por escrever |
| C | **PNG export** | S–M | Via SVG → resvg/tiny-skia |
| D | **HTML export** | M–L | Sondado em P526 |
| E | **Publicação** | M | Artigo sobre a arquitetura cristalina |
| C | Publicação | M | Artigo sobre a arquitetura cristalina |
| D | SVG export | M–L | Sondado em P526; especificação por escrever |
| E | PNG export | S–M | Depende de SVG |
| F | HTML export | M–L | Sondado em P526; especificação por escrever |
| G | IDE/LSP | L–XL | Sondado em P526; menor prioridade |
| H | Optimização | M | Cache de shaping, paralelização de subsetting |
| I | Manutenção | S | Bug fixes, refactor, documentação |

Sem recomendação fixa nesta versão do documento — decidir conforme prioridade do momento.

---

## 7. Convenções do Projeto

### 7.1 Formato de Passos

```markdown
---
# P### — Título
> **Passo:** ###
> **Data:** YYYY-MM-DD
> **Foco:** ...
> **Tipo:** Diagnóstico / Implementação / Documentação
> **Tamanho:** S / M / L / XL
> **ADR-XXXX ACEITE** — ...
> **Dependências:** P###, P###
---
```

### 7.2 Metodologia

1. **Sonda A.0** (ADR-0114): verificar se a funcionalidade já existe parcialmente, antes de escrever especificação.
2. **Medir baseline:** correr o corpus antes de tocar código.
3. **Implementar:** mudanças atómicas, uma sub-tarefa por vez.
4. **Medir depois:** correr o corpus depois de cada sub-tarefa.
5. **Documentar:** relatório de resultados.

### 7.3 Classificação de Resultados

- `MATCH` — output estruturalmente equivalente.
- `DIFF` — ambos produzem resultado, mas diferente.
- `ERRO_DESCRITIVO` — erro com mensagem clara (aceitável se for scope-out declarado).
- `PANIC` — crash. Deve ser zero.
- `AUSENTE` — funcionalidade não reconhecida.

### 7.4 Lição registada ao longo do projecto

Várias vezes uma alegação escrita num relatório ou neste handoff revelou-se errada quando sondada de facto (exemplos: "CFF é scope-out XL" — refutado em P522; "GPOS/GSUB removidas pelo subsetter" — refutado em P519, mas reapareceu por engano num relatório posterior antes de ser corrigido definitivamente em P524). Antes de repetir uma afirmação deste documento numa sessão nova, verificar se ainda é válida, em vez de assumir.

---

## 8. Notas para o Novo Assistente

1. Não declarar conclusões sem medição.
2. Verificar ADR-0108 (medir antes de decidir) e ADR-0114 (sonda antes de spec).
3. Seguir o formato de passo estabelecido.
4. Cada passo deve ser atómico, testável, sem regressão.
5. Benchmark P518 é o baseline de performance — comparar contra ele, não contra a memória do que "deveria" acontecer.
6. Paridade é de linguagem, não de mecânica — mas quando uma funcionalidade de linguagem (como `weight:`) não produz efeito visível, isso é regressão de linguagem, não questão mecânica, mesmo que a causa esteja em código de exportação PDF.
7. O usuário fala português.

---

*Documento actualizado em 2026-07-01, versão 1.1, para reflectir P519–P530.*
