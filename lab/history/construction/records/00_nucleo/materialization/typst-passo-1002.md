# Passo 1002 — Protótipo de fatiamento: `operators.rs` em nós (hub + folhas declarativas)

**Tipo**: Protótipo — primeiro caso real da ontologia hub/nó, para validar o método antes
de aplicar a `eval.md`/`bindings.rs` inteiros.
**Pré-condição**: `git status` limpo; confirmar HEAD ≥ Passo 1001.

---

## Estrutura alvo

```
engine/eval/operators.md          ← hub: só tabela de despacho, zero lógica própria
  ├── engine/eval/operators/arithmetic.md      (Declarativo)
  ├── engine/eval/operators/equality.md        (Declarativo)
  ├── engine/eval/operators/ordering.md        (Declarativo)
  ├── engine/eval/operators/coercion.md        (Declarativo)
  └── engine/eval/operators/error_formatting.md (Declarativo)
```

Todos os 5 caem do lado Declarativo (achado do reconhecimento anterior — `operators.rs`
nunca toca `EvalContext`/scope, por natureza de ser aritmética/comparação de `Value`).

## Fase A — Confirmar por leitura, não presumir

1. Ler `01_core/src/engine/eval/operators.rs` na íntegra — confirmar que a lista de funções
   do Passo 1000 continua exacta (`binary_mismatch`, `long_type_name`, `value_eq`,
   `values_eq`, `value_cmp`, `cmp_arrays`, `length_partial_cmp`, `rel_partial_cmp`,
   `sanitize_length_nan`) e que `eval_binary_op`/`eval_unary_op` (citados por
   `decimal-arithmetic.md`) estão lá.
2. Ler `00_nucleo/prompts/engine/eval/decimal-arithmetic.md` (o órfão) na íntegra — é o
   candidato a nó `arithmetic.md`. Confirmar se o conteúdo bate com o código actual
   (verificar deriva — o ficheiro pode ter mudado desde P404 sem o hash acompanhar, já que
   nunca foi o dono).
3. Aplicar os 4 critérios de medição a cada uma das 5 divisões propostas, com evidência
   concreta, não só a tabela já esboçada na conversa:
   - **Isolamento de teste**: para cada nó, escrever (ou confirmar que já existe) um teste
     que não precisa de nenhum tipo/estado dos outros 4 nós.
   - **Pureza vs estado**: confirmar por leitura que nenhuma das 9 funções toca
     `EvalContext`, `Scope`, ou qualquer coisa fora dos parâmetros recebidos.
   - **Co-mudança histórica**: verificar `git log -p --follow` (ou equivalente) nas 9
     funções — se possível, confirmar se `value_eq`/`values_eq` sempre mudaram juntas
     (esperado, mesmo nó) e nunca junto com `sanitize_length_nan` (esperado, nós
     diferentes). Reportar o que encontrar, mesmo que contradiga a divisão proposta.
   - **Correspondência vanilla**: já temos do Passo 1000 — vanilla mistura tudo isto num só
     `ops.rs` (152 linhas). Isto não invalida o corte (o vanilla é pequeno, cabe junto sem
     dor; o cristalino é maior por outras razões) — só registar que aqui o cristalino vai
     divergir da forma vanilla deliberadamente, e escrever a razão.

## Fase B — Materializar, só depois da Fase A confirmar os 4 critérios

1. Criar os 5 prompts finos (`operators/arithmetic.md` reaproveitando o conteúdo de
   `decimal-arithmetic.md` + os outros tipos já cobertos hoje — Int/Float/Length/etc.;
   os outros 4 de raiz).
2. Reescrever `operators.md` como hub — só a tabela "para este `BinOp`+tipos, ver nó X".
3. **Não** mexer no ficheiro `.rs` ainda nesta fase, a menos que a divisão em ficheiros
   `.rs` seja necessária para o `@prompt` 1:1 fazer sentido (decisão a tomar em Fase A, não
   presumir aqui se `operators.rs` precisa de virar 5 ficheiros `.rs` ou se pode continuar
   1 ficheiro com 5 prompts a apontar para regiões diferentes — **confirmar se
   `crystalline-lint` suporta múltiplos `@prompt` por ficheiro**, per a nota do relatório de
   P412 sobre "comportamento de troca de hashes com múltiplos `@prompt` no mesmo ficheiro").

## Fase C — Validar

- `crystalline-lint .` — zero violations, em particular confirmar que os 5 nós não geram
  órfão novo nem V5/V6 inesperado.
- Suite de testes inalterada (isto é reorganização de prompt/possivelmente de ficheiro, não
  mudança de comportamento — zero regressão esperada).
- Relatório final regista: os 4 critérios aplicados com evidência real (não a tabela
  hipotética desta conversa), e se algum dos 5 cortes propostos falhou algum critério na
  prática — corrigir a divisão antes de fechar, não forçar a divisão original contra a
  evidência.

## Resultado esperado

Primeiro caso real e verificado da ontologia hub/nó, servindo de modelo para o resto do
`eval.md`/`bindings.rs`/`rules.rs`. Se algum dos 4 critérios se mostrar inútil ou enganador
na prática, registar isso também — o método também está a ser testado, não só o corte.
