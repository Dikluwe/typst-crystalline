### 2. README.pt.md (Versão em Português)

# /tools — Manutenção e Automação

> **O Sistema Imunológico.** Scripts que impõem a estrutura e mapeiam o retículo.

## Propósito

Este diretório contém **scripts de automação** e utilitários projetados para gerar mapas de contexto de IA (`_MAP.md`), impor regras arquiteturais e reduzir o trabalho humano. Atua como o "Cartógrafo" do projeto.

---

## 💎 Formalismo Matemático ($\mathcal{L}_{tools}$)

As ferramentas atuam como **Operadores de Validação** que garantem que o estado do projeto permaneça dentro das fronteiras topológicas definidas:

* **O Mapeamento Cartográfico ($f_{map}$)**: Seja $G$ o Grafo do Projeto (arquivos e pastas). O Cartógrafo é uma função $f: G \to C$ que projeta a realidade física em um Modelo de Contexto $C$ para agentes de IA.
* **Verificação de Invariantes**: As ferramentas executam uma função de avaliação $v(x)$ para cada arquivo.
$$v(x) =
\begin{cases}
1 & \text{if } x \text{ satisfies } \mathcal{L}\_n \text{ invariants} \\
0 & \text{otherwise (Trigger Warning/Error)}
\end{cases}$$

* **Fechamento de Consistência**: O sistema é "Cristalino" se, e somente se, o estado físico corresponde ao estado da especificação ($State_{code} \equiv State_{spec}$). As ferramentas impõem essa identidade.

---

## O Mandato da Automação

> [!CAUTION]
> **Não edite arquivos `_MAP.md` manualmente.**
> Os mapas de contexto são **artefatos gerados**. Alterações manuais serão sobrescritas. Se precisar alterar uma descrição, edite o "Comentário Mágico" (primeira linha) do arquivo fonte.

## Estrutura de Diretórios

```
tools/
├── cartographer.rs  # Gerador de Mapas Fractais (Escaneia a topologia)
└── README.md        # Este arquivo

```

## Comentários Mágicos

Para popular os mapas, o Cartógrafo lê a **primeira linha** dos seus arquivos:

* **Rust (`.rs`)**: Use `//!` no topo absoluto.
* **Markdown/Scripts (`.md`, `.py`)**: Use `#` (título) ou `#` (comentário).

## Regras

1. **Auto-Documentação**: Cada arquivo de código DEVE começar com um comentário mágico.
2. **Contexto Automatizado**: Agentes de IA dependem do `_MAP.md`; garanta que o script rode antes dos commits.
3. **Sem Arquivos Fantasma**: Arquivos sem comentários mágicos aparecem como entradas vazias no mapa.
4. **Integridade da Ferramenta**: Ferramentas devem ser *stateless* e auto-detectar a raiz do projeto.

---
