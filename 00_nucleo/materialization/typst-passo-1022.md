# Passo 1022 — Fatiamento hub/nó `stdlib::text`

**Tipo**: Aplicação do método consolidado (`00_nucleo/prompts/auditar-fatiamento.md`) a
`compiler::stdlib::text` — último candidato da leva do Passo 1008, deixado de fora quando
`structural` foi tratado (P1014).
**Candidato confirmado no P1008**: agregado (sem `trait`), fan-in real de módulo (5
ficheiros distintos chamam as suas nativas — `eval/bindings.rs`, `eval/mod.rs`,
`eval/modules.rs`, `eval/rules.rs`, `stdlib/mod.rs`; `native_text`/`native_smallcaps` em
4 ficheiros cada, `native_underline`/`native_strike`/`native_overline`/`native_subscript`/
`native_superscript` em 3 cada; fan-in baixo do DSM é efeito da facade `stdlib/mod.rs`,
não ausência de consumo real).
**Pré-condição**: `git status` limpo. Pode correr em paralelo com o Passo 1021 (leitura
read-only do corpus) — sem sobreposição de ficheiros alterados; se `stdlib/text.md`
estiver a meio de reescrita quando o P1021 o ler, o catálogo desse ficheiro pode ficar
desactualizado, aceitável (é um caso entre ~275).

---

## Ler o método primeiro

`00_nucleo/prompts/auditar-fatiamento.md` — não repetir aqui o que já está consolidado
(ordem obrigatória: critério-zero → inventário genérico de visibilidade → critério 3
como decisor → critério 4 como hipótese → critério 2 em 3 classes → verificar órfãos →
materializar → validar → avaliar).

## Contexto específico deste ficheiro

`stdlib/text.rs`, 1212 linhas (per Passo 1004). Nativas conhecidas por amostra:
`native_text`, `native_upper`, `native_lower`, `native_replace`, `native_regex`,
`native_lorem`, decorações (`underline`/`strike`/`overline`/`subscript`/`superscript`),
`native_smallcaps`. **Confirmar lista completa na Fase A — a amostra do P1004 já se
mostrou incompleta duas vezes** (`structural.rs` tinha 50 funções, não a amostra de ~10;
`bindings.rs` tinha 43, não 23) — não presumir que `text.rs` é excepção.

**Vanilla**: `typst_library::text` (hub grande também no vanilla, per P1003 — fan-out 50,
maior até que o do cristalino). Isto já foi assinalado como sinal de que o ganho de
fatiar pode ser menor aqui do que nos casos anteriores — **confirmar com o critério 3,
não descartar o fatiamento só por causa deste sinal**, mas entrar sem expectativa de
grande divisão.

**Ligação a `Content::superscript`** (usado no Passo 1020, `footnote.rs`) — confirmar se
esse primitivo vive em `entities/content.rs` (como o relatório do P1020 disse) ou se há
alguma sobreposição com decorações de texto aqui. Não presumir relação, verificar.

## Fase A — Inventário completo

```
grep -nE '^(pub(\([a-z:) ]+\))? )?fn ' 01_core/src/compiler/stdlib/text.rs
```

## Fase B — Os 4 critérios, com evidência

1. **Critério-zero** — confirmar sem `trait` (já indicado no P1008, reconfirmar).
2. **Critério 2** (pureza, 3 classes) — hipótese: `native_regex`/`native_lorem` podem ser
   puras (`Str`/`Args` → `Value`); decorações (`underline` etc.) e `native_text` tocam
   `Engine`/estilo? Medir por `file:line`, não presumir a partir do domínio.
3. **Critério 3** (co-mudança) — usar `tools/analysis/cochange_metrics.py`, descontando
   ruído de resselo (já embutido na ferramenta desde P1013).
4. **Critério 4** (vanilla) — `typst_library::text` é hub também lá; não é garantido que
   exista fronteira interna clara a copiar. Se o critério 3 não revelar clusters nítidos,
   isso é um resultado válido (como P1006) — não forçar divisão sem sinal.

## Fase C — Verificar órfãos antes de escrever de raiz

Confirmar se existe algum prompt órfão remanescente relacionado com `text.rs` (nenhum
identificado no Passo 1001, mas reconfirmar — a lista desse passo tinha 10 itens, nenhum
sob `text`, mas vale grep rápido antes de assumir).

## Fase D — Materializar (só se o critério 3 sustentar divisão)

Se a evidência não sustentar corte claro: registar "não fatiar" como resultado válido
(precedente P1006), deixar `stdlib/text.md` como está, só actualizado se tiver referência
a passo por remover (Critério B, independente da decisão de fatiar).

Se sustentar: nós conforme decidido, L0 sem referência a passo, campo Técnica se
aplicável, ficheiro `.rs` próprio por nó (V15).

## Fase E — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão.

## Fase F — Avaliação do método

Actualizar a tabela de proveniência de `auditar-fatiamento.md` com esta quinta aplicação
— incluindo se o resultado foi "não fatiar" (reforça P1006) ou fatiamento real.

---

## Resultado esperado

`stdlib::text` tratado — fatiado com evidência, ou deixado como hub por decisão
justificada. Com este passo, a leva completa do Passo 1008 fica encerrada:
`rules`, `closures`, `bindings`, `structural`, `text`.
