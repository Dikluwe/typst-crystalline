# Auditoria fresh do projecto Typst Cristalino

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0-L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

Esta auditoria avalia a qualidade actual do projecto
inteiro (`01_core/` foco principal). Material auxiliar
em `00_nucleo/` está disponível para contexto factual mas
não para framing.

---

## Postura do auditor

**Importante**: o auditor pode ler ADRs, diagnósticos,
relatórios e qualquer outro material em `00_nucleo/`. Mas
o material lido é **contexto factual** — explica o que
existe, quando foi criado, por que razão histórica.

O material **não é justificação a aceitar**. Decisões
documentadas em ADRs podem ser reavaliadas. Diagnósticos
anteriores podem ter conclusões erradas. Relatórios de
passos podem ter omissões.

O auditor avalia o estado actual do código contra critérios
definidos abaixo, **independentemente do que foi decidido
antes**. Se uma decisão prévia produziu código que não
cumpre critérios, isso é finding válido — não desvio
desculpável.

---

## Escopo

**Primário**: `01_core/src/` — entities + rules + tudo dentro.

**Secundário**: `02_shell/`, `03_infra/`, `04_wiring/` —
inspeccionar como camadas se relacionam com L1.

**Referência comparativa**: `lab/typst-original/crates/` —
vanilla equivalente para cada estrutura.

**Material auxiliar disponível**:
- ADRs em `00_nucleo/adr/`.
- Diagnósticos em `00_nucleo/diagnosticos/`.
- Relatórios em `00_nucleo/materialization/`.
- Linter em `crystalline-lint/`.

---

## Critérios objectivos

Para cada estrutura significativa (struct, enum, módulo
funcional), medir:

### O1 — Tamanho

- Linhas do ficheiro.
- Número de fields públicos (para structs).
- Número de variants (para enums).
- Número de funções/métodos públicos.

### O2 — Acoplamento

- Fan-in: `grep -lrwE "<Type>" 01_core/src/` — quantos
  ficheiros referem este tipo.
- Fan-out: `use` declarations no ficheiro de definição.
- Indicar se fan-in/fan-out são proporcionais ao tamanho.

### O3 — Coesão

- Número de razões distintas para modificar o tipo
  (avaliação manual baseada em conceitos ortogonais
  agrupados).
- Indicar se cada field/variant pertence à mesma
  responsabilidade.

### O4 — Complexidade ciclomática

- Identificar funções com muitos paths (early returns,
  branches profundos).
- Listar funções com mais de 100 linhas ou mais de 20
  branches.

### O5 — Testabilidade

- O tipo pode ser testado isoladamente?
- Quais dependências são necessárias para testar uma
  feature pequena?
- Existem testes que precisam de instanciar metade do
  projecto?

---

## Critérios qualitativos

Para cada estrutura significativa, julgar (com
justificação literal):

### Q1 — Clareza de propósito

A estrutura tem propósito claro identificável só pelo
nome e tipo? Ou é nome genérico que esconde N
responsabilidades?

### Q2 — Apropriação de domínio

A estrutura modela conceito real do domínio (Typst
documents, layout, introspection, etc.) ou é colecção
ad-hoc de campos relacionados por proximidade?

### Q3 — Fluência de uso

Para um programador que precisa de usar este tipo, a
API é guiada (compilador ajuda) ou exige conhecimento
implícito (convenções não documentadas)?

### Q4 — Robustez ao crescimento

Se for adicionada feature relacionada, esta estrutura
cresce graciosamente (extension point claro) ou exige
modificação invasiva?

### Q5 — Honestidade do nome

O nome reflecte o que a estrutura faz, ou é histórico/
aspiracional/enganador?

### Q6 — Comparação com vanilla

Para o conceito equivalente em vanilla:
- Cristalino é mais simples? Mais complexo? Igual?
- Cristalino expõe menos? Mais? Igual?
- Cristalino tem invariantes mais fortes? Mais fracas?
- Conclusão: cristalino é melhor / igual / pior que vanilla
  para este conceito específico.

---

## Output esperado

Ficheiro único em
`00_nucleo/diagnosticos/auditoria-fresh-projecto.md` com
6 secções:

### Secção 1 — Inventário

Lista plana de todas as estruturas significativas
inspeccionadas, agrupadas por módulo:
- `01_core/src/entities/`
- `01_core/src/rules/`
- `02_shell/`
- `03_infra/`
- `04_wiring/`

Para cada uma: nome, ficheiro, linhas.

### Secção 2 — Tabela quantitativa (critérios objectivos)

Tabela com uma linha por estrutura, colunas O1-O5.
Valores numéricos onde aplicável.

### Secção 3 — Tabela qualitativa (critérios subjectivos)

Tabela com uma linha por estrutura, colunas Q1-Q6.
Cada célula tem 1-2 frases de julgamento + classificação
(claro/ambíguo, real/ad-hoc, etc.).

### Secção 4 — Findings

Lista de problemas concretos identificados, sem
hierarquia. Para cada finding:
- Estrutura(s) afectada(s).
- Critério violado (O1-O5 ou Q1-Q6).
- Descrição literal do problema.
- Magnitude estimada (em palavras: pequeno, médio,
  grande — sem números inventados).

Findings devem ser concretos: "CounterStateLegacy tem 14
fields públicos cobrindo 12 conceitos ortogonais (O3, Q2)"
não "código complicado".

### Secção 5 — Pontos fortes

Lista de aspectos onde o projecto está objectivamente
bem. Mesmos critérios mas em positivo.

Não inflar — só listar o que cumpre claramente os
critérios. Se nada cumprir, secção fica vazia (e isso é
finding implícito).

### Secção 6 — Resumo executivo

Texto curto (máximo 1 página) que responde:

1. Quantas estruturas inspeccionadas.
2. Quantas têm findings significativos (críticos ou
   médios).
3. Quantas estão claramente bem.
4. Quais são os 3-5 problemas mais pungentes (ordem
   livre, sem hierarquia inventada).
5. Quais são os 3-5 pontos fortes mais notáveis.
6. Avaliação geral em 1-2 frases.

A avaliação geral pode ser positiva ou negativa
conforme o que os números mostram. Não calibrar para
parecer balanceada — se o projecto está bem, dizer; se
não, também.

---

## Restrições

- **Não escrever ADR** sobre os achados.
- **Não criar reservas** de identificadores.
- **Não propor refactor** ou plano de acção.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Não classificar como passo PNNN**: é auditoria.
- **Sem código novo escrito**: trabalho de leitura
  apenas.
- **Não pedir confirmação ao humano antes de findings**:
  o auditor decide com critérios; humano lê depois.
- **Honestidade obrigatória**: se algo está mal, dizer
  literalmente. Se está bem, dizer literalmente. Sem
  "no entanto isto é compensado por..." que apaga
  findings.

---

## Sobre o uso do material auxiliar

**ADRs**: lê para perceber **decisões registadas**, não
para validar essas decisões. Se uma ADR justifica algo
que não cumpre critérios, isso é finding (a ADR pode
estar errada, ou cumpriu-se mal).

**Diagnósticos**: lê para perceber **estado factual
documentado**. Diagnósticos anteriores podem ter findings
que ainda são válidos — citar se aplicável.

**Relatórios de passos**: lê para perceber **história
das decisões**. Útil para identificar quando e como
estado actual foi atingido. Não para defender estado
actual.

**Linter `crystalline-lint`**: correr e reportar warnings
ou violations.

---

## Critério de conclusão

- Ficheiro `auditoria-fresh-projecto.md` produzido.
- 6 secções presentes.
- Cada estrutura significativa aparece nas tabelas das
  secções 2 e 3.
- Findings são concretos com referência a estruturas e
  critérios.
- Resumo executivo responde às 6 perguntas.
- Sem ADR nova.
- Sem reservas.
- Sem propostas de refactor.

A auditoria é instrumento. Decisões sobre o que fazer
com os resultados ficam para depois.
