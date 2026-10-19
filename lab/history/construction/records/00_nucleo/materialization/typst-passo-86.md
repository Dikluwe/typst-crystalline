# Passo 86 — Diagnóstico dos stubs comemo e padrão de uso no vanilla

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0001-*.md` — autorização de `comemo`
  em `[l1_allowed_external]` (Opção C).
- `00_nucleo/adr/typst-adr-0003-*.md` — status `IDEIA`, direcção
  de confinar `comemo` a L3 via trait `Trackable`. Referenciar
  em contraste no diagnóstico.
- `00_nucleo/adr/typst-adr-0018-*.md` — critério de pureza
  funcional para externos em L1.
- `00_nucleo/adr/typst-adr-0029-*.md` — pureza física de L1
  (sem I/O de sistema). Relevante para classificar `comemo`.
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla. Relevante para avaliar divergência estrutural do trait.
- `00_nucleo/adr/typst-adr-0034-*.md` — diagnóstico obrigatório
  antes de materializar tipo vanilla. Este passo cumpre a regra
  para os stubs `Route`, `Engine`, `Sink`, `Traced`.
- `00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md`
  — produzido no Passo 85. Ponto de partida para o diagnóstico
  deste passo.
- `01_core/src/world_types.rs` — onde os stubs vivem actualmente.

Pré-condição: `cargo test` — 911 testes (737 L1 + 174 L3, 6
ignorados pré-existentes), zero violations. Passo 85 concluído
(DEBT-41 encerrado, diagnóstico do Route produzido).

---

## Natureza deste passo

Passo único de verificação (não altera código). Duas tarefas
independentes, ambas produzem ficheiros `.md` em
`00_nucleo/diagnosticos/`:

1. **Tarefa A** — Inventário dos stubs `Tipo(())` com
   `#[comemo::track]` em L1 e catalogação das dependências que
   ainda não estão materializadas. Responde à pergunta: "que
   tipos precisam de ser materializados antes de `Route`,
   `Engine`, `Sink`, `Traced` poderem ser preenchidos?".

2. **Tarefa B** — Confirmação empírica do padrão de uso do
   `comemo` no vanilla: `#[track]` é aplicado a **structs
   concretas** ou a **trait definitions**? Responde à pergunta:
   "se o cristalino adoptar `#[track]` em trait (para ganhar
   testabilidade via dependency injection), qual é a magnitude
   da divergência estrutural face ao vanilla?".

Regra absoluta: este passo **não edita código**. Também **não
edita ADRs** nem `DEBT.md`. Apenas lê e produz 2 ficheiros de
diagnóstico.

---

## Decisões formalizadas neste passo

Nenhuma. Este passo alimenta as decisões dos passos seguintes:

- **Passo 87+ (provável)** — com base no diagnóstico da Tarefa A,
  materializar os tipos vanilla na ordem correcta das
  dependências.
- **ADR futuro** — com base no diagnóstico da Tarefa B, decidir
  entre (i) replicar padrão vanilla (struct concreta com
  `#[track]`) ou (ii) adoptar camada de abstracção (trait com
  `#[track]`) aceitando divergência estrutural documentada. Este
  ADR pode activar o ADR-0003 ou substituí-lo.

---

## Tarefa A — Inventário dos stubs e dependências em falta

### A.1 — Localizar todos os stubs com `#[comemo::track]`

```bash
# Ocorrências de #[comemo::track] em 01_core:
grep -rn "comemo::track\|#\[track\]" 01_core/src/ \
    --include="*.rs" | head -30

# Para cada stub Tipo(()), capturar estrutura actual:
grep -A 3 "pub struct Routines\|pub struct Engine\|pub struct Sink\|\
pub struct Traced\|pub struct Route" 01_core/src/world_types.rs

# Ficheiros que referenciam os stubs:
grep -rn "Routines\|Engine\|Sink\|Traced\|Route" 01_core/src/ \
    --include="*.rs" | grep -v "world_types.rs" | head -30
```

### A.2 — Para cada stub, comparar com a estrutura vanilla

Leituras em `lab/typst-original/`:

```bash
# Routines:
grep -rn "pub struct Routines\|type Routines" \
    lab/typst-original/ --include="*.rs" | head

# Engine:
grep -rn "pub struct Engine" lab/typst-original/ \
    --include="*.rs" | head

# Sink:
grep -rn "pub struct Sink" lab/typst-original/ \
    --include="*.rs" | head

# Traced:
grep -rn "pub struct Traced" lab/typst-original/ \
    --include="*.rs" | head

# Route (já coberto no Passo 85 — reutilizar diagnóstico existente):
# lab/typst-original/crates/typst-library/src/engine.rs:251
```

### A.3 — Para cada campo de cada struct vanilla, classificar

Classificação em 3 categorias por campo:

- **Disponível**: tipo já materializado em L1 (ex: `FileId`,
  `SyntaxNode`, `Value` se já existirem).
- **Dependência em cadeia**: tipo depende de outro tipo ainda
  não materializado.
- **Externo**: tipo vem de crate externa (`comemo::Tracked`,
  `AtomicUsize`, etc.) — precisa de autorização via ADR ou já
  tem.

Para cada dependência em cadeia, registar o tipo pendente e a
sua localização no vanilla.

### A.4 — Construir grafo de dependências

Uma lista ordenada topologicamente: "para materializar
`Engine`, preciso de ter materializado primeiro X, Y, Z".

Formato:

```
Route      -> depende de: FileId (✓ disponível),
                          AtomicUsize (✓ std),
                          Option<Tracked<...>> (comemo, ✓ autorizado)
           -> pronto a materializar.

Engine     -> depende de: Routines (pendente),
                          World (parcial — ver detalhes),
                          Introspector (pendente),
                          Traced (pendente),
                          Sink (pendente),
                          Route (pendente).
           -> bloqueado por 5 tipos.

Sink       -> depende de: Introspection (pendente),
                          Value (✓ disponível),
                          Styles (parcial).
           -> bloqueado por 2 tipos.

// ... etc.
```

### A.5 — Conteúdo mínimo do diagnóstico A

Produzir o ficheiro
`00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md`
com:

1. **Inventário dos stubs** — lista dos 4-5 stubs
   `Tipo(())` presentes em L1, com localização (`ficheiro:linha`).
2. **Estrutura vanilla de cada um** — snippet curto do `pub
   struct` original e campos.
3. **Classificação por campo** — tabela com campo | tipo |
   status (disponível/pendente/externo).
4. **Grafo de dependências** — o ordenamento topológico descrito
   em A.4.
5. **Tipos pendentes identificados** — lista consolidada de
   todos os tipos em "pendente" agregados do ponto 3. Para cada
   um, localização no vanilla.
6. **Candidatos "prontos a materializar"** — stubs cujas
   dependências estão todas disponíveis ou são externas
   autorizadas. Esta é a lista directamente accionável para o
   Passo 87.

Tamanho alvo: 150–300 linhas.

---

## Tarefa B — Padrão de uso do `comemo` no vanilla

### B.1 — Inventariar todos os `#[track]` no vanilla

```bash
# Todas as ocorrências:
grep -rn "#\[track\]\|#\[comemo::track\]" lab/typst-original/ \
    --include="*.rs" | head -40

# Contar por ficheiro:
grep -rln "#\[track\]\|#\[comemo::track\]" lab/typst-original/ \
    --include="*.rs" | sort -u
```

### B.2 — Para cada ocorrência, classificar aplicação

Cada `#[track]` é aplicado a uma de três coisas:

1. **`impl StructName`** — struct concreta, métodos inerentes.
2. **`impl TraitName for StructName`** — struct concreta,
   métodos de trait.
3. **`trait TraitName`** — definição de trait (pode ser usada
   como trait object `dyn TraitName`).

Para cada ocorrência, registar:
- Caminho do ficheiro.
- Categoria (1, 2, ou 3).
- Nome do tipo/trait.
- Quantos métodos tem no bloco `#[track]`.

### B.3 — Verificar uso de trait objects

```bash
# Procurar uso de Tracked<dyn Trait>:
grep -rn "Tracked<dyn " lab/typst-original/ --include="*.rs" | head

# Procurar uso de Tracked<'_, dyn Trait>:
grep -rn "Tracked<'_, dyn\|Tracked<'a, dyn" lab/typst-original/ \
    --include="*.rs" | head
```

Se houver ocorrências, o vanilla **usa** o padrão trait object
com tracking. Se não houver, o vanilla usa **apenas** structs
concretas.

### B.4 — Verificar se o trait `World` é um caso especial

O trait `World` é o único caso conhecido (pelo diagnóstico do
Passo 85) onde o cristalino usa `#[comemo::track]` num trait.
Verificar se o vanilla faz o mesmo:

```bash
grep -B 2 -A 5 "trait World\|impl.*for World\|#\[track\]" \
    lab/typst-original/crates/typst-library/src/world.rs 2>/dev/null

# Ou onde quer que World viva:
grep -rn "pub trait World" lab/typst-original/ --include="*.rs"
```

Reportar:
- O `World` do vanilla é `trait` ou `struct`?
- Tem `#[comemo::track]`? Em que forma (trait definition, impl
  block, outra)?
- É usado como `Tracked<dyn World>` ou `Tracked<SomeWorld>`?

### B.5 — Conteúdo mínimo do diagnóstico B

Produzir o ficheiro
`00_nucleo/diagnosticos/diagnostico-padrao-comemo-vanilla-passo-86.md`
com:

1. **Inventário de `#[track]` no vanilla** — contagem total,
   distribuição por crate.
2. **Classificação por categoria** — tabela com categoria (1, 2,
   ou 3 de B.2) e contagem. Exemplo de cada.
3. **Uso de trait objects** — se `Tracked<dyn Trait>` aparece
   no vanilla, quantas vezes e onde. Se não aparece, registar
   explicitamente "não observado".
4. **Caso `World`** — resposta directa ao B.4.
5. **Conclusão factual** — resposta à pergunta central da
   Tarefa B, num parágrafo: "o vanilla usa `#[track]` em
   structs concretas, em traits, ou mistura?". Não prescritivo
   (não recomenda adopção de um padrão para o cristalino —
   isso fica para ADR futuro).
6. **Magnitude da divergência estrutural** (se o cristalino
   adoptar trait + `#[track]` e o vanilla não) — factual: "N
   structs vanilla seriam substituídas por traits no
   cristalino; K impls". Sem opinião sobre se vale a pena.

Tamanho alvo: 80–150 linhas.

---

## Critérios de conclusão

- [ ] Ficheiro
      `00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md`
      existe com as 6 secções mínimas.
- [ ] Ficheiro
      `00_nucleo/diagnosticos/diagnostico-padrao-comemo-vanilla-passo-86.md`
      existe com as 6 secções mínimas.
- [ ] Em ambos os ficheiros, qualquer item não observável em
      `lab/typst-original/` está marcado literalmente como
      "não observável em `lab/typst-original/`". Nada é
      inventado.
- [ ] Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
      `04_wiring/` foi alterado.
- [ ] Nenhum ficheiro em `00_nucleo/adr/` foi alterado.
- [ ] Nenhuma entrada em `00_nucleo/DEBT.md` foi alterada.
- [ ] `cargo test` passa com os mesmos 911 testes,
      6 ignorados. `cargo run --package crystalline-lint`
      reporta zero violations (pré-condição inalterada —
      este passo não toca em código).

---

## Ao terminar, reportar

**Tarefa A** (inventário dos stubs):
- Número de stubs `Tipo(())` com `#[comemo::track]` encontrados
  em L1.
- Lista dos que estão "prontos a materializar" (todas as
  dependências disponíveis).
- Lista dos que estão bloqueados, com contagem de dependências
  pendentes por cada um.
- Tamanho final do diagnóstico (linhas).

**Tarefa B** (padrão vanilla):
- Total de ocorrências de `#[track]` no vanilla.
- Distribuição por categoria (structs concretas / impl de
  trait / definição de trait).
- Ocorrências de `Tracked<dyn Trait>` encontradas (número).
- Resposta factual de uma frase: "o vanilla usa
  predominantemente [struct concreta | trait definition |
  mistura]".
- Tamanho final do diagnóstico (linhas).

**Go/No-Go para o Passo 87**:

- **Go** se a Tarefa A identificar ao menos 1 stub pronto a
  materializar e a Tarefa B der resposta clara à pergunta do
  padrão vanilla.
- **No-Go** se:
  - nenhum stub estiver pronto a materializar (todos bloqueados
    por cadeia longa de dependências) — nesse caso, o passo
    seguinte deve ser materializar primeiro as dependências
    folha do grafo, não os stubs directamente; **ou**
  - `lab/typst-original/` não contém os crates necessários para
    verificar o padrão `#[track]` — nesse caso, passo seguinte
    é expandir o lab antes de decidir.
