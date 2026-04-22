# Passo 88 — Materializar `Traced` (warm-up da série comemo)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0033-*.md` — paridade funcional com
  vanilla. Relevante porque `Traced` é tipo vanilla.
- `00_nucleo/adr/typst-adr-0034-*.md` — diagnóstico obrigatório
  antes de materializar tipo vanilla.
- `00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md`
  — Tarefa A do Passo 86, secção que descreve `Traced`.
- `00_nucleo/diagnosticos/diagnostico-padrao-comemo-vanilla-passo-86.md`
  — Tarefa B do Passo 86, secção que classifica `Traced` como
  struct concreta com `#[comemo::track]`.
- `01_core/src/entities/world_types.rs` — onde o stub vive.
- `lab/typst-original/` — código vanilla de `Traced` para
  replicação estrutural.

Pré-condição: `cargo test` — 911 testes (737 L1 + 174 L3, 6
ignorados pré-existentes), zero violations. Passo 87 concluído.

---

## Natureza deste passo

Passo único de construção. Replica o padrão vanilla de `Traced`
em L1, substituindo o stub `Traced(())` pelo tipo funcional.

Escolha arquitectural: **struct concreta com `#[comemo::track]`
em `impl` block**, seguindo o padrão vanilla categoria 1
(observado em 6 das 8 ocorrências de `#[track]` no vanilla).

Justificação: `Traced` tem identidade local (um span específico
a rastrear) e não é injectável entre crates — é o mesmo critério
que levou o vanilla a escolher struct em vez de trait.

A política em vigor estabelece: passos de construção são únicos,
não se decompõem em sub-passos.

---

## Decisões formalizadas neste passo

Nenhuma decisão arquitectural nova. Este passo executa decisões
já tomadas:

- ADR-0033 (paridade funcional) — replicar comportamento vanilla.
- ADR-0034 (diagnóstico obrigatório) — satisfeito pelo
  diagnóstico do Passo 86.
- Padrão vanilla categoria 1 — confirmado empiricamente no
  Passo 86.

---

## Diagnóstico prévio

Ver
`00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md`
secção sobre `Traced`. Resumo dos 7 itens exigidos pela
ADR-0034:

1. **Localização**: confirmada no Passo 86 em
   `lab/typst-original/`. Claude Code deve reler o ficheiro
   vanilla antes de começar, para confirmar que a forma actual
   corresponde à documentada no diagnóstico.
2. **Definição estrutural**: `Traced { span: Option<Span> }`
   ou variante com campos adicionais, conforme vanilla.
3. **Operações**: método `get() -> Option<Span>` e construtor.
   Bloco `#[comemo::track]` com métodos que expõem o campo.
4. **Dependências**: `Span`, `FileId` — ambos já materializados
   em L1.
5. **Semântica**: `Traced` rastreia o span sob execução para
   permitir diagnósticos ricos. O tracking do `comemo` permite
   invalidação granular quando apenas o span muda.
6. **Mensagens de erro**: nenhuma — `Traced` é de infraestrutura.
7. **Divergências propostas**: **nenhuma**. Seguir vanilla
   linha-a-linha (excepto importações/caminhos).

---

## Tarefa única — Materializar `Traced`

### 1 — Reler o vanilla

```bash
# Localizar o struct Traced no vanilla:
grep -B 2 -A 15 "pub struct Traced" lab/typst-original/ \
    -r --include="*.rs"

# Localizar o #[comemo::track] impl block:
grep -B 2 -A 20 "#\[comemo::track\]\|#\[track\]" \
    lab/typst-original/crates/typst-library/src/engine.rs \
    2>/dev/null || \
    grep -rn "#\[track\]" lab/typst-original/ --include="*.rs" \
        | grep -i traced
```

Confirmar que a estrutura no vanilla corresponde ao descrito
no diagnóstico do Passo 86. Se divergir, **parar** e reportar
antes de prosseguir — diagnóstico desactualizado precisa ser
corrigido antes da materialização.

### 2 — Substituir o stub em `world_types.rs`

Localização: `01_core/src/entities/world_types.rs`.

Substituir:

```rust
pub struct Traced(());

#[comemo::track]
impl Traced {}
```

Por:

```rust
/// Rastreia o span sob execução para diagnósticos ricos.
///
/// Paridade com `Traced` do Typst vanilla (ADR-0033).
#[derive(Default, Clone)]
pub struct Traced {
    // Campos exactos a confirmar pelo Claude Code no passo 1
    // a partir da leitura do vanilla. Esperado: Option<Span>.
    span: Option<Span>,
}

impl Traced {
    /// Constrói um novo `Traced` sem span activo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Cria um `Traced` com span específico.
    pub fn with_span(span: Option<Span>) -> Self {
        Self { span }
    }
}

#[comemo::track]
impl Traced {
    /// Retorna o span sob execução, se algum.
    pub fn get(&self) -> Option<Span> {
        self.span
    }
}
```

**Importante**: o exemplo acima é ilustrativo. A forma exacta
(incluindo presença/ausência de campos adicionais, métodos
adicionais no bloco `#[track]`) deve ser determinada pela
leitura do vanilla no passo 1. Não inventar, não simplificar —
replicar.

### 3 — Actualizar `Default` se existia

Se o stub anterior tinha `impl Default for Traced` implementado
manualmente (derivado ou explícito), substituir pela versão
correcta do tipo funcional. Se não existia `Default`, confirmar
se o vanilla o tem.

### 4 — Actualizar pontos de uso (se houver)

```bash
# Localizar usos actuais de Traced em L1:
grep -rn "Traced" 01_core/src/ --include="*.rs"
```

Se algum uso actual passava `Traced(())` ou dependia do stub
vazio, actualizar para usar a nova API (`Traced::new()` ou
equivalente).

### 5 — Verificar tipos na cadeia

`Traced` é referenciado por `Engine` (que é outro stub). O stub
`Engine(())` **não é alterado** neste passo — continua vazio.
Apenas garantir que a mudança em `Traced` não quebra o stub
`Engine`.

### 6 — Testes

Adicionar no mínimo 3 testes unitários em
`01_core/src/entities/world_types.rs` (ou ficheiro de teste
apropriado):

- `traced_default_retorna_none` — `Traced::new().get()` devolve
  `None`.
- `traced_com_span_preserva_valor` — `Traced::with_span(Some(s))
  .get()` devolve `Some(s)`.
- `traced_integra_com_comemo_track` — criar `Traced`, chamar
  `.track()` (gerado pelo `#[comemo::track]`), confirmar que
  compila e retorna `Tracked<'_, Traced>`.

### 7 — Verificação final

```bash
# Contagem de testes (deve aumentar em +3):
cargo test --package typst-core 2>&1 | tail -5

# Workspace completo (deve manter 174 L3 + 6 ignorados):
cargo test --workspace 2>&1 | tail -10

# Linter:
cargo run --package crystalline-lint 2>&1 | tail -5
```

Esperado:
- 737 L1 + 3 novos = **740 testes L1**.
- 174 L3 inalterado, 6 ignorados inalterado.
- Zero violations.

---

## Critérios de conclusão

- [ ] Stub `Traced(())` substituído por tipo funcional com
      campo real (`span: Option<Span>` ou forma vanilla
      equivalente).
- [ ] Bloco `#[comemo::track] impl Traced { ... }` tem ao menos
      um método (`get`), conforme vanilla.
- [ ] 3 novos testes unitários passam.
- [ ] Contagem total: 740 L1 + 174 L3 + 6 ignorados = 920 +
      ignorados.
- [ ] `cargo run --package crystalline-lint` reporta zero
      violations.
- [ ] Nenhuma alteração a outros stubs (`Engine`, `Sink`,
      `Styles`, `Route`, `Routines` continuam `Tipo(())`).
- [ ] `00_nucleo/DEBT.md` não alterado (este passo não paga
      DEBT nenhum — é construção, não resolução de dívida).
- [ ] Nenhum ADR alterado.

---

## Ao terminar, reportar

- Linhas alteradas em `world_types.rs` (diff size).
- Campos finais de `Traced` (confirmar correspondência com
  vanilla).
- Métodos no bloco `#[comemo::track]` (listar nomes).
- Contagem final de testes.
- Confirmação de zero violations.

Go/No-Go para Passo 89:
- **Go** se `Traced` materializado passou todos os testes e
  linter. Passo 89 = estender ADR-0024 para autorizar `EcoVec`
  em L1 (preparação para `Sink`/`Styles`).
- **No-Go** se:
  - A forma do vanilla diferiu do diagnóstico do Passo 86 —
    corrigir o diagnóstico antes do Passo 89 (não invalida o
    P88 concluído, mas regista a lição para materializações
    futuras).
  - Materializar `Traced` revelou dependência oculta não
    prevista — investigar antes de avançar.
