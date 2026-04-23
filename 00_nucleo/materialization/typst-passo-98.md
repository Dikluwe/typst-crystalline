# Passo 98 — Extracção de `current_file` e `figure_numbering` do `EvalContext` (ADR-0036)

**Série**: 98 (único passo de construção; sub-passos só de
verificação).
**Precondição**: Passo 97 encerrado, DEBT-47 fechado, 764 L1 +
174 L3 testes a passar, zero violations.
**ADR aplicável**: ADR-0036 (atomização progressiva), Regra 1
(dependências declaradas na assinatura).

---

## Objectivo

Remover os dois últimos campos não-Regra-4 do `EvalContext`:
`current_file` e `figure_numbering`. Cada um passa a parâmetro
explícito nas funções `eval_*` que o usam, conforme o padrão
estabelecido nos Passos 92–95 (`route`, `styles`, `show_rules`,
`active_guards`).

Após este passo, `EvalContext<'w>` fica com apenas 4 campos,
todos justificados pela Regra 4 da ADR-0036:

```rust
pub struct EvalContext<'w> {
    pub world: &'w dyn World,        // handle externo — Regra 4
    pub loop_iterations: usize,      // contador monotónico — Regra 4
    pub max_loop_iterations: usize,  // limite estático — Regra 4
    pub next_rule_id: RuleId,        // alocador monotónico — Regra 4
}
```

A aplicação da ADR-0036 ao `EvalContext` fica encerrada neste
passo.

---

## Escopo

**Dentro**: `01_core/src/rules/eval/` e todos os call sites das
funções `eval_*` afectadas (tipicamente `rules/`, mas o grep do
sub-passo 98.A confirma).

**Fora**:
- Agrupamento em novo struct (`EvalEnv` ou afim). Esse trabalho
  é do candidato "Materializar `Engine<'a>`" do relatório de
  continuidade. Fazê-lo agora antecipa decisão não forçada pelos
  factos.
- Materialização de folhas (`Style`, `LazyHash`, `Introspection`).
- Qualquer reestruturação de ficheiros.

---

## Sub-passos de verificação

### 98.A — Inventário de leitores, escritores e cadeia transitiva

Inventário **tem de ser transitivo**, não directo. Uma função
que só passa `ctx` a outra que lê `ctx.current_file` também
ganha o parâmetro novo no refactor. Contar só leitores directos
subestima o raio de explosão.

1. Grep por `ctx.current_file`, `ctx.figure_numbering`,
   `self.current_file` e `self.figure_numbering` em
   `01_core/src/`. Os dois últimos são obrigatórios: dentro de
   `impl EvalContext` o acesso é `self.field`, não `ctx.field`,
   e o grep apenas por `ctx.` deixa-os passar.
2. Classificar cada match:
   - **Leitor directo (LD)**: lê o campo.
   - **Escritor (W)**: muta o campo.
3. Partindo dos LDs, subir a cadeia de chamadas para identificar
   **leitores transitivos (LT)** — funções que recebem `ctx` e
   chamam LDs ou outros LTs. Registar a profundidade máxima da
   cadeia entre o escritor mais profundo e o leitor directo mais
   profundo (importa para saber se threading manual é
   sustentável).
4. Para cada W, identificar o escopo da mutação (até onde o novo
   valor tem de viajar). Caso típico: `include`/`import` muda
   `current_file` durante a avaliação do ficheiro incluído.
5. Para `figure_numbering`, registar **o tipo da origem** do
   valor: vem de `Value::String` do AST? De um `EcoString`
   partilhado? De um `&'w str` do world? A resposta decide se a
   extracção pode usar `Option<&str>` (exige que o lifetime já
   exista na origem) ou tem de usar `Option<String>` por valor
   (quando a origem é owned sem lifetime disponível).
6. Escrever inventário em
   `00_nucleo/diagnosticos/inventario-evalcontext-passo-98.md`
   com formato:
   ```
   current_file:
     leitores directos: N sites em M ficheiros
     leitores transitivos: P funções que ganham parâmetro novo
     escritores: K sites
     profundidade máxima da cadeia: D níveis
     escopo de cada escrita: <descrição>

   figure_numbering:
     (idem — leitores, escritores, cadeia)
     tipo de origem: <String / EcoString / Arc<str> / &'w str>
     lifetime disponível na origem: <'static / 'w / nenhum>
     decisão de passagem: <Option<String> por valor / Option<&str> / &Option<String>>
   ```

**Critério de saída e gate de decisão**: números concretos
registados. A métrica decisiva é **leitores transitivos (LT)**,
não leitores directos. Se LT de `current_file` > 40 **ou**
profundidade > 6 níveis, parar e reabrir a decisão Opção A vs
Opção B com o utilizador em conversa. **Não avançar para 98.B
nem 98.C com execução parcial**: o objectivo do passo é fechar
a aplicação da ADR-0036 ao `EvalContext`, e fazer só um dos dois
campos deixa o fecho pendente. Se a decisão mudar para Opção B,
o passo será renumerado/reescrito para extrair os dois campos
em conjunto num agrupamento que antecipe parte do futuro
`Engine<'a>`.

### 98.B — Extracção de `current_file`

Só executar após 98.A confirmar ≤40 LTs e profundidade ≤6.

1. Adicionar `current_file: FileId` como parâmetro explícito nas
   funções `eval_*` que o lêem **e** nas funções intermédias
   que encaminham para elas (lista vem de 98.A).
2. Nos call sites que escrevem (shadowing durante
   `include`/`import`), usar o padrão de variável local:
   ```rust
   let current_file = new_file_id;
   eval_module(ctx, route, styles, ..., current_file, ...)?;
   ```
   Em vez de mutação de campo.
3. Remover o campo `current_file` de `EvalContext`.
4. Teste após cada ficheiro tocado:
   `cargo test -p typst-core`. Não continuar com falhas.

**Nota sobre segurança do padrão**: o padrão de variável local
**elimina** um bug latente do padrão actual, não o introduz. No
padrão actual (`ctx.field = novo; ...?; ctx.field = antigo;`), o
`?` salta antes da restauração e o `ctx.field` fica corrompido
para chamadas subsequentes. No padrão novo, o `current_file` do
chamador nunca foi tocado — só existe a variável local do
chamado, que morre naturalmente no return. Incluir um teste com
`include` aninhado onde o include intermédio falha para
confirmar que o valor do chamador permanece íntegro.

**Critério de saída**: zero referências a `ctx.current_file` e
`self.current_file` no workspace; `cargo test --workspace`
passa; `crystalline-lint` zero violations.

### 98.C — Extracção de `figure_numbering`

Idêntico a 98.B, trocando `current_file` por `figure_numbering`.
A decisão de tipo (`Option<String>` por valor vs `Option<&str>`
vs `&Option<String>`) é determinada pelo 98.A, não pelo gosto.
Regra explícita: **não forçar lifetime novo**. Se a origem é
owned sem lifetime disponível, usar `Option<String>` por valor.
O clone de uma string de numeração (tipicamente curta, ex:
`"1.a"`) é desprezável comparado com o custo de propagar
lifetime novo por toda a cadeia de eval.

`Option<&str>` é preferido **só** quando o lifetime já existe
naturalmente (ex: `&'w str` do world).

Se o call site precisa de distinguir `None` de `Some("")`,
preferir a forma que preserva a distinção.

**Critério de saída**: idem 98.B, para `figure_numbering`.

### 98.D — Verificação estrutural

1. `grep -rn 'ctx\.current_file\|ctx\.figure_numbering' 01_core/`
   retorna zero matches.
2. `grep -rn 'self\.current_file\|self\.figure_numbering'
   01_core/` também retorna zero matches (apanha acessos de
   dentro de `impl EvalContext`).
3. `grep -rn 'current_file\|figure_numbering' 01_core/src/rules/eval/mod.rs`
   e inspecção manual para confirmar que as ocorrências
   restantes são **só** assinaturas de funções (parâmetros), não
   acessos a campos.
4. A definição de `EvalContext` tem exactamente 4 campos:
   `world`, `loop_iterations`, `max_loop_iterations`,
   `next_rule_id`.
5. Cada campo restante tem comentário
   `// ADR-0036 Regra 4: <razão>` imediatamente acima.
6. `cargo test --workspace`: 764 L1 + 174 L3 + 6 ignorados
   (linha de base). Permitido aumentar (smoke V2 de ficheiros
   novos, se os houver — neste passo não deve haver). Nunca
   diminuir.
7. `crystalline-lint` com zero violations.

### 98.E — Encerramento

Escrever `typst-passo-98-relatorio.md` com:

- Números finais: leitores directos, leitores transitivos e
  escritores actualizados para `current_file` e
  `figure_numbering`.
- Profundidade máxima da cadeia de threading (dado empírico
  que justifica ou não um futuro `Engine<'a>`).
- Exemplos de assinatura antes/depois (uma função `eval_*`
  tocada).
- Contagem de parâmetros das funções `eval_*` antes e depois
  (era 6 explícitos além de `ctx`; passa a 8).
- Decisão final de tipo para `figure_numbering` com a
  justificação da origem.
- Nota sobre o fecho da aplicação da ADR-0036 ao `EvalContext`.
- Eventual DEBT aberto se algum call site revelou acoplamento
  implícito impossível de resolver neste passo.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 98.A escrito com leitores directos, transitivos,
   escritores, profundidade e origem de `figure_numbering`.
2. `EvalContext` com 4 campos, todos Regra 4 com comentário.
3. Zero `ctx.current_file`, `ctx.figure_numbering`,
   `self.current_file`, `self.figure_numbering` no workspace.
4. `cargo test --workspace` passa com contagem ≥ linha de base.
5. `crystalline-lint` zero violations.
6. Relatório 98.E escrito.

---

## O que pode sair errado

- **LTs > 40 ou profundidade > 6 níveis**. Gatilho para parar o
  passo e reabrir a decisão Opção A vs Opção B **antes** de
  executar qualquer extracção. Não executar meio passo: ou se
  faz os dois campos com Opção A, ou se faz os dois com Opção B
  (agrupamento). Fazer metade deixa a ADR-0036 pendente sem
  fechar.
- **Origem de `figure_numbering` sem lifetime disponível e com
  expectativa de zero clones**. Se o 98.A revelar que a origem
  é owned e o site de consumo é muito quente, o clone por valor
  pode aparecer em perfil. Registar como DEBT para considerar
  `Arc<str>` numa materialização futura; não tentar resolver
  com lifetime novo neste passo.
- **`figure_numbering` escrito em mais sítios que os set-rules
  esperados**. Pode indicar acoplamento escondido. Registar
  como DEBT se aparecer; não corrigir neste passo.
- **Acessos via `self` dentro de métodos de `EvalContext` que
  escapem ao grep por `ctx.`**. Já coberto pelo passo 98.D.2,
  mas vale avisar: `impl EvalContext { fn foo(&self) { ...
  self.current_file ... } }` é invisível a um grep só por
  `ctx.`. O grep por `self.` é obrigatório.

---

## Notas operacionais

- Este passo não toca visibilidade. Se surgir tentação de mexer
  em `pub(super)`, ignorar: foi auditado no Passo 97.
- Este passo não reestrutura ficheiros.
- A ordem 98.B antes de 98.C é arbitrária; se o 98.A mostrar
  que `figure_numbering` é trivialmente mais simples (ex: zero
  LTs intermédios), pode ser feito primeiro como aquecimento.
