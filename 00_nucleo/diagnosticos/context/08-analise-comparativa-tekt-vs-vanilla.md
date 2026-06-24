# Análise Comparativa: Tekt (Crystalline) vs. Typst Vanilla

**Data**: 2026-04-03
**Foco**: Comparação Arquitectural e Trade-offs das Decisões Base
**Fonte**: Conversas e Análises de Código (Passo 15 ao actual)

Este documento foi gerado para documentar objectivamente as divergências, ganhos de engenharia e débitos técnicos deliberados na adopção do padrão **Tekt** para o núcleo lógico do Typst, comparado com a implementação original.

---

## 1. Gestão de Dependências e Pureza (L1)

A principal diferença inicial reside na remoção de crates utilitárias no estrato L1 (Core), formalizada pela **ADR-0015**.

| Característica | Vanilla | Tekt | Veredicto |
| :--- | :--- | :--- | :--- |
| **Crate de Texto** | `ecow` (obrigatória) | `std::string::String` / `SyntaxText` (`Arc`) | **Maior Auditabilidade**. |
| **Alocação de Erros** | SSO (Small String Opt.) no heap | Alocação regular no heap (`String`) | **Menor Eficiência em pequenos textos**. |
| **Independência** | Acoplamento forte das bibliotecas | Núcleo limpo, livre de libs não-essenciais | **Melhoria Arquitectural**. |

*Nota Explicativa (SSO)*: A `ecow` usa Small String Optimization, o que agiliza micro-textos. A Tekt abandona esse ganho de *micro-performance* em favor da estabilidade, permitindo compilações em WASM puro e sistemas embedded sem dependências complexas.

---

## 2. Semântica de Tipos e Igualdade (ADR-0025)

Esta secção detalha a identidade de valores durante a execução do código (e.g. `1 == 1.0`).

| Atributo | Vanilla | Tekt | Vantagem Tekt |
| :--- | :--- | :--- | :--- |
| **PartialEq Base** | Implementação manual customizada | `#[derive(PartialEq)]` (Estrito) | Previsibilidade ao usar como chaves Rust. |
| **Avaliação (`eval`)** | Delega para trait | Coerção lógica isolada (`eval_binary_op`) | Rastreado explícito de mágicas de linguagem. |
| **Integridade Rust** | `1 == 1.0` é verdade em Rust | `1 == 1.0` é falso em Rust, mas verdade no Typst | Previne bugs difíceis em mapas e AST. |

---

## 3. Gestão de Escopos (Scopes)

A representação do estado das variáveis em escopos locais e globais através do tempo de análise (ADR-0023).

| Mecanismo | Vanilla | Tekt | Resultado Prático |
| :--- | :--- | :--- | :--- |
| **Coleção Base** | HashMap customizado/otimizado | `IndexMap` | **Determinismo Total**. |
| **Hasher Interno** | Padrão | `FxBuildHasher` | Performance melhorada para IDs pequenos. |

*Impacto Prático:* O compilador original exibe exportações de variáveis sem garantia de ordem. O Crystalline retêm a ordem original do utilizador, crucial para a previsibilidade da UI e serialização (`#show` / test snapshots).

---

## 4. Segurança de Execução e Prevenção de Falhas

O Type Tekt foca explicitamente na resiliência da máquina virtual.

* **Tekt Safety Guards**:
  * `ctx.tick_loop()`: Introduzido para atirar erro de *Timeout* ao exceder 1 milhão de loops, prevenindo ataques de DOS (Loop-bombing).
  * `max_call_depth`: Stack rígida (por defeito \~250 frames), previne Stack Overflow em chamadas recursivas directas.
  * `ImportGuard (RAII)`: Monitoriza determinísticamente os ciclos de importação.
* **Vanilla**: Confia intensivamente no processo de memoização (`comemo`) para evitar processamentos desnecessários, mas apresenta carências nas garantias explícitas de "panic guard" para lógicas infintas pelo utilizador em L1.

---

## 5. Estruturas em Memória (Value Enum & O(n) vs O(1))

No Vanilla, os nós são criados com o máximo de optimizações e referenciam-se mutualmente (`Value::Dyn`).
Na Tekt, os tipos são incorporados de forma contida. Tipos como Gradients ou Colors complexos não foram apressados para manter a solidez fundamental de L1 (ADR-0017).

**A Maior Piora Atual (Débito Técnico)**:
O uso do construtor de Arrays, ex: `Value::Array(Vec<Value>)`, incorre num clonar O(n).
*Vanilla* utiliza uma estrutura estilo Persistent-Vector (`EcoVec`), obtendo O(1) Clone.
Isto está referenciado, em **DEBT.md**, para ser trocado para `Arc<Vec<Value>>` futuramente sem quebrar a arquitectura modular da Tekt.

---

## 6. A "Ferrari Modular" (Conclusão do Design Pluggable)

A principal revelação desta análise reside na justificação arquitectural para adoptar o "Crystalline":

A versão Vanilla foca extrema integração *(uma placa de chip unificada)*. É excepcionalmente rápida, embora fechada a sub-componentes isolados.
A versão Tekt actua em modo PnP (Plug & Play) via "Traits" e "Strata". Podes substituir o motor L1 por um núcleo optimizado para performance máxima, mantendo a camada de rede e pacote em L2 incólumes.

**A Flexibilidade foi privilegiada.** Embora perca na micro-alocação original devida à limpeza da *crate* `ecow`, os alicerces (`comemo` adapters, traits limpos) permitem implementar a lógica performante numa estrutura purificada no futuro.
