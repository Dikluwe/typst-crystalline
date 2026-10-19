# Passo 1015 — Relatório: dedup de `long_type_name`

**Resultado**: deduplicado, 3 → 1. Mas a Fase A encontrou **seis** cópias da mesma
função, não três — em duas grafias. As três do âmbito do passo foram unificadas; as
outras três ficam **por decidir do dono**, porque unificá-las exige escolher qual nome é
o canónico, e isso não é presumível.

**Proveniência**: `HEAD = d3f69cf65` (Passo 1016 Parte 3), 2026-08-12 ~19:25 −03.
`git status` limpo antes de começar.

---

## Fase A — Não são idênticas, e não são três

O passo pedia explicitamente para não presumir. Bem pedido.

### As três do âmbito

| Ficheiro | Visibilidade | Forma |
|---|---|---|
| `eval/bindings/access.rs:34` | `pub(crate)` | `match value { Value::Int(_) => "integer", … other => other.type_name() }` |
| `eval/operators/join.rs:86` | privada | `match v.type_name() { "int" => "integer", … other => other }` |
| `stdlib/foundations.rs:1692` | privada | **byte-idêntica** à de `join.rs` |

`join.rs` e `foundations.rs` são idênticas entre si. **`access.rs` difere** — despacha
pela variante do `Value`, não pela string devolvida por `type_name()`.

### É divergência de comportamento ou só de mecânica?

Só de mecânica, e a prova é mecânica também. As duas formas coincidem se e só se
`type_name()` não mapear duas variantes diferentes para a mesma string entre
`"int"`/`"str"`/`"bool"`. Medido em `entities/value.rs:285-323`:

```
arms de type_name(): 36   |   strings distintas: 36   |   duplicadas: nenhuma
```

O mapa variante→string é bijectivo, logo `matches!(v, Value::Int(_))` ≡
`v.type_name() == "int"` (idem str/bool), e todas as outras variantes caem no mesmo
`other`. **Comportamento idêntico, mecânica diferente** — que é precisamente a
divergência que a ADR-0107 diz não contar como divergência. Dedup autorizada.

A equivalência ficou fixada em teste (`p1015_forma_por_string_e_forma_por_variante_coincidem`),
não só no raciocínio: o teste reconstrói a forma removida e compara as duas sobre as
sete variantes relevantes.

### O que o passo não sabia: há mais três, com outro nome

`grep 'fn .*type_name'` sobre todo o L1 revela uma família paralela chamada
`vanilla_type_name`, sobre `&Value`, que computa **exactamente a mesma coisa**:

| Ficheiro | Nome | Visibilidade | Forma | Relação com `access.rs` |
|---|---|---|---|---|
| `stdlib/loading.rs:561` | `vanilla_type_name` | privada | 3 arms + fallthrough | **byte-idêntica**, só o nome muda |
| `stdlib/pdf.rs:88` | `vanilla_type_name` | privada | 3 arms + fallthrough | **byte-idêntica**, só o nome muda |
| `eval/operators/error_formatting.rs:32` | `vanilla_type_name` | `pub(crate)` | tabela explícita de 36 arms | equivalente — ver abaixo |

A versão de 36 arms foi comparada arm a arm contra `type_name()`:

```
divergências fora de int/str/bool: NENHUMA
int/str/bool: Int→"integer", Str→"string", Bool→"boolean"
```

Ou seja: escreve por extenso o que as outras obtêm por delegação. Mesma função.

**Total: seis implementações da mesma função**, em duas grafias (`long_type_name`,
`vanilla_type_name`) e duas formas (delegar vs tabela completa).

Fora desta família, e **não** confundir:
- `entities/font_variations.rs:185` — `vanilla_type_name(name: &str)`, assinatura e
  domínio diferentes.
- `stdlib/structural/math.rs:134` — `vanilla_type_name_class`, a tabela das 10 classes
  do cast de `math.class` (P825a). Outra coisa.

---

## Fase B — Ponto único de verdade (âmbito do passo)

Canónica: **`eval/bindings/access.rs`** — já era `pub(crate)`, já re-exportada por
`bindings/mod.rs` e por `eval/mod.rs`.

1. Removida a cópia de `eval/operators/join.rs`; passa a
   `use crate::compiler::eval::long_type_name;`.
2. Removida a cópia de `stdlib/foundations.rs`; mesmo import.
3. Comentário no sítio de cada remoção a apontar para o dono, para que a próxima pessoa
   não a volte a escrever localmente.

### L0 corrigidos

- **`compiler/eval/operators/join.md`** — a secção dizia *"`long_type_name` é deste nó
  porque co-muda com `join`: foi criado para a mensagem de join e nunca mudou
  independentemente dela"*, afirmando posse. Reescrita: o nó **não possui** a função;
  o dono é `bindings/access.md`.
- **`compiler/eval/bindings/access.md`** — passa a declarar-se ponto único de verdade,
  com a prova de bijectividade e a nota de fronteira sobre a família `vanilla_type_name`.

---

## Fase C — Validação

| Verificação | Resultado |
|---|---|
| `grep -rn 'fn long_type_name' 01_core/src` | **1 ocorrência** (era 3) |
| `cargo build -p typst-core` | ✅ ok; os 2 warnings em `foundations.rs` (`format_radix` mut, `native_state` dead) são **pré-existentes**, não deste passo |
| `crystalline-lint .` | 0 errors; 3 × V7 órfãos (2 untracked do dono + 1 pré-existente) |
| `cargo test --workspace` | **5837 passed, 0 failed, 3 ignored** |
| `#[test]` HEAD → WT | 5835 → 5837 (+2: os dois testes de contrato) |

Zero regressão, como esperado — é remoção de duplicação sem mudança de comportamento.
As mensagens de erro que usam a função (`cannot join integer with string`,
`expected integer, found …`) continuam idênticas: são o mesmo cálculo, agora numa só
implementação.

---

## Fora de âmbito — decisão do dono

As três cópias de `vanilla_type_name` **não** foram tocadas. A operação seria a mesma,
mas há uma decisão real por trás que não presumo: **qual das duas grafias é a canónica**.

Argumentos que consigo medir, sem escolher:

| | `long_type_name` (`bindings/access.rs`) | `vanilla_type_name` (`operators/error_formatting.rs`) |
|---|---|---|
| visibilidade | `pub(crate)` | `pub(crate)` |
| consumidores hoje | `eval/mod.rs`, `call_dispatch.rs`, `join.rs`, `stdlib/*` | `error_formatting.rs` (erro binário genérico) |
| forma | delega em `type_name()` fora dos 3 casos | tabela explícita de 36 arms |
| a favor | menos código; um sítio a mudar se `Value` ganhar variante | explícita: uma variante nova obriga a decidir o nome, não herda um por acidente |
| contra | uma variante nova herda silenciosamente o nome curto | 36 arms a manter em sincronia com `Value` |

O trade-off é real e é de gosto arquitectural, não medível: a forma por delegação é mais
curta mas falha em aberto quando `Value` cresce; a tabela completa é verbosa mas força a
decisão. Por isso parei.

Se quiseres fechar, é um passo pequeno em qualquer das direcções:
- **canónica = `long_type_name`** → apagar as 3 `vanilla_type_name` e reapontar
  `error_formatting.rs`, `loading.rs`, `pdf.rs`;
- **canónica = `vanilla_type_name`** (tabela completa) → apagar `long_type_name`,
  reapontar os 4 consumidores, e corrigir `bindings/access.md` e `join.md` outra vez.

O que **não** recomendo é deixar as duas famílias vivas com um `pub(crate)` cada: é o
estado que produziu seis cópias.
