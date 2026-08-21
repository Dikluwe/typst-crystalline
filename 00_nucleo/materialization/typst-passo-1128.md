# L0 — Passo 1128: Reabrir e Corrigir de Facto `§2` (`+`/`dif`) e `§5` (`|` como Fence)

**Gate**: `ADR-0127` — mudança de comportamento por defeito, reabertura de
correcção já apresentada como fechada mas não confirmada em build
independente.

**Base**: reteste (nota externa) confirma 5 correcções reais (`log_a`/
`log_2`, secção 35, 40, 41, `stack(dir:ltr)`), mas mostra `§2` (secção
27, `+`/`dif`) e `§5` (`|` como fence, secções 26/28/9) **sem nenhuma
mudança de magnitude** face ao estado anterior à correcção alegada —
mesmo padrão do que já aconteceu com `§11` (`stack(dir:ttb)`,
apresentado como "calibrado" sem ter sido corrigido de facto).

---

## 0. Confirmar proveniência antes de reimplementar às cegas

**Não presumir que o código nunca foi tocado** — confirmar primeiro:

1. Qual commit/timestamp gerou o build usado neste reteste.
2. Se esse commit é **anterior** aos commits que alegadamente
   implementaram `§2`/`§5` (nesse caso, é só build desactualizado, sem
   contradição real — re-gerar com o commit certo e remedir antes de
   qualquer outra coisa).
3. Se é **posterior** (nesse caso, `§2`/`§5` nunca chegaram a ser
   aplicados ao binário testado, apesar da evidência anteriormente
   recebida — mesma situação de `§11`).

**Só prosseguir para reimplementação (§1/§2 abaixo) se confirmado que o
build é posterior e a correcção genuinamente não está presente.**

## 1. `§2` — espaço `+`/`dif` (secção 27, prioridade alta — afecta largura de página)

Se confirmado não aplicado: ler `spacing.rs` real (não presumir a partir
da descrição anterior "mapeado via classe Unary com precedência sobre
binary" — essa descrição nunca teve o mesmo tratamento de "antes/depois
medido" que `§11` teve depois de reaberto). Reimplementar com a mesma
disciplina agora estabelecida: mecanismo de precedência/`max()` entre o
espaçamento de classe `Binary` e o `HSpace` próprio de `dif`, com número
medido **depois** da correcção, não só descrição da causa.

**Verificação**: largura de página do `comprehensive-test` convergindo de
`+7.245pt` para `0.0000pt` (ou dentro do ruído já aceite); gap `+→dif`
nas três ocorrências da secção 27 (`+dif x²`, `+dif y²`, `+dif z²`)
convergindo para `1.83pt` cada, não `4.28pt`.

## 2. `§5` — `|` como `Fence` (secções 26/28/9, prioridade média)

Mesma disciplina — se confirmado não aplicado, reimplementar
`spacing_between_class` com o braço `(MathClass::Fence, _)` real,
confirmado por leitura do código actual, não presumido já existente.

**Verificação**: os pares de glifo já medidos nas notas originais das
secções 26/28 (`⟨φ|`, `|E(G)|`, etc.) convergindo para gap `0.00pt`
(fence-fence e fence-conteúdo comum), não `~3.65pt`.

## 3. Não misturar com o padrão recorrente ainda em aberto

As transições `7→8`, `9→10`, `14→15`, `22→23` continuam sem causa
isolada dentro de cada secção — per a própria nota, "ainda não isolei
onde exactamente". Não investigar isso neste passo — é trabalho
separado, já reconhecido como pendente, não issue nova.

## Critérios de verificação

1. §0: proveniência do build reconciliada, causa da discrepância
   determinada (desactualizado vs nunca aplicado).
2. §1: secção 27 com número real medido depois da correcção, não só
   descrição.
3. §2: secções 26/28/9 com número real medido depois da correcção.
4. Re-rodar P1086-1127 por inteiro, com atenção a `stack(dir:ttb)`
   (`§11`, já reaberto e corrigido em P1127) para confirmar que esta
   reabertura não regride essa correcção.
5. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §0 respondido antes de qualquer código tocado.
- §1/§2 com medição "antes" e "depois" lado a lado, mesmo padrão que
  finalmente fechou `§11` na sua terceira tentativa — não aceitar
  descrição de causa sem número de confirmação desta vez.
- Nenhuma correcção nova apresentada como "fechada" sem o par completo
  antes/depois.
