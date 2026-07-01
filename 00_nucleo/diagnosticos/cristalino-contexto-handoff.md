---

# Documento de Contexto — Projeto Cristalino (Typst)

> **Data:** 2026-07-01
> **Versão:** 1.0
> **Propósito:** Handoff para nova conversa. Este documento contém todo o estado relevante do projeto cristalino para que um novo assistente possa continuar sem perda de contexto.
> **Projeto:** typst-crystalline (reimplementação de Typst 0.15.0 com arquitetura cristalina)
> **Localização:** `/home/dikluwe/Documentos/Antigravity/typst-crystalline`

---

## 1. Resumo Executivo

O projeto cristalino é uma **reimplementação do Typst 0.15.0** com arquitetura cristalina (atomizada, documentada, testada). O objetivo é provar que engenharia estruturada supera abordagens "vanilla".

**Estado atual:**
- ✅ **Paridade de linguagem:** Completa (sintaxe, semântica, morfologia)
- ✅ **Paridade de produção:** Completa (shaping, fontdb, subsetting, system fonts)
- ⏸️ **Inovações:** Pausadas (Lookahead não é prioridade atual)
- 📋 **Próximos passos:** A decidir pelo usuário

**Benchmark:** Cristalino vs vanilla 0.15.0 — mediana 1.10×, macro 0.30× (mais rápido em documentos grandes).

---

## 2. O que foi Feito (P490–P518)

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
| P503 | Re-baseline 0.15.0 | 20/20 MATCH (estável) |
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
| P514 | **Relatório: Paridade de linguagem completa** | 37/37 MATCH |

### 2.2 Paridade de Produção (P515–P518)

| Passo | Foco | Resultado |
|-------|------|-----------|
| P515 | fontdb + font fallback | System fonts descobertas, fallback por caractere |
| P516 | Subsetting TrueType | 559 KB → 30 KB (94% redução), `AAAAAA+` prefix |
| P517 | System fonts por defeito + marcação de subset | CLI amigável, PDFs marcados |
| P518 | **Benchmark revalidado** | Mediana 1.10×, macro 0.30× |

---

## 3. Arquitetura do Projeto

### 3.1 Estrutura de Diretórios

```
typst-crystalline/
├── 00_nucleo/                    # Documentação, ADRs, diagnósticos
│   ├── adrs/                     # Architecture Decision Records
│   ├── diagnosticos/             # Relatórios de paridade (P490-P518)
│   └── prompts/                  # Passos executados (typst-passo-*.md)
├── 01_core/                      # Core do compilador
│   ├── src/
│   │   ├── entities/             # Tipos de dados (Value, Content, Selector, etc.)
│   │   ├── rules/                # Regras (eval, layout, stdlib, lexer, parser)
│   │   │   ├── eval/             # Eval engine
│   │   │   ├── layout/           # Layout engine
│   │   │   ├── stdlib/           # Standard library
│   │   │   └── lexer/            # Lexer
│   │   └── infra/                # Infraestrutura (fontdb, shaper, PDF, CLI)
│   └── tests/                    # Testes unitários
├── 03_infra/                     # Infraestrutura (typst-wiring, query helpers)
├── lab/                          # Laboratório
│   ├── parity/                   # Corpus de paridade (P490 + P500)
│   │   ├── corpus/p490/          # 20 ficheiros .typ
│   │   └── corpus/p500/          # 17 ficheiros .typ
│   └── typst-original/           # Vanilla 0.15.0 para comparação
├── tools/                        # Ferramentas
│   ├── perf/                     # Benchmark scripts
│   └── crystalline-lint/        # Linter customizado
└── Cargo.toml
```

### 3.2 Camadas (L0, L1, L3)

| Camada | Descrição | Ficheiros |
|--------|-----------|-----------|
| **L0** | Prompts, ADRs, documentação | `00_nucleo/`, `rules/*.md`, `entities/*.md` |
| **L1** | Core Rust (entities, rules, eval, layout) | `01_core/src/` |
| **L3** | Infraestrutura (CLI, wiring, helpers) | `03_infra/src/` |

### 3.3 ADRs Relevantes

| ADR | Descrição | Estado |
|-----|-----------|--------|
| ADR-0026 | Estrutura cristalina (L0/L1/L3) | ✅ Ativa |
| ADR-0054 | Graded parity (MATCH/DIFF/ERRO/PANIC/AUSENTE) | ✅ Ativa |
| ADR-0075 | Comparação via `typst query --format json` | ✅ Ativa |
| ADR-0107 | Paridade é de linguagem, não de mecânica | ✅ Ativa |
| ADR-0108 | Medir antes de decidir; língua vs mecânica | ✅ Ativa |
| ADR-0109 | Atomização de código | ✅ Ativa |
| ADR-0114 | Sonda A.0 antes de spec | ✅ Ativa |
| ADR-0115 | Infra de benchmark | ✅ Ativa |

---

## 4. Estado dos Testes

```bash
# Comandos de verificação (copiar para nova conversa)
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Build
cargo build --release -p typst-wiring

# Linter
crystalline-lint .

# Testes unitários
cargo test -p typst-core        # 3550 passed; 0 failed
cargo test -p typst-wiring      # 21 passed; 0 failed

# Bateria de paridade
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
# Esperado: 37/37 OK

# Verificar fontes no PDF
pdffonts /tmp/out.pdf

# Benchmark
python3 tools/perf/benchmark-p507.py
```

---

## 5. Brechas Remanescentes

### 5.1 Paridade de Produção (Trilha 5 — Fechada)

| Brecha | Estado | Notas |
|--------|--------|-------|
| Shaping real (rustybuzz) | ✅ Fechado em P515 | Font fallback por caractere |
| fontdb system discovery | ✅ Fechado em P515 | `with_system_fonts` |
| Subsetting TrueType | ✅ Fechado em P516 | `oxifont-subset` |
| System fonts por defeito | ✅ Fechado em P517 | CLI default |
| Marcação de subset | ✅ Fechado em P517 | `AAAAAA+` prefix |
| CFF subsetting | ✅ Fechado em P523 | Funcional via `oxifont-subset`; polimento de descritor PDF (CID Type 0C) pendente sonda P524 se necessário |
| Variation fonts (VF) | ⚠️ Parcial — P525 (MVP shaper) | Shaper aplica `wght`/`ital` correctamente nos avanços; **export PDF embebe sempre a instância default**, pelo que `text(weight: 700)` numa fonte VF não é visualmente bold. Fix real requer instanciar VF estaticamente por combinação de peso/estilo usada no documento. |
| Kerning no subset | ✅ Fechado em P520/P521 | Delta model no operador TJ; validado com corpus dedicado (lab/parity/corpus/p520/) |

### 5.2 Fora de Escopo (Declarado)

| Funcionalidade | Razão |
|----------------|-------|
| HTML export | PDF-only |
| SVG export | PDF-only |
| Raster render (PNG) | PDF-only |
| IDE / LSP | Fora de escopo |
| Plugin system | Fora de escopo |

---

## 6. Próximos Passos Possíveis

O usuário deve escolher a direção. Opções:

| # | Passo | Tamanho | Descrição |
|---|-------|---------|-----------|
| A | **P528** | S–M | Fix real de Variation Fonts: instanciar VF estaticamente por combinação peso/estilo; subsetar cada instância separadamente; referenciar a instância correcta no PDF |
| B | **P519** | L | Lookahead Layout Engine (inovação — já escrito, aguardando execução) |
| C | **P520** | M | Publicação / artigo sobre arquitetura cristalina |
| D | **Trilha 6** | XL | CFF subsetting (completa paridade de produção) |
| E | **Trilha 7** | L | Variation fonts (VF) support |
| F | **Otimização** | M | Cache de shaping, paralelização de subsetting |
| G | **Nova funcionalidade** | ? | A definir pelo usuário |
| H | **Manutenção** | S | Bug fixes, refatoração, documentação |

**Recomendação do assistente:** O benchmark P518 confirma que o cristalino é competitivo (mediana 1.10×). O Lookahead (P519) é a inovação mais impactante já planejada. Se o usuário quer provar superioridade sobre vanilla, P519 é o caminho.

---

## 7. Convenções do Projeto

### 7.1 Formato de Passos

Cada passo segue o template:
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

1. **Sonda A.0** (se aplicável, ADR-0114): Verificar se funcionalidade já existe parcialmente.
2. **Medir baseline:** Rodar corpus antes de tocar código.
3. **Implementar:** Mudanças atômicas, 1 sub-tarefa por vez.
4. **Medir pós:** Rodar corpus após cada sub-tarefa.
5. **Documentar:** Relatório de resultados + sentinela.

### 7.3 Classificação de Resultados

- `MATCH` — output estruturalmente equivalente
- `DIFF` — ambos produzem resultado mas diferente
- `ERRO_DESCRITIVO` — erro com mensagem clara (aceitável se scope-out)
- `PANIC` — crash (bug prioritário, deve ser zero)
- `AUSENTE` — funcionalidade não reconhecida

---

## 8. Links e Referências

### 8.1 Passos Escritos (disponíveis em `/mnt/agents/output/`)

| Passo | Ficheiro | Estado |
|-------|----------|--------|
| P490 | `typst-passo-490.md` | Executado |
| P494 | `typst-passo-494.md` | Executado |
| P495 | `typst-passo-495.md` | Executado |
| P496 | `typst-passo-496.md` | Executado |
| P497 | `typst-passo-497.md` | Executado |
| P498 | `typst-passo-498.md` | Executado |
| P500 | `typst-passo-500.md` | Executado |
| P501 | `typst-passo-501.md` | Executado |
| P502 | `typst-passo-502.md` | Executado |
| P503 | `typst-passo-503.md` | Executado |
| P504 | `typst-passo-504.md` | Executado |
| P505 | `typst-passo-505.md` | Executado |
| P506 | `typst-passo-506.md` | Executado |
| P507 | `typst-passo-507.md` | Executado |
| P508 | `typst-passo-508.md` | Executado |
| P509 | `typst-passo-509.md` | Executado |
| P510 | `typst-passo-510.md` | Executado |
| P511 | `typst-passo-511.md` | Executado |
| P512 | `typst-passo-512.md` | Executado |
| P513 | `typst-passo-513.md` | Executado |
| P514 | `typst-passo-514.md` | Executado |
| P515 | `typst-passo-515.md` | Executado |
| P516 | `typst-passo-516.md` | Executado |
| P517 | `typst-passo-517.md` | Executado |
| P518 | `typst-passo-518.md` | Executado |
| P519 | `typst-passo-519.md` | **Escrito, aguardando execução** |

### 8.2 Comandos Rápidos

```bash
# Navegar para o projeto
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Build release
cargo build --release -p typst-wiring

# Compilar um documento
target/release/typst compile documento.typ saida.pdf

# Verificar fontes no PDF
pdffonts saida.pdf

# Rodar testes
cargo test -p typst-core
cargo test -p typst-wiring

# Linter
crystalline-lint .

# Benchmark
python3 tools/perf/benchmark-p507.py
```

---

## 9. Notas para o Novo Assistente

1. **Sempre perguntar antes de assumir:** O usuário é engenheiro de software experiente que valoriza rigor. Não declarar conclusões sem medição.

2. **Respeitar ADRs:** Sempre verificar ADR-0108 (medir antes de decidir) e ADR-0114 (sonda A.0 antes de spec).

3. **Formato de passos:** Seguir o template estabelecido (cabeçalho com Passo, Data, Foco, Tipo, Tamanho, ADRs, Dependências).

4. **Atomização:** Cada passo deve ser atômico, testável, e não causar regressão.

5. **Benchmark:** Antes de declarar sucesso, medir. O benchmark P518 é o baseline de performance.

6. **Língua vs Mecânica:** Paridade é de linguagem (sintaxe, semântica, morfologia), não de mecânica (bytes de PDF, estrutura interna).

7. **O usuário fala português:** Comunicar em português, com termos técnicos em inglês quando apropriado.

---

*Documento produzido em 2026-07-01 para handoff de contexto do projeto typst-crystalline.*
