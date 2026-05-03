# Passo 184A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.756 verdes; zero violations.
- M9 ✅ 11/11 (P182 fechou lacuna #4).
- M4 parcialmente concluído via P183: C4 resolved label
  migrado (ou pendente em P183E a executar); C1, C2, C3
  bloqueados em P183B/C/D por gates substanciais
  segundo regra dos 2 eixos (P183C §6).
- DEBT M4-residual previsto para abertura em P183F
  cobrindo C1+C2+C3.
- P184 abre série M4-residual com 5 passos planeados:
  P184 (C3 isolado), P185 (location-aware Layouter), P186
  (Equation locatable), P187 (migrar C1), P188 (migrar
  C2). Cada um inicia com `*A` diagnóstico-primeiro.

Material de partida verificado:

- `00_nucleo/diagnosticos/diagnostico-p183d-bloqueio.md` —
  P183D identificou causa raíz de C3 bloqueio: `from_tags`
  arm `Figure` em `from_tags.rs:71-95` usa chave global
  `"figure"` em vez da convenção `figure:{kind}`
  documentada em `element_payload.rs:52` mas nunca
  implementada. Campo `kind: Option<String>` é
  silenciosamente ignorado via `..` pattern.
- `00_nucleo/materialization/typst-passo-183d-relatorio.md`
  §7 — DEBT M4-residual antecipado: C3 marcado como
  "**mais barato** para desbloquear individualmente (não
  exige cross-cutting M6+ change)".
- `00_nucleo/materialization/typst-passo-183c-relatorio.md`
  §6 — Regra operacional dos 2 eixos: "(1) existem dados
  no sub-store correspondente para a chave?
  (2) a semântica temporal coincide? Se qualquer falhar,
  declarar gate substancial e escalar para DEBT."

P184A é o passo de diagnóstico que precede a
implementação de C3. Sem decisões fixadas em P184A,
P184B+ herda problema do plano monolítico que padrão
P181A/P182A/P183A evita.

---

## Postura do auditor / executor

P184A é passo **L0-puro / diagnóstico-primeiro**, no mesmo
registo de P181A/P182A/P183A.

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** novos ou modificados.
- **Pode criar** ADR `PROPOSTO` se decisão arquitectural
  o exigir.
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não modifica** `from_tags` arm Figure — P184B+.
- **Não adiciona** método trait — P184B+.
- **Não migra** consumer C3 — P184B+.

**Regra dos 2 eixos pré-aplicada**: P183D já confirmou
que C3 falha em **eixo 2** (dados ausentes — chave
global em vez de per-kind). **Eixo 1** (semântica
temporal) é **OK** para C3 — figures têm contador
fixado por figure após walk; snapshot-final adequado.

P184A não re-faz a auditoria dos 2 eixos. Aceita
o resultado de P183D como verificado e desenha o
caminho de desbloqueio do eixo 2.

---

## Escopo

**Primário**: desbloquear C3 (figure auto-number per
kind) refinando arm `Figure` em `from_tags` para usar
chave `figure:{kind}` em vez de chave global `"figure"`.

**Confirmação**: validar que o diagnóstico P183D
continua factualmente correcto (linhas, comportamento
do arm, ausência de método trait dedicado).

**Decisões a tomar** — 6 cláusulas:

1. **Convenção de chave** — `figure:{kind}` (per
   `element_payload.rs:52`) vs `figure_{kind}` vs
   variante. Default no kind: `Option<String>` é `None`
   — kind default mapeia para que string?
2. **Método trait** — assinatura exacta para
   `figure_number_at_index` (ou nome alternativo). Forma
   do retorno (`Option<usize>` vs outra).
3. **Sub-store alvo** — `CounterRegistry` (per inventário
   P183D) vs sub-store dedicado.
4. **Forma de migração de consumer** — substitution-
   with-fallback (replica P168/P181G/P182D) vs alternativa.
5. **Compatibilidade legacy durante transição** —
   `state.figure_numbers` legacy continua populado em
   paralelo? Achado bonus P183D §1: leitura legacy é
   "dead code em produção" (nunca copiado ao Layouter).
   Decisão: manter, eliminar paralelamente, ou ignorar.
6. **Critério de fecho de C3** — Opção 3 simétrica com
   P181/P182 (infra pronta + consumer migrado; legacy
   preservado até M6). Confirmar.

**Fora de escopo**:

- Localização de C1+C2 (P185+).
- `Content::Equation` locatable (P186).
- Walk puro (P189 — M5).
- Eliminação `CounterStateLegacy` (P190 — M6).

---

## Critérios objectivos

Para cada decisão das 6 cláusulas, registar:

### O1 — Inputs verificáveis

`grep -rn "figure_numbers\|figure_label_numbers\|figure:"
01_core/src/`. Para cláusula 1 (convenção), confirmar
estado actual de `element_payload.rs:52` (a referência
P183D usa essa linha). Para cláusula 2, listar métodos
existentes do trait com naming pattern análogo
(`bib_number_for_key`, `figure_number_for_label` se
existir).

### O2 — Alternativas

Mínimo 2 quando há margem real. Para cláusula 1
(convenção de chave), as alternativas são 3+:
`figure:{kind}`, `figure_{kind}`, kind separado de
chave (`figure` + `kind` argumentos).

### O3 — Critério de escolha

Convenção `numbering_active:<feature>` estabelecida em
P182A/B/C como prefixo `:`. Simetria sugere
`figure:{kind}`. Outros critérios: ADR-0036 atomização,
custo de implementação, simetria com vanilla (vanilla
não tem este split — figures resolvidas via element
fields).

### O4 — Magnitude

Trivial vs substancial. Cada cláusula é independente.

### O5 — Reversibilidade

Reversível ou fixa direção cara mudar.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Convenção de chave replica `numbering_active:heading`
(P182A) padrão? Método trait replica `bib_number_for_key`
(P181F) ou `is_numbering_active` (P182B)?

### Q2 — Honestidade de magnitude

P183D §7 declarou C3 como "mais barato individualmente".
Confirma S agregado para P184B–F (refinar arm + método
trait + migração + tests + relatório)?

### Q3 — Cobertura sem regressão

Refinar arm `Figure` muda o que `kind_index` recebe?
Outros consumers do `kind_index[Figure]` (figure-ref
P168) ainda funcionam? Confirmar empiricamente em `.A`.

### Q4 — Fechamento de C3

Critério de fecho verificável: consumer C3 consulta
Introspector com fallback; tests E2E confirmam paridade;
DEBT M4-residual reduz de "C1+C2+C3" para "C1+C2"
em P184F (P183F já não pode fechar — está pré-P184).

### Q5 — Granularidade

5 sub-passos típicos para passo S agregado: refinar arm
+ método trait + migrar consumer + tests + relatório.
Pode ser comprimido em menos se cada peça for trivial.

---

## Sub-passos de P184A

### Sub-passo 184A.A — Validação do estado actual

Auditor confirma empiricamente:

1. `from_tags` arm `Figure` em
   `01_core/src/rules/introspect/from_tags.rs:71-95`
   continua a usar chave `"figure"` global e a ignorar
   `kind` via `..` pattern (per P183D §1).

2. `element_payload.rs:52` continua a documentar
   convenção `figure:{kind}` (que não foi
   implementada).

3. `Content::Figure` arm em `extract_payload` produz
   `ElementPayload::Figure { kind: Option<String>, ... }`
   actual.

4. `kind_index[ElementKind::Figure]` em
   `TagIntrospector` — verificar se outros consumers
   (figure-ref P168) consultam por kind ou por algo
   diferente.

5. Consumer C3 em `mod.rs:435–439`:
   - `state.counter.figure_numbers.get(kind_key).and_then(|v| v.get(idx))`.
   - Confirmar o `kind_key` e `idx` actuais.

6. Achado bonus P183D §1 ("dead code em produção"):
   - Confirmar: `state.figure_numbers` legacy é alguma
     vez copiado para Layouter?
   - `grep` em `mod.rs:1414, 1442` (copy-sites legacy).
   - Se confirmado dead code: cláusula 5 fica trivial
     (não há paridade observable a preservar para o
     path legacy).

Output: tabela com item + estado confirmado / linha
actual / observação.

### Sub-passo 184A.B — Decisão cláusula 1 (convenção de chave)

Avaliar `figure:{kind}` vs alternativas.

**Opção A** — `figure:{kind}` quando `Some`,
`figure:image` quando `None` (default kind = "image").

**Opção B** — `figure:{kind}` quando `Some`, `figure`
sem prefixo quando `None`.

**Opção C** — sempre `figure:{kind}`, com kind default
fixado em código.

Critério de escolha:
- Opção A alinha com vanilla (default kind = "image"
  por convenção implícita).
- Opção B mantém retrocompatibilidade com chave global
  actual.
- Opção C é mais limpa mas exige decisão sobre default.

P182A já estabeleceu padrão `<feature>:<sub-feature>`.
Para `numbering_active:heading` o sub-feature é
obrigatório. Para `figure:?` o sub-feature pode ser
ausente.

Output: decisão fixada com justificação literal.

### Sub-passo 184A.C — Decisão cláusula 2 (método trait)

Avaliar nome e assinatura.

**Opção α** — `figure_number_at_index(&self, kind:
&str, idx: usize) -> Option<usize>`.

**Opção β** — `figure_number_for_kind_index(&self,
kind: &str, idx: usize) -> Option<usize>`.

**Opção γ** — `figure_number_at(&self, kind: &str,
location: Location) -> Option<usize>` — location-aware.
Diferente. Mais alinhado com pendência P182E §5.2 mas
exige Layouter conhecer location (problema P185).

Critério: P181F estabeleceu `bib_number_for_key`.
Simetria sugere Opção α. Opção γ atrai pendência
location-aware para C3 — pode ser desejável mas mistura
escopo P184 com P185.

Output: decisão. Provável Opção α.

Magnitude: trivial (decisão de nome).

### Sub-passo 184A.D — Decisão cláusula 3 (sub-store alvo)

`CounterRegistry` (per P183D inventário) é candidato
único ou há alternativas?

**Opção 1** — `CounterRegistry` populado pelo arm
`Figure` refinado.

**Opção 2** — sub-store dedicado `FigureNumbersRegistry`
(análogo a `BibStore` P181).

Critério: cristalino tem 5 sub-stores estabelecidos
(`LabelRegistry`, `CounterRegistry`, `MetadataStore`,
`StateRegistry`, `BibStore`). Adicionar 6º para apenas
1 caso de uso é overhead. `CounterRegistry` já cobre
(é exactamente isto que é).

Output: Opção 1.

### Sub-passo 184A.E — Decisão cláusula 4 (forma de migração)

Substitution-with-fallback per P168/P181G/P182D vs
alternativa.

Achado P183D §1: legacy é dead code. Se confirmado em
`.A.6`, fallback é defensivo apenas (reversibilidade)
mas não preserva paridade observable de path legacy
real (não existe).

Output: substitution-with-fallback (replicar padrão).

### Sub-passo 184A.F — Decisão cláusula 5 (legacy paralelo)

Walk arm `introspect.rs` para `Content::Figure`
continua a popular `state.figure_numbers` legacy?

**Opção 1** — manter legacy paralelo (M6 elimina).

**Opção 2** — eliminar legacy paralelo já em P184
(se confirmado dead code em `.A.6`).

Critério: padrão P181/P182 manteve legacy até M6 mesmo
com Opção 3. Mas se legacy é dead code factualmente,
"manter paralelo" não tem benefício real — só custo
de manutenção.

Output: provável Opção 1 por simetria, mas confirmar
em `.A.6` se Opção 2 é defensável.

### Sub-passo 184A.G — Decisão cláusula 6 (critério de fecho de C3)

Opção 3 simétrica com P181/P182:

> C3 fecha quando: infra pronta (arm `Figure` refinado +
> método trait `figure_number_at_index` + impl)
> **e** consumer C3 migrado **e** tests E2E confirmam
> paridade.

DEBT M4-residual em P183F **deve ser actualizado** se
P184 fechar antes do DEBT ser aberto:

- Se P183F ainda não correu: DEBT abre cobrindo
  C1+C2 apenas (não C3).
- Se P183F já correu: DEBT precisa de update após
  P184F para remover C3 da lista.

Output: critério literal verificável.

### Sub-passo 184A.H — Validação do plano de sub-passos

Tabela esperada:

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | Refinar `from_tags` arm `Figure` (chave `figure:{kind}`) + L0 | S | — |
| `.C` | Adicionar `figure_number_at_index` ao trait + impl | S | `.B` |
| `.D` | Migrar consumer C3 (`mod.rs:435–439`) com fallback | S | `.B`, `.C` |
| `.E` | Tests E2E paridade C3 | S | `.D` |
| `.F` | Relatório + actualização DEBT M4-residual | S | `.E` |

Sem cláusulas condicionais.

Output: tabela final.

### Sub-passo 184A.I — ADR

Avaliar:

- Convenção de chave `figure:{kind}` replica padrão
  P182 — **não ADR**.
- Método trait `figure_number_at_index` replica P181F —
  **não ADR**.
- Refinamento de arm existente, não novo locatable
  kind — **não ADR**.

Conclusão esperada: **não cria ADR**.

### Sub-passo 184A.J — Outputs

Produzir 3 ficheiros (padrão P181A/P182A/P183A):

1. **`00_nucleo/diagnosticos/diagnostico-figure-per-kind-passo-184a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual.
   - §2 Decisões cláusula 1–6 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação.
   - §7 Relação com P183D bloqueio (eixo 2 desbloqueado).
   - §8 Próximo sub-passo (P184B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-184a-relatorio.md`**
   — relatório com 14 secções (padrão P182A/P183A).

3. **Actualização preventiva** — se P183F ainda não
   correu, registar em P184A relatório §13 que P183F
   abrirá DEBT M4-residual cobrindo apenas C1+C2 (não
   C3). Se P183F já correu, P184F vai actualizar o DEBT
   removendo C3.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar `from_tags`** — P184B.
- **Não adicionar trait method** — P184C.
- **Não migrar consumer** — P184D.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória**: se "dead code em
  produção" achado P183D §1 for confirmado, registar
  como tal — não rebaptizar como "redundância
  defensiva".
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-figure-per-kind-passo-184a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-184a-relatorio.md`
  com 14 secções produzido.
- 6 cláusulas fechadas com decisão literal.
- Plano de 5 sub-passos sem condicionais.
- Magnitude S agregada confirmada.
- Critério de fecho C3 fixado.
- ADR avaliada (esperado: não criada).
- Actualização preventiva sobre DEBT M4-residual
  registada.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.756 inalterados.
- `crystalline-lint .` zero violations.

P184A é instrumento. Refinamento concreto da arm Figure
e migração de C3 começam em P184B.
