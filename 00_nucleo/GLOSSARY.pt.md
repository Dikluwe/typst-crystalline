# 📖 Glossário Typst Crystalline

> Termos e conceitos chave para navegação no projeto.

---

## Arquitetura Crystalline

| Termo | Definição |
|-------|-----------|
| **Núcleo (00_nucleo)** | Camada zero: documentação, especificações, ADRs e contratos. Não contém código executável. |
| **Core (01_core)** | Crates fundamentais do compilador. Dependências fluem para fora, nunca para dentro. |
| **Shell (02_shell)** | Interfaces de usuário: CLI e IDE. Consome o core sem modificá-lo. |
| **Infra (03_infra)** | Exportadores (PDF, SVG, HTML) e ferramentas auxiliares. |
| **Wiring (04_wiring)** | Fachada de orquestração. Une todas as camadas em uma API coesa. |

---

## Fases de Compilação

| Fase | Entrada → Saída | Crate |
|------|-----------------|-------|
| **Parsing** | `&str` → `SyntaxNode` | `typst-syntax` |
| **Evaluation** | `Source` → `Module` (Content + Scope) | `typst-eval` |
| **Realization** | `Content` → Elementos layoutáveis | `typst-realize` |
| **Layout** | `Content` → `Frame[]` (um por página) | `typst-layout` |
| **Export** | `Frame[]` → PDF/SVG/PNG/HTML | `typst-pdf`, `typst-svg`, etc. |

---

## Conceitos do Compilador

| Termo | Definição |
|-------|-----------|
| **SyntaxNode** | Nó da árvore sintática concreta. Mantém whitespace e comentários. Parsing nunca falha. |
| **Span** | Identificador único de um nó na árvore sintática. Usado para rastrear erros até o código-fonte. |
| **Content** | Tipo principal do Typst. Representa conteúdo tipográfico a ser layoutado. |
| **Frame** | Resultado do layout. Contém elementos posicionados prontos para exportação. |
| **Region** | Espaço disponível para layout (e.g., área de uma página). |
| **Module** | Resultado da avaliação de um arquivo. Contém `Content` e um `Scope` com bindings. |
| **World** | Interface para dependências do sistema (arquivos, fontes, configuração). |
| **Vm** | Máquina virtual do interpretador. Mantém a pilha de escopos durante avaliação. |
| **LinkedNode** | Abstração sobre `SyntaxNode` para IDE, com acesso a pais e vizinhos. |
| **Scope** | Tabela de símbolos contendo definições de variáveis e funções. |
| **Closure** | Função que encapsula variáveis do escopo léxico (imutável, por valor). |
| **Show Rule** | Regra de transformação de conteúdo (aplicada durante a fase de realization). |
| **Introspection** | Processo iterativo para resolução de dependências dinâmicas (ex.: referências cruzadas, numeração de páginas). |

---

## Tipos Fundamentais (Foundations)

| Tipo | Definição |
|------|-----------|
| **none** | Valor nulo. Representa ausência de valor. |
| **auto** | Valor automático. Deixa o compilador decidir. |
| **bool** | Valor booleano (`true` ou `false`). |
| **int** | Número inteiro de 64 bits. |
| **float** | Número de ponto flutuante de 64 bits. |
| **decimal** | Número decimal de precisão arbitrária. |
| **str** | String UTF-8 imutável. |
| **bytes** | Sequência de bytes brutos. |
| **array** | Lista ordenada heterogênea. Imutável por padrão. |
| **dictionary** | Mapa chave-valor. Chaves são strings, valores são heterogêneos. |
| **function** | Função de primeira classe. Pode ser nativa ou definida pelo usuário. |
| **selector** | Padrão para selecionar elementos no documento. |
| **regex** | Expressão regular para matching de texto. |
| **datetime** | Data e/ou hora. Suporta fuso horário. |
| **duration** | Intervalo de tempo (diferença entre datetimes). |
| **version** | Número de versão semântica. |
| **label** | Identificador para referenciar elementos no documento. |
| **symbol** | Símbolo tipográfico (ex.: emojis, caracteres especiais). |
| **module** | Resultado da avaliação de um arquivo fonte. |
| **plugin** | Extensão WebAssembly carregada dinamicamente. |
| **type** | Metadados de um tipo Typst. |
| **args** | Argumentos de função (posicionais e nomeados). |

---

## Incrementalidade

| Termo | Definição |
|-------|-----------|
| **comemo** | Framework de compilação incremental usado pelo Typst. Memoiza resultados de funções puras. |
| **Introspection Loop** | Loop que re-executa layout até estabilizar (máx. 5 iterações). Resolve dependências cíclicas como TOC. |
| **Capture** | Variáveis externas capturadas por closures. Estabilidade de captures melhora incrementalidade. |

---

## Crates do Projeto

### 01_core (Fundamentais)

| Crate | Responsabilidade |
|-------|------------------|
| `typst-syntax` | Parser e definição da árvore sintática |
| `typst-eval` | Interpretador da linguagem Typst |
| `typst-realize` | Subsistema de realização (aplicação de show rules) |
| `typst-layout` | Motor de layout |
| `typst-library` | Biblioteca padrão do Typst (funções, tipos) |
| `typst-macros` | Macros procedurais para o compilador |
| `typst-utils` | Utilitários compartilhados |
| `typst-timing` | Medição de performance |

### 02_shell (Interfaces)

| Crate | Responsabilidade |
|-------|------------------|
| `typst-cli` | Interface de linha de comando |
| `typst-ide` | Funcionalidades IDE (autocomplete, hover, etc.) |

### 03_infra (Exportadores)

| Crate | Responsabilidade |
|-------|------------------|
| `typst-pdf` | Exportador para PDF |
| `typst-svg` | Exportador para SVG |
| `typst-html` | Exportador para HTML |
| `typst-render` | Renderizador para pixel buffer |
| `typst-kit` | Implementações padrão para CLI |

### 04_wiring (Orquestração)

| Crate | Responsabilidade |
|-------|------------------|
| `typst` | Fachada principal que une todas as partes |

---

## Destilação (Antigravity)

| Termo | Definição |
|-------|-----------|
| **Especificação (Spec)** | Arquivo `.md` que descreve garantias e propósito de um módulo Rust, não sua implementação. |
| **Lei vs. Implementação** | Specs descrevem "a Lei" (contratos, invariantes), não "o Código" (detalhes de implementação). |
| **Isomorfismo de Pastas** | Estrutura de specs em `00_nucleo/specs/` espelha a estrutura do código. |
| **Impureza Justificada** | I/O no core é permitido apenas com justificativa documentada. |

---

## Comandos Úteis

```bash
# Verificar build
source $HOME/.cargo/env && cargo check --workspace

# Rodar testes
cargo test --workspace

# Build CLI release
cargo build -p typst-cli --release

# Verificar formatação
cargo fmt --check
```

---

## Fluxo de Dependências

```
┌─────────────────────────────────────────────────────┐
│                    04_wiring                        │
│                      typst                          │
└─────────────────────────────────────────────────────┘
         ▲                              ▲
         │                              │
┌─────────────────────┐    ┌─────────────────────────┐
│     02_shell        │    │       03_infra          │
│  typst-cli          │    │  typst-pdf, typst-svg   │
│  typst-ide          │    │  typst-html, typst-kit  │
└─────────────────────┘    └─────────────────────────┘
         ▲                              ▲
         │                              │
         └──────────────┬───────────────┘
                        │
         ┌──────────────┴───────────────┐
         │           01_core            │
         │  typst-syntax, typst-eval    │
         │  typst-layout, typst-library │
         │  typst-realize, typst-macros │
         └──────────────────────────────┘
```

> **Regra:** Setas apontam para quem é dependido. Upper layers dependem de lower layers, nunca o contrário.
