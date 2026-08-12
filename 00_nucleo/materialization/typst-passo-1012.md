# Passo 1012 — Fatiamento hub/nó `eval::closures` + fechar a pergunta pendente sobre `font_dict`

**Tipo**: Duas partes. Parte 1 (obrigatória primeiro): responder com evidência à pergunta
deixada em aberto sobre a classificação de `font_dict` no Passo 1011. Parte 2: aplicar o
método completo do P1002 a `eval::closures`, próximo candidato da ordem do Passo 1008.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1011.

---

## Parte 1 — Fechar `font_dict`: mecânico, não de propósito

O Passo 1011 classificou `font_dict` como "declarativo" apesar de receber `Scopes`,
`EvalContext`, `Engine` — justificado por propósito ("transforma sintaxe em dicionário
normalizado"), não pelo teste mecânico do Critério 2 (P1002: toca estado com efeito, ou é
`Value → Value` puro). Isto precisa de resposta objectiva antes de aceitar como fechado.

1. Ler `parse_font_dict_named_fields`/`parse_font_dict_legacy` (agora em
   `compiler/eval/font_dict.rs`) na íntegra.
2. Para cada chamada a `eval_expr` (ou equivalente) dentro destas funções: a chamada **lê**
   um valor de `Scopes`/`EvalContext` sem escrever em nada com efeito lateral (sink,
   active_guards, counters, warnings), ou **muta** algo em `Engine`/`ctx` como parte do
   processo?
3. Responder com `file:line` de cada ponto de contacto com `Engine`/`EvalContext`, não com
   impressão geral do ficheiro.
4. **Resultado A** (só leitura, sem efeito): a classificação "declarativo" está correcta,
   mas por razão mecânica (recebe contexto só para resolver valores, nunca escreve) — 
   reescrever a frase do L0 de `font_dict.md` para dizer isto explicitamente, substituindo
   a justificação por propósito.
5. **Resultado B** (há mutação/efeito real): a classificação está errada. Não é preciso
   desfazer a separação de `rules.rs` — o Critério 3 (co-mudança, já confirmado no P1009)
   sustenta o corte sozinho — mas corrigir o L0 para não afirmar "declarativo" onde não é,
   e registar `font_dict` como nó *stateful* como o resto do hub original.

Corrigir o L0 de `font_dict.md` conforme o resultado, antes de avançar para a Parte 2.

---

## Parte 2 — `eval::closures`, método completo do P1002

Candidato confirmado no Passo 1008: agregado (sem `trait`), fan-in real de módulo (14
ficheiros chamam os seus símbolos, `apply_func` em 12 — não inflacionado por interface).

### Fase A — Inventário completo

```
grep -n '^pub fn \|^pub(crate) fn \|^fn ' 01_core/src/compiler/eval/closures.rs
```
Incluir `pub(crate)` desde o início (lição do P1000 — não repetir a omissão).

### Fase B — Os 4 critérios, com evidência

1. **Isolamento de teste** — confirmar por leitura se cada função pode ser testada com
   `Value`/`Content` construídos à mão, ou se precisa necessariamente de `Engine`/`Scopes`
   reais (esperado que sim, dado o domínio — aplicação de closures precisa de scope).
2. **Pureza vs estado** — `apply_func`, `trace_call`, `merge_with_args`, `call_plugin`,
   `eval_location_method` (lista do P1000, confirmar contra o ficheiro actual, pode ter
   mudado). Esperado: maioritariamente stateful (chama `Engine`), mas confirmar cada uma,
   não presumir em bloco — pode haver alguma puramente de merge/validação de argumentos
   sem tocar `Engine` (ex.: `merge_with_args` pode ser só `Args`/`Args` → `Args`).
3. **Co-mudança histórica** — mesma técnica dos passos anteriores, descontando ruído de
   resselo de hash.
4. **Correspondência vanilla** — já sabemos do P1003: `typst_eval::call` (chamadas gerais)
   + `typst_library::foundations::func` (aplicação de closures em si). O cristalino
   mistura os dois. Confirmar se o corte por co-mudança aponta para a mesma fronteira.

### Fase C — Materializar

Nós conforme a Fase B decidir — não pré-definir aqui. Cada nó com L0 (campo Técnica
preenchido só se houver algo nomeável — aplicação de closures/binding de argumentos pode
corresponder a conceitos de linguagens funcionais, currying/partial application parcial se
for o caso; confirmar antes de nomear, não forçar) + ficheiro `.rs` próprio (V15).

### Fase D — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão.

### Fase E — Avaliação do método

Registar se o Critério 2 discriminou (esperado que sim, dado o domínio) e se alguma função
surpreendeu por ser pura onde se esperava estado, ou vice-versa.

---

## Resultado esperado

`font_dict.md` corrigido com justificação mecânica, não de propósito. `eval::closures`
fatiado com evidência dos 4 critérios, zero regressão. Próximo da ordem do P1008:
`bindings` já confirmado (P1008/P1010) — decidir depois deste se avança para lá ou para
`stdlib::structural`.
