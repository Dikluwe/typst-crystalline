# Passo 974 — símbolo `√` usa altura fixa, não escala com o tamanho real do radicando

**Precede este passo**: achado da auditoria externa (v5, 2026-08-05) — medição por extensão de
tinta real (não bounding box de caractere): `√(a²+b²)` (radicando maior, dois termos + expoente)
mede 10.80pt de altura do símbolo no cristalino contra 12.96pt no vanilla; `³√x` (radicando menor,
um caractere) mede 10.80pt nos dois lados — **mesma altura fixa no cristalino nos dois casos**,
enquanto o vanilla escala (12.96pt vs 11.04pt) conforme o conteúdo. Isto substitui a leitura
anterior do achado 9.1 (que tratava "índice mal posicionado" e "traço encostando no radicando" como
dois bugs separados) — é uma causa única: o símbolo não recalcula altura a partir do radicando real.

**Índice de raiz (tamanho/posição) já corrigido por P970** — este passo é sobre a altura do próprio
símbolo `√`, não do índice.

**Pré-condição de árvore**: `git status`. Confirmar P970-973 presentes.

---

## Fase A — confirmar a causa

1. Reproduzir isoladamente `$ sqrt(a^2+b^2) $` e `$ root(3, x) $` (ou equivalente com radicando de
   uma letra só) e medir a altura real do símbolo `√` nos dois casos, no binário actual — confirmar
   que a altura sai igual nos dois (mesmo com radicandos de tamanho bem diferente).
2. Ler `root.rs` — confirmar como o alvo de altura para a seleção da variante/assembly do `√` é
   calculado a partir do radicando. Candidato a causa: o alvo pode estar a usar uma medida errada
   do radicando (por exemplo, `advance`/bounding box em vez de altura de tinta real — mesma classe
   de erro já vista em P945/952/957 com extents simétricos/errados), ou pode estar hardcoded/preso
   a um valor por defeito em vez de derivado do conteúdo real.
3. Ler a fórmula real do vanilla (`radical.rs`) para o alvo de altura do símbolo — já parcialmente
   lida em P901/P970 (convenção de baseline, kerns de índice) — confirmar especificamente o termo
   que decide a altura do próprio traço/símbolo a partir do radicando.
4. Confirmar se isto está relacionado ao achado já registado no adendo de P970
   (`radical_vertical_gap` vs `radical_display_style_vertical_gap` — constante errada em modo
   Display) — pode ser a mesma causa, ou uma causa adicional que se soma a essa.

## Fase B — Implementação (protocolo de dois agentes de P898 — geometria, afeta todo uso de `sqrt`/
`root` no documento)

1. Agente A escreve testes com pelo menos três radicandos de altura visivelmente diferente
   (um caractere simples; expressão com expoente; fração dentro da raiz, se suportado) — confirmar
   que a altura do símbolo escala proporcionalmente em todos, com os valores reais medidos contra
   o vanilla.
2. Agente B implementa a correção do alvo de altura.
3. Revisão do orquestrador — confirmar que a correção não quebra o caso já corrigido por P970
   (índice de raiz) nem introduz sobreposição em nenhum dos casos testados.
4. Suíte completa verde, discriminada por crate.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir os mesmos dois casos da auditoria (`√(a²+b²)` e `³√x`) — confirmar altura escalando
   (12.96pt/11.04pt ou próximo, não mais 10.80pt fixo nos dois).
2. `compare.py` nas seções com raiz (1, 13, 14). Confirmação visual a alta resolução, sem
   sobreposição entre traço e radicando em nenhum caso.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta confirmada: alvo de altura do `√` mal calculado a partir do radicando, ou constante
  errada (possivelmente ligada ao achado já registado em P970).
- Altura do símbolo escalando corretamente com o radicando, testado em pelo menos três tamanhos.
- Benchmark sem regressão.
