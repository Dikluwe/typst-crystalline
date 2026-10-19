# L0 — Passo 1075: Captura Regex Não-Participante Devolve `""` em vez de `none` — Achado #10 do P1031

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer uso de
`regex(...)` com grupos opcionais é afectado). **Requer confirmação do dono antes
de codificar.**

**Base**: P1031, Achado #10 (citação literal confirmada no P1071):
`"ab".match(regex("a(x)?(b)")).captures`: vanilla `(none, "b")`, cristalino
`("", "b")`. Código já citado no P1071:
`01_core/src/entities/regex.rs:65,85` —
`captures.push(caps.get(i).map(|g| g.as_str().to_string()).unwrap_or_default())`.

---

## 1. Ler antes de codificar

Tenho só a linha isolada citada no P1071, não o arquivo completo nem o tipo real
do container `captures`. Pedir:

- `00_nucleo/prompts/entities/regex.md`
- `01_core/src/entities/regex.rs`

Preciso de confirmar, antes de desenhar a correcção:
1. O tipo de `captures` — `Vec<String>` ou `Vec<Value>` (ou outro)? Se for
   `Vec<String>`, `unwrap_or_default()` produz `""` porque é o único valor
   possível do tipo — a correcção não é só trocar a chamada, é mudar o tipo do
   vector inteiro para acomodar `none` como valor distinto de string vazia.
2. Quem consome `captures` depois — `stdlib` de regex (`match()`, provavelmente),
   e o que esse consumidor espera receber para grupos não participantes hoje.

## 2. Duas linhas, não uma — confirmar se são o mesmo padrão

O P1071 cita duas linhas (`:65` e `:85`) com o mesmo trecho. Confirmar se são
duas funções diferentes com a mesma lógica duplicada (candidato a consolidar
numa função só, mesmo espírito do achado do P1054 sobre duplicação) ou se é
citação repetida do mesmo ponto por engano — não presumir.

## 3. Mecanismo (esboço, a confirmar contra o tipo real)

Se `captures` for `Vec<Value>` (ou puder ser mudado para tal sem quebrar
consumidores):

```rust
captures.push(match caps.get(i) {
    Some(g) => Value::Str(g.as_str().to_string()),
    None => Value::None,
});
```

Se for `Vec<String>` hoje, a correcção exige decidir o tipo novo — não inventar
sem ver o que os consumidores fazem com o resultado (podem já esperar `Value`,
podem esperar `Option<String>`, podem ter lógica própria a montante que precisa
de ajuste).

## 4. Medição

```
#regex("a(x)?(b)").match("ab").captures
```

Confirmar no vanilla que o resultado é `(none, "b")`, não outra forma (ex.:
lista com posição, ou dict nomeado se o grupo tiver nome) — o P1031 já tem esta
medição, mas confirmar de novo antes de codificar é a disciplina já seguida em
todos os passos anteriores desta conversa.

## 5. Critérios de verificação

1. Grupo não participante → `none`, não `""`.
2. Grupo participante → string capturada, sem alteração (não regressão).
3. Grupos nomeados (se existirem na sintaxe suportada) — mesmo comportamento
   para não participante, confirmar se aplicável.
4. `crystalline-lint .` — 0 erros.
5. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Arquivo real lido, tipo de `captures` confirmado.
- §2 respondido — duplicação real ou citação repetida por engano.
- Mecanismo implementado de acordo com o tipo real, não um esboço genérico.
- Critérios de verificação confirmados.
