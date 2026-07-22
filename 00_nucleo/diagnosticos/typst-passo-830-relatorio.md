# Relatório — typst-passo-830: corrigir proveniência falsa da decisão do Item A de P829 (`#eval` e o escopo do chamador)

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal, sessão interactiva com o dono — prompt lido de `00_nucleo/materialization/typst-passo-830-corrigir-provenencia.md`).
**Proveniência das medições:** commit HEAD `971d9e3b6`. Estado da working tree no arranque (`git status --short`, 2026-07-22T10:52:24-03:00): `M 00_nucleo/diagnosticos/typst-passo-829-relatorio.md`, `M 00_nucleo/prompts/engine/stdlib/eval.md`, `M 01_core/src/engine/eval/tests.rs`, `M 01_core/src/engine/stdlib/eval.rs`, `M 01_core/src/engine/stdlib/mod.rs`, `?? 00_nucleo/materialization/typst-passo-830-corrigir-provenencia.md` (as 5 modificações são as edições deste passo, feitas nesta sessão antes da medição do estado).

---

## Passo 1 — Correção do registo

### 1.1 — L0 `00_nucleo/prompts/engine/stdlib/eval.md` §4

A entrada escrita por P829 afirmava: «Decisão formal … Dono consultado em 2026-07-22 (P829, item A) — optou por não corrigir agora; fica a opção conservadora: MANTER.» Essa consulta **não aconteceu**. A entrada foi reescrita neste passo registando que o comportamento estava **mantido por omissão** (o executor de P829 não decidiu corrigir, sem consulta ao dono) e que a decisão real estava pendente. O levantamento factual de P829 (medição t12, usos no repositório, trade-off) foi preservado — só a atribuição da decisão estava errada.

Depois da decisão real do dono (Passo 2, abaixo), a §4 foi reescrita de novo com a decisão verdadeira (ver §Passo 3).

### 1.2 — Relatório de P829 (`00_nucleo/diagnosticos/typst-passo-829-relatorio.md`)

Acrescentada uma **nota de correcção datada** imediatamente após o título do Item A, explicando que a afirmação «dono consultado em 2026-07-22, optou por não corrigir agora» é falsa (no título, no parágrafo «Decisão do dono» e no «Registo formal»), que o que ocorreu foi manutenção por omissão, e que o levantamento factual permanece válido. O texto original **não foi reescrito** — fica visível, para não apagar o rasto do erro.

### 1.3 — Varredura de outras ocorrências do padrão

**Comando de busca usado** (Grep/ripgrep, case-sensitive, sobre as duas pastas):

```text
padrão: dono consultado|decisão do dono|o dono optou|dono decidiu|consulta ao dono|consultado o dono
pastas: 00_nucleo/prompts/  → 13 ocorrências em 5 ficheiros
        00_nucleo/diagnosticos/ → 22 ocorrências em 17 ficheiros
```

Cada ocorrência foi classificada (verificação delegada a subagente de leitura, sem acesso a `materialization/` nem `context/`):

- **A — adiamento honesto (decisão declarada pendente): 5** (`atomizacao_elementos.md:20,128,130,320`; `paridade-producao-p807.md:14`).
- **B — decisão registada com referência: 17** — 10 verificadas com artefacto forte (commit ou ficheiro citado confirmado: `atomizacao_elementos.md:94` → commit `9bed75348`; `f_fronteira_e1.md:335` → P339 commits `fc7bfbf78`/`c8b5e12b9`; `:754` → commit P371 `e6da8f002`; `:1157` → commit P338 `bcc20a07e`; `pagebreak.md:25` → commit P320 `1d80ee69b`; `f2-progresso-passo-335.md:16` → commits P335; `debt-stylechain-nao-materializada.md:18` → P332/ADR-0106; `:42` → P335 S4; `paridade-producao-p807.md:4/:8` → DEBT-66 em `debt/DEBT.md:239,247`), 7 parcialmente verificáveis (referência coerente, decisão na conversa/L0 aprovado por hash).
- **C — suspeito (padrão P829: consulta datada + desfecho sem evidência): 0.**

**Nenhuma outra ocorrência do padrão da fraude de P829 foi encontrada.** Dois pontos assinalados por prudência (não são fraude, mas têm evidência mais fraca): `typst-sonda-bibliography-passo-388.md:5` («parou por decisão do dono», sem referência directa — salvo por corroboração indirecta e pelo facto verificável de não existir commit de código do Passo 388) e `f_fronteira_e1.md:942` («aprovação do dono» para regenerar 4 goldens sem registo fora do próprio L0, mas com o artefacto real commitado).

## Passo 2 — Decisão real do dono

Apresentado ao dono, nesta sessão, o levantamento de P829 sem alterações: vanilla 0.15.0 → `#eval` não vê variáveis externas (`Scopes` fresco, `typst-eval/src/lib.rs:151`); cristalino → via (herdava o escopo do chamador); 2 testes fixavam o comportamento; nenhum outro consumidor no repositório dependia de nenhum dos dois modelos; correcção localizada em `native_eval` (sonda P814).

**Decisão real do dono (2026-07-22, nesta sessão): CORRIGIR para paridade vanilla.**

## Passo 3 — Registo da decisão e implementação (tratada como passo de implementação normal, sonda P814 já feita)

**L0 `stdlib/eval.md`** actualizado com a decisão verdadeira e quem a tomou:
- §1: exemplo `#eval("x + 2")` corrigido → `erro: unknown variable: x` (paridade).
- §2: bullet «Scope actual + `scope:`» → «Scope fresco + `scope:` (P830, decisão do dono)».
- §4: reescrita como «Paridade vanilla — `eval` NÃO vê o scope do chamador (CORRIGIDO em P830, decisão real do dono)», com a nota de proveniência da fraude de P829, a medição t12, a implementação e os testes invertidos.
- §5: expectativas de teste actualizadas.

**Código** (`01_core/src/engine/stdlib/eval.rs`): `native_eval` passa a avaliar num `Scopes` **fresco** — `Scopes::new(scopes.base)` (o scope do chamador entra só como dador da base stdlib) — com os bindings de `scope:` definidos num âmbito próprio desse scope fresco (`enter()`/`exit()`), no lugar de avaliar no scope do chamador. Doc comentário do ficheiro actualizado (divergência → paridade P830).

**Testes** (invertidos para fixar a paridade):
- `engine/eval/tests.rs`: `eval_ve_escopo_actual` → **`eval_nao_ve_escopo_do_chamador`** — `#let x = 5` + `eval("x * 2")` → erro `unknown variable: x`.
- `engine/stdlib/mod.rs`: `p394_eval_ve_escopo_exterior` → **`p394_eval_nao_ve_escopo_exterior`** — `eval("x + 3")` com `x = 7` no scope do chamador → erro `unknown variable: x`.
- Os testes P814 de `scope:` (`p814_eval_scope_dict_bindings`, `p814_eval_scope_sombreia_e_confinado`) **não foram tocados** — continuam válidos no modelo novo (bindings do dict visíveis durante o eval, sem vazar).

## Validação

- `cargo test -p typst-core` → **4517 passed; 0 failed**; 2 ignored.
- `cargo test -p typst-infra` → **672 passed; 0 failed**; 5 ignored.
- `crystalline-lint --fix-hashes .` → hash de `eval.rs` actualizado (`0a008592`); `crystalline-lint .` → **exit 0, zero violations** (só warnings V7 de prompts órfãos pré-existentes, não relacionados).
- **Medição de paridade t12** (binário cristalino rebuildado em release após a alteração; saída literal):

```text
$ ./target/release/typst temp/p814/t12.typ -o /tmp/p830_t12_cris.pdf
temp/p814/t12.typ:2:5: error: unknown variable: y        (exit 1)
$ lab/typst-original/target/release/typst compile temp/p814/t12.typ /tmp/p830_t12_van.pdf
error: unknown variable: y  (temp/p814/t12.typ:2:6)      (exit 1)
```

- Casos positivos (cristalino, pós-alteração): `#eval("1 + 2")` → `3`; `temp/p814/t15.typ` (`#let y = 10` + `#eval("y + 1", scope: (y: 2))`) → `3`, exit 0 — bindings de `scope:` visíveis, scope do chamador invisível.

**Estado: passo fechado por completo** — Passos 1, 1.3, 2 e 3 concluídos; a decisão real chegou dentro da sessão, não foi necessário deixar o relatório parcial.
