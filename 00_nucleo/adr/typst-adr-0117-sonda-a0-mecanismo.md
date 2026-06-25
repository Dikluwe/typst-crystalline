# ADR-0117 — Sonda A.0: mecanismo operacional de verificação antes da spec

**Estado:** `EM VIGOR` (Passo 453 — mecanismo proposto e adoptado como extensão operacional da ADR-0114).  
**Decisão do dono (registada):** toda spec de materialização deve ser precedida de uma sonda A.0 que produza evidência empírica; a sonda pode ser um script, uma varredura mecânica ou um conjunto de greps, desde que produza output imutável com file:line e decisão derivada.  
**ADRs relacionadas:** ADR-0114 (sonda antes da spec), ADR-0065 (inventariar antes de decidir), ADR-0084 (Fase A antes de decisão), ADR-0085 (diagnósticos imutáveis).

---

## Contexto

A ADR-0114 estabeleceu o princípio: sonda do substrato (Fase A.0) é pré-condição para redigir uma spec de materialização. Entre P388 e P427, cinco specs foram escritas antes da sonda, forçando reclassificações retroativas. A ADR-0114 definiu o gate conceptual, mas não especificou o mecanismo operacional. Esta ADR transforma o princípio num processo verificável.

## Decisão

### 1. Formato da sonda

A sonda A.0 é um artefacto imutável em `00_nucleo/diagnosticos/` que contém:

1. **Comandos executados** (scripts, greps, queries).
2. **`file:line` dos substratos verificados**.
3. **Resultado pass/fail por critério**.
4. **Decisão derivada** do resultado.

A sonda não precisa ser automatizada numa única ferramenta; pode ser um passo-a-passo reproduzível.

### 2. Mecanismos operacionais

| Mecanismo | Quando usar | Exemplo |
|-----------|-------------|---------|
| **Script de grep** | Verificar existência de tipo/função/variante | `grep -rn "FrameItem::Link" 01_core/src/` |
| **Varredura mecânica** | Inventariar coverage de um domínio | `p425-varredura-mecanica.md` |
| **Diagnóstico empírico** | Validar estado de DEBTs, ADRs ou performance | `diagnostico-auditoria-passo-275.md` |
| **Teste de compilação** | Verificar se código proposto já compila/funciona | `cargo test --workspace` com filtro |

### 3. Evidência obrigatória por item A.0

Cada item da Fase A.0 numa spec deve ter:

- **Referência concreta**: ficheiro e linha no codebase actual.
- **Commit de referência**: hash do commit onde o substrato foi verificado.
- **Comando/observação**: como a evidência foi obtida.

### 4. Gate no linter

`crystalline-lint` mantém a verificação de drift entre prompts L0 e código L1–L4. Quando um novo ficheiro `.rs` já existe mas o prompt declara materialização, o linter deve flagar como potencial sonda-ausente. Esta flag é **heurística**, não bloqueante — a decisão final continua a ser humana.

## Consequências

- **Positivas.** Reduz reclassificações retroativas; força medição antes de estimativa; torna o gate da ADR-0114 operacional.
- **Custos.** Adiciona um artefacto obrigatório antes da redação da spec; specs de emergência precisam do mesmo rigor.
- **Riscos.** Sondas mal desenhadas podem dar falsos negativos; mitigação via revisão humana do output.

## Anti-padrões

- **Não** escrever uma spec de materialização baseada apenas em memória.
- **Não** confundir "a sonda pode correr depois" com "a sonda é opcional".
- **Não** apagar uma spec escrita antes da sonda; convertê-la em verificação retroativa preserva histórico.

## Aplicação em P453

Esta ADR foi motivada pela auditoria pós-P450–P452, que revelou:

- P452: Link annotation já existia desde P422–P424 — reclassificado de S-M para XS.
- P450: Parser BibTeX em L1 em vez de L3 — reconciliação de camada documentada.

Ambos os casos seriam detectáveis por uma sonda A.0 com grep mecânico antes da redação da spec.
