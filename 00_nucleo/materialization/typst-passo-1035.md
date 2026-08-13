# Passo 1035 — Argumentos `label` rejeitados em `cite`/`link`/`footnote`

**Tipo**: Investigar → gate → corrigir. Achado #3 do P1031 — padrão sistémico, três
funções, tratado num só passo por serem a mesma causa de fundo (aceitar/rejeitar `label`
onde a linguagem o espera).
**Medição do P1031**:

| Entrada | Vanilla | Cristalino |
|---|---|---|
| `#cite("netwok")` | `error: expected label, found string` | aceita → `[netwok]` |
| `#link(<intro>)[Ir para intro]` | compila | `error: link() espera URL como string, recebeu label` |
| `A#footnote[Um] <fn> B#footnote(<fn>) C#footnote[Dois]` | `A1 B1 C2` | `error: footnote() espera content ou string, recebeu label` |

**Nota do relatório original**: no caso de `cite`, a polaridade está invertida — o
cristalino aceita `string` onde o vanilla exige `label` e erra. Não é só "falta suportar
label", é "aceita o tipo errado".

**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1033.

---

## Fase A — Confirmar a assinatura esperada de cada função no vanilla

1. `cite` — confirmar que o parâmetro é `label` obrigatório (não `string`), per o erro
   vanilla `expected label, found string`.
2. `link` — confirmar que aceita `label` como alternativa a `str` (URL) para "ir para"
   dentro do documento (label-based navigation), não só string de URL externa.
3. `footnote` — confirmar que aceita `label` como referência a nota já definida noutro
   sítio (o caso do exemplo: `B#footnote(<fn>)` reaproveita a nota `<fn>`).

## Fase B — Localizar a causa comum, se houver

Verificar se as três funções partilham o mesmo mecanismo de validação de argumento
(ex.: um cast genérico que não inclui `Value::Label` na lista de tipos aceites para estes
parâmetros), ou se são três implementações independentes com o mesmo defeito por
coincidência. Isto decide se o fix é um só (mecanismo partilhado) ou três (cada função).

## Fase C — Gate (ADR-0127, categoria 2/3)

```
Dado #cite(<key>) (label, não string)
Quando avaliado
Então aceita e resolve a citação; #cite("string") deve passar a erro (como vanilla),
  não silenciosamente aceitar

Dado #link(<label>)[conteúdo]
Quando avaliado
Então compila, navega para o elemento com esse label

Dado #footnote(<label>) reaproveitando nota já definida
Quando avaliado
Então reaproveita o número da nota original, batendo com vanilla (A1 B1 C2, não A1 B2 C3)
```

**Atenção à mudança de polaridade em `cite`**: `#cite("string")` hoje é aceite; passa a
ser erro. Isto pode quebrar documentos existentes que (incorrectamente, mas
silenciosamente) usavam string. Confirmar se há uso real disto no corpus canónico antes
de aplicar — se houver, decidir se corrige os documentos de teste ou se mantém
retrocompatibilidade com aviso, per decisão do dono.

## Fase D — Implementar e validar

```
crystalline-lint .
cargo test --workspace
```
Confirmar que `#cite` com `label` correcto continua a funcionar, e que a mudança de
polaridade não quebra nenhum teste/corpus existente sem ser assinalada explicitamente.

---

## Resultado esperado

`cite`/`link`/`footnote` aceitam `label` onde o vanilla o exige, e `cite` deixa de aceitar
`string` silenciosamente. Mudança de polaridade em `cite` confirmada como segura antes de
aplicar, não presumida.
