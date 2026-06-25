# Relatório de Execução — P455 — Fecho das correções retroativas P453 + Cláusula 4 da ADR-0117

> **Passo:** 455  
> **Data de execução:** 2026-06-25  
> **Executor:** Kimi Code CLI  
> **Estado:** Concluído

---

## 1. Resumo

P455 fecha duas pontas soltas identificadas na auditoria pós-P454:

1. **Errata no spec P453-C5:** o spec pedia 67 ADRs, mas P453 criou a
   ADR-0117, elevando o total para 68. Foi anexada nota de correção
   retroativa sem reescrever o documento histórico.
2. **Lacuna na ADR-0117:** as cláusulas 1–3 cobriam verificação empírica de
   ficheiros e infraestrutura, mas não a verificação de decisões
   arquitectónicas registadas (ADRs/fronteiras) antes de propor estrutura para
   elementos existentes. A Cláusula 4 foi anexada ao ADR-0117 para fechar essa
   lacuna, usando o P454 como evidência exemplo.

---

## 2. Arquivos alterados / criados

| Arquivo | Mudança |
|---------|---------|
| `00_nucleo/materialization/typst-passo-453-nota-adr68.md` | Criado — nota de correção retroativa do spec P453-C5. |
| `00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md` | Atualizado — cabeçalho refere 4 cláusulas; Cláusula 4 anexada com procedimento, evidência do P454, mecanismo de grep sugerido e impacto. |
| `00_nucleo/materialization/typst-passo-455-relatorio.md` | Este relatório. |

---

## 3. Verificação do documento de cobertura

O documento de cobertura (`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`)
já reflecte **68 ADRs** na linha de snapshot do P447/P453:

> "Cristalino snapshot: Passo 447/P453; **68 ADRs** [...]"

O `00_nucleo/adr/README.md` também já lista o total de 68 ADRs:

> "**Total**: 68 ADRs (67 números únicos; ADR-0026 tem variante -R1 [...])"

Logo, **não foi necessária correção adicional** no documento de cobertura nem no
README de ADRs.

---

## 4. Decisão sobre o README de ADRs

O README não possui um "sumário de cláusulas" para ADR-0117; lista apenas o
título curto e o status na tabela "Estado por ADR". Como a tabela não enumera
cláusulas, não houve alteração a fazer no README. O status de ADR-0117
permanece `EM VIGOR`.

---

## 5. Resultados

### 5.1 `cargo test --workspace`

Zero código alterado. Executado com `RUST_MIN_STACK=16777216` para contornar
stack overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`.

Resultado: **todos os testes passaram**.

### 5.2 `crystalline-lint .`

Zero código alterado. Resultado: apenas os warnings pré-existentes de prompts
órfãos (`adr-stub-vs-fallback.md`, `show-regex.md`). **Nenhuma nova violação**
relacionada a este passo.

---

## 6. Checklist de fecho

- [x] `typst-passo-453-nota-adr68.md` criado em `00_nucleo/materialization/`.
- [x] Cláusula 4 anexada a `00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md`.
- [x] Sumário de ADR-0117 actualizado no cabeçalho (referência às 4 cláusulas).
- [x] Documento de cobertura verificado — já reflecte 68 ADRs.
- [x] `crystalline-lint` sem novas violações.
- [x] `cargo test --workspace` verde.

---

## 7. Notas

- Este passo é puramente documental/processual; nenhum código de produção foi
  alterado.
- A nota de correção P453-C5 preserva o spec original intacto, anexando a
  errata como documento separado, em linha com a política de imutabilidade de
  documentos históricos em `00_nucleo/materialization/`.
- A Cláusula 4 da ADR-0117 formaliza a lição aprendida no P454: antes de
  propor estrutura para elementos existentes, verificar ADRs e fronteiras que
  decidiram a forma actual.

---

**Fim do relatório.**
