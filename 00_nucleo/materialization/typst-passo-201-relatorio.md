# Relatório do passo P201

**Data de execução**: 2026-05-05.
**Executor**: Claude Code (LLM externa, modelo Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-201.md`.
**Natureza**: passo L0-puro / administrativo. Zero código.
Zero testes. Zero ADRs criadas. Zero DEBTs criados.

---

## §1 O que foi feito

P201 produziu três outputs documentais conforme spec §5:

### Output 1 — `00_nucleo/historiograma-passos.md` (actualizado)

Substituiu o anterior. Cobertura P0-P200 com:
- Sumário executivo (delta P156A→P200).
- Linha temporal completa (§1.1 Pré-P156 agregada;
  §1.2 ciclo P156B-P200 detalhada com ~105 linhas-passo).
- Padrões agregados (8 padrões novos detectados;
  comparação face a P156A).
- Recuos e reformulações.
- Cadeias de dependência (declaradas + transversais ADR-0069).
- Métricas cumulativas.
- Convenções operacionais.

**Backup do anterior** colocado em
`00_nucleo/historiograma-passos.P156A.md` (preservação
histórica per spec §5).

### Output 2 — `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`

Auditoria focada apenas no delta P157-P200 (forma de
"leitura rápida" complementar). Estrutura per spec §5:
- §1 Estado consolidado pré-M8 com 11 itens etiquetados
  CONFIRMADO/DIVERGÊNCIA/NÃO APLICÁVEL.
- §2 Lacunas residuais (#1, #1b, #2 residuais; #3-#7
  fechadas no ciclo).
- §3 Padrões novos (9 padrões emergentes desde P156A).
- §4 ADRs novas (11 ADRs no ciclo; 6 ACEITES estritas).
- §5 Marcos fechados (M1-M7, M9, F1; F3 parcial).
- §6 Métricas comparadas.
- §7 Divergências detectadas (4; uma estrutural).
- §8 Notas de auditor (5 itens para investigação humana).
- §9 Sugestão de próximos passos (não-vinculativa, 4
  caminhos).

### Output 3 — este relatório

Identificação dos outputs, tempo de execução, decisões
tomadas, sugestões.

---

## §2 Tempo de execução

Sessão única; ordem grosseira:
- Leitura do spec P201 e do historiograma anterior P156A
  (~15 min).
- Validação empírica C6 (cargo test, crystalline-lint,
  greps, leitura de structs Rust): ~10 min.
- Delegação a subagente para extracção tabular dos
  relatórios P156B-P200 (~106 sub-passos): ~14 min em
  background; ~250 ferramentas usadas pelo subagente.
- Análise integrativa, resolução de divergências entre
  spec e empírico: ~10 min.
- Redacção do historiograma actualizado: ~15 min
  (output ~22 KB).
- Redacção da auditoria delta: ~10 min (output ~13 KB).
- Redacção deste relatório: ~5 min.

**Total**: ~80 min de sessão (com paralelismo via subagente).

---

## §3 Decisões tomadas durante a leitura

Decisões resolvidas internamente (sem perguntar ao humano)
quando a spec era ambígua:

### 3.1 Localização dos outputs

Spec §5 declara "`00_nucleo/historiograma-passos.md`"
para Output 1, mas o ficheiro anterior estava em
`00_nucleo/diagnosticos/historiograma-passos.md`.

**Decisão**: honrar spec literal — escrever novo em
`00_nucleo/historiograma-passos.md`; backup em
`00_nucleo/historiograma-passos.P156A.md`. **Não removi**
o ficheiro antigo em `diagnosticos/` — preservei como
fonte canónica do conteúdo P156A original (referência
histórica). O backup `historiograma-passos.P156A.md`
em `00_nucleo/` é uma cópia idêntica.

**Implicação**: existem agora dois ficheiros idênticos
(`00_nucleo/diagnosticos/historiograma-passos.md`
original P156A; `00_nucleo/historiograma-passos.P156A.md`
backup canónico). Auditor pode querer remover um deles
em passo administrativo posterior.

### 3.2 Contagem de aplicações diagnóstico-primeiro

Spec declara "33 aplicações consecutivas". Empíricamente
contei ~35 (incluindo P156L e P159B como diag formal).

**Decisão**: registar como **DIVERGÊNCIA marginal
interpretativa** (auditoria delta §1 #7 e §7.4); manter
range "~33-35" no historiograma; não tentar reconciliar
forçadamente.

### 3.3 ADRs ACEITES no ciclo M5/M6/M7 — 7 vs 6

Spec declara 7 ACEITES; empíricamente 6 estritas (0066,
0068, 0069, 0070, 0071, 0072) ou 8 (incluindo 0064, 0065
EM VIGOR).

**Decisão**: registar como **DIVERGÊNCIA marginal**;
hipótese mais provável é que ADR-0067 era esperada em
ACEITE pelo consolidado P190 §1 mas o ficheiro mantém
PROPOSTO. Documentar em auditoria delta §8.1 como nota
para auditor humano.

### 3.4 Layouter 19 vs 22 fields

Spec declara 19 fields, sem `counter`. Empíricamente 22.

**Decisão**: registar como **DIVERGÊNCIA estrutural
relevante** em auditoria delta §7.1. Não corrigir o
snapshot 2026-05-05 (não é responsabilidade de P201);
apenas reportar. Sugerir caminho B em §9.2 da auditoria
delta como passo administrativo se humano quiser
reconciliar.

### 3.5 Sub-passos sem relatório individual

P160B, P183B, P190B-H foram registados como "buracos de
numeração" no historiograma §1.2. Cada caso documentado
com causa (descartado / absorvido pelo consolidado /
absorvido pelo posterior).

**Decisão**: tratá-los como **passos completos para fins
de inventário** (não omiti-los na linha temporal); marcar
"(s/relatório)" ou "(P190 série)" para distinguir de
passos com relatório próprio.

### 3.6 Datação anómala P195-P200 vs P190/P191/P192

Relatórios P195-P200 datados 2026-05-03/04 mas semanticamente
dependem de M6 fechado (P190I 2026-05-05). Em vez de
forçar uma reordenação, **decidi reportar a anomalia em
auditoria delta §8.4** e manter datas de relatório como
estão.

### 3.7 Sub-agente para leitura corpus

Para evitar exhaustão do contexto principal por leitura de
~150 ficheiros relatório, deleguei a um subagente
general-purpose com prompt detalhado (~2.5 KB) que pediu
uma tabela estruturada e 6 secções analíticas. Subagente
devolveu output tabular de ~12 KB que serviu como base
factual primária do historiograma §1.2 e da auditoria
delta.

**Decisão**: **trust but verify** — validei pontos chave
(walk fn signature, fixpoint MAX, Introspector trait, ADR
status) directamente no código e nos ficheiros ADR antes
de incorporar no historiograma. Subagente reportou
fielmente; verificações cruzadas convergiram.

---

## §4 Sugestões para o próximo passo (não-vinculativas)

Per spec §6, P201 não decide. As 4 sugestões não-vinculativas
estão na auditoria delta §9. Repito-as aqui em forma compacta:

- **A** — P202A diagnóstico-primeiro de M8 (caminho default
  da spec P201).
- **B** — P202 administrativo de reconciliação do snapshot
  2026-05-05 (Layouter 22 fields; tests 1823; ACEITES 6).
- **C** — P202 administrativo de clarificação ADR-0067
  (status PROPOSTO vs referência ACEITE em consolidado P190).
- **D** — P202 endereçar lacunas residuais #1, #1b
  (Position) antes de M8 se forem pré-condição crítica.

A escolha cabe ao humano com base no que P201 revelou.

---

## §5 Critério de progressão (verificação)

Per spec §6, P201 está concluído quando:

- [x] Os 3 ficheiros existem.
  - `/home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/historiograma-passos.md`
  - `/home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
  - `/home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/materialization/typst-passo-201-relatorio.md`
- [x] C1-C9 todos endereçados:
  - C1 (linha temporal completa): historiograma §1.
  - C2 (padrões agregados): historiograma §2.
  - C3 (comparação P156A): historiograma §2.13.
  - C4 (recuos / reformulações): historiograma §3.
  - C5 (cadeias de dependência): historiograma §4.
  - C6 (estado consolidado pré-M8): auditoria delta §1.
  - C7 (métricas cumulativas): historiograma §5;
    auditoria delta §6.
  - C8 (lacunas residuais): auditoria delta §2.
  - C9 (convenções estabelecidas): historiograma §6.
- [x] Etiquetas de C6 todas aplicadas: 7 CONFIRMADO,
  4 DIVERGÊNCIA registadas com evidência.
- [x] Backup do historiograma anterior existe:
  `00_nucleo/historiograma-passos.P156A.md`
  (57954 bytes).

---

## §6 Não-objectivos respeitados

Per spec §8, P201 não:

- [x] Não decidiu o caminho de M8 (apenas listou opções
  em §9 da auditoria delta).
- [x] Não propôs ADR-0073 nem qualquer ADR.
- [x] Não tocou em código.
- [x] Não substituiu o snapshot 2026-05-05 — verificou-o
  e reportou divergências.
- [x] Não pré-definiu sub-passos M8 (`*B-G` ou outros).
- [x] Não introduziu padrões novos — apenas registou os
  detectados empíricamente.

---

## §7 Notas operacionais

### 7.1 Pasta materialization

A spec P201 §3 autorizou explicitamente leitura de
`00_nucleo/materialization/`, `00_nucleo/adr/`,
`00_nucleo/diagnosticos/` e historiograma anterior. Sem a
autorização explícita, a restrição CLAUDE.md "Restrição de
leitura — pastas de materialização e context" teria
bloqueado este passo. Convenção respeitada: pasta
`00_nucleo/context/` **não foi acedida** (não é referida
pela spec P201).

### 7.2 Comando `crystalline-lint`

Inicialmente tentei `cargo run -p crystalline-lint --bin
crystalline-lint -- .` mas o binário não está no workspace
membro. Encontrei-o em `/home/dikluwe/.cargo/bin/crystalline-lint`
(instalado globalmente). Output: `✓ No violations found`.

### 7.3 Tests workspace

`cargo test --workspace 2>&1 | grep "^test result" | awk
'{sum += $4} END {print sum}'` reporta **1823 testes
verdes** consistentemente. Confirmado em duas execuções.

### 7.4 Estrutura do output

Optei por **historiograma denso** (tabelas + narrativa) em
detrimento de "linha por passo isolada" — a tabela compacta
§1.2 do historiograma serve como índice navegável; secções
§2-§6 oferecem agregação semântica.

A **auditoria delta** é deliberadamente leve (~13 KB vs
~22 KB do historiograma) — propósito de leitura rápida
("o que mudou") respeitado.

---

**Fim do relatório P201.**
