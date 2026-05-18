# Trabalho prévio externo necessário — P273.14 CMYK-ICC paridade (NO-GO output)

**Data**: 2026-05-18.
**Passo origem**: P273.14 (NO-GO Fase A).
**Status**: documento de **pré-requisitos** para futuro hipotético GO.
**Cluster**: Visualize / Gradient.

---

## §0 — Propósito

Este documento é o **output legítimo** da decisão NO-GO em P273.14
(per spec §A.4 + §6 workflow). Descreve o trabalho **externo a um
passo futuro** que precisaria de ser concluído antes de P273.14
poder voltar como GO.

P273.14 NO-GO **não é falha**. É cumprimento honesto do critério
"verificar Fase A". A pendência P-Gradient-CMYK-ICC permanece como
item formal aberto mas com **trabalho prévio externo identificado**.

---

## §1 — Pré-requisitos para reanálise futura GO

### 1.1 — Decisão arquitectural sobre invariante L0 export.md

**Estado actual**: `00_nucleo/prompts/infra/export.md` linha 18 declara:

> "Este módulo (L3) converte essa geometria pura em bytes estruturados
> de PDF-1.7, sem `crates` externas de PDF — geração manual de
> objectos, xref e trailer."

**Decisão a tomar** (fora do escopo P273.14):
- Manter invariante literal (resultado: caminho 1 permanente vedado).
- Revogar parcialmente (resultado: ADR nova específica autorizando
  `qcms` ou crate análogo apenas para ICC profile handling).
- Clarificar escopo: "crates específicas para domínio auxiliar
  permitidas; crates que geram PDF directamente vedadas" — distingue
  `qcms` (color management) de `pdf-writer` (PDF generator).

**Forma do trabalho**:
- **ADR nova** dedicada (ex.: ADR-009X) com diagnóstico análogo
  ADR-0019 (precedente: autorização de `ttf-parser` + `rustybuzz`
  para parsing/shaping de fontes como excepção justificada ao
  princípio "L1+L3 puro").
- Aplicar Pattern 2 ADR-0093 ou criar PROPOSTO→IMPLEMENTADO mesmo
  passo se cristalino tomar decisão.

**Magnitude estimada**: S documental + 0 código L3 (a ADR autoriza;
não implementa).

### 1.2 — Profile ICC concreto + licença

**Estado actual**: não existe profile CMYK royalty-free industry-
recognized para redistribuição em produto:
- US Web Coated SWOP v2 — proprietário Adobe.
- FOGRA39/51/52 — proprietário ECI.
- GRACoL 2013 — proprietário IDEAlliance.
- "Generic CMYK no-profile" royalty-free — não existe (ICC.org Tech
  Note 7 explícito).

**Decisão a tomar** (fora do escopo P273.14):
1. **Adquirir licença** comercial para um dos profiles proprietários
   (custo: ~USD 100-500 + revisão legal redistribuição).
2. **Aguardar surgir** profile royalty-free industry-recognized
   (não previsto; ICC.org Tech Note 7 sugere ausência por design
   industry).
3. **Gerar profile cristalino** próprio (trabalho M+ de pesquisa
   colour science; fora do escopo cluster Gradient).
4. **Usar profile derivado de** dados open-source com gestão de
   licensing complex (e.g. variantes ECI documentadas em research
   papers).

**Forma do trabalho**:
- Decisão executiva/legal sobre opção (não decisão técnica do
  agente).
- Se opção 1: aquisição + documentação licença + auditoria.

**Magnitude estimada**: variável (1 dia se opção 1; meses+ se
opção 3).

### 1.3 — Decisão arquitectural sobre PDF size impact

**Estado actual**: profiles CMYK típicos 500 KB - 1.5 MB. Cada PDF
gerado com gradient CMYK carregaria este blob.

**Decisão a tomar** (fora do escopo P273.14):
- Aceitar inflation cada PDF (custo aceitável para PDF/A compliance).
- Implementar **inclusão condicional** — profile só embebido se
  documento usa gradient CMYK.
- Aplicar **dedup global** — múltiplos gradients CMYK no mesmo PDF
  partilham referência ao profile (similar pattern P273.12 DedupKey
  para gradients).
- Fallback per-config — utilizador escolhe `/DeviceCMYK` (size
  mínimo) vs `/ICCBased` (PDF/A) via opção CLI.

**Forma do trabalho**:
- ADR nova ou anotação cumulativa P273.X (consoante magnitude).
- Implementação L3 do mecanismo escolhido (~50-100 LOC).

**Magnitude estimada**: S-M.

---

## §2 — Critérios para reabrir P273.14 como GO futuro

Reanálise GO viável apenas quando **todos** os 3 itens §1 forem
resolvidos:

1. ✅ ADR autorizando crate específica (caminho 1) **OU** invariante
   L0 actualizado para permitir profile bytes hardcoded.
2. ✅ Profile concreto identificado E licença confirmada
   redistribuível.
3. ✅ Decisão sobre PDF size impact (dedup, inclusão condicional,
   ou aceitação).

Se algum item permanece pendente — NO-GO continua a ser o resultado
correcto.

---

## §3 — Pendência registada permanente

P-Gradient-CMYK-ICC permanece como **pendência aberta cluster
Visualize/Gradient** com:

- **Status**: scope-out reconfirmado por Fase A factual P273.14.
- **Pré-requisitos**: §1 acima (3 itens).
- **Reanálise**: quando §2 critérios cumpridos.
- **ADR-0091 §"ICC profile scope-out"** preserved literal — decisão
  P270.2 reconfirmada por evidência empírica P273.14.

**Cluster Gradient pode declarar-se feature-complete** sem este
refino. `/DeviceCMYK` continua como caminho actual; interpretação
device-dependent preservada; PDF/A compliance pendência inalterada.

---

## §4 — Sub-padrão "Scope-out reconfirmado por Fase A" N=1 inaugural

P273.14 inaugura sub-padrão emergente:

**Descrição**: passo executado até critério go/no-go binário; quando
empírica revela inviabilidade (custo, licensing, pré-requisitos
arquitecturais), output legítimo é **documento de pendência preserved
+ trabalho prévio externo identificado**.

**Distingue de**:
- "Bug arquitectural intencional corrigido" P273.12 (limitação
  fechada por refino deliberado quando contexto madura).
- "Refino qualitativo opcional materializado" (sub-padrão GO-only
  para passos que materializam — NÃO foi inaugurado por P273.14).

**Precedente metodológico**: cluster admite que nem toda pendência é
fechável — algumas exigem trabalho prévio externo. Honestidade
arquitectural sobre o que pode/não pode ser feito sem trabalho prévio
identificado.

---

## §5 — Referências

- Spec P273.14 — `00_nucleo/materialization/typst-passo-273-14.md`.
- Diagnóstico Fase A — `00_nucleo/diagnosticos/typst-passo-273-14-diagnostico.md`.
- ADR-0091 §"ICC profile scope-out" — decisão original P270.2.
- ADR-0019 — precedente "autorização de crates externas para domínio
  específico" (ttf-parser/rustybuzz para fontes).
- ADR-0029 — Pureza física L1 (preserved; este passo não toca L1).
- ADR-0054 — Critério fecho DEBT-1 graded (NO-GO output legítimo).
- L0 `00_nucleo/prompts/infra/export.md` linha 18 — invariante
  "sem crates externas de PDF" preserved literal.
- ICC.org Tech Note 7 — ausência design industry de profile CMYK
  genérico royalty-free.

---

*Documento imutável produzido em 2026-05-18 como output legítimo
da decisão NO-GO Fase A P273.14. Trabalho prévio externo (3 itens)
identificado; reanálise futura GO viável apenas quando §2 critérios
cumpridos. Cluster Gradient avança sem este refino — pendência
permanente registada.*
