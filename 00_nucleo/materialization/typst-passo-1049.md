# Passo 1049 — V16 Classe A: 84 projecções neutras/predicados

**Tipo**: Classificar e decidir caso a caso, mesma disciplina do P1041/P1046 — nenhum
`_ =>`/`other =>` se resolve por suprimir o lint. "Classe A" foi definida no P1045 como
`is_*`/`as_*`/`to_*`/predicados booleanos/comparação — mais simples por natureza do que a
Classe B, mas não presumir que "mais simples" significa "sem risco".
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1048.

---

## Lição a aplicar desde já (não repetir o processo de descoberta)

Do P1046: mesmo em classes "simples", pelo menos um caso pode estar ligado a um achado
antigo não resolvido (o item 19, ligado ao Achado C). Não presumir que os 84 são
homogéneos só por partilharem categoria sintáctica — verificar cada um contra o histórico
de achados desta frente antes de classificar como trivial.

Do P1041: a maioria dos casos vai ser genuinamente neutra (nenhum bug em 132 antes,
confirmado por amostra). Não é preciso tratar os 84 com a mesma suspeita profunda que a
Classe B — mas a amostra de verificação empírica continua obrigatória, não opcional.

## Fase 0 — Confirmar a contagem exacta

```
crystalline-lint --checks v16 . | grep -E '\bis_|_\bas_|_\bto_|partial_cmp'
```
(ajustar padrão conforme a saída real do linter) — confirmar 84, não presumir.

## Fase A — Classificar todos os 84 rapidamente (padrão, não profundidade)

Para cada caso: nome da função, ficheiro, o que o wildcard devolve por defeito. Produzir
tabela única — isto é dado mecânico, não precisa de investigação por item.

**Verificação cruzada obrigatória**: para cada um dos 84, confirmar se o nome do
ficheiro/função aparece nalgum achado já catalogado nesta frente (P987, P1024, P1026,
P1029, P1031, P998/Achado C, achado do offset de blocos do P1048, etc.) antes de
classificar como trivial. Um grep simples do nome do ficheiro contra os relatórios já
produzidos é suficiente — não é preciso reler tudo, só cruzar.

## Fase B — Amostra de verificação empírica

Escolher **8 dos 84** (não os mais fáceis) — priorizar:
1. Qualquer caso que a Fase A tenha ligado a um achado antigo.
2. Casos em `math/` ou `layout/` (áreas com mais bugs reais encontrados nesta frente).
3. Um ou dois casos aleatórios das restantes áreas, para não enviesar a amostra só para
   onde já se sabe que há risco.

Para cada um dos 8: `.typ` mínimo que force o caminho do wildcard, comparar output
cristalino vs vanilla directamente — mesmo padrão de todos os passos anteriores.

## Fase C — Anotar inline

Para os 84 (incluindo os 8 já verificados empiricamente e os restantes 76 por leitura de
domínio, mesma proporção de rigor já aceite no P1041): `other => <default> // neutro:
<razão>`, citando `file:line` do vanilla quando a razão for sobre paridade de
comportamento.

## Fase D — Validar

```
crystalline-lint --checks v16 .
cargo build --workspace --release
cargo test --workspace
```
Zero regressão nos casos confirmados neutros. Qualquer achado real dos 8 verificados
empiricamente que revele bug: não corrigir inline, escalar per gate `ADR-0127`, mesmo
tratamento do achado `cases()` que saiu da Classe B.

---

## Resultado esperado

84 casos classificados e anotados. Amostra de 8 com prova empírica real, não só leitura.
Qualquer ligação a achado antigo (Fase A) resolvida ou escalada, não deixada em silêncio.
Com este passo, V16 fica completo (DENY + neutros originais + Classe A + Classe B).
