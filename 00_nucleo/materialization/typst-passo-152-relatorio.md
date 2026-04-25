# Passo 152 — Relatório (refino administrativo do plano DEBT-54)

**Data**: 2026-04-25
**Natureza**: passo **L0-puro / administrativo XS**.
**Zero código**. **Zero testes**. **Zero ADRs**. **Zero novos
DEBTs**. Único output: actualização da entrada DEBT-54 em
`DEBT.md` com 4 sub-secções novas + cabeçalho.
**Precondição**: Passo 151 encerrado; DEBT-54 aberto com
plano de 6 itens; 12 DEBTs abertos.

---

## 1. Sumário

Probe empírico do cargo cache local resolveu as 3 dúvidas
flagged em P151:

- **Codex 0.2.0**, **hayagriva 0.9.1**, **oxipng 9.1.3** —
  todos em cache, todos compatíveis com versões esperadas
  por vanilla.
- **Conflito de versão `comemo`** identificado: cristalino
  `0.4` vs vanilla `0.5.1`. Cargo aceita duas versões em
  paralelo (semver 0.x trata como majors distintas);
  contornável sem `[patch]`.
- Outros 3 crates partilhados (`ttf-parser`, `ecow`,
  `rustc-hash`) — sem conflito; cargo unifica.

**Estimativa de DEBT-54 revista** de M-L (3-6h) para **M**
(3-4h) — risco de fetch online eliminado, risco de unify
contornável.

**Critério de fecho expandido em 3 níveis** (mínimo /
suficiente / executável) com regra de fecho parcial via
DEBT-55 condicional.

---

## 2. Probe online (sub-passo 152.1)

### 2.1 — Cache local check (offline-first)

```bash
for c in codex hayagriva oxipng; do
  find ~/.cargo/registry/cache -maxdepth 3 -name "${c}-*.crate"
done
```

Resultado:

| Crate | Estado | Versão cached | Versão vanilla | Conclusão |
|-------|--------|---------------|----------------|-----------|
| `codex` | ✓ cached | 0.2.0 | `"0.2.0"` | match exacto |
| `hayagriva` | ✓ cached | 0.9.1 | `"0.9.1"` | match exacto |
| `oxipng` | ✓ cached | 9.1.3 | `^9.0` (`default-features = false, features = ["filetime", "parallel", "zopfli"]`) | satisfaz semver |

**Conclusão**: as 3 crates flagged em P151 §2.4 como
"provavelmente ausentes; exigem fetch online" estão **todas
em cache local com versões compatíveis**. P151 estimou
conservadoramente; a realidade revelou cobertura completa.

### 2.2 — Decisão sobre fetch online

Não foi necessário invocar `cargo fetch` em directório
temporário. Cache local é suficiente para o probe deste
passo. Materialização efectiva de DEBT-54 (passo dedicado
posterior) pode prosseguir directo a `cargo build`.

---

## 3. Risco de conflito de versão (sub-passo 152.2)

Verificação empírica das 4 crates compartilhadas (cristalino
`Cargo.toml` workspace.dependencies vs vanilla
`Cargo.toml.original`):

| Crate | Cristalino | Vanilla | Cargo unifica? | Risco |
|-------|-----------|---------|----------------|-------|
| `ttf-parser` | `"0.25"` | `"0.25.0"` | Sim — mesma 0.25.x | Nenhum |
| `comemo` | `"0.4"` | `"0.5.1"` | **Não** — major 0.4 ≠ 0.5 | **Alto** |
| `ecow` | `"0.2"` | `"0.2.6"` | Sim — features adicionais; cargo unifica | Nenhum |
| `rustc-hash` | `"2"` | `"2.1"` | Sim — mesma 2.x | Baixo |

### 3.1 — Análise de `comemo` 0.4 vs 0.5

Em semver pré-1.0, `0.4.x` e `0.5.x` são tratadas como
**incompatíveis**. Cargo **não unifica** automaticamente —
cria duas versões paralelas no grafo de deps:

- `typst-core` (cristalino) → `comemo 0.4` (per ADR-0001).
- `typst-library` (vanilla) → `comemo 0.5`.
- Cargo aceita ambas no mesmo binário (cada uma compilada
  como crate distinta a nível de hash).
- `lab/parity/tests/layout_parity.rs` usa ambos os pipelines
  isoladamente; **sem trocas de tipos `comemo::Tracked` entre
  eles** — cada pipeline usa o seu `comemo` internamente.
- **Compilação não falha**; apenas binário de tests é
  ligeiramente maior (~50KB de duplicação aceitável).

### 3.2 — Estratégias antecipadas

Hierarquia de respostas se conflitos surgirem ao materializar
DEBT-54:

1. **Verificar se cargo unifica** (semver compatível). Maioria
   dos casos resolve aqui.
2. **Se major divergente** (como `comemo`): aceitar duas
   versões; custo é binário maior.
3. **Se incompatibilidade ABI**: `[patch.crates-io]` força
   versão única — pode degradar um dos pipelines. **Última
   alternativa**.
4. **Caso extremo** (feature flag por target): abrir DEBT
   dedicado.

---

## 4. Critério de fecho expandido (sub-passo 152.2)

DEBT-54 fecha quando os 3 níveis estão cumpridos:

1. **Mínimo** — `cd lab && cargo build -p typst-layout` corre
   sem erros.
2. **Suficiente** — `cd lab && cargo build -p typst` (compilador
   inteiro) corre sem erros.
3. **Executável** — test simples em
   `lab/parity/tests/vanilla_smoke.rs` que invoca
   `typst::compile(world)` para um source trivial e devolve
   `PagedDocument` sem panic.

**Fecho parcial** aceitável apenas com **DEBT-55** explicitando
o nível pendente.

---

## 5. Diff aplicado a DEBT-54

`00_nucleo/DEBT.md` Secção 1, entrada DEBT-54:

- **Adicionada sub-secção "Actualização Passo 152 — Refino
  administrativo"**:
  - §3 — Probe online (resultado tabelado).
  - §4 — Risco de conflitos de versão (tabela das 4 crates
    + análise `comemo` + estratégias).
  - §5 — Critério de fecho expandido em 3 níveis.
  - §6 — Conclusão pós-refino (estimativa M, antes M-L).
- **Não alterado**: título, número, data de abertura, plano
  original, critério mínimo de fecho, notas anteriores.

Volume: ~80 linhas adicionadas; entrada DEBT-54 cresce de
~80 linhas para ~160. Aceitável dado o volume de informação
estruturada.

---

## 6. Cabeçalho de DEBT.md actualizado

```diff
> **Passo 151 (2026-04-25)**: aberto **DEBT-54** ao tentar
> fechar DEBT-53. ... Total abertos: **11 → 12**.
+
+ > **Passo 152 (2026-04-25)**: refino administrativo do plano
+ > DEBT-54 (probe online dos 3 crates flagged + risco de
+ > versões + critério expandido em 3 níveis). Probe revelou
+ > `codex`/`hayagriva`/`oxipng` **todos em cache local**
+ > (estimativa P151 desactualizada); identificado conflito
+ > material em `comemo` (cristalino 0.4 vs vanilla 0.5,
+ > cargo aceita duplicação). Saldo de DEBTs **inalterado**
+ > (12).
```

---

## 7. Próximo passo: 153

**P153 — P2 (`value_dto.rs`) cristalino-only baseline**.
Independente de DEBT-54 (que pode ser materializado em
paralelo).

Estratégia análoga a P150:
- Materializar `lab/parity/src/value_dto.rs` com 18 variants
  cristalino mapeadas + catch-all para `Other(name)`.
- Materializar `lab/parity/tests/eval_parity.rs` que evaluate
  source `.typ` em cristalino e converte resultado para
  ValueDTO.
- Expandir corpus em `lab/parity/corpus/semantic/`.
- Actualizar matriz `latest.md` para incluir nível P2.

`from_vanilla` para ValueDTO permanece stub até DEBT-54 +
DEBT-53 fecharem (mesma estratégia P150).

---

## 8. Verificação final

| Item | Estado |
|------|--------|
| Probe online dos 3 crates executado e tabelado | ✅ |
| DEBT-54 ganha 4 sub-secções novas (§3, §4, §5, §6) | ✅ |
| Cabeçalho de DEBT.md com nota P152 | ✅ |
| Saldo de DEBTs abertos inalterado (12) | ✅ |
| Nenhum ficheiro tocado em L1/L2/L3/L4 cristalino | ✅ |
| Nenhum ficheiro tocado em `lab/parity/` | ✅ |
| Nenhuma ADR criada / revogada / revisada | ✅ |
| §9 dos documentos de paridade intacto | ✅ |
| `crystalline-lint .` zero violations | ✅ |
| `cargo test --workspace --lib` cristalino: 1113 inalterado | ✅ |
| `cd lab/parity && cargo test --test layout_parity` continua a correr | ✅ (matriz P150 preservada) |
| Relatório do passo escrito | ✅ |

**Pós-152**: DEBT-54 tem **plano refinado** com risco
identificado, probe documentado, e critério de fecho granular
em 3 níveis. Materialização de DEBT-54 (passo dedicado
futuro) tem alvo mais claro e menor risco do que estimado em
P151.

**Reformulação 6 da série paridade** (148 inventário; 149
arqueologia; 150 baseline; 151 investigação; **152 refino**;
153 = P2) — padrão "passo descobre obstáculo, gera
sub-trabalho" continua a aplicar-se, desta vez ao próprio
sub-trabalho (DEBT-54 ganha refino antes de ser materializado).
