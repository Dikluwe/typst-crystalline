---
# P772e — Reconfirmação de `lacuna-inventario` e decisão de continuidade da varredura

> **Passo:** 772e
> **Data:** 2026-07-16
> **Foco:** A série P765a→P772d cobriu `foundations::calc`, `foundations::ops`, `diag`, `math::style`, `image::raster`, `pdf::accessibility`, `syntax::span`, `syntax::package` — oito módulos, com 8 bugs reais corrigidos (título, símbolo, math style ×4, cross-file span, I/O span) e um número maior de itens correctamente classificados como mecânica/scope-out. Este passo reconta o que resta em `lacuna-inventario`, mede o rendimento até agora (bugs reais por módulo investigado), e decide — com dados, não por cansaço — se vale continuar.
> **Tipo:** Sonda + Decisão registada (regra 1).
> **Tamanho:** S.
> **ADR-0108 EM VIGOR** — decisão de continuar/parar baseada em dados, não em suposição sobre "rendimento decrescente".
> **Dependências:** P765a, P765b, P772, P772a, P772b, P772c, P772d (todos os lotes já fechados).

---

## Sonda — reconfirmar a lista restante

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn
```

Excluir todos os módulos já tratados (`foundations::calc`, `foundations::ops`, `diag`, `math::style`, `image::raster`, `pdf::accessibility`, `syntax::span`, `syntax::package`) e listar o que resta, com contagem por módulo — a lista completa, não só o topo.

### Calcular o rendimento até agora

| Módulo | Itens | Bugs reais encontrados | Taxa |
|---|---:|---:|---:|
| `foundations::calc`/`ops` | 65 | (preencher, de P765a) | |
| `diag` | 24 | 0 | |
| `math::style` | 32 | 4 | |
| `image::raster` | 13 | 2 (+ desdobramentos: DPI, rotação, clip, color space) | |
| `pdf::accessibility` | 12 | 0 | |
| `syntax::span` | 10 | 1 | |
| `syntax::package` | 8 | 1 (achado por verificação extra, não pela varredura directa) | |

Somar: total de itens varridos até agora, total de bugs reais, taxa global (bugs/item e bugs/módulo).

### Estimar o restante

Para os módulos que restam na lista (tipicamente cada vez menores, conforme já observado por P772/P772a), aplicar o mesmo tipo de julgamento rápido usado em `pdf::accessibility` (que teve 0 bugs) e `diag` (0 bugs): módulos de infra-estrutura Rust pura (nomes como `typst_utils`, `typst_syntax::reparser`, `foundations::scope`) têm probabilidade baixa de conter símbolos de língua, mas a lição de `image::raster` (13 itens, maioria infra, 2 bugs reais de alto impacto) mostra que tamanho pequeno não é garantia de zero risco.

---

## Decisão a registar (regra 1)

Com os dados acima, escolher entre:

| Opção | Quando escolher |
|---|---|
| **Continuar a varredura completa** | Se a taxa de bugs reais por módulo ainda for razoável (ex: >10-15% dos módulos investigados renderam pelo menos 1 bug real) e os módulos restantes não forem triviais em contagem total |
| **Continuar só para módulos com risco de efeito observável** | Se a maioria dos módulos restantes for claramente infra-estrutura Rust pura (parsing interno, reparser incremental, utils), mas alguns tiverem nomes que sugerem efeito observável (ex: mensagens de erro, formatos de exportação) |
| **Encerrar a varredura sistemática** | Se o total restante for pequeno e majoritariamente infra-estrutura, e o esforço por módulo (como visto em P772a-d, cada um exigindo leitura de código + testes) não se justificar face ao retorno esperado |

Registar a decisão com os números, não com "parece que já deu".

---

## Critério de fecho do passo

- [ ] Lista `lacuna-inventario` restante reconfirmada por contagem directa.
- [ ] Rendimento até agora calculado (itens varridos, bugs reais, taxa).
- [ ] Decisão registada com base nos números, entre as três opções acima.
- [ ] Se decidir continuar: identificar o próximo módulo e abrir P772f com o mesmo padrão.
- [ ] Se decidir encerrar: registar isso como fecho da série de varredura sistemática (P765a-P772e), com um resumo do que foi coberto e do que ficou fora por decisão consciente (não por esquecimento).
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772e.md`.

---

## Próximo passo

Conforme a decisão registada — P772f (próximo módulo) ou fecho da série.
