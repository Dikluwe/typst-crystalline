# Passo 1010 — Correcção: evidência de fan-in de `compiler::eval::bindings` (P1008)

**Tipo**: Correcção de registo — não é fatiamento, não escreve L0, não move código.
**Motivo**: o relatório do Passo 1008 atribuiu a `bindings.rs` o símbolo `long_type_name`
com "36 usos" como prova de fan-in real. `long_type_name` não vive em `bindings.rs` — pelo
inventário do Passo 1000 sempre esteve em `operators.rs`, e desde o Passo 1002 foi movido
para `compiler/eval/operators/join.rs` (confirmado no relatório desse passo). A linha de
evidência está a citar o ficheiro errado.
**Pré-condição**: `git status` limpo.

---

## Fase A — Encontrar a evidência real

```bash
grep -n '^pub(crate) fn \|^pub fn \|^fn ' 01_core/src/compiler/eval/bindings.rs
```

Para cada símbolo `pub(crate)` exportado de `bindings.rs` (candidatos prováveis, a
confirmar: `access`, `content_field`, `destructure_pattern`, `state_at_dispatch`,
`value_to_query_selector`, ou outro — não presumir qual, medir), contar usos reais fora do
próprio ficheiro:

```bash
for sym in <lista de símbolos pub(crate) encontrada acima>; do
  echo "--- $sym ---"
  grep -rn "\b$sym\b" 01_core 02_shell 03_infra 04_wiring \
    --include=*.rs | grep -v "compiler/eval/bindings.rs" | wc -l
done
```

Identificar qual símbolo (ou quais) realmente sustentam o fan-in alto atribuído no P1008,
com contagem correcta.

## Fase B — Corrigir o registo

Actualizar a linha da tabela do Passo 1008 (candidato 1, `compiler::eval::bindings`) com o
símbolo e contagem correctos. Não é preciso reescrever o relatório inteiro — um adendo que
substitui só essa célula, citado como correcção, com a evidência nova.

## Fase C — Confirmar que o veredicto não muda

Com o símbolo certo identificado: o veredicto "Prosseguir para P1002-completo" continua a
sustentar-se? (Espera-se que sim — `bindings.rs` tem conteúdo real de acesso/destructuring
usado amplamente dentro de `eval`, só a evidência citada estava errada — mas confirmar,
não presumir.)

---

## Resultado esperado

Tabela do Passo 1008 corrigida, com o símbolo real de `bindings.rs` e contagem verificada,
substituindo a referência incorrecta a `long_type_name`. Veredicto reconfirmado ou revisto
com base na evidência certa.
