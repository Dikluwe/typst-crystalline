---
# P772o — Investigação instrumentada: colapso de espaço em fontes variáveis de peso alto

> **Passo:** 772o
> **Data:** 2026-07-16
> **Foco:** P772m encontrou, fora do inventário nomeado, um bug real: em `Ubuntu Sans` (fonte variável, eixo `wght`), pesos ≥770 fazem palavras colarem sem espaço nenhum entre si (confirmado por inspeção visual, não é antialiasing). Não ocorre em `Noto Sans` (também variável, mesmo tipo de eixo). Causa suspeita, não confirmada: dois caminhos desacoplados de largura de glifo — texto normal via `rustybuzz` com variações ao vivo (`shaper.rs`), espaço via `ttf_parser` sobre uma `cached_face` que pode não refletir a instância correta do eixo `wght` pedido (`FallbackFontMetrics::advance`, `03_infra/src/font_metrics.rs:704-746`). Este passo instrumenta e confirma (ou refuta) essa hipótese antes de qualquer correção — P772m foi explícito que um patch às cegas aqui seria irresponsável.
> **Tipo:** Sonda instrumentada. Sem implementação, a menos que a causa seja confirmada com confiança suficiente para uma correção pequena e isolada.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — instrumentar e medir antes de decidir, exatamente como P772m pediu.
> **Dependências:** P772m (achado original, ambiente de fonte variável já configurado — `TYPST_CRYSTALLINE_PYTHON` + venv com `fonttools`).

---

## Passo 0 — Reproduzir com o ambiente já validado por P772m

```bash
# reconstituir o venv de sondagem, se não persistiu
python3 -m venv /tmp/p772o/venv
/tmp/p772o/venv/bin/pip install fonttools
export TYPST_CRYSTALLINE_PYTHON=/tmp/p772o/venv/bin/python3
```

```bash
cat > /tmp/p772o-test.typ <<'EOF'
#set text(font: "Ubuntu Sans", weight: 800)
Weight test here
EOF
./target/release/typst compile /tmp/p772o-test.typ /tmp/p772o-800.pdf
mutool draw -o /tmp/p772o-800.png -r 300 /tmp/p772o-800.pdf
```

Confirmar reprodução (deve mostrar "Weighttest here" ou similar, palavras coladas).

---

## Passo 1 — Instrumentar `space_width()` e o caminho de shaping em paralelo

Adicionar instrumentação temporária (`eprintln!`/`dbg!`, não código permanente) em:

```bash
grep -n "fn space_width\|fn advance\b" 01_core/src/rules/layout/cursor.rs 03_infra/src/font_metrics.rs
```

Registar, para o mesmo documento de teste (peso 800):
1. O valor devolvido por `space_width()` (caminho não-shapeado, via `FallbackFontMetrics::advance`).
2. Que face exatamente `cached_face` está a servir nesse momento — instanciada em que peso? Comparar o eixo `wght` efetivo da face cacheada com o peso pedido (800).
3. Para comparação, a largura que `rustybuzz` calcularia para um espaço no mesmo peso, se for possível invocar o mesmo caminho de shaping manualmente para esse glifo (ainda que não seja o caminho usado em produção).

```bash
grep -n "cached_face\|fn.*cache" 03_infra/src/font_metrics.rs
```

Confirmar exatamente quando `cached_face` é populada/invalidada — por peso, por família, ou de forma que ignore o eixo variável.

---

## Passo 2 — Isolar a variável: é o cache, é o `ttf_parser`, ou é outra coisa?

### Testar sem cache (se possível desativar temporariamente)

Se a hipótese do cache for confirmável de forma barata (ex: forçar recriação da face a cada chamada, só para teste), medir se o colapso desaparece. Se desaparecer, a causa está confirmada.

### Testar com outras fontes variáveis além das duas já testadas por P772m

```bash
# repetir o teste com pelo menos 2 fontes variáveis adicionais do corpus de testes
# (03_infra/fixtures/fonts/*.ttf — Cantarell-VF, outras)
```

Para determinar se o âmbito é "específico de Ubuntu Sans" (dados de `gvar` incomuns) ou mais geral — P772m deixou isso como pergunta em aberto.

---

## Passo 3 — Decisão

- **Se a causa for confirmada** (cache de face desatualizada ou qualquer outra causa isolável): decidir se a correção é pequena o suficiente para este mesmo passo (ex: invalidar/recriar `cached_face` por instância de eixo, não só por família/peso nominal) ou se precisa de L0 próprio (se mexer em estrutura de cache compartilhada com outros caminhos).
- **Se a causa não for isolável com confiança nesta rodada**: registrar o que foi descartado (não é X, não é Y) e o que ainda falta, sem forçar uma correção especulativa. Não é falha do passo — é resultado válido, como já aconteceu noutros passos desta conversa (P763h Parte B, por exemplo).

---

## Validação (se corrigido)

```bash
./target/release/typst compile /tmp/p772o-test.typ /tmp/p772o-800-depois.pdf
mutool draw -o /tmp/p772o-800-depois.png -r 300 /tmp/p772o-800-depois.pdf
```

Confirmar visualmente e por medição de largura de espaço que as palavras voltam a separar-se em todos os pesos testados (300-900), nas fontes onde o bug foi confirmado.

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar sem regressão nos testes de fonte variável já existentes (P666-669, P525, P530).

---

## Critério de fecho do passo

- [ ] Reprodução confirmada com o mesmo ambiente de P772m.
- [ ] `space_width()` e a face cacheada instrumentados e comparados com o caminho de shaping normal, para o mesmo peso.
- [ ] Causa confirmada ou explicitamente descartada por eliminação (cache vs outra causa), com evidência, não suposição.
- [ ] Testado em pelo menos 2 fontes variáveis além de Ubuntu Sans/Noto Sans, para determinar âmbito (isolado vs geral).
- [ ] Se corrigido: validação visual e numérica em todos os pesos testados, sem regressão em testes de fonte existentes.
- [ ] Se não corrigido: registrado o que foi descartado, para o próximo passo não repetir o mesmo caminho.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772o.md`.

---

## Próximo passo

Se corrigido: nenhuma dependência conhecida — item fecha isolado, conforme já avaliado (não destrava outras partes da migração).
Se não corrigido: registrar como item de investigação em aberto, com o que já foi eliminado, para retomar depois com mais dados ou mais tempo de instrumentação.
