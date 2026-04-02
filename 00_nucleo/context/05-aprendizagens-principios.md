# Aprendizagens-Chave e Princípios

**Data**: 2026-04-01

---

## 1. Princípios da Arquitetura Cristalina

### I — Nucleação
Prompt antes de código. Sem prompt em `00_nucleo/` → sem geração. A ausência de prompt não é um detalhe — é uma violação do contrato da arquitectura.

### II — Contenção
A estrutura de directórios é uma fronteira física, não decoração. Uma dependência que cruza uma fronteira não autorizada é uma violação estrutural, independentemente da sua correcção funcional.

### III — Gravidade
Dependências fluem apenas em direcção aos estratos mais estáveis. Inverter esta direcção é uma fractura estrutural. Ciclos são degenerações.

### IV — Isolamento de Fase
Código experimental vive em `lab/`. Para migrar, deve ser reescrito com um novo prompt. Não há atalho.

### V — Primazia dos Invariantes
Uma modificação que preserva funcionalidade mas viola invariantes é uma regressão estrutural. A estabilidade do sistema é determinada pela preservação contínua da sua estrutura.

---

## 2. Lições aprendidas durante a migração

### Pureza L1 = zero system I/O, não "zero crates externas"

A maior correcção conceptual do projecto. ADR-0007 removeu `rustc_hash` por ser "externo" — foi um erro. O critério correcto: a crate tem I/O? Estado global mutável? Efeitos colaterais? Se não, é elegível para L1.

`Arc<T>`, `rustc_hash`, `indexmap`, `ecow` — tudo isto é gestão de RAM e computação pura. Proibir crates puras em L1 degrada performance sem ganho arquitectural.

### `eval()` é domínio, não infra

O facto de `eval()` ter muitas dependências no código original não muda a sua natureza. É o motor central do compilador — lógica de avaliação, não I/O. A classificação correcta é L1; as dependências de I/O são acidentais e devem ser separadas.

### `comemo` + supertrait não funciona

`#[comemo::track]` gera tipos internos que não implementam supertraits automaticamente. A solução B3 (blanket impl sem supertrait) é a única viável. A invariante é mantida pelo blanket impl (dessincronização detectada em tempo de compilação), não pelo sistema de tipos.

### Dívida registada antes, paga depois

O padrão DEBT-first garante que simplificações intencionais (flat `TextStyle` em vez de `StyleChain`, bullet ASCII em vez de Unicode) são rastreáveis e têm plano de eliminação. Dívida não registada é dívida invisível.

### Diagnóstico antes de implementação

Cada passo começa com comandos `grep`/`find` no oráculo (`lab/typst-original/`). A API real do código original muitas vezes diverge do que se espera. Implementar sem diagnosticar produz retrabalho.

### Granularidade compensa

O Passo 4 original foi dividido em Passos 4, 5 e 6 porque o scope era largo demais. Passos pequenos permitem verificação incremental e reduzem o risco de regressão.

### `pub use self::X::Y` dispara V14 incorrectamente

Em `mod.rs`, o padrão correcto é `pub mod X` apenas. `pub use self::X::Y` faz o linter interpretar como re-export de tipo externo.

### `assert_approx_eq!` é test-only

Nunca usar em implementações de `PartialEq` de produção. Comparação aproximada é comportamento de teste, não de domínio.

---

## 3. Sobre a Arquitetura Cristalina como framework

A Arquitetura Cristalina é descrita como "proposição, não prática validada" no próprio Manifesto. A hipótese central é que prompts estruturados e versionados dentro do projecto reduzem o crescimento amorfo em sistemas desenvolvidos com agentes de IA.

O projecto typst-crystalline é o primeiro teste sistemático desta hipótese. Após ~27 passos de migração com zero violations mantidas, os dados iniciais sugerem que:

- A estrutura de camadas force a separação de concerns de forma verificável
- O linter impede regressões estruturais silenciosas
- Os prompts L0 permitem reproduzir e auditar decisões
- O padrão DEBT-first torna simplificações intencionais rastreáveis

O teste final — comparar velocidade e consistência de modificação após meses de evolução — ainda não foi realizado.

---

## 4. Mapeamento com padrões da indústria

| Cristalina | Clean Architecture | Hexagonal | DDD |
|-----------|-------------------|-----------|-----|
| L₀ (Seed) | — | — | Ubiquitous Language |
| L₁ (Core) | Entities | Application Core | Domain Layer |
| L₂ (Shell) | Interface Adapters | Primary Adapters | Application Layer |
| L₃ (Infra) | Frameworks & Drivers | Secondary Adapters | Infrastructure |
| L₄ (Wiring) | Main | — | Composition Root |
| lab | — | — | Spikes / POCs |

A distinção fundamental: L₀ não existe em nenhum dos outros frameworks. É a inovação central — fazer do contexto de geração um artefacto de primeira classe, não documentação descartável.
