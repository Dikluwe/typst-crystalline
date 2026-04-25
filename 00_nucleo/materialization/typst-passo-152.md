# Passo 152 — Refino administrativo do plano DEBT-54

**Série**: 152 (passo **L0-puro / administrativo**; escopo
**XS**; refino do plano de DEBT-54 antes de materialização
substantiva).
**Precondição**: Passo 151 encerrado; DEBT-54 aberto com
plano de 6 itens; DEBT-53 actualizado com bloqueio por
DEBT-54; 12 DEBTs abertos; 1113 tests cristalino.

**Numeração**: 152 ocupa posição administrativa antes do
substantivo 153 (P2 cristalino-only). Reformulação 6 da
série paridade — anteriormente §9 dizia P152 = P2; agora
P152 = refino DEBT-54, P153 = P2.

**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs**. **Zero novos
DEBTs**. Único output: actualização da entrada DEBT-54 em
`00_nucleo/DEBT.md` + relatório curto.

**Contexto**: feedback humano sobre o relatório 151 sinalizou
3 pontos de atenção que devem entrar em DEBT-54 antes de ser
materializado:

1. **Probe online dos 3 crates** marcados "provavelmente
   ausentes" no relatório 151 §2.4 (`codex`, `hayagriva`,
   `oxipng`). Sem confirmação, estimativa M-L do DEBT-54 é
   especulativa.
2. **Flag de risco de conflito de versão** entre cristalino
   e vanilla para crates compartilhadas (`ttf-parser` é o
   exemplo identificado). Pode forçar decisão arquitectural
   sobre dois grafos de deps separados.
3. **Critério de fecho expandido**: `cargo build -p
   typst-layout` é mínimo necessário mas não suficiente
   para destrancar DEBT-53. Precisa também de `cargo run`
   exercitar `typst::compile<PagedDocument>` para validar
   que vanilla é executável, não só compilável.

**ADRs aplicáveis**:
- Nenhuma directamente. Refino administrativo de entrada
  DEBT.

---

## Objectivo

Ao fim do passo:

1. **DEBT-54 actualizado** em `00_nucleo/DEBT.md` Secção 1
   com:
   - **Sub-secção "Probe online de deps externas"**:
     verificação se `codex`, `hayagriva`, `oxipng` estão em
     `~/.cargo/registry/cache/`. Se ausentes: lista de
     fetches necessários (`cargo fetch <crate>` ou
     equivalente) integrada no checklist do plano.
   - **Sub-secção "Risco identificado: conflitos de
     versão"**: `ttf-parser` cristalino vs vanilla
     explicitamente. Outros candidatos prováveis listados
     se possível (`comemo`, `ecow`, `rustc-hash` —
     estão no cache cristalino e vanilla pode usar versões
     diferentes). Decisão antecipada para o cenário "sem
     unificação possível": dois grafos separados em
     `lab/parity` via `[patch]` ou aliasing de crates.
   - **Critério de fecho expandido**:
     - (mínimo) `cd lab && cargo build -p typst-layout` corre
       sem erros.
     - (suficiente) `cargo build -p typst` (compilador
       inteiro) corre.
     - (executável) test simples em `lab/parity/tests/`
       que invoca `typst::compile(world)` para um source
       trivial e devolve `PagedDocument` sem panic.
   - **Não** alterar o título, número, data de abertura ou
     o saldo de DEBTs abertos (continua 12).

2. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-152-relatorio.md`
   com:
   - Resultado do probe online (lista de cache hits + misses).
   - Diff aplicado a DEBT-54 (antes/depois).
   - Próximo passo: 153 (P2 cristalino-only).

3. **Sem actualização de §9** dos documentos de paridade —
   refino de DEBT-54 não muda numeração de items § 9 (apenas
   a renumeração que já aconteceu em P151 é preservada).

Este passo **não**:

- Toca código em L1/L2/L3/L4 cristalino ou em `lab/parity/`.
- Materializa DEBT-54 (escopo separado, futuro).
- Materializa DEBT-53 (bloqueado).
- Cria ADRs novas.
- Modifica outros DEBTs.
- Decide sobre opção operacional da quarentena vanilla
  (assumida opção 3 — princípio sem regra absoluta; P9
  precedente).

---

## Decisões já tomadas

1. **Quarentena vanilla é princípio operacional, não regra
   absoluta**. `lab/parity` é caso especial já contemplado
   desde P9 (workspace separado). Sem ADR nova; documentação
   informal em CLAUDE.md ou README do `lab/` é candidato
   futuro de baixa prioridade.
2. **Probe online é parte do refino**, não trabalho de
   materialização. Apenas listagem de fetches necessários,
   não execução.
3. **Conflito de versão é risco identificado**, não problema
   resolvido. Plano antecipa estratégia (`[patch]` ou
   aliasing) sem comprometer.
4. **Critério de fecho expandido em 3 níveis** (mínimo /
   suficiente / executável). Permite fecho parcial se
   problema surge a meio.

## Decisões diferidas (resolvidas neste passo)

5. **Forma de probe online**: `cargo fetch <crate>` em
   directório temporário com `Cargo.toml` mínimo,
   verificando se cargo encontra o crate no registry e
   resolve deps transitivas básicas. Decisão default.
6. **Lista exacta de candidatos a conflito**: além de
   `ttf-parser`, listar `comemo`, `ecow`, `rustc-hash`
   conhecidamente partilhados. Outros (`unicode-*`, etc)
   podem ser identificados quando DEBT-54 for materializado.

---

## Escopo

**Dentro**:

- Probe online dos 3 crates flagged.
- Edição de entrada DEBT-54 em `00_nucleo/DEBT.md`.
- Relatório do passo.

**Fora**:

- Modificação de qualquer ficheiro `.rs` ou `.toml`.
- Execução de `cargo build` para validar (probe é só
  resolução, não build).
- Materialização de DEBT-54.
- Modificação de outros DEBTs ou ADRs.
- Modificação de §9 dos documentos de paridade.
- Decisão sobre prioridade entre P153 (P2) e materialização
  de DEBT-54 — decisão humana posterior.

---

## Sub-passos

### 152.1 — Probe online dos 3 crates

```bash
mkdir -p /tmp/probe-deps && cd /tmp/probe-deps
cat > Cargo.toml <<'EOF'
[package]
name = "probe"
version = "0.0.1"
edition = "2024"

[dependencies]
codex = "*"
hayagriva = "*"
oxipng = "*"
EOF
mkdir -p src && touch src/lib.rs
cargo fetch 2>&1 | tee /tmp/probe-output.txt
```

Se `cargo fetch` resolve cada crate, registar versão exacta
detectada. Se algum crate não resolve (ex: nome diverge,
não publicado), registar como **fetch não-trivial** —
candidato a fonte alternativa (Git? path? typst-original
embebido?).

**Alternativa se probe online indisponível**: listar deps
de `lab/typst-original/crates/typst-library/Cargo.toml` e
`typst-pdf/Cargo.toml`; verificar quais já estão em
`~/.cargo/registry/cache/`.

### 152.2 — Composição da actualização DEBT-54

Esboço (forma final em 152.3):

```markdown
### Probe online (Passo 152)

| Crate     | Estado em probe     | Versão                  | Notas |
|-----------|---------------------|-------------------------|-------|
| codex     | <fetch ok / falha>  | <X.Y.Z ou ?>            | <…>   |
| hayagriva | <fetch ok / falha>  | <X.Y.Z ou ?>            | <…>   |
| oxipng    | <fetch ok / falha>  | <X.Y.Z ou ?>            | <…>   |

**Conclusão**: <todas resolvem | N falham; lista>.

### Risco: conflitos de versão entre cristalino e vanilla

Crates conhecidamente partilhados (cache cristalino +
vanilla):

- **`ttf-parser`** — cristalino na sua versão (verificar);
  vanilla provavelmente outra. **Risco alto** se as APIs
  divergirem.
- **`comemo`** — vanilla v0.X (≥ 0.4 conforme passos 92+);
  cristalino na sua versão. Probabilidade de conflito:
  média.
- **`ecow`** — cristalino estabelecido em ADR-0035 e
  ADR-0024; vanilla na sua. Probabilidade: média.
- **`rustc-hash`** — cristalino ADR-0018; vanilla na sua.
  Probabilidade: baixa (ambos estáveis na linha 2.x).

**Estratégia antecipada se conflito surgir**:
- Investigar se cargo unifica versões automaticamente
  (preferido).
- Se não unifica: usar `[patch.crates-io]` em
  `lab/Cargo.toml` para forçar uma versão única (decisão
  arquitectural — pode degradar vanilla ou cristalino se
  versões são incompatíveis).
- Caso extremo: aceitar dois grafos de deps separados via
  feature flag por target. **Custo alto**; última
  alternativa.

### Critério de fecho (expandido)

DEBT-54 fecha quando:

1. **Mínimo**: `cd lab && cargo build -p typst-layout`
   corre sem erros.
2. **Suficiente**: `cargo build -p typst` (compilador
   inteiro vanilla) corre sem erros.
3. **Executável**: test simples em
   `lab/parity/tests/vanilla_smoke.rs` que invoca
   `typst::compile(world)` para um source trivial (e.g.
   `Hello`) e devolve `PagedDocument` sem panic.

Os 3 níveis devem todos ser cumpridos para o fecho ser
considerado completo. Fecho parcial (níveis 1 ou 1+2 mas
não 3) é aceitável apenas se o passo de materialização
identificar bloqueio justificado e abrir DEBT-55 para o
nível pendente.
```

### 152.3 — Aplicar actualização à entrada DEBT-54

Editar `00_nucleo/DEBT.md` Secção 1, entrada DEBT-54:

```diff
  ## DEBT-54 — Setup vanilla `typst` workspace em `lab/parity`

  **Aberto em**: Passo 150 (registado como pré-condição em
  Passo 151).
  ...

  ### Plano original (Passo 151)
  - [ ] item 1
  - [ ] item 2
  ...
  - [ ] item 6
  
+ ### Actualização Passo 152 — Refino administrativo
+ 
+ Adicionado:
+ - **Probe online** dos 3 crates flagged em P151 §2.4.
+   Resultado: ver tabela em §3 do plano abaixo.
+ - **Risco identificado**: conflitos de versão entre
+   crates partilhadas (ttf-parser; comemo; ecow;
+   rustc-hash). Estratégia antecipada: cargo unify →
+   [patch] → grafos separados. Ver §4.
+ - **Critério de fecho expandido**: 3 níveis (mínimo /
+   suficiente / executável). Ver §5.
+ 
+ ### §3. Probe online (resultado de Passo 152)
+ <tabela do 152.1>
+ 
+ ### §4. Risco: conflitos de versão
+ <texto do 152.2>
+ 
+ ### §5. Critério de fecho expandido
+ <texto do 152.2>
```

### 152.4 — Cabeçalho de DEBT.md

```diff
  > **Passo 151 (2026-04-25)**: investigação de DEBT-53
  > revelou pré-condição não-trivial; **DEBT-54 aberto**.
  > Total abertos: 11 → 12.
+ 
+ > **Passo 152 (2026-04-25)**: refino administrativo do
+ > plano DEBT-54 (probe online + risco de versões + critério
+ > expandido). Saldo de DEBTs **inalterado** (12).
```

### 152.5 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-152-relatorio.md`.

Secções:
1. Sumário (1-2 frases).
2. Resultado do probe online (tabela 152.1).
3. Texto adicionado a DEBT-54 (cópia integral das §3-§5
   da actualização).
4. Diff do cabeçalho de DEBT.md.
5. Próximo passo: **153** (P2 cristalino-only).
6. Verificação final.

---

## Verificação

1. ✅ Probe online executado e resultado registado.
2. ✅ DEBT-54 actualizado com 3 sub-secções novas (§3,
   §4, §5).
3. ✅ Cabeçalho de DEBT.md com nota P152.
4. ✅ Saldo de DEBTs abertos inalterado (12).
5. ✅ Nenhum ficheiro tocado em L1/L2/L3/L4 cristalino.
6. ✅ Nenhum ficheiro tocado em `lab/parity/`.
7. ✅ Nenhuma ADR criada / revogada / revisada.
8. ✅ §9 dos documentos de paridade intacto.
9. ✅ `crystalline-lint .` zero violations.
10. ✅ `cargo test --workspace --lib`: 1113 inalterado.
11. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. DEBT-54 ganha as 3 sub-secções (probe + risco +
   critério).
2. Probe online tem resultado factual (cache hit, fetch ok,
   ou não-trivial).
3. Próximo passo (153) tem âncora documental clara.
4. Sem materialização de código.
5. Relatório escrito.

---

## O que pode sair errado

- **Probe online indisponível** (sem rede, registry
  inacessível): usar alternativa (listar deps de
  `lab/typst-original/crates/...` + verificar cache local).
  Resultado fica menos preciso mas válido. Documentar a
  limitação no relatório.

- **Cargo registry resolve mas com versão diferente da
  esperada por vanilla**: ex, `codex` no registry é v1.x
  mas vanilla espera v0.x. Incluir nota de "versão divergente"
  no risco da §4.

- **Crate `codex` não existe ou foi renomeado**: vanilla
  usa nome interno que pode não corresponder ao crate
  publicado. Se acontecer, marcar como fetch não-trivial.

- **3 crates ausentes do cache cristalino**: aceitável.
  Refino apenas regista; materialização (passo dedicado
  futuro) executa fetch.

- **Actualização DEBT-54 cresce demasiado**: §3+§4+§5
  podem somar 30-50 linhas. Aceite — DEBT precisa do
  detalhe; alternativa (separar em ficheiro próprio) é
  desproporcional para escopo XS.

- **Probe revela conflitos resolvidos automaticamente**:
  ex, cargo unifica `ttf-parser` v0.20 e v0.25 sem
  problemas (improvável dada a major version 0.x). Documentar
  no relatório como "risco baixo" em vez de remover.

---

## Notas operacionais

- **Reformulação 6 da série paridade**: 148 inventário; 149
  arqueologia; 150 baseline; 151 investigação + DEBT-54;
  **152 refino DEBT-54**; 153 (próximo) = P2 cristalino-only.
  Padrão "passo descobre obstáculo, gera sub-trabalho"
  continua a aplicar-se — desta vez ao próprio sub-trabalho
  (DEBT-54 ganha refino antes de ser materializado).

- **Modelo: passo administrativo XS**, análogo ao componente
  administrativo de P147 ou P145 mas com escopo mais
  restrito (uma única entrada DEBT). Quase mecânico:
  probe + 3 sub-secções + cabeçalho.

- **Quarentena vanilla**: assumida como **opção 3** (princípio
  operacional, não regra absoluta). `lab/parity` é caso
  especial já contemplado por P9. Decisão registada nas
  notas; sem ADR formal. Se priorização futura justificar
  formalização, abrir passo dedicado análogo a P149
  arqueologia.

- **Pós-152**: DEBT-54 tem plano refinado com risco
  identificado, probe documentado, e critério de fecho
  granular em 3 níveis. Materialização de DEBT-54
  (passo dedicado futuro) tem alvo mais claro.

- **Não bloqueia P153**: refino DEBT-54 é independente de
  P2 cristalino-only. P153 pode arrancar imediatamente.
