# ⚖️ ADR-0085: Diagnóstico imutável — artefacto produzido por audit

**Status**: `EM VIGOR`
**Data**: 2026-05-15
**Autor**: Humano + IA
**Validado**: Passo 260 — formaliza padrão N=4 das aplicações
P255/P257/P258/P259.
**Estende**: ADR-0034 (diagnóstico canónico).

---

## Contexto

ADR-0034 (diagnóstico canónico) estabeleceu que diagnósticos
de tipos vanilla obrigatórios são imutáveis após criação.
Quatro passos consecutivos (P255/P257/P258/P259) produziram
**diagnósticos de audit Fase A** com a mesma propriedade —
ficheiros marcados explicitamente "Imutável após criação per
ADR-0034" em
`00_nucleo/diagnosticos/diagnostico-<modulo>-fase-a-passo-NNN.md`.

**N=4 cumulativo**:

| Passo | Ficheiro imutável | Função |
|-------|-------------------|--------|
| P255 | `diagnostico-math-fase-a-passo-255.md` | Audit DEBT-8 Math |
| P257 | `diagnostico-color-vanilla-passo-257.md` | Audit Color vanilla 8/8 |
| P258 | `diagnostico-model-fase-a-passo-258.md` | Audit Model 22 entradas |
| P259 | `diagnostico-visualize-fase-a-passo-259.md` | Audit Visualize 27 subsistemas |

**Limiar formalização N=3-4 atingido**. Próxima aplicação
audit cumprirá automaticamente.

## Decisão

**Diagnósticos de audit Fase A** (per ADR-0084) **são artefactos
imutáveis** em `00_nucleo/diagnosticos/` análogos a
diagnósticos de tipo vanilla (ADR-0034).

### Propriedades obrigatórias

1. **Localização canónica**: `00_nucleo/diagnosticos/`.
2. **Nome canónico**:
   `diagnostico-<modulo>-fase-a-passo-NNN.md` para audits
   estruturados (ADR-0084), ou
   `diagnostico-<tipo>-vanilla-passo-NNN.md` para audits
   de tipo vanilla (ADR-0029 §"Diagnosticar primeiro").
3. **Marcador explícito de imutabilidade**: "Imutável após
   criação per ADR-0034" (ou ADR-0085 a partir deste passo).
4. **Cabeçalho canónico**:
   - Data.
   - Executor (humano vs Claude Code).
   - Padrão (ADR-0034 / ADR-0084 / ADR-0065).
   - Diagnóstico pai (referência cruzada).
   - Análogo estrutural (passo precedente, se aplicável).
5. **Conteúdo literal**, não interpretativo (output `grep`/
   `view` colado; classificações em coluna separada da
   evidência).
6. **Tabelas estruturadas** (Tabela A entradas;
   Tabela B agregada).
7. **Secção "Decisão"** explícita (B1/B2/B3 conforme
   ADR-0084).

### Diferença vs diagnósticos transientes

| Tipo | Imutável? | Localização | Função |
|------|-----------|-------------|--------|
| Diagnóstico Fase A audit (ADR-0084) | **Sim** | `00_nucleo/diagnosticos/` | Evidência factual |
| Diagnóstico tipo vanilla (ADR-0029) | **Sim** | `00_nucleo/diagnosticos/` | Evidência tipo |
| Diagnóstico preparatório/planeamento | Não | `00_nucleo/diagnosticos/` | Pode ser actualizado |
| Inventário trivial inline | n/a | inline no passo | Auto-aplicação ADR-0065 |

### Distinção crítica vs ADR-0034

| Aspecto | ADR-0034 (precedente) | ADR-0085 (este passo) |
|---------|----------------------|------------------------|
| Âmbito | Tipo vanilla a materializar | Audit empírico de módulo |
| Gatilho | Novo tipo a entrar em L1 | Cobertura ambígua (ADR-0084) |
| Output | Estrutura/operadores tipo | Tabelas A/B + decisão B1/B2/B3 |
| Imutabilidade | Sim (per ADR-0034) | Sim (este ADR) |

ADR-0085 **estende** ADR-0034 cobrindo o novo âmbito (audit
empírico) sem alterar a regra original (tipo vanilla).

## Consequências

### Positivas

- **Evidência factual preservada**: histórico empírico não
  pode ser re-escrito retroactivamente.
- **Cross-references estáveis**: passos posteriores podem
  citar diagnósticos imutáveis com confiança.
- **Auditoria de processo**: futuro humano pode verificar se
  decisões pós-audit foram coerentes com evidência.

### Negativas

- **Sem actualização** se evidência factual mudar (audit
  futuro produz novo ficheiro com novo número).

### Neutras

- Diagnósticos preparatórios (não-imutáveis) podem coexistir
  na mesma directoria; distinção é por marcador explícito de
  imutabilidade.

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| Permitir actualização de diagnósticos Fase A | Mais flexível | Perde rastreabilidade; evidência factual pode ser revista |
| Revisão `-R1` de ADR-0034 em vez de ADR-0085 | Centraliza regras | Mistura âmbitos (tipo vanilla vs audit) |
| **Decisão adoptada: ADR-0085 nova estendendo ADR-0034** | **Foco preservado; novo âmbito tem ADR próprio** | **+1 ADR; aceitável dado padrão N=4** |

## Referências

- ADR-0034 — Diagnóstico canónico (estendido por este ADR).
- ADR-0029 — Pureza física L1 + diagnóstico vanilla obrigatório
  (cita diagnósticos imutáveis de tipo).
- ADR-0084 — Auditoria condicional (consumidor primário deste
  ADR).
- Aplicações:
  - P255 — `diagnostico-math-fase-a-passo-255.md`.
  - P257 — `diagnostico-color-vanilla-passo-257.md`.
  - P258 — `diagnostico-model-fase-a-passo-258.md`.
  - P259 — `diagnostico-visualize-fase-a-passo-259.md`.

---

## Auto-aplicação

P260 não produz diagnóstico Fase A imutável (passo
administrativo XS). Inventário §A inline no relatório cumpre
ADR-0065 critério #5 §"Neutras" (inline aceitável para
inventário trivial).
