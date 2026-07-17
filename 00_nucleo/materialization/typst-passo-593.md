---
# P593 — Unificar a cascata de largura: letra → palavra → linha

> **Passo:** 593
> **Data:** 2026-07-05
> **Foco:** As correcções de P591 e P592 resolveram dois problemas distintos, cada um numa camada diferente do cálculo de largura — palavra (P591, letras ligadas vs separadas) e linha (P592, posição do cursor vs largura real do conteúdo). Cada correcção foi feita no sítio onde o sintoma apareceu, não como parte de uma conta única, reutilizada em todos os sítios. Este passo confirma se existe agora uma única fonte de verdade para cada nível — letra, palavra, linha — ou se ainda há mais do que uma versão da mesma conta espalhada pelo código, à espera de divergir outra vez.
> **Tipo:** Sonda + Consolidação.
> **Tamanho:** L. Toca todos os ficheiros já mexidos nesta sequência inteira (P544 a P592).
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — confirmar o estado real do código antes de decidir o que falta unificar.
> **Dependências:** Todo o histórico de P544 a P592 — cada passo corrigiu uma camada da mesma cascata, sem nenhum deles ter sido escrito com o objectivo de a unificar.

---

## Contexto

Ao longo desta sequência, o mesmo tipo de problema apareceu quatro vezes, em quatro sítios diferentes:

1. **P544/P546/P548:** a largura de palavra usada para decidir a quebra de linha não vinha da fonte real (`FixedMetrics` estático), depois foi corrigida para vir da fonte real, com cache.
2. **P591:** a largura de palavra, mesmo vindo da fonte real, não aplicava a forma de escrita (letras ligadas do árabe), só somava letras separadas.
3. **P587/P588:** o início de uma linha contava um espaço que não devia contar.
4. **P592:** o fim de uma linha, em RTL, usava a posição do cursor (que podia incluir um espaço sem desenho) em vez da largura real do último item desenhado.

Cada uma foi uma correcção local, no ficheiro onde o sintoma apareceu. Nenhuma foi acompanhada de uma pergunta mais ampla: existe hoje, no código, uma função só para "largura de uma letra", uma função só para "largura de uma palavra" (que chame a de letra, ou o mecanismo de shaping, consoante o script), e uma função só para "largura de uma linha" (que some larguras de palavra, não posições de cursor)? Ou continuam a existir várias versões paralelas desta conta, em `cursor.rs`, `layout_bidi.rs`, `font_metrics.rs`, `helpers.rs`, e nos ficheiros tocados por P579/P580 (`grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`)?

---

## Sonda

### Listar todos os sítios que calculam largura de texto, de alguma forma

```bash
grep -rn "fn.*width\|fn.*advance\|word_width\|line_width\|content_right\|advance_shaped" 01_core/src/ 03_infra/src/ --include="*.rs"
```

Para cada função encontrada, classificar:

1. Calcula largura de uma letra/glifo isolado?
2. Calcula largura de uma palavra (soma de letras, ou shaped)?
3. Calcula largura de uma linha/conteúdo (soma de palavras, ou posição de cursor)?
4. É chamada por quem, e onde?

### Confirmar duplicação

Para cada par de funções que pareçam fazer a mesma coisa em ficheiros diferentes (por exemplo, `word_width` em `cursor.rs` vs alguma coisa parecida em `layout_bidi.rs` ou `helpers.rs`), confirmar se calculam exactamente da mesma forma, ou se divergem em algum detalhe — foi exactamente essa divergência que causou os quatro problemas já corrigidos.

### Critério de fecho da sonda

- [ ] Lista completa de funções de cálculo de largura, classificadas por nível (letra, palavra, linha).
- [ ] Confirmado quais destas são únicas (uma só implementação, reutilizada) e quais são duplicadas (mais do que uma versão da mesma conta).
- [ ] Para cada duplicação encontrada: confirmado se as versões calculam o mesmo valor, ou se já divergem sem ninguém ter reparado ainda.

---

## Consolidação

Para cada duplicação confirmada onde as versões deviam ser a mesma conta: escolher uma implementação como a fonte de verdade, e fazer todas as outras chamarem essa, em vez de recalcular à parte.

A estrutura pretendida, de baixo para cima:

1. **Largura de glifo/letra** — uma função só, que consulta a fonte real (com cache, como já estabelecido em P548).
2. **Largura de palavra** — uma função só, que decide entre somar letras ou usar shaping (`advance_shaped`, já criado em P591) consoante o script, e é chamada por tudo o que precisa de largura de palavra — `layout_word`, `layout_bidi`, `grid`, `placement`, etc., não cada um com a sua cópia.
3. **Largura de linha/conteúdo** — uma função só, que soma larguras de palavra reais mais espaçamento, usando a posição real do último item desenhado (como corrigido em P592), não a posição do cursor.

### Critério de fecho da consolidação

- [ ] Uma função por nível, reutilizada em todo o código, sem cópias paralelas.
- [ ] Todos os ficheiros identificados na sonda como tendo versões próprias, actualizados para chamar a função única.
- [ ] Testes de regressão para cada um dos quatro casos já corrigidos (P544/546/548, P591, P587/588, P592), confirmando que continuam correctos depois da consolidação — não é para desfazer o que já está certo, é para garantir que fica certo por uma razão estrutural, não por coincidência de quatro correcções separadas.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

Repetir todos os documentos de referência já usados nesta sequência (P563, P566, P577, P586, P590, P591, P592), confirmando que nenhum regride depois da consolidação.

---

## Critério de fecho do passo

- [ ] Sonda completa — lista de funções de largura, classificadas, duplicações identificadas.
- [ ] Consolidação feita onde havia duplicação real.
- [ ] Os quatro casos já corrigidos re-testados e confirmados, depois da consolidação, não só antes.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] Sem regressão de desempenho medida.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p593.md`, com a lista completa de funções encontradas e o que foi feito a cada uma.

---

## Nota

Isto não garante que nunca mais apareça um problema parecido — pode haver um quinto nível ainda por descobrir (por exemplo, largura de parágrafo com várias linhas, ou largura de página com várias colunas). Mas reduz a hipótese de o mesmo tipo de erro se repetir por a mesma conta estar escrita duas vezes, de forma ligeiramente diferente, em dois sítios que ninguém olhou lado a lado até agora.
