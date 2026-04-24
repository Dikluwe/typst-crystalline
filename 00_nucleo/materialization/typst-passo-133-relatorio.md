# Passo 133 — Relatório (activar target `par` em `eval_set_rule`)

**Data**: 2026-04-24
**Precondição**: Passo 132B encerrado; 1083 total tests; zero
violations; 53 ADRs activas; 11 DEBTs abertos.
**Natureza**: passo S em L1. Infra do dispatcher (não captura).
Pattern DEBT-1 XS **não se aplica** — é ativação de target.
**ADR**: **não tocada**. Extensão de dispatcher é infraestrutura,
não decisão arquitectural nova.

---

## Sumário

Target `par` é agora **known** em `eval_set_rule`. Propriedades
de `#set par(...)` caem num arm dedicado que emite warning de
**propriedade** (não de **target**). Sem arms concretos ainda —
134 migra `leading`.

Testes que assertavam "par = unknown target" foram **adaptados**
(L1: `eval_set_target_desconhecido_ignora` migrou de `par` para
`list`; L3: idem; L1 `eval_set_par_leading_..._passo_128`
**invertido** para `eval_set_par_known_target_sem_arms_passo_133`).

Hint da mensagem de target desconhecido actualizada para listar
`par` entre os suportados.

**852 L1 + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1083 total** (inalterado — test do 128 foi substituído in-place
pelo 133). Zero violations.

---

## 133.A — Inventário confirmatório

### Estrutura actual de `eval_set_rule`

Dispatcher é **função única `eval_set_rule`** (linha 190 de
`rules.rs`) com `if target == "X"` sequenciais:
- linha 202: `if target == "heading"` (→ Content::SetHeadingNumbering).
- linha 219: `if target == "page"` (→ Content::SetPage).
- linha 248: `if target == "figure"` (→ Content::SetFigureNumbering).
- linha 270: `if target != "text"` → fallback `unsupported_target_warn`.
- linha 276+: text processing inline (sem função dedicada).

**Ausência de `eval_set_text` como função**: o código assume
text como caminho principal; targets especiais são `if`
branches com early `return`. Padrão "function dedicada" que o
spec sugeria não se aplica.

**Decisão adaptada**: adicionar `if target == "par"` block
similar aos outros 3, em vez de criar função separada. Bloco
itera args, emite per-property warning, early return.

### Pool de targets unknown

Pré-133: tudo que não é `heading/page/figure/text`.
Pós-133: tudo que não é `heading/page/figure/text/par`.

Candidatos unknown disponíveis: `list`, `enum`, `table`, `raw`,
`grid`, `place`, `rect`, `circle`, ... (lista grande).

### Testes afectados

- **L1** `eval_set_target_desconhecido_ignora:1195` — input
  `#set par(leading: 1em)`.
- **L1** `eval_set_par_leading_ainda_emite_warning_target_passo_128:1315`
  — input `#set par(leading: 10pt)` + assertion `'par'` + "target".
- **L3** `debt49_set_target_desconhecido_emite_warning:2271` —
  input `#set par(leading: 10pt)` + assertion `'par'` + "target".

### Gate 133.A

**Passa**. Candidato substituto `list` disponível e robusto.

---

## 133.B/C — Arm `par` em `eval_set_rule`

Adicionado bloco entre `figure` (linha 268) e `text-fallback`
(linha 270):

```rust
if target == "par" {
    // Passo 133: target `par` activado sem arms concretos.
    // Propriedades caem no fallback até 134 migrar `leading`.
    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str().to_owned();
            let (msg, hint) = unsupported_property_warn("par", &key);
            engine.sink.warn_note(
                named.name().to_untyped().span(),
                &msg,
                &hint,
            );
        }
    }
    return Ok(Value::None);
}
```

Usa helper existente `unsupported_property_warn` com target="par".
Mensagem: `"par: propriedade '{X}' ainda não suportada"`.

### Hint do target-unknown actualizado

```
-    "targets suportados: heading, page, figure, text"
+    "targets suportados: heading, page, figure, text, par"
```

Pequeno mas mantém consistência com o estado pós-133.

---

## 133.D/E — Tests migrados

### L1: `eval_set_target_desconhecido_ignora`

Input: `#set par(leading: 1em)` → `#set list(indent: 1em)`.
Comentário actualizado referenciando Passo 133.

### L1: `eval_set_par_leading_ainda_emite_warning_target_passo_128` → **INVERTIDO**

Renomeado para `eval_set_par_known_target_sem_arms_passo_133`.
Semântica invertida:

- **Antes (128)**: assertava que `par` ainda era unknown target.
- **Depois (133)**: asserta que `par` é known mas **sem arms**,
  portanto `leading` emite property warning. E asserta que
  `par` **não** aparece como target unknown.

Este é o **contrato executável** do que mudou em 133. Se
alguém reverte, o teste falha.

### L3: `debt49_set_target_desconhecido_emite_warning`

Input: `#set par(leading: 10pt)` → `#set list(indent: 10pt)`.
Assertions `'par'` → `'list'`.

---

## 133.F — Prompts L0

`rules/eval.md` não enumera targets conhecidos (descrição é
mais abstracta). Nada a actualizar. `crystalline-lint --fix-hashes`
reportou `Nothing to fix`.

---

## 133.G — Verificação

### Cargo tests

```
test result: ok. 852 passed ...       (L1 inalterado)
test result: ok. 186 passed, 6 ign    (L3 — 1 test migrado)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 inalterado)
```

**Nota**: spec esperava +1 L1 test. Realidade: 0 net. Razão:
spec não considerou que o test do 128 (`eval_set_par_leading_..._passo_128`)
era exactamente redundante com o novo que spec pedia criar.
**Substitui in-place** em vez de adicionar — mesmo propósito.

### `crystalline-lint .`

```
✓ No violations found
```

### Manual — 5 cenários

```bash
$ typst pl.typ    # #set par(leading: 1em)
pl.typ:1:10: warning: par: propriedade 'leading' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set par
exit=0

$ typst pj.typ    # #set par(justify: true)
pj.typ:1:10: warning: par: propriedade 'justify' ainda não suportada
exit=0

$ typst ul.typ    # #set list(indent: 1em) — unknown target
ul.typ:1:6: warning: set: target 'list' ainda não suportado
  hint: targets suportados: heading, page, figure, text, par
exit=0

$ typst t.typ     # #set text(font: "Arial") — regressão 132B
exit=0, stderr: (vazio)

$ typst h.typ     # canary hyphenate
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
exit=0
```

5 comportamentos correctos.

---

## Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/rules/eval/rules.rs` | +arm `par` em dispatcher (13 linhas); +`par` no hint de unknown target |
| `01_core/src/rules/eval/tests.rs` | 2 testes adaptados (1 migrado, 1 invertido) |
| `03_infra/src/integration_tests.rs` | 1 teste migrado |

**ADR-0038**: não tocada (infra, não decisão nova).
**ADR nova**: nenhuma.

### Números finais

| Métrica | Antes (132B) | Depois |
|---------|------:|-------:|
| L1 tests | 852 | 852 |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1083** | **1083** |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 53 | 53 |
| DEBTs abertos | 11 | 11 |

---

## Mudança observável

**Utilizadores que escrevam `#set par(...)`**:
- **Antes**: `warning: set: target 'par' ainda não suportado`.
- **Depois**: `warning: par: propriedade 'X' ainda não suportada`
  (para cada propriedade no set rule).

Semanticamente **mais informativo**: em vez de "tudo rejeitado"
a mensagem diz explicitamente qual propriedade não é suportada.
Quando 134 migrar `leading`, `#set par(leading: 1em)` deixa de
emitir warning (capturado) — sem mais mudança necessária no
utilizador.

---

## Decisões face ao spec

1. **Sem função `eval_set_par` dedicada**: spec sugeria espelhar
   `eval_set_text` como função. Realidade: não existe
   `eval_set_text` como função — tudo é inline em
   `eval_set_rule`. Adaptação: `if target == "par"` block
   coerente com os outros `if target == "X"` existentes.
2. **Test 128 substituído em vez de adicionar novo**: spec
   pedia +1 test mas o teste do 128 tinha exactamente a
   semântica oposta ao novo teste 133. Substituir in-place é
   mais limpo que deixar ambos (o 128 ficaria obsoleto).
3. **`list` como substituto unknown**: candidato simples e
   canonicamente unknown em Typst. Sintaxe `indent: length`
   é parseável.

---

## Lições

1. **Inventário 133.A evitou refactor desnecessário**: spec
   assumia `eval_set_text` como função dedicada a replicar.
   Real: dispatcher tipo "cascade of ifs". Ajuste foi natural
   (adicionar mais um `if`). Sem o inventário, poderia ter
   perdido tempo tentando criar função + refactorizar inline
   text.

2. **Inversão de teste é documentação activa**: o teste do
   128 tornou-se obsoleto (assertion "`par` unknown" era
   verdadeira, agora é falsa). **Inverter** a assertion
   transforma-o em "documentação executável do estado actual"
   em vez de deletar. Próximas regressões apanhadas.

3. **Hint texto é trivial mas importante**: `targets
   suportados: heading, page, figure, text, par` agora reflecte
   o estado real. Utilizador que escreva `#set list(...)` vê
   a lista completa do que é suportado.

4. **Infra sem captura é passo pequeno**: 133 foi < 1h.
   Contraste com 132B (~2-3h) e 131B (~1.5h). Split
   133/134 foi acertado — risco de infra isolado do risco
   semântico.

5. **Warning hint `ADR-0040`**: o helper `unsupported_property_warn`
   emite hint referenciando ADR-0040 (cobre set text).
   Para set par, o ADR de referência seria diferente ou
   inexistente. Pequeno absurdo cosmético aceite —
   consolidar quando migrações forem completas.

---

## Estado pós-Passo 133

### Preparação para 134

`leading` continua a ser capturado em `#set text` (Passo 128).
Próximo passo 134:
- Adicionar arm `"leading"` em bloco `par` (análogo ao
  arm em text hoje).
- Remover arm `"leading"` do bloco text.
- Teste `eval_set_text_leading_passo_128` inverte para
  `eval_set_text_leading_emite_warning_passo_134` (ou similar).
- Novo teste `eval_set_par_leading_captura_passo_134`.

### DEBT-1 progresso

- 9 propriedades capturadas em text.
- `par` target activado (infra).
- Aguarda migração de `leading` (134).
- Aguarda fecho formal (135).

### Roadmap restante

- **134**: migrar `leading` de text para par — semântica.
- **135**: fechar DEBT-1 em DEBT.md — documentação.

Estimativa: **1-1.5h cumulativo**.
