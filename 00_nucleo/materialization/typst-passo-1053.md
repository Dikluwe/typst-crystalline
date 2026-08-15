# Passo 1053 — Auditoria de placeholders: constantes usadas para "fechar buraco"

**Tipo**: Desenho de método primeiro (Fase 0-A), depois execução. Terceira ocorrência
deste padrão nesta frente (`gap = style.size * 0.2` no P1042; `Ascent 800/Descent -200`
hardcoded no P1051) — deixa de ser coincidência, passa a merecer busca sistemática.
**Diferença deste caso para V16-V20**: não é uma regra de linter já escrita a produzir
uma lista — o método tem de ser desenhado primeiro, porque "placeholder para fechar
buraco" não tem assinatura sintáctica única (ao contrário de `_ =>` ou profundidade de
padrão). Isto é mais parecido com o desenho do `auditar-fatiamento.md` do que com correr
`crystalline-lint --checks vNN`.
**Pré-condição**: `git status` limpo.

---

## Fase 0 — Definir o que conta como "placeholder de fechar buraco", com exemplos reais

Não é qualquer constante — Rust tem constantes legítimas em todo o lado (`MAX_SHOW_RULE_DEPTH = 64`, por exemplo, é uma constante deliberada e documentada, não um placeholder). A diferença, com os dois casos reais já encontrados:

| Característica | `gap = style.size * 0.2` (P1042) | `Ascent 800` (P1051) | Constante legítima |
|---|---|---|---|
| Fonte do valor | Inventado/aproximado, não lido de nenhuma fonte de verdade | Genérico, não específico à fonte real | Decisão documentada, com razão |
| Varia por contexto real? | Sim (por fonte) — mas tratado como fixo | Sim (por fonte) — mas tratado como fixo | Não varia, ou varia e o código lê a variação |
| Proveniência no L0/comentário | Nenhuma, ou "parece certo" | Nenhuma | `file:line` do vanilla, ou razão explícita |

**Definição de trabalho para este passo**: um valor numérico/string hardcoded que **deveria** vir de uma fonte de verdade externa que varia por contexto (fonte activa, configuração do documento, tabela de especificação como OpenType MATH) mas está fixo no código como se fosse universal.

## Fase A — Estratégias de busca, combinadas (nenhuma sozinha é suficiente)

1. **Grep por padrões de valor numérico suspeito** em código de layout/export/fontes
   (áreas onde já encontrámos os dois casos reais):
   ```
   grep -rnE '\b(style\.size|font_size|em)\s*\*\s*0\.[0-9]+' 01_core/src/compiler/math 01_core/src/compiler/layout
   grep -rnE '/(Ascent|Descent|CapHeight|ItalicAngle|StemV)\s+[0-9]' 03_infra/src/export
   ```
2. **Cruzar com a emenda do P1042** — qualquer L0 de `math/layout/` que ainda não tenha
   sido revisto por essa regra (a emenda foi aplicada retroactivamente só a
   `cases.md`/`matrix.md` até agora; confirmar se mais L0s de math/layout têm o mesmo
   problema).
3. **Procurar por comentários que admitem aproximação** — `grep -rn 'aproxima\|approx\|
   TODO\|FIXME\|placeholder\|hardcod' 01_core 03_infra` — muitas vezes quem escreve um
   placeholder deixa rasto em comentário, mesmo sem seguir a regra de proveniência.
4. **Áreas de maior risco por histórico** — priorizar `03_infra/src/export/` (onde o
   `FontDescriptor` vivia) e `compiler/math/layout/` (onde `gap`/`padding` viviam) antes
   de expandir a outras áreas.

## Fase B — Classificar cada achado

Para cada valor suspeito encontrado:
1. Confirmar se devia vir de fonte externa (tabela de fonte, configuração do documento,
   spec OpenType) — se sim, é achado real.
2. Medir o impacto: comparar com vanilla usando um caso de teste que force uma fonte ou
   contexto diferente do "normal" (mesma técnica que revelou os dois casos anteriores —
   `Ascent`/`Descent` só divergiu porque testaram fonte real vs valor fixo).
3. Classificar por gravidade: afecta output visual directamente (gate `ADR-0127`) vs
   afecta só metadado (mesmo tratamento do `Ascent`/`Descent`, fluxo contínuo).

## Fase C — Não corrigir tudo de uma vez

Mesmo espírito do V16/V20: catalogar primeiro, corrigir por prioridade, não em bloco.
Se o volume for pequeno (dezenas), tratar neste passo. Se for grande, dimensionar como
leva própria (mesmo padrão da leva V16-V20).

## Fase D — Registar o método para o futuro

Com o que for aprendido aqui, expandir a emenda do P1042 (que hoje só fala de constantes
*geométricas/tipográficas*) para a categoria mais ampla de "valor que devia vir de fonte
externa variável, tratado como fixo" — cobrindo também casos como o `FontDescriptor`, que
não é geometria de layout mas é o mesmo padrão de erro.

---

## Resultado esperado

Um método reproduzível para encontrar este padrão (não só os dois casos já conhecidos),
aplicado uma primeira vez, com achados catalogados e priorizados. A emenda do P1042
generalizada para cobrir a classe inteira, não só o caso original que a motivou.
