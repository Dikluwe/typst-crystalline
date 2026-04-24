# Passo 133 — Activar target `par` em `eval_set_rule`

**Série**: 133 (passo **S** em L1; terceiro de quatro passos
restantes para fechar DEBT-1).
**Precondição**: Passo 132B encerrado; 1083 total tests; zero
violations; 53 ADRs activas (ADR-0052 + ADR-0053 `IMPLEMENTADO`);
11 DEBTs abertos.

**ADRs aplicáveis**:
- **ADR-0033** (paridade funcional) — activar target `par` é
  pré-requisito para resolver a divergência temporal de
  `leading` (Passo 128, resolve no 134).
- **ADR-0037** (coesão por domínio) — `eval_set_par` espelha
  `eval_set_text` como função dedicada.
- **ADR-0038** — esperado: sem nota (extensão de dispatcher é
  infra, não variante de pattern DEBT-1).

**Natureza**: passo L1. Adiciona `par` como target conhecido
em `eval_set_rule`. Cria `eval_set_par` vazio (estrutura
presente, sem arms). Migra 2 testes que hoje assertam `par`
como unknown target.

Pattern DEBT-1 XS **não se aplica** — infra do dispatcher, não
captura de propriedade.

---

## Contexto

Passo 128 capturou `leading` em `#set text` por **divergência
temporal aceite** — `par` não era target conhecido. O relatório
128.E registou candidato futuro: "migrar `leading` para
`eval_set_par` quando activado".

Após 131B (Lang) e 132B (FontList), a lista canónica DEBT-1
está toda capturada. O único resíduo estrutural é `leading` no
contexto errado.

Este passo **habilita** a activação de `par` sem ainda mover o
`leading`. Split 133 + 134 separa:

- **133**: infra (target reconhecido, função criada, testes
  migrados).
- **134**: semântica (`leading` migrado de text para par).

---

## Contexto estratégico

Penúltimo passo estrutural antes de fechar DEBT-1:

- **133** (este): activar target `par` — infra.
- **134**: migrar `leading` de text para par — semântica.
- **135**: fechar DEBT-1 no DEBT.md — documentação.

---

## Objectivo

Ao fim do passo:

1. `par` é target conhecido em `eval_set_rule` (ou função
   equivalente do dispatcher).
2. Função `eval_set_par` criada, espelhando estrutura de
   `eval_set_text` — signature idêntica, corpo sem arms.
3. Qualquer propriedade em `#set par(...)` produz warning
   "propriedade não suportada" via fallback (mesmo comportamento
   que `#set text(propriedade_nova)` tem hoje).
4. Teste L1 `eval_set_target_desconhecido_ignora` — migrado
   para outro target desconhecido, ou renomeado/reformulado.
5. Teste L3 `debt49_set_target_desconhecido_emite_warning` —
   migrado para outro target desconhecido, ou renomeado.
6. Novo teste L1 que asserta "par é known target sem
   propriedades capturadas ainda" — documentação executável do
   estado pós-133.
7. `cargo test --workspace` passa. `crystalline-lint` zero.

Este passo **não**:

- Captura propriedade de `par`.
- Move `leading` de text para par (Passo 134).
- Adiciona ADR nova (infra, não decisão arquitectural).
- Toca L4.
- Fecha DEBT-1.

---

## Decisões já tomadas

1. **Função dedicada `eval_set_par`** — espelha estrutura de
   `eval_set_text`. Simetria explícita no dispatcher. Quando
   propriedades de `par` chegarem (`justify`, `first-line-indent`),
   vão para esta função.

2. **133 sem captura** — função fica com estrutura mas sem
   arms. Propriedades caem no fallback → warning "propriedade
   não suportada".

3. **Testes que assertam "par = unknown target" migram** —
   substituídos por novo target desconhecido. Candidatos:
   `figure`, `heading`, `page` (confirmar em 133.A quais já
   são known; os restantes são candidatos). Se todos já são
   known, escolher `list`, `enum`, `table`, `raw` (provavelmente
   unknown).

## Decisões diferidas (133.A)

4. **Assinatura exacta de `eval_set_par`**: idêntica a
   `eval_set_text`? Diferenças se o contexto par é parse
   diferente. Confirmar no inventário.

5. **Target substituto para testes migrados**: depende de
   quais outros targets são known vs unknown em
   `eval_set_rule` hoje.

6. **Contabilização de testes**: o teste `eval_set_target_desconhecido_ignora`
   usa `#set par(leading: 1em)`. Após 133, `par` é known e
   `leading` não é capturado lá → warning de propriedade em
   vez de warning de target. Pode migrar input para outro
   target OU reformular assertion.

---

## Escopo

**Dentro**:
- `01_core/src/rules/eval/rules.rs` — `eval_set_par` nova,
  dispatcher adaptado.
- `01_core/src/rules/eval/tests.rs` — 1 teste migrado +
  1 teste novo.
- `03_infra/src/integration_tests.rs` — 1 teste migrado.
- `00_nucleo/prompts/rules/eval.md` (ou equivalente) + hash.

**Fora**:
- Captura de propriedades em `par`.
- `leading` migration.
- L2, L4.
- ADR nova.

---

## Sub-passos

### 133.A — Inventário confirmatório

**A.1 — Estrutura actual de `eval_set_rule`**:

`grep -n "eval_set\|set_rule\|match target\|\"text\"\|\"par\"" 01_core/src/rules/eval/rules.rs`

Registar:
- Onde vive o dispatcher (função, match, estrutura).
- Quais targets são conhecidos hoje.
- Como é tratado target desconhecido (mensagem de warning).
- Como a função `eval_set_text` é invocada (assinatura exacta).

**A.2 — Assinatura de `eval_set_text`**:

Ler corpo completo. Registar:
- Parâmetros.
- Tipo de retorno.
- Como faz iteração sobre argumentos.
- Onde emite fallback warning para propriedade desconhecida.

**A.3 — Pool de targets unknown**:

`grep -rn "unknown target\|target.*não suportado\|target.*support" 01_core/src/`

Listar targets que são **explicitamente unknown** em `eval_set_rule`
hoje. Candidatos prováveis: `figure`, `heading`, `page`, `list`,
`enum`, `table`, `raw`.

Confirmar pelo menos 1 candidato que continua unknown após 133
(para testes migrarem para lá).

**A.4 — Testes afectados**:

Em `01_core/src/rules/eval/tests.rs`:
- Procurar `#set par(` — listar todos.
- Para cada, registar:
  - Assertion actual.
  - Se testa target-unknown vs propriedade-unknown.
  - Migração necessária.

Em `03_infra/src/integration_tests.rs`:
- `debt49_set_target_desconhecido_emite_warning` — confirmar
  caminho + conteúdo.

**A.5 — Contagem base**:
- L1: 852.
- L3: 186.
- Total: 1083.

**Gate 133.A**:
- Se A.3 revela que **todos os targets candidatos já são known**:
  ataca opções alternativas (targets inventados/sintéticos) ou
  reformulação do teste. Reportar antes de prosseguir.
- Se `eval_set_text` tem assinatura muito acoplada a contexto
  `text` (não trivialmente transponível para `par`): ajustar
  plano, pode exigir refactor maior que S.
- Outros casos: prosseguir.

### 133.B — Criar `eval_set_par`

**Ficheiro**: `01_core/src/rules/eval/rules.rs`.

Modelo esperado (ajustar a assinatura real após 133.A.2):

```rust
/// Aplica `#set par(...)` a `StyleDelta`.
///
/// **Estado em 133**: função criada mas sem arms. Todas as
/// propriedades caem no fallback e emitem warning
/// "propriedade não suportada". Arms concretos são adicionados
/// em passos futuros (ex: 134 migra `leading` de text para
/// par; futuro captura `justify`, `first-line-indent`).
fn eval_set_par(
    args: &[Arg],
    delta: &mut StyleDelta,
    sink: &mut Sink,
) -> Result<(), Vec<SourceDiagnostic>> {
    for arg in args {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str();
            let val = named.expr().eval(...)?;
            match key {
                // Arms concretos ausentes intencionalmente.
                // Adicionar conforme propriedades de par forem
                // activadas em passos futuros (134+).
                _ => {
                    sink.warn(
                        named.expr().span(),
                        format!("par: propriedade '{}' ainda não suportada", key),
                    );
                }
            }
        }
    }
    Ok(())
}
```

**Notas**:
- Assinatura real espelha `eval_set_text` confirmado em 133.A.2.
  Se há sutilezas (ex: parâmetros de contexto, scoping), replicar.
- Fallback warning usa mesma forma que `eval_set_text` usa hoje.
- O match vazio (apenas `_`) é válido em Rust quando todos os
  casos caem no fallback.
- Comentário explicita que arms chegam em passos seguintes
  para evitar ambiguidade.

### 133.C — Adaptar dispatcher

**Ficheiro**: `01_core/src/rules/eval/rules.rs`.

No dispatcher (match sobre target em `eval_set_rule`), adicionar
arm:

```rust
// antes:
match target.as_str() {
    "text" => eval_set_text(...),
    // outros known ...
    unknown => {
        sink.warn(
            span,
            format!("set: target '{}' ainda não suportado", unknown),
        );
    }
}

// depois:
match target.as_str() {
    "text" => eval_set_text(...),
    "par" => eval_set_par(...),
    // outros known ...
    unknown => {
        sink.warn(
            span,
            format!("set: target '{}' ainda não suportado", unknown),
        );
    }
}
```

Adaptação exacta depende do estilo do dispatcher actual
(confirmado em 133.A.1).

### 133.D — Migrar teste L1

**Ficheiro**: `01_core/src/rules/eval/tests.rs`.

**D.1 — `eval_set_target_desconhecido_ignora`**:

Opção A: migrar target para outro unknown.

```rust
// antes:
#[test]
fn eval_set_target_desconhecido_ignora() {
    let src = "#set par(leading: 1em)";
    let (result, _) = eval_inline(src);
    assert!(result.is_ok());
}

// depois:
#[test]
fn eval_set_target_desconhecido_ignora_passo_133() {
    // Após Passo 133, `par` é known target. Usar `figure`
    // (ou outro confirmado unknown em 133.A.3) como substituto.
    let src = "#set figure(numbering: \"1\")";
    let (result, _) = eval_inline(src);
    assert!(result.is_ok());
}
```

Opção B: renomear e manter intenção distinta (se relevante).

Escolha depende de 133.A.3. Se `figure` é unknown, usar figura.
Se todos candidatos são known, discutir antes.

**D.2 — Novo teste positivo**:

```rust
#[test]
fn eval_set_par_known_target_sem_arms_passo_133() {
    // Documenta por assertion o estado pós-133: `par` é known
    // target mas não captura propriedades ainda. Qualquer
    // propriedade produz warning de propriedade (não de target).
    let src = "#set par(leading: 1em)";
    let (result, warnings) = eval_inline(src);
    assert!(result.is_ok());

    // Warning é sobre PROPRIEDADE (leading), não TARGET (par):
    assert!(warnings.iter().any(|w|
        w.message.contains("'leading'") && !w.message.contains("target")
    ));
    // Confirma que 'par' não aparece como target unknown:
    assert!(warnings.iter().all(|w| !w.message.contains("target 'par'")));
}
```

Este teste é o **contrato executável** do que mudou. Se alguém
reverte 133, este teste falha.

### 133.E — Migrar teste L3

**Ficheiro**: `03_infra/src/integration_tests.rs`.

**E.1 — `debt49_set_target_desconhecido_emite_warning`**:

```rust
// antes (pseudo):
let src = "#set par(leading: 10pt)";
assert warning contains "'par'" + "target";

// depois:
let src = "#set figure(numbering: \"1\")";  // ou outro unknown
assert warning contains "'figure'" + "target";
```

Renomear para `debt49_set_target_desconhecido_emite_warning_passo_133`
se o convention exige sufixo; ou manter nome e só alterar input
+ assertion.

### 133.F — Prompt L0

Se `00_nucleo/prompts/rules/eval.md` descreve o dispatcher
de `eval_set_rule`, actualizar para incluir `par` como known
target. Correr `crystalline-lint --fix-hashes .`.

### 133.G — Verificação

1. `cargo test -p typst-core` — L1: 852 → **853** (+1 teste
   novo `eval_set_par_known_target_sem_arms_passo_133`;
   `eval_set_target_desconhecido_ignora` renomeado ou
   adaptado, não adicionado).

2. `cargo test -p typst-infra` — L3: **186** (teste migrado
   mas não adicionado nem removido).

3. `cargo test --workspace` — total ≥ 1084.

4. `crystalline-lint` zero violations.

5. Manual:

```bash
$ cat p.typ
#set par(leading: 1em)
Texto
$ typst p.typ -o p.pdf
p.typ:1:10: warning: par: propriedade 'leading' ainda não suportada
exit=0

$ cat pu.typ
#set par(justify: true)
Texto
$ typst pu.typ -o pu.pdf
pu.typ:1:10: warning: par: propriedade 'justify' ainda não suportada
exit=0

$ cat u.typ
#set figure(numbering: "1")    # ou outro unknown confirmado em 133.A.3
Texto
$ typst u.typ -o u.pdf
u.typ:1:6: warning: set: target 'figure' ainda não suportado
exit=0

$ cat t.typ
#set text(font: "Arial")       # regressão 132B
Texto
$ typst t.typ -o t.pdf
exit=0, stderr: (vazio)

$ cat h.typ
#set text(hyphenate: true)     # canary (regressão 132B)
Texto
$ typst h.typ -o h.pdf
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
exit=0
```

### 133.H — Encerramento

Relatório em `typst-passo-133-relatorio.md`:

- Inventário 133.A (targets known vs unknown, assinatura de
  eval_set_text, lista de testes afectados).
- Assinatura escolhida para `eval_set_par`.
- Decisão do target substituto para testes migrados.
- Diff por ficheiro.
- Números finais.
- Mudança observável: `#set par(...)` agora emite warning de
  propriedade em vez de warning de target.
- Preparação para 134 (leading migration).

---

## Critério de conclusão

1. Inventário 133.A escrito com 5 pontos verificados.
2. Função `eval_set_par` criada em `rules.rs` com estrutura
   similar a `eval_set_text` e sem arms concretos.
3. Dispatcher `eval_set_rule` invoca `eval_set_par` quando
   target é `"par"`.
4. Teste L1 `eval_set_target_desconhecido_ignora` migrado
   (input alterado para outro target unknown confirmado em
   133.A.3).
5. Teste L1 novo `eval_set_par_known_target_sem_arms_passo_133`
   documenta comportamento pós-133 por assertion.
6. Teste L3 `debt49_set_target_desconhecido_emite_warning`
   migrado (input alterado).
7. L1 tests: **853** (+1 net).
8. L3: 186 (inalterado).
9. `cargo test --workspace` passa (≥ 1084).
10. `crystalline-lint` zero violations.
11. Teste manual confirma 5 cenários.
12. Relatório 133.H escrito.

---

## O que pode sair errado

- **Todos os candidatos de target unknown já são known**:
  improvável pela lista típica de Typst, mas possível. Se
  acontece, discutir alternativas: (a) inventar target
  sintético só para teste (ex: `xpto`), (b) reformular teste
  para testar erro de parser em vez de warning de target,
  (c) remover o teste como obsoleto e documentar.

- **Assinatura de `eval_set_text` muito acoplada**: parâmetros
  específicos a contexto text (ex: referência a elemento texto
  paralelo). Se replicação para `par` exige refactor
  significativo, reportar e discutir — pode exigir abstracção
  comum primeiro (S+).

- **Warning de propriedade para par tem forma diferente**:
  `eval_set_text` emite `"text: propriedade 'X' ainda não
  suportada"`. `eval_set_par` deve emitir `"par: propriedade
  'X' ainda não suportada"` — variação do prefixo. Se o helper
  de warning é partilhado, passar prefixo como parâmetro.

- **Dispatcher usa enum em vez de string match**: se targets
  são enum variants (ex: `enum Target { Text, Par, Unknown }`),
  adicionar `Par` variant pode ter ripple em outros sítios.
  Confirmar em 133.A.1.

- **Teste L1 migrado falha por sintaxe do target substituto**:
  `#set figure(numbering: "1")` pode ter sintaxe própria que
  parser recusa antes de chegar ao eval. Validar sintaxe do
  substituto escolhido.

---

## Notas operacionais

- **Este passo é infra**. Não adiciona comportamento
  observável útil ao utilizador final (continua a ver warnings
  para propriedades de `par`), mas habilita o 134 que resolve
  a divergência `leading`.

- **Função vazia** (`eval_set_par` com match só `_`) pode
  parecer estranha. É deliberada — estrutura presente para
  replicação futura de pattern DEBT-1 XS. Alternativa era
  adiar a criação até 134 incluir a primeira propriedade;
  preferir split porque infra e semântica têm riscos
  diferentes.

- **Teste novo `eval_set_par_known_target_sem_arms_passo_133`
  é contrato executável**: a forma exacta do warning que o
  utilizador vê muda de "target 'par' ainda não suportado"
  para "par: propriedade 'X' ainda não suportada". Esta
  diferença pode afectar quem já tinha código Typst com
  `#set par(...)` silenciosamente ignorado. Documentar em
  relatório como mudança observável suave.

- **Ritmo estimado**: S. ≈1h. Menor que 131B/132B porque
  não há tipo novo e não há migração massiva de canary.
  A maior parte do esforço é no inventário 133.A (confirmar
  estrutura do dispatcher).

- **Candidato registado continua pendente**: "extract helper
  `eval_with_warnings`" no test harness (Passo 127). Ganha
  peso em cada novo teste. Priorizar após 135 (fecho DEBT-1).
