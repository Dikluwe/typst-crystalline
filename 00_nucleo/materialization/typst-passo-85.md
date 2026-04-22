# Passo 85 — DEBT-41 (sealed traits) e diagnóstico do DEBT-40 (Route do vanilla)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/DEBT.md` — Secção 1, entradas DEBT-40 e DEBT-41 completas.
- `00_nucleo/adr/typst-adr-0032-*.md` — política de `unsafe` em L1.
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com vanilla.
- `00_nucleo/adr/typst-adr-0034-*.md` — diagnóstico obrigatório antes
  de materializar tipo do vanilla.
- `01_core/src/rules/lexer/scanner.rs` — alvo do DEBT-41.
- `01_core/src/rules/eval.rs` (linhas 200–260 aproximadamente) —
  `EvalContext.import_stack`, `ImportGuard`, `enter_import`. Alvo do
  diagnóstico do DEBT-40.

Pré-condição: `cargo test` — 911 testes (737 L1 + 174 L3, 6 ignorados
pré-existentes), zero violations. Passo 84.8 concluído.

---

## Natureza deste passo

Passo único com duas tarefas independentes:

1. **Tarefa A — Execução**: resolver DEBT-41 (sealed traits no
   scanner). Refactor mecânico. Zero custo de runtime. Fecha uma
   decisão já tomada na ADR-0032.

2. **Tarefa B — Verificação**: produzir diagnóstico em
   `00_nucleo/diagnosticos/` sobre o mecanismo `Route` do Typst
   vanilla, para habilitar a decisão sobre como migrar o DEBT-40
   (`ImportGuard::drop` com raw pointer) num passo futuro dedicado.

A Tarefa B **não altera código**. É leitura do código vanilla em
`lab/typst-original/` e produção de um único ficheiro `.md` factual.

A ordem das tarefas é livre — podem ser executadas em qualquer ordem
dentro do passo, e os resultados são independentes.

---

## Decisões formalizadas neste passo

Nenhuma. Este passo executa uma decisão já formalizada (ADR-0032
aplicada ao DEBT-41) e prepara uma decisão futura (diagnóstico para
DEBT-40). Decisões arquitecturais sobre DEBT-40 ficam para passo
posterior após leitura do diagnóstico.

---

## Tarefa A — DEBT-41: sealed traits no scanner

### A.1 — Inventariar as ocorrências

```bash
grep -n "unsafe" 01_core/src/rules/lexer/scanner.rs
```

Esperado: 6 ocorrências de `unsafe impl Sealed<T>` + 1 ocorrência de
`pub unsafe trait Sealed<T>` + 7 ocorrências de
`unsafe { self.string.get_unchecked(...) }` (estas últimas pertencem
ao DEBT-42, bloqueado — **não tocar** nestas neste passo).

Se a contagem diferir, reportar antes de continuar.

### A.2 — Aplicar o padrão "sealed via private module"

Substituir:

```rust
pub unsafe trait Sealed<T> { ... }
unsafe impl Sealed<char> for ... { ... }
unsafe impl Sealed<&str> for ... { ... }
// ... (6 impls no total)
```

Por:

```rust
mod sealed {
    pub trait Sealed<T> {
        // Mesma assinatura dos métodos originais, se houver.
    }
}

// Usar sealed::Sealed como bound nas traits públicas:
pub trait Pattern: sealed::Sealed<char> { ... }

// Impls sem unsafe:
impl sealed::Sealed<char> for ... { ... }
impl sealed::Sealed<&str> for ... { ... }
// ... (6 impls, todos sem unsafe)
```

Detalhes:

- O módulo `sealed` é privado ao ficheiro (`mod sealed` sem `pub`).
  Tipos externos ao ficheiro não podem implementar `sealed::Sealed`
  porque não podem nomear o trait — é o mecanismo de selamento.
- Preservar os métodos do trait se existirem. O selamento é
  ortogonal à API do trait.
- Verificar que os `use` statements que referenciam `Sealed` em
  outros ficheiros continuam a funcionar. Se o trait `Pattern`
  (ou equivalente) é que é exportado, os consumidores não devem
  ver diferença.

### A.3 — Verificação da Tarefa A

```bash
# Zero ocorrências de "unsafe" relacionadas com Sealed:
grep -n "unsafe trait Sealed\|unsafe impl Sealed" \
    01_core/src/rules/lexer/scanner.rs
# Esperado: zero linhas.

# As 7 ocorrências de get_unchecked permanecem (DEBT-42):
grep -c "get_unchecked" 01_core/src/rules/lexer/scanner.rs
# Esperado: 7.

# Testes passam:
cargo test --package typst-core
# Esperado: mesmo número de testes, zero falhas, zero violations.

# Linter:
cargo run --package crystalline-lint
# Esperado: zero violations.
```

### A.4 — Actualizar DEBT.md

Mover a entrada DEBT-41 da Secção 1 (em aberto) para a Secção 2
(encerrados). Manter o texto original da entrada e acrescentar uma
linha final:

```
**Resolvido no Passo 85.** Padrão "sealed via private module"
aplicado. Zero `unsafe` associadas a Sealed em `scanner.rs`.
Get_unchecked permanece (DEBT-42, bloqueado por benchmark).
```

Não renumerar outras entradas. Preservar a ordem das entradas nas
respectivas secções.

---

## Tarefa B — Diagnóstico do DEBT-40: mecanismo Route do vanilla

### B.1 — Localizar o código no vanilla

```bash
# Procurar definição de Route:
grep -rn "struct Route\|enum Route" lab/typst-original/ \
    --include="*.rs" | head -20

# Procurar uso de Route em import/eval:
grep -rn "Route" lab/typst-original/ \
    --include="*.rs" | grep -iE "import|module|eval" | head -30

# Procurar a função que invoca eval de módulo importado:
grep -rn "import_file\|eval_file\|import_module" lab/typst-original/ \
    --include="*.rs" | head -20
```

Se a estrutura `Route` não existir em `lab/typst-original/`, procurar
alternativas: `Tracer`, `import_stack`, ou qualquer outro mecanismo
de detecção de ciclos. Reportar o que foi encontrado.

### B.2 — Conteúdo mínimo do diagnóstico

Produzir o ficheiro
`00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md` com
as 7 secções mínimas da ADR-0034, adaptadas ao mecanismo `Route`:

1. **Localização** — caminho(s) do ficheiro no vanilla onde `Route`
   é definido e usado. Linha aproximada do `struct`/`enum`.

2. **Definição estrutural** — os campos/variantes exactos da
   estrutura, citados com snippet curto. Incluir derives
   (`#[derive(...)]`) e atributos relevantes.

3. **Operações** — quais métodos a estrutura expõe (`::root()`,
   `::insert(...)`, `::contains(...)`, etc.) e a assinatura de cada
   um. Para cada método, uma frase sobre o que faz.

4. **Mecanismo de recursão** — como um novo frame é criado e ligado
   ao pai. Snippet do ponto de chamada típico (pode ser de
   `eval_module` ou equivalente). Resposta clara à pergunta: o frame
   é passado por referência (`&Route`) ou por valor? A ligação ao
   pai é `&Route`, `Arc<Route>`, `Option<&Route>`, ou outra?

5. **Mecanismo de detecção de ciclo** — como `contains()` funciona
   (iteração linear, HashSet, outra). Qual é a complexidade amortizada
   da verificação por cada `#import`.

6. **Integração com comemo (se relevante)** — se `Route` é `Track`ed
   ou participa no tracking de `comemo`, descrever como. Se não,
   registar que não é. Esta informação é relevante para a eventual
   integração futura no cristalino.

7. **Divergências actuais do cristalino** — contraste factual entre
   `Route` do vanilla e `import_stack: Vec<FileId> + ImportGuard`
   do cristalino. Não propor resolução — apenas listar as
   divergências observáveis.

Regra estrita: o diagnóstico é **descritivo**, não prescritivo. Não
incluir recomendações, não propor opções de migração, não estimar
esforço. Esse trabalho fica para o passo que executar o DEBT-40.

Exemplo de frase aceitável: "No vanilla, `Route::contains` percorre
linked list via `upstream`; complexidade O(profundidade)."

Exemplo de frase inaceitável: "O cristalino deveria migrar para
`Route` porque é mais limpo." (prescritivo — fica para outro passo).

### B.3 — Limite de tamanho do diagnóstico

Alvo: 80–200 linhas. Se ultrapassar 250 linhas, simplificar
(remover citações longas, preservar referências de linha).

Se algum dos 7 itens não tiver informação disponível no código
vanilla acessível em `lab/typst-original/` (ex: `Route` vive noutro crate
não incluído no lab), registar literalmente "não observável em
`lab/typst-original/`" nesse item. Não inventar.

### B.4 — Verificação da Tarefa B

```bash
# Ficheiro existe e tem as 7 secções:
ls -la 00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md

grep -c "^## " \
    00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md
# Esperado: >= 7.

# Tamanho razoável:
wc -l 00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md
# Esperado: entre 80 e 250 linhas.
```

---

## Critérios de conclusão

- [ ] Zero ocorrências de `unsafe trait Sealed` ou `unsafe impl
      Sealed` em `01_core/src/rules/lexer/scanner.rs`.
- [ ] As 7 ocorrências de `unsafe { get_unchecked(...) }` permanecem
      intactas em `scanner.rs` (DEBT-42 não é tocado).
- [ ] `cargo test --package typst-core` passa com o mesmo número
      de testes que a pré-condição (737 L1).
- [ ] `cargo test --workspace` passa com os mesmos 911 testes, 6
      ignorados pré-existentes.
- [ ] `cargo run --package crystalline-lint` reporta zero violations.
- [ ] Entrada DEBT-41 movida para a Secção 2 do `DEBT.md` com linha
      de resolução adicionada.
- [ ] Ficheiro
      `00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md`
      existe, tem >= 7 secções, entre 80 e 250 linhas.
- [ ] Nenhum ficheiro em `00_nucleo/adr/` foi alterado.
- [ ] Nenhum ficheiro em `01_core/src/rules/eval.rs` foi alterado
      (o DEBT-40 é apenas diagnosticado, não executado).

---

## Ao terminar, reportar

Tarefa A:
- Número de linhas alteradas em `scanner.rs` (diff size).
- Confirmação da contagem de testes antes e depois.
- Confirmação de zero violations.

Tarefa B:
- Caminho(s) no vanilla onde `Route` (ou equivalente) foi encontrado.
- Se cada um dos 7 itens do diagnóstico foi preenchido com informação
  observada ou com "não observável em `lab/typst-original/`".
- Tamanho final do diagnóstico (linhas).

Go/No-Go para o passo seguinte:
- **Go para DEBT-40** se o diagnóstico da Tarefa B permitir decisão
  entre as 4 opções (1-3 do DEBT.md + 4 `Route`-baseada).
- **No-Go** se o diagnóstico revelar que `Route` não é observável
  em `lab/typst-original/` — nesse caso, passo seguinte tem de ser
  "alargar `lab/typst-original/` para incluir o crate relevante" antes do
  DEBT-40 poder ser atacado.
