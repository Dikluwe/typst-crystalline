---
# P674 — O custo fixo de arranque presente em todos os documentos micro

> **Passo:** 674
> **Data:** 2026-07-10
> **Foco:** Depois de P657-673 fecharem os gargalos de `shape_ms`, o `macro-10x` está a 1,26× do vanilla. Mas todos os documentos micro (34 de 34) continuam entre 1,5× e 2× — incluindo `test-array`, que mal faz nada. O vanilla demora ~90-110ms nestes casos; o cristalino demora sempre ~200ms, sem variar muito com o conteúdo. Isto sugere um custo fixo de arranque, presente em todos os documentos, não ligado ao trabalho de shaping já corrigido. Este passo localiza-o.
> **Tipo:** Sonda + Implementação, se confirmado.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P618/P657/P670-673 (histórico de medições, onde este padrão já aparecia mas nunca foi isolado do resto).

---

## Sonda

### Confirmar o custo com o documento mais simples possível

```bash
cat > /tmp/p674-vazio.typ <<'EOF'
a
EOF
hyperfine --warmup 5 --runs 20 './target/release/typst /tmp/p674-vazio.typ /tmp/p674.pdf'
hyperfine --warmup 5 --runs 20 'lab/typst-original/target/release/typst compile /tmp/p674-vazio.typ /tmp/p674-vanilla.pdf'
```

Com um documento de uma letra só, qualquer diferença de tempo restante é quase certamente custo de arranque do processo, não de processamento de conteúdo.

### Confirmar as fases com `--timings-json` neste documento mínimo

```bash
./target/release/typst /tmp/p674-vazio.typ /tmp/p674.pdf --timings-json /tmp/p674-timings.json
cat /tmp/p674-timings.json
```

Se a soma das fases (`parse_ms` + `eval_ms` + ... + `render_ms`) for muito menor do que o tempo total medido pelo `hyperfine`, confirma que o custo está **fora** das fases já instrumentadas — antes do parse, ou depois do render (arranque do processo, carregamento de recursos, etc.).

### Localizar o que acontece antes de `parse_ms` começar a contar

```bash
grep -n "fn main\|FontBook::\|scan_fonts\|load_fonts\|Instant::now" 04_wiring/src/main.rs 03_infra/src/pipeline.rs | head -30
```

Confirmar se há alguma operação a acontecer no arranque — por exemplo, escanear o sistema por todas as fontes instaladas, carregar um `FontBook` completo, ou inicializar alguma coisa pesada — antes de o relógio de `parse_ms` começar a contar.

### Medir separadamente com `strace -c` ou equivalente

```bash
strace -c ./target/release/typst /tmp/p674-vazio.typ /tmp/p674.pdf 2>&1 | tail -30
```

Confirmar quantas chamadas de sistema (leitura de ficheiros, por exemplo) acontecem, e se há um padrão de muitas leituras pequenas (sugerindo escaneamento do sistema de fontes) logo no arranque.

### Critério de fecho da sonda

- [ ] Tempo isolado com `hyperfine`, documento mínimo, comparado directamente com o vanilla.
- [ ] Confirmado se o custo está dentro ou fora das fases já instrumentadas.
- [ ] Localizado, com `file:line`, o que acontece no arranque antes do processamento do documento.
- [ ] Confirmado com `strace` se há um padrão de I/O (leitura de muitas fontes, por exemplo) a explicar o custo.

---

## Implementação, condicional ao resultado da sonda

Depende inteiramente do que a sonda encontrar. Se for escaneamento de fontes no arranque: considerar um cache persistente entre execuções (se a arquitectura permitir), ou escaneamento lazy (só carregar informação de fonte quando de facto necessário, não todas as fontes do sistema à partida).

---

## Validação

```bash
hyperfine --warmup 5 --runs 20 './target/release/typst /tmp/p674-vazio.typ /tmp/p674-depois.pdf'
```

Comparar com o tempo já medido do vanilla para o mesmo documento.

```bash
cargo test --workspace
crystalline-lint .
python3 tools/perf/benchmark-p507.py
```

---

## Critério de fecho do passo

- [ ] Sonda completa, custo de arranque isolado e localizado com números.
- [ ] Se corrigível: implementado, medido antes/depois.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Benchmark completo repetido, confirmando o efeito nos documentos micro.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p674.md`, com hash do commit.
