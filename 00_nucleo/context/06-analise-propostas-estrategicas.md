# Análise das 5 Propostas Estratégicas

**Data**: 2026-04-01
**Contexto**: Avaliação técnica contra o knowledge base do projecto typst-crystalline

---

## 1. Correção Estratégica: O Marco do Passo 36

### Proposta
Aprovar a execução dos passos 56–75 (matemática) imediatamente após o passo 35, argumentando que saltar matemática tornaria o compilador inútil para o público-alvo do Typst (escrita académica e científica).

### Avaliação: **Concordo com a recomendação, com uma nuance de sequenciamento.**

O argumento central está correcto — o Typst é usado maioritariamente para documentos académicos. Um "subconjunto viável de produção" sem `$x^2 + 1$` é um subconjunto viável para quase ninguém no público real do Typst. O parser já reconhece `SyntaxKind::Equation` (confirmado nos testes do Passo 4), o que significa que documentos com matemática já são parseados mas não avaliados — a experiência seria de degradação silenciosa, pior do que uma falha explícita.

A nuance: os passos 36–55 (layout engine) incluem a infra-estrutura de caixas, flow e posicionamento que o motor de matemática vai precisar. Equações são, fundamentalmente, problemas de layout. Mover matemática para antes do layout engine real criaria uma dependência circular — o motor de equações precisa de `hbox`, `vbox`, e alinhamento vertical que ainda não existem.

**Recomendação concreta**: Manter a sequência layout (36–55) → matemática (56–75), mas **eliminar a decisão pendente do Passo 36** sobre "saltar matemática". A fase de matemática não é opcional — deve ser declarada como obrigatória no roadmap. Formalizar isto num ADR (ADR-0032) para tornar a decisão vinculativa.

O marco de "subconjunto viável de produção" deve ser redefinido do Passo 50 para o Passo 75 (após matemática), ou então dividido em dois marcos: Passo 50 (viável para documentos não-académicos) e Passo 75 (viável para documentos académicos).

---

## 2. Automação e Redução de Trabalho Manual

### Proposta
Criar uma ferramenta CLI em L2 para automatizar a etapa de diagnóstico (grep/find no oráculo), extraindo assinaturas de funções automaticamente.

### Avaliação: **A intenção é boa, mas a camada está errada e o valor é limitado.**

**Problema de camada**: A ferramenta lê `lab/typst-original/` — isto é I/O de filesystem. Pertence a L3 (infra), não a L2 (shell). L2 conhece apenas L1 e traduz input do utilizador. Uma ferramenta que varre directórios e extrai AST de ficheiros Rust é trabalho de infraestrutura.

**Problema de valor**: O diagnóstico não é apenas "extrair assinaturas". Os comandos `grep`/`find` do workflow são intencionalmente manuais porque cada passo tem perguntas diferentes:

- Passo 10: "FontInfo tem campos primitivos ou tipos de ttf_parser?"
- Passo 14: "comemo::track(&w) ou w.track() via trait?"
- Passo 22: ".body() retorna SyntaxNode, Markup, ou outro?"

Uma ferramenta genérica de extracção de assinaturas não responde a estas perguntas — são análises qualitativas que dependem do contexto do passo. O que automatiza bem é o que já é automatizado: o linter verifica violations, `cargo test` verifica correcção.

**Onde a automação faria sentido**: Um script (não ferramenta CLI, não crate — um script bash em `lab/scripts/`) que gera um relatório padronizado para cada módulo do oráculo: exports públicos, dependências externas, contagem de linhas, classificação tentativa L1/L3. Isto não substitui o diagnóstico manual, mas reduz os 6-8 comandos grep de cada passo a um único comando. E não requer ADR nem prompt L0 — é uma utilidade de lab, não código de produção.

---

## 3. Expansão do crystalline-lint para Garantia de Pureza

### Proposta
Expandir o linter para inspecionar a AST dos ficheiros em L1, bloqueando chamadas a bibliotecas padrão de rede, filesystem ou relógio.

### Avaliação: **O linter já faz isto — mas há espaço para melhorar.**

V4 (ImpureCore / ForbiddenSymbol) já existe e opera exactamente sobre a AST. A documentação do linter confirma:

> "Em Rust, aliases de importação são resolvidos para FQN antes da verificação — `use std::fs as f; f::read(...)` é detectado como `std::fs::read`."

Portanto, o linter já resolve aliases e verifica FQN contra uma lista de símbolos proibidos por linguagem via `forbidden_symbols_for(language)`. Não é revisão manual — é verificação mecânica.

**O que pode ser melhorado** (e aqui a proposta tem mérito):

1. **Cobertura de `std::time::SystemTime` e `std::time::Instant`**: Se não estão na lista de `forbidden_symbols_for("rust")`, devem ser adicionados. São relógio do sistema — violam pureza L1 pela definição ADR-0029.

2. **`std::env`**: Acesso a variáveis de ambiente é I/O do sistema. Confirmar que está na blocklist.

3. **Dependências transitivas**: V4 verifica imports directos, mas não verifica se uma crate autorizada em `[l1_allowed_external]` re-exporta funções de I/O. Exemplo hipotético: se `indexmap` re-exportasse `std::fs` (não re-exporta, mas o princípio importa). Esta é uma limitação real, mas resolver requer análise de dependências transitivas — trabalho significativo para o linter.

4. **`unsafe` blocks em L1**: Blocos `unsafe` podem chamar funções de sistema via FFI sem passar por `std::`. Adicionar uma regra V15 que sinaliza `unsafe` em L1 como warning (não error, porque `unsafe` pode ser puro — ex: `std::mem::transmute`) seria uma rede de segurança adicional.

**Recomendação**: Não é necessário "expandir para inspecionar AST" — V4 já faz isso. O trabalho é auditar e completar a lista de `forbidden_symbols_for("rust")` para garantir cobertura total de `std::time::SystemTime`, `std::env`, `std::process::Command`, e considerar uma regra de warning para `unsafe` em L1.

---

## 4. Rastreabilidade de Prompts (Camada L0)

### Proposta
Tornar obrigatória uma secção "Changelog" ou "Motivo da Falha" dentro do prompt sempre que houver alteração de versão (v1 → v2 → v3).

### Avaliação: **Excelente proposta. Deve ser implementada.**

Este é um gap real na rastreabilidade. Actualmente:

- Prompts versionados existem (ex: `typst-passo-4-parse.md` → `typst-passo-4-parse-v2.md`)
- Mas a razão da versão está dispersa — pode estar no relatório do passo, numa mensagem de chat, ou na memória do Diego
- Quando alguém (humano ou agente) lê `typst-passo-6-eval-v3.md`, não sabe o que falhou na v1 e v2 sem procurar em múltiplas fontes

**Implementação sugerida**: Adicionar uma secção obrigatória no template de prompt:

```markdown
## Histórico de versões

| Versão | Data | Motivo da revisão |
|--------|------|-------------------|
| v1 | 2026-03-23 | Criação inicial |
| v2 | 2026-03-24 | API de comemo incompatível com Cenário A — migrar para Cenário D |
| v3 | 2026-03-25 | LetBindingKind::Normal requer .bindings() não .ident() — corrigir travessia |
```

O campo "Motivo da revisão" deve capturar **o que falhou**, não o que mudou (a diff mostra o que mudou — o motivo é o contexto que a diff não preserva).

**Enforcement**: Adicionar uma regra ao linter (V16 — PromptVersionWithoutChangelog) que dispara warning quando um ficheiro de prompt tem sufixo `v2+` mas não contém a secção de histórico. Enforcement mecânico é melhor que disciplina humana.

Alternativamente, e de forma mais simples: em vez de criar ficheiros separados com sufixo de versão, manter um único ficheiro e registar o histórico de revisões inline. Isto é mais alinhado com o padrão de ADRs (que usam um campo "Status" mutável, não ficheiros separados por versão).

---

## 5. Resolução da Dívida DEBT-6

### Proposta
Mover a construção do ambiente de teste (`eval_for_test`, `MockWorld`) de L1 para L4 (Wiring), argumentando que manter código de teste misturado com lógica de domínio viola o isolamento de fases.

### Avaliação: **Discordo fundamentalmente. A proposta parte de uma premissa incorrecta.**

**O que `eval_for_test` realmente é**: Uma função `#[cfg(test)]` que cria os wrappers de `comemo` necessários para chamar `eval()` num contexto de teste. Existe porque a assinatura pública de `eval()` requer `Tracked<dyn TrackedWorld>`, e criar um `Tracked<>` manualmente em cada teste é verboso e frágil. É uma conveniência de teste, compilada condicionalmente — não existe no binário de produção.

**`#[cfg(test)]` não é "código misturado"**: Em Rust, `#[cfg(test)]` é compilação condicional. O bloco inteiro não existe no binário release. Não é uma violação de isolamento de fases — é o mecanismo padrão de Rust para co-locação de testes. O `CLAUDE.md` do projecto explicita:

> "Testes co-localizados no mesmo ficheiro via `#[cfg(test)]`. Nunca ficheiros `_test.rs` separados."

Mover `MockWorld` para L4 criaria três problemas concretos:

1. **Violação de topologia**: L4 é composição de produção. Colocar mocks de teste em L4 mistura infraestrutura de teste com wiring de produção — exactamente o tipo de contaminação que o isolamento de fases pretende evitar.

2. **Dependência circular**: Testes de L1 dependeriam de L4 para ter os mocks. Mas L4 depende de L1. Isto é um ciclo — proibido pela Regra de Gravidade.

3. **Perda de localidade**: O padrão actual permite ler `eval.rs` e ver, no mesmo ficheiro, os testes que validam o comportamento. Mover os mocks para L4 fragmenta o contexto — o agente (Claude Code) que implementa um passo teria de navegar entre duas crates para entender o que está a testar.

**O que DEBT-6 realmente precisa**: DEBT-6 não é sobre onde vivem os mocks — é sobre cobertura cega. O problema é que `eval_for_test` pode esconder comportamentos de `eval()` que dependem de `comemo` tracking real (ex: invalidação incremental, memoização). Os testes passam com o mock mas falhariam com uma implementação real de `TrackedWorld` porque o mock não exerce os paths de tracking.

A solução correcta é adicionar testes de integração em L4 que usem `SystemWorld` real (de L3) contra fixtures conhecidas. Estes testes complementam os testes unitários de L1, não os substituem. L1 testa lógica pura com mocks; L4 testa integração com implementações reais.

---

## Resumo de acções recomendadas

| # | Proposta | Veredicto | Acção |
|---|---------|-----------|-------|
| 1 | Matemática obrigatória | ✅ Aceitar | ADR-0032 declarando fase matemática como obrigatória; redefinir marco de viabilidade |
| 2 | Ferramenta CLI de diagnóstico | ⚠️ Aceitar parcialmente | Script em `lab/scripts/`, não ferramenta em L2; sem ADR necessário |
| 3 | Expansão do linter | ⚠️ Aceitar parcialmente | Auditar `forbidden_symbols_for("rust")`; considerar V15 para `unsafe` em L1 |
| 4 | Changelog em prompts | ✅ Aceitar | Secção obrigatória no template; considerar V16 para enforcement mecânico |
| 5 | Mover mocks para L4 | ❌ Rejeitar | Criar testes de integração em L4, mantendo mocks em L1 `#[cfg(test)]` |
